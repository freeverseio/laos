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

	#[test]
	fn migration_updates_vesting_schedule_with_correct_starting_block_and_per_block_rate() {
		ExtBuilder::default().build().execute_with(|| {
			let alice = setup_account(ALICE, 10000 * UNIT);
			let bob = setup_account(BOB, 0);

			// Verify current block number is the initial value
			assert_eq!(frame_system::Pallet::<Runtime>::block_number(), 0);

			let locked_amount = 1000 * UNIT;
			let per_block_reward = 2 * UNIT;
			let starting_block = 10;
			let vesting_schedule =
				pallet_vesting::VestingInfo::new(locked_amount, per_block_reward, starting_block);

			// Alice transfers vested tokens to Bob
			assert_ok!(Vesting::vested_transfer(
				RuntimeOrigin::signed(alice.clone()),
				bob.clone(),
				vesting_schedule
			));

			// Execute the migration
			assert_eq!(
				VestingMigrationTo6SecBlockTime::on_runtime_upgrade(),
				Weight::from_parts(250000000, 0)
			);

			// Verify vesting schedule adjustments
			let schedules = pallet_vesting::Vesting::<Runtime>::get(&bob).unwrap();
			assert_eq!(schedules.len(), 1);
			assert_eq!(schedules[0].starting_block(), 20);
			assert_eq!(schedules[0].locked(), locked_amount);
			assert_eq!(schedules[0].per_block(), 1 * UNIT);
		});
	}

	#[test]
	fn migration_when_current_block_is_before_starting_block() {
		ExtBuilder::default().build().execute_with(|| {
			frame_system::Pallet::<Runtime>::set_block_number(5);

			let alice = setup_account(ALICE, 10000 * UNIT);
			let bob = setup_account(BOB, 0);

			let locked_amount = 1000 * UNIT;
			let per_block_reward = 2 * UNIT;
			let starting_block = 10;
			let vesting_schedule =
				pallet_vesting::VestingInfo::new(locked_amount, per_block_reward, starting_block);

			// Alice transfers vested tokens to Bob
			assert_ok!(Vesting::vested_transfer(
				RuntimeOrigin::signed(alice.clone()),
				bob.clone(),
				vesting_schedule
			));

			// Execute the migration
			assert_eq!(
				VestingMigrationTo6SecBlockTime::on_runtime_upgrade(),
				Weight::from_parts(250000000, 0)
			);

			// Verify vesting schedule adjustments
			let schedules = pallet_vesting::Vesting::<Runtime>::get(&bob).unwrap();
			assert_eq!(schedules.len(), 1);
			assert_eq!(schedules[0].starting_block(), 15);
			assert_eq!(schedules[0].locked(), locked_amount);
			assert_eq!(schedules[0].per_block(), 1 * UNIT);
		});
	}

	#[test]
	fn migration_when_current_block_is_at_starting_block() {
		ExtBuilder::default().build().execute_with(|| {
			let starting_block = 10;
			frame_system::Pallet::<Runtime>::set_block_number(starting_block);

			let alice = setup_account(ALICE, 10000 * UNIT);
			let bob = setup_account(BOB, 0);

			let locked_amount = 1000 * UNIT;
			let per_block_reward = 2 * UNIT;
			let vesting_schedule =
				pallet_vesting::VestingInfo::new(locked_amount, per_block_reward, starting_block);

			// Alice transfers vested tokens to Bob
			assert_ok!(Vesting::vested_transfer(
				RuntimeOrigin::signed(alice.clone()),
				bob.clone(),
				vesting_schedule
			));

			// Execute the migration
			assert_eq!(
				VestingMigrationTo6SecBlockTime::on_runtime_upgrade(),
				Weight::from_parts(250000000, 0)
			);

			// Verify vesting schedule adjustments
			let schedules = pallet_vesting::Vesting::<Runtime>::get(&bob).unwrap();
			assert_eq!(schedules.len(), 1);
			assert_eq!(schedules[0].starting_block(), 10);
			assert_eq!(schedules[0].locked(), locked_amount);
			assert_eq!(schedules[0].per_block(), 1 * UNIT);
		});
	}

	#[test]
	fn migration_when_current_block_is_after_starting_block_but_within_schedule() {
		ExtBuilder::default().build().execute_with(|| {
			frame_system::Pallet::<Runtime>::set_block_number(15);

			let alice = setup_account(ALICE, 10000 * UNIT);
			let bob = setup_account(BOB, 0);

			let locked_amount = 1000 * UNIT;
			let per_block_reward = 2 * UNIT;
			let starting_block = 10;
			let vesting_schedule =
				pallet_vesting::VestingInfo::new(locked_amount, per_block_reward, starting_block);

			// Alice transfers vested tokens to Bob
			assert_ok!(Vesting::vested_transfer(
				RuntimeOrigin::signed(alice.clone()),
				bob.clone(),
				vesting_schedule
			));

			// Execute the migration
			assert_eq!(
				VestingMigrationTo6SecBlockTime::on_runtime_upgrade(),
				Weight::from_parts(250000000, 0)
			);

			// Verify vesting schedule adjustments
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

	#[test]
	fn migration_when_schedule_is_already_expired() {
		ExtBuilder::default().build().execute_with(|| {
			let alice = setup_account(ALICE, 10000 * UNIT);
			let bob = setup_account(BOB, 0);

			let locked_amount = 1000 * UNIT;
			let per_block_reward = 2 * UNIT;
			let starting_block = 10;
			let vesting_schedule =
				pallet_vesting::VestingInfo::new(locked_amount, per_block_reward, starting_block);

			// Alice transfers vested tokens to Bob
			assert_ok!(Vesting::vested_transfer(
				RuntimeOrigin::signed(alice.clone()),
				bob.clone(),
				vesting_schedule
			));

			frame_system::Pallet::<Runtime>::set_block_number(5000);

			// Execute the migration
			assert_eq!(
				VestingMigrationTo6SecBlockTime::on_runtime_upgrade(),
				Weight::from_parts(250000000, 0)
			);

			// Verify vesting schedule adjustments
			let schedules = pallet_vesting::Vesting::<Runtime>::get(&bob).unwrap();
			assert_eq!(schedules.len(), 1);
			assert_eq!(schedules[0].starting_block(), 10);
			assert_eq!(schedules[0].locked(), locked_amount);
			assert_eq!(schedules[0].per_block(), 2 * UNIT);
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
			VestingMigrationTo6SecBlockTime::on_runtime_upgrade();

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
			VestingMigrationTo6SecBlockTime::on_runtime_upgrade();

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
			VestingMigrationTo6SecBlockTime::on_runtime_upgrade();

			// check that the number of schedules does not exceed the maximum allowed
			let schedules = pallet_vesting::Vesting::<Runtime>::get(&bob).unwrap();
			assert!(schedules.len() <= MaxVestingSchedulesGet::<Runtime>::get() as usize);
		});
	}

	fn setup_account(account_str: &str, balance: u128) -> AccountId {
		let account = AccountId::from_str(account_str).unwrap();
		assert_ok!(Balances::force_set_balance(RuntimeOrigin::root(), account.clone(), balance));
		account
	}
}
