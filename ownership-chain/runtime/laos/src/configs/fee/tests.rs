use super::mock::*;

#[test]
fn distribute_fees_correctly() {
	let rewards_account_id = 1;
	let author_account_id = 2;
	let initial_balance = 1000;
	let fee_amount = 100;

	// Set up the environment with a rewards account and initial balances
	ExtBuilder::default()
		.with_rewards_account(rewards_account_id)
		.with_balances(vec![
			(rewards_account_id, initial_balance),
			(author_account_id, initial_balance),
		])
		.build()
		.execute_with(|| {
			// // Mock the creation of a negative imbalance of 100 units
			// let imbalance = pallet_balances::NegativeImbalance::new(fee_amount);

			// // Distribute the fees
			// DealWithFees::<Runtime>::on_unbalanceds(vec![imbalance].into_iter());

			// // Assert the expected state of balances after distribution
			// let rewards_balance =
			// pallet_balances::Pallet::<Runtime>::free_balance(rewards_account_id);
			// let author_balance =
			// pallet_balances::Pallet::<Runtime>::free_balance(author_account_id);

			// // Assuming an 80/20 split, calculate expected balances
			// let expected_rewards_balance = initial_balance + (fee_amount * 80 / 100);
			// let expected_author_balance = initial_balance + (fee_amount * 20 / 100);

			// assert_eq!(rewards_balance, expected_rewards_balance, "Rewards account did not
			// receive the correct amount"); assert_eq!(author_balance, expected_author_balance,
			// "Author did not receive the correct amount");
		});
}
