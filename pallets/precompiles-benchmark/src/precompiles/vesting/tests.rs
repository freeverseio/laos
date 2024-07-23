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

//! Living assets precompile tests.

use super::*;
use frame_support::assert_ok;
use mock::*;
use pallet_vesting::{Pallet, VestingInfo as VestingInfoPallet};
use precompile_utils::testing::{Alice, Bob, Precompile1, PrecompileTesterExt};
use sp_core::H160;

/// Get precompiles from the mock.
fn precompiles() -> LaosPrecompiles<Test> {
	PrecompilesInstance::get()
}

#[test]
fn selectors() {
	assert!(PrecompileCall::vest_selectors().contains(&0x458EFDE3));
	assert!(PrecompileCall::vest_other_selectors().contains(&0x55E60C8));
	assert!(PrecompileCall::vesting_selectors().contains(&0xE388C423));
}

#[test]
fn vesting_for_account_with_no_vesting_returns_empty_vec() {
	ExtBuilder::default().build().execute_with(|| {
		precompiles()
			.prepare_test(
				Alice,
				Precompile1,
				PrecompileCall::vesting { account: Address(Alice.into()) },
			)
			.execute_returns(Vec::<VestingInfo>::new())
	});
}

#[test]
fn vesting_for_account_with_one_vesting_returns_vesting_info_vec() {
	ExtBuilder::default()
		.with_balances(vec![(Bob.into(), 100u128)])
		.build()
		.execute_with(|| {
			let locked = 100;
			let per_block = 10;
			let starting_block = 0;
			assert_ok!(Pallet::<Test>::vested_transfer(
				RuntimeOrigin::signed(Bob.into()),
				Alice.into(),
				VestingInfoPallet::new(locked, per_block, starting_block),
			));
			precompiles()
				.prepare_test(
					Alice,
					Precompile1,
					PrecompileCall::vesting { account: Address(Alice.into()) },
				)
				.execute_returns(vec![VestingInfo {
					locked: locked.into(),
					per_block: per_block.into(),
					starting_block: starting_block.into(),
				}])
		});
}

#[test]
fn vesting_for_account_with_two_vestings_returns_vesting_info_vec() {
	ExtBuilder::default()
		.with_balances(vec![(Bob.into(), 1000u128)])
		.build()
		.execute_with(|| {
			let locked = 100;
			let per_block = 10;
			let starting_block = 0;

			assert_ok!(Pallet::<Test>::vested_transfer(
				RuntimeOrigin::signed(Bob.into()),
				Alice.into(),
				VestingInfoPallet::new(locked, per_block, starting_block),
			));
			assert_ok!(Pallet::<Test>::vested_transfer(
				RuntimeOrigin::signed(Bob.into()),
				Alice.into(),
				VestingInfoPallet::new(locked, per_block, starting_block),
			));
			precompiles()
				.prepare_test(
					Alice,
					Precompile1,
					PrecompileCall::vesting { account: Address(Alice.into()) },
				)
				.expect_cost(56898396)
				.execute_returns(vec![
					VestingInfo {
						locked: locked.into(),
						per_block: per_block.into(),
						starting_block: starting_block.into(),
					},
					VestingInfo {
						locked: locked.into(),
						per_block: per_block.into(),
						starting_block: starting_block.into(),
					},
				])
		});
}

#[test]
fn vest_reverts_no_vested_funds() {
	ExtBuilder::default().build().execute_with(|| {
		precompiles()
			.prepare_test(Alice, Precompile1, PrecompileCall::vest {})
			.execute_reverts(|r| r == b"NotVesting");
	});
}

#[test]
fn vest_increases_usable_balance() {
	ExtBuilder::default()
		.with_balances(vec![(Bob.into(), 100u128)])
		.build()
		.execute_with(|| {
			let locked = 10;
			let per_block = 1;
			let starting_block = 0;
			let end_block = 5u32;

			assert_ok!(Pallet::<Test>::vested_transfer(
				RuntimeOrigin::signed(Bob.into()),
				Alice.into(),
				VestingInfoPallet::new(locked, per_block, starting_block),
			));
			assert_eq!(
				Balances::usable_balance(H160::from(Alice)),
				1,
				"1 free balance because 1 block has passed"
			);
			roll_to(end_block.into());
			precompiles()
				.prepare_test(Alice, Precompile1, PrecompileCall::vest {})
				.expect_cost(472000000)
				.execute_some();

			assert_eq!(Balances::usable_balance(H160::from(Alice)), end_block as u128);
		});
}

#[test]
fn vest_other_reverts_no_vested_funds() {
	ExtBuilder::default().build().execute_with(|| {
		precompiles()
			.prepare_test(
				Alice,
				Precompile1,
				PrecompileCall::vest_other { account: Address(Bob.into()) },
			)
			.execute_reverts(|r| r == b"NotVesting");
	});
}

#[test]
fn vest_other_increases_other_usable_balance() {
	ExtBuilder::default()
		.with_balances(vec![(Bob.into(), 100u128)])
		.build()
		.execute_with(|| {
			let locked = 10;
			let per_block = 1;
			let starting_block = 0;
			let end_block = 5u32;

			assert_ok!(Pallet::<Test>::vested_transfer(
				RuntimeOrigin::signed(Bob.into()),
				Alice.into(),
				VestingInfoPallet::new(locked, per_block, starting_block),
			));
			assert_eq!(
				Balances::usable_balance(H160::from(Alice)),
				1,
				"1 free balance because 1 block has passed"
			);
			roll_to(end_block.into());
			precompiles()
				.prepare_test(
					Bob,
					Precompile1,
					PrecompileCall::vest_other { account: Address(Alice.into()) },
				)
				.expect_cost(472000000)
				.execute_some();

			assert_eq!(Balances::usable_balance(H160::from(Alice)), end_block as u128);
		});
}
