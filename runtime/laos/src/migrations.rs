// Copyright 2023-2024 Freeverse.io
// This file is part of LAOS.

// LAOS is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// LAOS is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with LAOS.  If not, see <http://www.gnu.org/licenses/>.

use crate::{Runtime, Weight};
use frame_support::{
	traits::{Currency, OnRuntimeUpgrade},
	BoundedVec,
};
use frame_system::pallet_prelude::BlockNumberFor;
use pallet_vesting::MaxVestingSchedulesGet;

pub type Migrations = (
	cumulus_pallet_xcmp_queue::migration::v4::MigrationToV4<Runtime>,
	VestingBlockTimeMigrationTo6Sec,
);

pub struct VestingBlockTimeMigrationTo6Sec;

type BalanceOf<T> = <<T as pallet_vesting::Config>::Currency as Currency<
	<T as frame_system::Config>::AccountId,
>>::Balance;

impl OnRuntimeUpgrade for VestingBlockTimeMigrationTo6Sec {
	fn on_runtime_upgrade() -> Weight {
		let mut reads = 0u64;
		let mut writes = 0u64;

		// Drain existing vesting schedules
		for (account_id, mut schedules) in pallet_vesting::Vesting::<Runtime>::drain() {
			reads += 1;

			// Create a new set of schedules with adjusted starting blocks
			let mut new_schedules: BoundedVec<
				pallet_vesting::VestingInfo<BalanceOf<Runtime>, BlockNumberFor<Runtime>>,
				MaxVestingSchedulesGet<Runtime>,
			> = BoundedVec::new();

			for schedule in &mut schedules {
				// Adjust starting block and period for the 6-second block time
				let adjusted_schedule = pallet_vesting::VestingInfo::new(
					schedule.locked(),
					schedule.per_block(),
					schedule.starting_block().saturating_mul(2u32.into()),
				);

				// Attempt to add adjusted schedule, handling potential overflow gracefully
				if new_schedules.try_push(adjusted_schedule).is_err() {
					log::warn!("Failed to push vesting schedule for account {:?}", account_id);
				}
			}

			// Update storage with the new schedules
			pallet_vesting::Vesting::<Runtime>::insert(&account_id, &new_schedules);
			writes += 1;
		}

		// Calculate weight based on database operations
		<Runtime as frame_system::Config>::DbWeight::get().reads_writes(reads + 1, writes + 1)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{
		currency::UNIT, tests::ExtBuilder, AccountId, Balance, Balances, BlockNumber,
		RuntimeOrigin, Vesting,
	};
	use frame_support::{assert_noop, assert_ok};
	use sp_runtime::traits::Saturating;
	use std::str::FromStr;

	pub(crate) const ALICE: &str = "0xf24FF3a9CF04c71Dbc94D0b566f7A27B94566cac";
	pub(crate) const BOB: &str = "0x6c2b9c9b5007740e52d80dddb8e197b0c844f239";

	#[test]
	fn migration_adjusts_starting_block_correctly() {
		ExtBuilder::default().build().execute_with(|| {
			let alice = AccountId::from_str(ALICE).unwrap();
			assert_ok!(Balances::force_set_balance(
				RuntimeOrigin::root(),
				alice.clone(),
				10000 * UNIT
			));

			let bob = AccountId::from_str(BOB).unwrap();
			let locked = 1000 * UNIT;
			let per_block = 1 * UNIT;
			let starting_block = 10;

			let schedule = pallet_vesting::VestingInfo::new(locked, per_block, starting_block);

			assert_ok!(Vesting::vested_transfer(RuntimeOrigin::signed(alice), bob, schedule));

			// execute the migration
			assert_eq!(
				VestingBlockTimeMigrationTo6Sec::on_runtime_upgrade(),
				Weight::from_parts(250000000, 0)
			);

			// check that the schedule has been adjusted
			let schedules = pallet_vesting::Vesting::<Runtime>::get(&bob).unwrap();
			assert_eq!(schedules.len(), 1);
			assert_eq!(schedules[0].starting_block(), 20);
			assert_eq!(schedules[0].locked(), locked);
			assert_eq!(schedules[0].per_block(), per_block);
		});
	}
}
