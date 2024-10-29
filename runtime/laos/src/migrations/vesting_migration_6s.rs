use crate::{Runtime, Weight};
use frame_support::{
	traits::{Currency, OnRuntimeUpgrade},
	BoundedVec,
};
use frame_system::pallet_prelude::BlockNumberFor;
use pallet_vesting::MaxVestingSchedulesGet;

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
	use crate::{currency::UNIT, tests::ExtBuilder, AccountId, Balances, RuntimeOrigin, Vesting};
	use frame_support::assert_ok;
	use sp_core::Get;
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

			// check current block
			assert_eq!(frame_system::Pallet::<Runtime>::block_number(), 0);

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

	#[test]
	fn migration_handles_multiple_schedules_correctly() {
		ExtBuilder::default().build().execute_with(|| {
			let alice = AccountId::from_str(ALICE).unwrap();
			assert_ok!(Balances::force_set_balance(
				RuntimeOrigin::root(),
				alice.clone(),
				20000 * UNIT
			));

			let bob = AccountId::from_str(BOB).unwrap();
			let locked1 = 1000 * UNIT;
			let per_block1 = 1 * UNIT;
			let starting_block1 = 10;

			let locked2 = 2000 * UNIT;
			let per_block2 = 2 * UNIT;
			let starting_block2 = 15;

			let schedule1 = pallet_vesting::VestingInfo::new(locked1, per_block1, starting_block1);
			let schedule2 = pallet_vesting::VestingInfo::new(locked2, per_block2, starting_block2);

			assert_ok!(Vesting::vested_transfer(
				RuntimeOrigin::signed(alice.clone()),
				bob.clone(),
				schedule1
			));
			assert_ok!(Vesting::vested_transfer(
				RuntimeOrigin::signed(alice),
				bob.clone(),
				schedule2
			));

			// execute the migration
			VestingBlockTimeMigrationTo6Sec::on_runtime_upgrade();

			// check that both schedules have been adjusted
			let schedules = pallet_vesting::Vesting::<Runtime>::get(&bob).unwrap();
			assert_eq!(schedules.len(), 2);
			assert_eq!(schedules[0].starting_block(), 20);
			assert_eq!(schedules[1].starting_block(), 30);
		});
	}

	#[test]
	fn migration_handles_empty_schedules() {
		ExtBuilder::default().build().execute_with(|| {
			let bob = AccountId::from_str(BOB).unwrap();

			// There are no vesting schedules initially
			assert!(pallet_vesting::Vesting::<Runtime>::get(&bob).is_none());

			// execute the migration
			VestingBlockTimeMigrationTo6Sec::on_runtime_upgrade();

			// There should still be no vesting schedules
			assert!(pallet_vesting::Vesting::<Runtime>::get(&bob).is_none());
		});
	}

	#[test]
	fn migration_handles_schedule_overflow_gracefully() {
		ExtBuilder::default().build().execute_with(|| {
			let alice = AccountId::from_str(ALICE).unwrap();
			assert_ok!(Balances::force_set_balance(
				RuntimeOrigin::root(),
				alice.clone(),
				50000 * UNIT
			));

			let bob = AccountId::from_str(BOB).unwrap();
			let locked = 1000 * UNIT;
			let per_block = 1 * UNIT;
			let starting_block = 10;

			// Attempt to create more schedules than allowed by MaxVestingSchedulesGet
			for _ in 0..(MaxVestingSchedulesGet::<Runtime>::get() + 1) {
				let schedule = pallet_vesting::VestingInfo::new(locked, per_block, starting_block);
				let _ = Vesting::vested_transfer(
					RuntimeOrigin::signed(alice.clone()),
					bob.clone(),
					schedule,
				);
			}

			// execute the migration
			VestingBlockTimeMigrationTo6Sec::on_runtime_upgrade();

			// check that the number of schedules does not exceed the maximum allowed
			let schedules = pallet_vesting::Vesting::<Runtime>::get(&bob).unwrap();
			assert!(schedules.len() <= MaxVestingSchedulesGet::<Runtime>::get() as usize);
		});
	}
}
