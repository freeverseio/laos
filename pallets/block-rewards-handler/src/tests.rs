use crate::mock::*;

#[test]
fn send_rewards_when_source_account_does_not_exist() {
	let destination = 0;
	let amount = 100;
	ExtBuilder::default().build().execute_with(|| {
		let initial_destination_balance = Balances::free_balance(&destination);
		assert!(BlockRewardsHandler::rewards_account().is_none());
		assert_eq!(BlockRewardsHandler::send_rewards(destination, amount).unwrap(), 0);
		assert_eq!(Balances::free_balance(&destination), initial_destination_balance);
	});
}

#[test]
fn send_rewards_when_source_account_has_no_enough_balance() {
	let destination = 0;
	let rewards_account: u64 = 1;
	let amount = 100;
	ExtBuilder::default()
		.with_rewards_account(rewards_account)
		.build()
		.execute_with(|| {
			let initial_destination_balance = Balances::free_balance(&destination);
			assert!(BlockRewardsHandler::rewards_account().is_some());
			assert_eq!(BlockRewardsHandler::send_rewards(destination, amount).unwrap(), 0);
			assert_eq!(Balances::free_balance(&destination), initial_destination_balance);
		});
}

#[test]
fn send_rewards_works() {
	let destination = 0;
	let rewards_account = 1;
	let amount = 100;
	ExtBuilder::default()
		.with_rewards_account(rewards_account)
		.with_balances(vec![(rewards_account, 1000)])
		.build()
		.execute_with(|| {
			let initial_destination_balance = Balances::free_balance(&destination);
			assert!(BlockRewardsHandler::rewards_account().is_some());
			assert_eq!(BlockRewardsHandler::send_rewards(destination, amount).unwrap(), amount);
			assert_eq!(Balances::free_balance(&rewards_account), 1000 - amount);
			assert_eq!(Balances::free_balance(&destination), initial_destination_balance + amount);
		});
}
