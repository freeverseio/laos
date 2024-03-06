use super::{mock::*, *};

#[test]
fn distribute_fees_correctly() {
	let fee_amount = 100;

	// Set up the environment with a rewards account and initial balances
	ExtBuilder::default().build().execute_with(|| {
		System::set_block_number(1);

		// get author of the block
		let author = pallet_authorship::Pallet::<Test>::author().unwrap();

		// initial author balance
		let initial_author_balance = pallet_balances::Pallet::<Test>::free_balance(author);

		// Mock the creation of a negative imbalance of 100 units
		let imbalance = pallet_balances::NegativeImbalance::new(fee_amount);

		// Distribute the fees
		DealWithFees::<Test>::on_unbalanceds(vec![imbalance].into_iter());

		// Assert the expected state of balances after distribution
		let author_balance = pallet_balances::Pallet::<Test>::free_balance(author);

		// Assuming all fees are distributed to the author
		let expected_author_balance = fee_amount;

		assert_eq!(
			author_balance, expected_author_balance,
			"Author did not receive the correct amount"
		);
	});
}
