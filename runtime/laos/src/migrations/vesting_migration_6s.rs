use crate::{AccountId, Runtime, Weight};
use frame_support::{
	traits::{Currency, OnRuntimeUpgrade},
	BoundedVec,
};
use frame_system::pallet_prelude::BlockNumberFor;
use pallet_vesting::MaxVestingSchedulesGet;
use parity_scale_codec::{Decode, Encode};
use sp_core::{ConstU32, Get};
use sp_runtime::DispatchError;
use sp_std::{vec, vec::Vec};

pub struct VestingMigrationTo6SecBlockTime;

const LAOS_VESTING_MIGRATION_6S: &[u8] = b":laos:vesting_migration_6s:";

// Type alias for Balance to improve readability in function signatures and types
type BalanceOf<T> = <<T as pallet_vesting::Config>::Currency as Currency<
	<T as frame_system::Config>::AccountId,
>>::Balance;

type VestingSchedule = pallet_vesting::VestingInfo<BalanceOf<Runtime>, BlockNumberFor<Runtime>>;

impl OnRuntimeUpgrade for VestingMigrationTo6SecBlockTime {
	#[cfg(feature = "try-runtime")]
	/// Logs the total number of accounts with vesting schedules, the count of schedules per
	/// account, and the maximum schedule count per account before migration.
	fn pre_upgrade() -> Result<Vec<u8>, DispatchError> {
		let vesting_accounts_count = pallet_vesting::Vesting::<Runtime>::iter().count();
		let mut schedules_per_account = vec![];
		let mut max_schedule_count = 0;

		if sp_io::storage::exists(LAOS_VESTING_MIGRATION_6S) {
			return Ok(vec![]);
		}

		for (account_id, schedules) in pallet_vesting::Vesting::<Runtime>::iter() {
			let schedule_count = schedules.len();
			schedules_per_account.push((account_id, schedule_count as u32));
			if schedule_count > max_schedule_count {
				max_schedule_count = schedule_count;
			}
			// Logging account vesting schedule details
			log::debug!(
				target: "runtime::migration",
				"Pre-migration: Account {:?} has {} vesting schedules",
				account_id,
				schedule_count
			);
		}

		// Logging summary information before migration
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
		let mut weight: Weight = Weight::zero();

		if sp_io::storage::exists(LAOS_VESTING_MIGRATION_6S) {
			log::info!(
				target: "runtime::migration",
				"VestingMigrationTo6SecBlockTime::on_runtime_upgrade - Migration already applied ... skipping"
			);
			read_count += 1;
		} else {
			log::info!(
				target: "runtime::migration",
				"VestingMigrationTo6SecBlockTime::on_runtime_upgrade - Starting migration"
			);

			weight = weight.saturating_add(migrate_vesting_pallet_max_schedules());
			weight = weight.saturating_add(migrate_schedules());

			sp_io::storage::set(LAOS_VESTING_MIGRATION_6S, &[]);
			write_count += 1;

			log::info!(
				target: "runtime::migration",
				"VestingMigrationTo6SecBlockTime::on_runtime_upgrade - Migration ended"
			);
		}

		weight +
			<Runtime as frame_system::Config>::DbWeight::get()
				.reads_writes(read_count, write_count)
	}

	#[cfg(feature = "try-runtime")]
	/// Verifies migration by checking the vesting data has been migrated, allows for possible
	/// splits in schedules.
	fn post_upgrade(encoded_data: Vec<u8>) -> Result<(), DispatchError> {
		if encoded_data.is_empty() {
			return Ok(());
		}

		let (old_account_count, schedules_per_account, old_max_schedule_count): (
			u32,
			Vec<(AccountId, u32)>,
			u32,
		) = Decode::decode(&mut &encoded_data[..])
			.map_err(|_| DispatchError::Other("Failed to decode migration data"))?;

		let new_account_count = pallet_vesting::Vesting::<Runtime>::iter().count();
		let mut max_schedule_count_post_migration = 0;

		// Assert that the number of accounts has not changed after migration
		assert_eq!(
			new_account_count, old_account_count as usize,
			"Mismatch in vesting account count after migration: expected {}, got {}",
			old_account_count, new_account_count
		);

		// Logging successful migration
		log::info!(
			target: "runtime::migration",
			"VestingMigrationTo6SecBlockTime::post_upgrade - Migration successful. \
			 Account count before: {}, after: {}",
			old_account_count,
			new_account_count
		);

		// Verify each account's schedule count
		for (account_id, expected_schedule_count) in schedules_per_account {
			let new_schedule_count = pallet_vesting::Vesting::<Runtime>::get(account_id)
				.map(|schedules| schedules.len())
				.unwrap_or(0);

			// Update max schedule count after migration
			if new_schedule_count > max_schedule_count_post_migration {
				max_schedule_count_post_migration = new_schedule_count;
			}

			// Log any increase in schedule count due to migration adjustments
			if new_schedule_count > expected_schedule_count as usize {
				log::debug!(
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

		// Logging the maximum schedule count comparison
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

/// Migrates vesting schedules to conform to the new `MaxVestingSchedulesGet` limit.
/// This ensures that no account has more vesting schedules than the allowed maximum.
fn migrate_vesting_pallet_max_schedules() -> Weight {
	// The old maximum number of vesting schedules per account
	const OLD_MAX_VESTING_SCHEDULES: u32 = 28;
	let mut reads_writes = 0;

	// Logging the maximum schedule count comparison
	log::info!(
		target: "runtime::migration",
		"VestingMigrationTo6SecBlockTime::migrate_vesting_pallet_max_schedules from {} to {} max vested",
		OLD_MAX_VESTING_SCHEDULES,
		MaxVestingSchedulesGet::<Runtime>::get()
	);

	// Translate the old vesting schedules to fit the new maximum limit
	pallet_vesting::Vesting::<Runtime>::translate::<
		BoundedVec<
			pallet_vesting::VestingInfo<BalanceOf<Runtime>, BlockNumberFor<Runtime>>,
			ConstU32<OLD_MAX_VESTING_SCHEDULES>,
		>,
		_,
	>(|_key, vesting_info| {
		reads_writes += 1;

		// Create a new bounded vector with the updated maximum limit
		let mut new_vesting_infos: BoundedVec<
			pallet_vesting::VestingInfo<BalanceOf<Runtime>, BlockNumberFor<Runtime>>,
			MaxVestingSchedulesGet<Runtime>,
		> = BoundedVec::new();

		// Attempt to migrate each vesting schedule to the new bounded vector
		for v_info in vesting_info {
			new_vesting_infos.try_push(v_info).ok();
		}

		// Return the new bounded vector to update the storage
		new_vesting_infos.into()
	});

	// Calculate the total weight based on reads and writes performed
	<Runtime as frame_system::Config>::DbWeight::get().reads_writes(reads_writes, reads_writes)
}

/// Migrates all vesting schedules to adjust for the new 6-second block time.
fn migrate_schedules() -> Weight {
	let mut read_count = 0u64;
	let mut write_count = 0u64;

	log::info!(
		target: "runtime::migration",
		"VestingMigrationTo6SecBlockTime::migrate_schedules - Starting migration"
	);

	for (account_id, schedules) in pallet_vesting::Vesting::<Runtime>::drain() {
		read_count += 1;

		// Adjust vesting schedules for the new block time
		let adjusted_schedules =
			adjust_schedule(frame_system::Pallet::<Runtime>::block_number(), schedules.to_vec());

		// Insert the adjusted schedules back into storage
		pallet_vesting::Vesting::<Runtime>::insert(account_id, adjusted_schedules);
		write_count += 1;
	}

	// Logging completion of the runtime upgrade
	log::info!(
		target: "runtime::migration",
		"VestingMigrationTo6SecBlockTime::migrate_schedules - Migration completed with {} reads and {} writes",
		read_count,
		write_count
	);

	<Runtime as frame_system::Config>::DbWeight::get().reads_writes(read_count, write_count)
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
	schedules: Vec<VestingSchedule>,
) -> BoundedVec<VestingSchedule, MaxVestingSchedulesGet<Runtime>> {
	let mut adjusted_schedules = BoundedVec::new();

	for schedule in schedules {
		if current_block <= schedule.starting_block() {
			// Case 1: Current block is before or at the starting block, so adjust the schedule for
			// 6-second block time.
			let new_schedule = adjust_future_schedule(schedule, current_block);
			try_push_schedule(&mut adjusted_schedules, new_schedule);
		} else if remaining_vesting(schedule, current_block) > 0 {
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
	schedule: VestingSchedule,
	current_block: BlockNumberFor<Runtime>,
) -> VestingSchedule {
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
	schedule: VestingSchedule,
	current_block: BlockNumberFor<Runtime>,
) -> (VestingSchedule, VestingSchedule) {
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
	schedule: VestingSchedule,
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
	schedules: &mut BoundedVec<VestingSchedule, MaxVestingSchedulesGet<Runtime>>,
	schedule: VestingSchedule,
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
				RuntimeOrigin::signed(alice),
				bob,
				vesting_schedule
			));

			assert_eq!(
				VestingMigrationTo6SecBlockTime::on_runtime_upgrade(),
				Weight::from_parts(350000000, 0)
			);

			let schedules = pallet_vesting::Vesting::<Runtime>::get(bob).unwrap();
			assert_eq!(schedules.len(), 1);
			assert_eq!(schedules[0].starting_block(), 20);
			assert_eq!(schedules[0].locked(), 1000 * UNIT);
			assert_eq!(schedules[0].per_block(), UNIT);
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
				RuntimeOrigin::signed(alice),
				bob,
				vesting_schedule
			));

			frame_system::Pallet::<Runtime>::set_block_number(5);

			assert_eq!(
				VestingMigrationTo6SecBlockTime::on_runtime_upgrade(),
				Weight::from_parts(350000000, 0)
			);

			let schedules = pallet_vesting::Vesting::<Runtime>::get(bob).unwrap();
			assert_eq!(schedules.len(), 1);
			assert_eq!(schedules[0].starting_block(), 15);
			assert_eq!(schedules[0].locked(), 1000 * UNIT);
			assert_eq!(schedules[0].per_block(), UNIT);
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
				RuntimeOrigin::signed(alice),
				bob,
				vesting_schedule
			));

			frame_system::Pallet::<Runtime>::set_block_number(10);

			assert_eq!(
				VestingMigrationTo6SecBlockTime::on_runtime_upgrade(),
				Weight::from_parts(350000000, 0)
			);

			let schedules = pallet_vesting::Vesting::<Runtime>::get(bob).unwrap();
			assert_eq!(schedules.len(), 1);
			assert_eq!(schedules[0].starting_block(), 10);
			assert_eq!(schedules[0].locked(), 1000 * UNIT);
			assert_eq!(schedules[0].per_block(), UNIT);
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
				RuntimeOrigin::signed(alice),
				bob,
				vesting_schedule
			));

			frame_system::Pallet::<Runtime>::set_block_number(15);

			assert_eq!(
				VestingMigrationTo6SecBlockTime::on_runtime_upgrade(),
				Weight::from_parts(350000000, 0)
			);

			let schedules = pallet_vesting::Vesting::<Runtime>::get(bob).unwrap();
			assert_eq!(schedules.len(), 2);
			assert_eq!(schedules[0].starting_block(), 10);
			assert_eq!(schedules[0].locked(), 10 * UNIT);
			assert_eq!(schedules[0].per_block(), 2 * UNIT);
			assert_eq!(schedules[1].starting_block(), 15);
			assert_eq!(schedules[1].locked(), 990 * UNIT);
			assert_eq!(schedules[1].per_block(), UNIT);
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
				RuntimeOrigin::signed(alice),
				bob,
				vesting_schedule
			));

			frame_system::Pallet::<Runtime>::set_block_number(5000);
			assert_eq!(
				VestingMigrationTo6SecBlockTime::on_runtime_upgrade(),
				Weight::from_parts(350000000, 0)
			);

			let schedules = pallet_vesting::Vesting::<Runtime>::get(bob).unwrap();
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

			let schedule1 = pallet_vesting::VestingInfo::new(1000 * UNIT, UNIT, 10);
			let schedule2 = pallet_vesting::VestingInfo::new(2000 * UNIT, 2 * UNIT, 15);

			assert_ok!(Vesting::vested_transfer(RuntimeOrigin::signed(alice), bob, schedule1));
			assert_ok!(Vesting::vested_transfer(RuntimeOrigin::signed(alice), bob, schedule2));

			assert_eq!(
				VestingMigrationTo6SecBlockTime::on_runtime_upgrade(),
				Weight::from_parts(350000000, 0)
			);

			let schedules = pallet_vesting::Vesting::<Runtime>::get(bob).unwrap();
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

			assert!(pallet_vesting::Vesting::<Runtime>::get(bob).is_none());

			assert_eq!(
				VestingMigrationTo6SecBlockTime::on_runtime_upgrade(),
				Weight::from_parts(100000000, 0)
			);

			assert!(pallet_vesting::Vesting::<Runtime>::get(bob).is_none());
		});
	}

	/// Tests that the migration does not exceed the allowed maximum number of vesting schedules
	/// (`MaxVestingSchedulesGet`), even when all schedules are split during migration.
	#[test]
	fn migrate_max_schedule_limit_with_all_splitting() {
		ExtBuilder::default().build().execute_with(|| {
			// Setup Alice's account with a large balance
			let alice = setup_account(ALICE, 5_000_000_000 * UNIT);
			// Setup Bob's account
			let bob = AccountId::from_str(BOB).unwrap();

			// Define a vesting schedule with total amount, per period amount, and number of periods
			let vesting_schedule = pallet_vesting::VestingInfo::new(1_000 * UNIT, UNIT, 10);

			// Calculate half of the maximum number of vesting schedules allowed
			let half_max_schedules = MaxVestingSchedulesGet::<Runtime>::get() / 2;

			// Alice transfers multiple vesting schedules to Bob
			for _ in 0..half_max_schedules {
				let _ =
					Vesting::vested_transfer(RuntimeOrigin::signed(alice), bob, vesting_schedule);
			}

			// Advance the block number to a point where vesting schedules may split during
			// migration
			frame_system::Pallet::<Runtime>::set_block_number(15);

			// Execute the migration and verify the expected weight is consumed
			assert_eq!(
				VestingMigrationTo6SecBlockTime::on_runtime_upgrade(),
				Weight::from_parts(350_000_000, 0)
			);

			// Retrieve Bob's vesting schedules after migration
			let schedules = pallet_vesting::Vesting::<Runtime>::get(bob).unwrap();

			// Calculate the expected number of schedules after migration
			// Each original schedule may split into two due to the migration logic,
			// but the total should not exceed the maximum allowed
			let expected_schedules = MaxVestingSchedulesGet::<Runtime>::get();

			// Assert that the number of schedules does not exceed the maximum allowed
			assert_eq!(schedules.len(), expected_schedules as usize);
		});
	}
	/// Helper function to set up an account with a given balance.
	fn setup_account(account_str: &str, balance: u128) -> AccountId {
		let account = AccountId::from_str(account_str).unwrap();
		assert_ok!(Balances::force_set_balance(RuntimeOrigin::root(), account, balance));
		account
	}
}
