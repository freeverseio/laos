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
		let expected_author_balance = initial_author_balance + fee_amount;

		assert_eq!(
			author_balance, expected_author_balance,
			"Author did not receive the correct amount"
		);
	});
}

#[test]
fn distribute_fees_and_tips_correctly() {
	let fee_amount = 100;
	let tip_amount = 50;

	ExtBuilder::default().build().execute_with(|| {
		System::set_block_number(1);

		// Assuming the block author is set up
		let author = pallet_authorship::Pallet::<Test>::author().unwrap();
		let initial_author_balance = pallet_balances::Pallet::<Test>::free_balance(author);

		// Mock the creation of a negative imbalance for fees and tips
		let fees = pallet_balances::NegativeImbalance::new(fee_amount);
		let tips = pallet_balances::NegativeImbalance::new(tip_amount);

		// Distribute the fees and tips
		DealWithFees::<Test>::on_unbalanceds(vec![fees, tips].into_iter());

		// Verify the author's balance is updated correctly
		let expected_author_balance = initial_author_balance + fee_amount + tip_amount;
		assert_eq!(
			pallet_balances::Pallet::<Test>::free_balance(author),
			expected_author_balance,
			"Author did not receive the correct amount from fees and tips"
		);
	});
}

#[test]
fn distribute_ethereum_based_fees_correctly() {
	let fee_amount = 200;

	ExtBuilder::default().build().execute_with(|| {
		System::set_block_number(1);

		let author = pallet_authorship::Pallet::<Test>::author().unwrap();
		let initial_author_balance = pallet_balances::Pallet::<Test>::free_balance(author);

		// Mock the creation of a negative imbalance from Ethereum-based transaction fees
		let imbalance = pallet_balances::NegativeImbalance::new(fee_amount);

		// Distribute the Ethereum-based transaction fees
		DealWithFees::<Test>::on_nonzero_unbalanced(imbalance);

		// Assert the author's balance is correctly updated
		let expected_author_balance = initial_author_balance + fee_amount;
		assert_eq!(
			pallet_balances::Pallet::<Test>::free_balance(author),
			expected_author_balance,
			"Author did not receive the correct amount from Ethereum-based fees"
		);
	});
}
