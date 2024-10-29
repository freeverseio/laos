use crate::{Runtime, Weight};
use frame_support::{
	traits::{Currency, OnRuntimeUpgrade},
	BoundedVec,
};
use frame_system::pallet_prelude::BlockNumberFor;
use pallet_vesting::MaxVestingSchedulesGet;

pub struct VestingMigrationTo6SecBlockTime;

// Define a type alias for Balance, simplifying the readability of types later in the code
type BalanceOf<T> = <<T as pallet_vesting::Config>::Currency as Currency<
	<T as frame_system::Config>::AccountId,
>>::Balance;

impl OnRuntimeUpgrade for VestingMigrationTo6SecBlockTime {
	// Function called during runtime upgrade to migrate vesting schedules for 6-second block time
	fn on_runtime_upgrade() -> Weight {
		// Initialize counters for database reads and writes
		let mut read_count = 0u64;
		let mut write_count = 0u64;

		// Drain all existing vesting schedules from storage
		// `drain()` will remove all items from `Vesting` storage and iterate over them
		for (account_id, mut schedules) in pallet_vesting::Vesting::<Runtime>::drain() {
			read_count += 1; // Increment read counter as we read the schedules from storage

			// Create a new collection to hold updated vesting schedules with adjusted starting
			// blocks
			let mut updated_schedules: BoundedVec<
				pallet_vesting::VestingInfo<BalanceOf<Runtime>, BlockNumberFor<Runtime>>,
				MaxVestingSchedulesGet<Runtime>,
			> = BoundedVec::new();

			// Iterate over each existing vesting schedule to adjust for the 6-second block time
			for schedule in &mut schedules {
				let current_block = frame_system::Pallet::<Runtime>::block_number();
				let starting_block = schedule.starting_block();

				if current_block <= starting_block {
					let adjusted_starting_block = 2 * schedule.starting_block() - current_block;
					let adjusted_per_block = schedule.per_block().saturating_div(2u32.into());
					let adjusted_schedule = pallet_vesting::VestingInfo::new(
						schedule.locked(),
						adjusted_per_block,
						adjusted_starting_block,
					);
					if updated_schedules.try_push(adjusted_schedule).is_err() {
						log::warn!(
							"Failed to push adjusted vesting schedule for account {:?}",
							account_id
						);
					}
				} else if (current_block as u128 - starting_block as u128) *
					(schedule.per_block() as u128) <
					schedule.locked()
				{
					// we need to split it in 2 schedules
					let how_much_left = schedule.locked() -
						schedule.per_block() * (current_block as u128 - starting_block as u128);
					let past_schedule = pallet_vesting::VestingInfo::new(
						schedule.locked() - how_much_left,
						schedule.per_block(),
						starting_block,
					);
					let new_schedule = pallet_vesting::VestingInfo::new(
						how_much_left,
						schedule.per_block().saturating_div(2u32.into()),
						current_block,
					);
					if updated_schedules.try_push(past_schedule).is_err() {
						log::warn!(
							"Failed to push adjusted vesting schedule for account {:?}",
							account_id
						);
					}
					if updated_schedules.try_push(new_schedule).is_err() {
						log::warn!(
							"Failed to push adjusted vesting schedule for account {:?}",
							account_id
						);
					}
				} else {
					if updated_schedules.try_push(schedule.clone()).is_err() {
						log::warn!(
							"Failed to push adjusted vesting schedule for account {:?}",
							account_id
						);
					}
				}
			}

			// Update storage with the new set of adjusted schedules for the account
			pallet_vesting::Vesting::<Runtime>::insert(&account_id, &updated_schedules);
			write_count += 1; // Increment write counter as we write the updated schedules to storage
		}

		// Calculate the total weight based on the number of database reads and writes performed
		<Runtime as frame_system::Config>::DbWeight::get()
			.reads_writes(read_count + 1, write_count + 1)
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
