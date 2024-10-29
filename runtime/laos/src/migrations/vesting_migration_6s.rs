use crate::{Runtime, Weight};
use frame_support::{
	traits::{Currency, OnRuntimeUpgrade},
	BoundedVec,
};
use frame_system::pallet_prelude::BlockNumberFor;
use pallet_vesting::MaxVestingSchedulesGet;

pub struct VestingMigrationTo6SecBlockTime;

// Type alias for Balance to improve readability in function signatures and types
type BalanceOf<T> = <<T as pallet_vesting::Config>::Currency as Currency<
	<T as frame_system::Config>::AccountId,
>>::Balance;

impl OnRuntimeUpgrade for VestingMigrationTo6SecBlockTime {
	#[cfg(feature = "try-runtime")]
	/// Logs the total number of accounts with vesting schedules, the count of schedules per
	/// account, and the maximum schedule count per account before migration.
	fn pre_upgrade() -> Result<Vec<u8>, DispatchError> {
		let vesting_accounts_count = pallet_vesting::Vesting::<Runtime>::iter().count();
		let mut schedules_per_account = vec![];
		let mut max_schedule_count = 0;

		for (account_id, schedules) in pallet_vesting::Vesting::<Runtime>::iter() {
			let schedule_count = schedules.len();
			schedules_per_account.push((account_id.clone(), schedule_count as u32));
			if schedule_count > max_schedule_count {
				max_schedule_count = schedule_count;
			}
			log::info!(
				target: "runtime::migration",
				"Pre-migration: Account {:?} has {} vesting schedules",
				account_id,
				schedule_count
			);
		}

		log::info!(
			target: "runtime::migration",
			"VestingMigrationTo6SecBlockTime::pre_upgrade - Found {} accounts with vesting schedules, \
			 maximum schedules per account: {}",
			vesting_accounts_count,
			max_schedule_count
		);

		// Encode the number of accounts, schedule counts per account, and max schedule count
		Ok((vesting_accounts_count as u32, schedules_per_account, max_schedule_count as u32)
			.encode())
	}

	fn on_runtime_upgrade() -> Weight {
		let mut read_count = 0u64;
		let mut write_count = 0u64;

		for (account_id, schedules) in pallet_vesting::Vesting::<Runtime>::drain() {
			read_count += 1;

			// Call the separate logic function to adjust schedules
			let adjusted_schedules = adjust_schedule(
				frame_system::Pallet::<Runtime>::block_number(),
				schedules.to_vec(),
			);

			// Insert the adjusted schedules back into storage
			pallet_vesting::Vesting::<Runtime>::insert(&account_id, adjusted_schedules);
			write_count += 1;
		}

		<Runtime as frame_system::Config>::DbWeight::get()
			.reads_writes(read_count + 1, write_count + 1)
	}

	#[cfg(feature = "try-runtime")]
	/// Verifies migration by checking the vesting data has been migrated, allows for possible
	/// splits in schedules.
	fn post_upgrade(encoded_data: Vec<u8>) -> Result<(), DispatchError> {
		let (old_account_count, schedules_per_account, old_max_schedule_count): (
			u32,
			Vec<(AccountId, u32)>,
			u32,
		) = Decode::decode(&mut &encoded_data[..])
			.map_err(|_| DispatchError::Other("Failed to decode migration data"))?;

		let new_account_count = pallet_vesting::Vesting::<Runtime>::iter().count();
		let mut max_schedule_count_post_migration = 0;

		assert_eq!(
			new_account_count, old_account_count,
			"Mismatch in vesting account count after migration: expected {}, got {}",
			old_account_count, new_account_count
		);

		log::info!(
			target: "runtime::migration",
			"VestingMigrationTo6SecBlockTime::post_upgrade - Migration successful. \
			 Account count before: {}, after: {}",
			old_account_count,
			new_account_count
		);

		// Verify each account has at least the expected number of schedules; additional schedules
		// are allowed due to splitting.
		for (account_id, expected_schedule_count) in schedules_per_account {
			let new_schedule_count = pallet_vesting::Vesting::<Runtime>::get(&account_id)
				.map(|schedules| schedules.len())
				.unwrap_or(0);

			// Update max schedule count after migration
			if new_schedule_count > max_schedule_count_post_migration {
				max_schedule_count_post_migration = new_schedule_count;
			}

			// Log any increase in schedule count due to migration adjustments
			if new_schedule_count > expected_schedule_count as usize {
				log::info!(
					target: "runtime::migration",
					"Post-migration: Account {:?} had {} schedules pre-migration, now has {} schedules due to splits",
					account_id,
					expected_schedule_count,
					new_schedule_count
				);
			} else {
				assert_eq!(
                    new_schedule_count, expected_schedule_count as usize,
                    "Account {:?} schedule count mismatch: expected {} schedules after migration, found {}",
                    account_id, expected_schedule_count, new_schedule_count
                );
			}
		}

		// Log the maximum schedule count post-migration, even if it may have increased due to
		// schedule splits.
		log::info!(
			target: "runtime::migration",
			"VestingMigrationTo6SecBlockTime::post_upgrade - Max schedules per account pre-migration: {}, \
			 max schedules per account post-migration: {}",
			old_max_schedule_count,
			max_schedule_count_post_migration
		);

		Ok(())
	}
}
/// Adjusts vesting schedules for a new 6-second block time and handles various cases like schedule
/// splitting.
///
/// # Parameters
/// - `current_block`: The current block number at which the migration is taking place.
/// - `schedules`: A vector of vesting schedules to be adjusted.
///
/// # Returns
/// A bounded vector of updated vesting schedules that respect the new 6-second block time.
fn adjust_schedule(
	current_block: BlockNumberFor<Runtime>,
	schedules: Vec<pallet_vesting::VestingInfo<BalanceOf<Runtime>, BlockNumberFor<Runtime>>>,
) -> BoundedVec<
	pallet_vesting::VestingInfo<BalanceOf<Runtime>, BlockNumberFor<Runtime>>,
	MaxVestingSchedulesGet<Runtime>,
> {
	let mut adjusted_schedules = BoundedVec::new();

	for schedule in schedules {
		if current_block <= schedule.starting_block() {
			// Case 1: Current block is before or at the starting block, so adjust the schedule for
			// 6-second block time.
			let new_schedule = adjust_future_schedule(schedule, current_block);
			try_push_schedule(&mut adjusted_schedules, new_schedule);
		} else if remaining_vesting(schedule.clone(), current_block) > 0 {
			// Case 2: Current block is within the schedule range, so split it into past and new
			// vesting schedules.
			let (past_schedule, new_schedule) = split_schedule(schedule, current_block);
			try_push_schedule(&mut adjusted_schedules, past_schedule);
			try_push_schedule(&mut adjusted_schedules, new_schedule);
		} else {
			// Case 3: Current block is beyond the end of the schedule; add the schedule as-is.
			try_push_schedule(&mut adjusted_schedules, schedule);
		}
	}
	adjusted_schedules
}

/// Adjusts a vesting schedule when the current block is before or at the schedule's start,
/// halving the per-block rate for 6-second block time and recalculating the starting block.
///
/// # Parameters
/// - `schedule`: The original vesting schedule.
/// - `current_block`: The current block number.
///
/// # Returns
/// A new vesting schedule with adjusted starting block and per-block rate.
fn adjust_future_schedule(
	schedule: pallet_vesting::VestingInfo<BalanceOf<Runtime>, BlockNumberFor<Runtime>>,
	current_block: BlockNumberFor<Runtime>,
) -> pallet_vesting::VestingInfo<BalanceOf<Runtime>, BlockNumberFor<Runtime>> {
	let adjusted_starting_block = 2 * schedule.starting_block() - current_block;
	let adjusted_per_block = schedule.per_block().saturating_div(2u32.into());
	pallet_vesting::VestingInfo::new(schedule.locked(), adjusted_per_block, adjusted_starting_block)
}

/// Splits a vesting schedule into two parts: a past vesting schedule covering blocks
/// before `current_block`, and a new vesting schedule starting at `current_block`.
///
/// # Parameters
/// - `schedule`: The original vesting schedule to be split.
/// - `current_block`: The current block number, marking the split point.
///
/// # Returns
/// A tuple with:
/// - `past_schedule`: Covers vesting before `current_block`.
/// - `new_schedule`: Covers vesting from `current_block`.
fn split_schedule(
	schedule: pallet_vesting::VestingInfo<BalanceOf<Runtime>, BlockNumberFor<Runtime>>,
	current_block: BlockNumberFor<Runtime>,
) -> (
	pallet_vesting::VestingInfo<BalanceOf<Runtime>, BlockNumberFor<Runtime>>,
	pallet_vesting::VestingInfo<BalanceOf<Runtime>, BlockNumberFor<Runtime>>,
) {
	let vested_amount =
		schedule.per_block() * (current_block as u128 - schedule.starting_block() as u128);
	let remaining_amount = schedule.locked() - vested_amount;

	let past_schedule = pallet_vesting::VestingInfo::new(
		vested_amount,
		schedule.per_block(),
		schedule.starting_block(),
	);

	let new_per_block = schedule.per_block().saturating_div(2u32.into());
	let new_schedule =
		pallet_vesting::VestingInfo::new(remaining_amount, new_per_block, current_block);

	(past_schedule, new_schedule)
}

/// Calculates the remaining vesting amount for a schedule from the current block
///
/// # Parameters
/// - `schedule`: The vesting schedule to check.
/// - `current_block`: The current block number.
///
/// # Returns
/// The remaining amount of tokens to vest after the current block.
fn remaining_vesting(
	schedule: pallet_vesting::VestingInfo<BalanceOf<Runtime>, BlockNumberFor<Runtime>>,
	current_block: BlockNumberFor<Runtime>,
) -> BalanceOf<Runtime> {
	if current_block > schedule.starting_block() {
		let elapsed_blocks = current_block as u128 - schedule.starting_block() as u128;
		schedule.locked().saturating_sub(schedule.per_block() * elapsed_blocks)
	} else {
		schedule.locked()
	}
}

/// Attempts to push a schedule to the bounded vector and logs a warning if unsuccessful.
///
/// # Parameters
/// - `schedules`: The bounded vector to which the schedule should be added.
/// - `schedule`: The vesting schedule to add.
fn try_push_schedule(
	schedules: &mut BoundedVec<
		pallet_vesting::VestingInfo<BalanceOf<Runtime>, BlockNumberFor<Runtime>>,
		MaxVestingSchedulesGet<Runtime>,
	>,
	schedule: pallet_vesting::VestingInfo<BalanceOf<Runtime>, BlockNumberFor<Runtime>>,
) {
	if schedules.try_push(schedule).is_err() {
		log::warn!("Failed to push vesting schedule due to bounded vector limit");
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

	/// Tests that migration correctly updates vesting schedule
	/// when `current_block` is at block 0 and the schedule requires full adjustment.
	#[test]
	fn migrate_updates_schedule_with_adjusted_starting_block_and_per_block_rate() {
		ExtBuilder::default().build().execute_with(|| {
			let alice = setup_account(ALICE, 10000 * UNIT);
			let bob = setup_account(BOB, 0);

			assert_eq!(frame_system::Pallet::<Runtime>::block_number(), 0);

			let vesting_schedule = pallet_vesting::VestingInfo::new(1000 * UNIT, 2 * UNIT, 10);
			assert_ok!(Vesting::vested_transfer(
				RuntimeOrigin::signed(alice.clone()),
				bob.clone(),
				vesting_schedule
			));

			assert_eq!(
				VestingMigrationTo6SecBlockTime::on_runtime_upgrade(),
				Weight::from_parts(250000000, 0)
			);

			let schedules = pallet_vesting::Vesting::<Runtime>::get(&bob).unwrap();
			assert_eq!(schedules.len(), 1);
			assert_eq!(schedules[0].starting_block(), 20);
			assert_eq!(schedules[0].locked(), 1000 * UNIT);
			assert_eq!(schedules[0].per_block(), 1 * UNIT);
		});
	}

	/// Tests that migration adjusts future schedules when `current_block` is before the starting
	/// block.
	#[test]
	fn migrate_adjusts_future_schedule_when_current_block_is_before_starting_block() {
		ExtBuilder::default().build().execute_with(|| {
			let alice = setup_account(ALICE, 10000 * UNIT);
			let bob = setup_account(BOB, 0);

			let vesting_schedule = pallet_vesting::VestingInfo::new(1000 * UNIT, 2 * UNIT, 10);
			assert_ok!(Vesting::vested_transfer(
				RuntimeOrigin::signed(alice.clone()),
				bob.clone(),
				vesting_schedule
			));

			frame_system::Pallet::<Runtime>::set_block_number(5);

			assert_eq!(
				VestingMigrationTo6SecBlockTime::on_runtime_upgrade(),
				Weight::from_parts(250000000, 0)
			);

			let schedules = pallet_vesting::Vesting::<Runtime>::get(&bob).unwrap();
			assert_eq!(schedules.len(), 1);
			assert_eq!(schedules[0].starting_block(), 15);
			assert_eq!(schedules[0].locked(), 1000 * UNIT);
			assert_eq!(schedules[0].per_block(), 1 * UNIT);
		});
	}

	/// Tests that no adjustment is made when `current_block` equals the schedule's starting block.
	#[test]
	fn migrate_keeps_schedule_unchanged_when_current_block_matches_starting_block() {
		ExtBuilder::default().build().execute_with(|| {
			let alice = setup_account(ALICE, 10000 * UNIT);
			let bob = setup_account(BOB, 0);

			let vesting_schedule = pallet_vesting::VestingInfo::new(1000 * UNIT, 2 * UNIT, 10);
			assert_ok!(Vesting::vested_transfer(
				RuntimeOrigin::signed(alice.clone()),
				bob.clone(),
				vesting_schedule
			));

			frame_system::Pallet::<Runtime>::set_block_number(10);

			assert_eq!(
				VestingMigrationTo6SecBlockTime::on_runtime_upgrade(),
				Weight::from_parts(250000000, 0)
			);

			let schedules = pallet_vesting::Vesting::<Runtime>::get(&bob).unwrap();
			assert_eq!(schedules.len(), 1);
			assert_eq!(schedules[0].starting_block(), 10);
			assert_eq!(schedules[0].locked(), 1000 * UNIT);
			assert_eq!(schedules[0].per_block(), 1 * UNIT);
		});
	}

	/// Tests that the schedule is split into two parts if `current_block` is within vesting range.
	#[test]
	fn migrate_splits_schedule_when_current_block_is_within_schedule_range() {
		ExtBuilder::default().build().execute_with(|| {
			let alice = setup_account(ALICE, 10000 * UNIT);
			let bob = setup_account(BOB, 0);

			let vesting_schedule = pallet_vesting::VestingInfo::new(1000 * UNIT, 2 * UNIT, 10);
			assert_ok!(Vesting::vested_transfer(
				RuntimeOrigin::signed(alice.clone()),
				bob.clone(),
				vesting_schedule
			));

			frame_system::Pallet::<Runtime>::set_block_number(15);

			assert_eq!(
				VestingMigrationTo6SecBlockTime::on_runtime_upgrade(),
				Weight::from_parts(250000000, 0)
			);

			let schedules = pallet_vesting::Vesting::<Runtime>::get(&bob).unwrap();
			assert_eq!(schedules.len(), 2);
			assert_eq!(schedules[0].starting_block(), 10);
			assert_eq!(schedules[0].locked(), 10 * UNIT);
			assert_eq!(schedules[0].per_block(), 2 * UNIT);
			assert_eq!(schedules[1].starting_block(), 15);
			assert_eq!(schedules[1].locked(), 990 * UNIT);
			assert_eq!(schedules[1].per_block(), 1 * UNIT);
		});
	}

	/// Tests that no migration changes are made to schedules already completed by `current_block`.
	#[test]
	fn migrate_keeps_expired_schedule_unchanged() {
		ExtBuilder::default().build().execute_with(|| {
			let alice = setup_account(ALICE, 10000 * UNIT);
			let bob = setup_account(BOB, 0);

			let vesting_schedule = pallet_vesting::VestingInfo::new(1000 * UNIT, 2 * UNIT, 10);
			assert_ok!(Vesting::vested_transfer(
				RuntimeOrigin::signed(alice.clone()),
				bob.clone(),
				vesting_schedule
			));

			frame_system::Pallet::<Runtime>::set_block_number(5000);
			assert_eq!(
				VestingMigrationTo6SecBlockTime::on_runtime_upgrade(),
				Weight::from_parts(250000000, 0)
			);

			let schedules = pallet_vesting::Vesting::<Runtime>::get(&bob).unwrap();
			assert_eq!(schedules.len(), 1);
			assert_eq!(schedules[0].starting_block(), 10);
			assert_eq!(schedules[0].locked(), 1000 * UNIT);
			assert_eq!(schedules[0].per_block(), 2 * UNIT);
		});
	}

	/// Tests that the migration properly adjusts multiple schedules for a single account.
	#[test]
	fn migrate_adjusts_multiple_schedules_for_single_account() {
		ExtBuilder::default().build().execute_with(|| {
			let alice = setup_account(ALICE, 20000 * UNIT);
			let bob = AccountId::from_str(BOB).unwrap();

			let schedule1 = pallet_vesting::VestingInfo::new(1000 * UNIT, 1 * UNIT, 10);
			let schedule2 = pallet_vesting::VestingInfo::new(2000 * UNIT, 2 * UNIT, 15);

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

			assert_eq!(
				VestingMigrationTo6SecBlockTime::on_runtime_upgrade(),
				Weight::from_parts(250000000, 0)
			);

			let schedules = pallet_vesting::Vesting::<Runtime>::get(&bob).unwrap();
			assert_eq!(schedules.len(), 2);
			assert_eq!(schedules[0].starting_block(), 20);
			assert_eq!(schedules[1].starting_block(), 30);
		});
	}

	/// Tests migration behavior when no schedules are present for an account.
	#[test]
	fn migrate_handles_no_schedules_gracefully() {
		ExtBuilder::default().build().execute_with(|| {
			let bob = AccountId::from_str(BOB).unwrap();

			assert!(pallet_vesting::Vesting::<Runtime>::get(&bob).is_none());

			assert_eq!(
				VestingMigrationTo6SecBlockTime::on_runtime_upgrade(),
				Weight::from_parts(125000000, 0)
			);

			assert!(pallet_vesting::Vesting::<Runtime>::get(&bob).is_none());
		});
	}

	/// Tests that migration does not exceed the allowed maximum number of schedules
	/// (`MaxVestingSchedulesGet`).
	#[test]
	fn migrate_respects_max_schedule_limit() {
		ExtBuilder::default().build().execute_with(|| {
			let alice = setup_account(ALICE, 5000000000 * UNIT);
			let bob = AccountId::from_str(BOB).unwrap();

			let vesting_schedule = pallet_vesting::VestingInfo::new(1000 * UNIT, 1 * UNIT, 10);

			// Attempt to create more schedules than allowed by MaxVestingSchedulesGet
			for _ in 0..(MaxVestingSchedulesGet::<Runtime>::get() + 1) {
				let _ = Vesting::vested_transfer(
					RuntimeOrigin::signed(alice.clone()),
					bob.clone(),
					vesting_schedule.clone(),
				);
			}

			frame_system::Pallet::<Runtime>::set_block_number(15);

			// Execute the migration
			assert_eq!(
				VestingMigrationTo6SecBlockTime::on_runtime_upgrade(),
				Weight::from_parts(250000000, 0)
			);

			// Check that the number of schedules does not exceed the maximum allowed
			let schedules = pallet_vesting::Vesting::<Runtime>::get(&bob).unwrap();
			assert!(schedules.len() <= MaxVestingSchedulesGet::<Runtime>::get() as usize);
		});
	}

	/// Helper function to set up an account with a given balance.
	fn setup_account(account_str: &str, balance: u128) -> AccountId {
		let account = AccountId::from_str(account_str).unwrap();
		assert_ok!(Balances::force_set_balance(RuntimeOrigin::root(), account.clone(), balance));
		account
	}
}
