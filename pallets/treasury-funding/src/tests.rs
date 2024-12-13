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

use crate::mock::*;
use frame_support::assert_ok;

#[test]
fn test_fund_treasury_without_vesting() {
	new_test_ext().execute_with(|| {
		let vault_account = TreasuryFunding::account_id();
		let treasury_account = Treasury::account_id();

		// Step 1: Fund the vault account with an initial balance.
		let initial_balance = 1_000;
		let _ = Balances::force_set_balance(RuntimeOrigin::root(), vault_account, initial_balance);

		// Step 2: Verify the initial balances of the vault and treasury accounts.
		assert_eq!(Balances::free_balance(vault_account), initial_balance);
		assert_eq!(Balances::free_balance(treasury_account), 0);

		// Step 3: Call the fund_treasury function to transfer funds.
		assert_ok!(TreasuryFunding::fund_treasury(RuntimeOrigin::signed(vault_account)));

		// Step 4: Verify the final balances after the transfer.
		assert_eq!(Balances::free_balance(vault_account), 0);
		assert_eq!(Balances::free_balance(treasury_account), initial_balance);
	});
}

#[test]
fn test_fund_treasury_during_active_vesting() {
	new_test_ext().execute_with(|| {
		let vault_account = TreasuryFunding::account_id();
		let treasury_account = Treasury::account_id();

		// Step 1: Fund the vault account with an initial balance.
		let initial_balance = 1_000;
		let _ = Balances::force_set_balance(RuntimeOrigin::root(), vault_account, initial_balance);

		// Step 2: Create a vesting schedule for the vault account.
		Vesting::vested_transfer(
			RuntimeOrigin::signed(vault_account),
			vault_account,
			pallet_vesting::VestingInfo::new(1_000, 1, 0),
		)
		.unwrap();

		// Step 3: Simulate the passage of time (to block 500).
		System::set_block_number(500);

		// Step 4: Verify the initial balances and vesting schedule.
		assert_eq!(Balances::free_balance(vault_account), initial_balance);
		assert!(Vesting::vesting(vault_account).is_some());

		// Step 5: Call the fund_treasury function during active vesting.
		assert_ok!(TreasuryFunding::fund_treasury(RuntimeOrigin::signed(vault_account)));

		// Step 6: Verify the final balances after the transfer and vesting handling.
		assert_eq!(Balances::free_balance(vault_account), 500); // Remaining vested balance.
		assert_eq!(Balances::free_balance(treasury_account), 500); // Transferred amount.
	});
}

#[test]
fn test_fund_treasury_with_expired_vesting() {
	new_test_ext().execute_with(|| {
		let vault_account = TreasuryFunding::account_id();
		let treasury_account = Treasury::account_id();

		// Step 1: Fund the vault account with an initial balance.
		let initial_balance = 1_000;
		let _ = Balances::force_set_balance(RuntimeOrigin::root(), vault_account, initial_balance);

		// Step 2: Create a vesting schedule for the vault account.
		Vesting::vested_transfer(
			RuntimeOrigin::signed(vault_account),
			vault_account,
			pallet_vesting::VestingInfo::new(1_000, 1, 0),
		)
		.unwrap();

		// Step 3: Simulate the passage of time (to block 1,001, beyond vesting expiry).
		System::set_block_number(1_001);

		// Step 4: Verify the initial balances and expired vesting schedule.
		assert_eq!(Balances::free_balance(vault_account), initial_balance);
		assert!(Vesting::vesting(vault_account).is_some());

		// Step 5: Call the fund_treasury function after vesting expiry.
		assert_ok!(TreasuryFunding::fund_treasury(RuntimeOrigin::signed(vault_account)));

		// Step 6: Verify the final balances after the transfer.
		assert_eq!(Balances::free_balance(vault_account), 0); // All funds transferred.
		assert_eq!(Balances::free_balance(treasury_account), initial_balance); // Full amount received by
		                                                                 // treasury.
	});
}
