use crate::mock::*;
use frame_support::assert_ok;

#[test]
fn test_fund_treasury_no_vesting() {
	new_test_ext().execute_with(|| {
		let vault_account = TreasuryFunding::account_id();
		let treasury_account = Treasury::account_id();

		// Fund the vault account with some balance.
		let initial_balance = 1_000;
		let _ = Balances::force_set_balance(RuntimeOrigin::root(), vault_account, initial_balance);

		// Check initial balances.
		assert_eq!(Balances::free_balance(&vault_account), initial_balance);
		assert_eq!(Balances::free_balance(&treasury_account), 0);

		// Call the fund_treasury function.
		assert_ok!(TreasuryFunding::fund_treasury(RuntimeOrigin::signed(vault_account.clone())));

		// Check final balances.
		assert_eq!(Balances::free_balance(&vault_account), 0);
		assert_eq!(Balances::free_balance(&treasury_account), initial_balance);
	});
}

#[test]
fn test_fund_treasury_in_middle_of_vesting() {
	new_test_ext().execute_with(|| {
		let vault_account = TreasuryFunding::account_id();
		let treasury_account = Treasury::account_id();

		// Fund the vault account with some balance.
		let initial_balance = 1_000;
		let _ = Balances::force_set_balance(RuntimeOrigin::root(), vault_account, initial_balance);

		// Create a vesting schedule for the vault account.
		Vesting::vested_transfer(
			RuntimeOrigin::signed(vault_account.clone()),
			vault_account.clone(),
			pallet_vesting::VestingInfo::new(1_000, 1, 0),
		)
		.unwrap();

		System::set_block_number(500);

		// Check initial balances and vesting schedule.
		assert_eq!(Balances::free_balance(&vault_account), initial_balance);
		assert!(Vesting::vesting(vault_account.clone()).is_some());

		// Call the fund_treasury function.
		assert_ok!(TreasuryFunding::fund_treasury(RuntimeOrigin::signed(vault_account.clone())));

		// Check final balances and ensure vesting is handled.
		assert_eq!(Balances::free_balance(&vault_account), 500);
		assert_eq!(Balances::free_balance(&treasury_account), 500);
	});
}

#[test]
fn test_fund_treasury_with_expired_vesting() {
	new_test_ext().execute_with(|| {
		let vault_account = TreasuryFunding::account_id();
		let treasury_account = Treasury::account_id();

		// Fund the vault account with some balance.
		let initial_balance = 1_000;
		let _ = Balances::force_set_balance(RuntimeOrigin::root(), vault_account, initial_balance);

		// Create a vesting schedule for the vault account.
		Vesting::vested_transfer(
			RuntimeOrigin::signed(vault_account.clone()),
			vault_account.clone(),
			pallet_vesting::VestingInfo::new(1_000, 1, 0),
		)
		.unwrap();

		System::set_block_number(1_001);

		// Check initial balances and vesting schedule.
		assert_eq!(Balances::free_balance(&vault_account), initial_balance);
		assert!(Vesting::vesting(vault_account.clone()).is_some());

		// Call the fund_treasury function.
		assert_ok!(TreasuryFunding::fund_treasury(RuntimeOrigin::signed(vault_account.clone())));

		// Check final balances and ensure vesting is handled.
		assert_eq!(Balances::free_balance(&vault_account), 0);
		assert_eq!(Balances::free_balance(&treasury_account), initial_balance);
	});
}
