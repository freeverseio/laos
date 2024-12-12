use crate::mock::*;
use frame_support::{assert_ok, traits::OnInitialize};
use sp_runtime::traits::AccountIdConversion;

#[test]
fn test_fund_treasury_no_vesting() {
	new_test_ext().execute_with(|| {
		let vault_account = TreasuryFunding::account_id();
		let treasury_account = Treasury::account_id();

		// Fund the vault account with some balance.
		let initial_balance = 1_000;
		Balances::force_set_balance(RuntimeOrigin::root(), vault_account, initial_balance);

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
fn test_fund_treasury_with_vesting() {
	new_test_ext().execute_with(|| {
		let vault_account = TreasuryFunding::account_id();
		let treasury_account = Treasury::account_id();

		// Fund the vault account with some balance.
		let initial_balance = 1_000;
		Balances::force_set_balance(RuntimeOrigin::root(), vault_account, initial_balance);

		// Create a vesting schedule for the vault account.
		let vesting_schedule =
			pallet_vesting::VestingSchedule { locked: 500, per_block: 100, starting_block: 1 };
		Vesting::add_vesting_schedule(
			RuntimeOrigin::root(),
			vault_account.clone(),
			vesting_schedule.locked,
			vesting_schedule.per_block,
			vesting_schedule.starting_block,
		)
		.unwrap();

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

// #[test]
// fn test_account_id_generation() {
//     new_test_ext().execute_with(|| {
//         let pallet_account = TreasuryFunding::account_id();
//         let expected_account = PalletId(*b"testfund").into_account_truncating();

//         assert_eq!(pallet_account, expected_account);
//     });
// }
