use crate::mock::*;
use frame_support::weights;
use pallet_parachain_staking::PayoutCollatorReward;

#[test]
fn payout_when_rewards_account_does_not_exist() {
	let collator = 0;
	let amount = 100;
	ExtBuilder::default().build().execute_with(|| {
		let initial_collator_balance = Balances::free_balance(&collator);
		assert!(BlockRewardsSource::rewards_account().is_none());
		assert_eq!(
			BlockRewardsSource::payout_collator_reward(0, collator, amount),
			weights::Weight::zero()
		);
		assert_eq!(Balances::free_balance(&collator), initial_collator_balance);
	});
}

#[test]
fn payout_when_rewards_account_has_no_enough_balance() {
	let collator = 0;
	let rewards_account = 1;
	let amount = 100;
	ExtBuilder::default()
		.with_rewards_account(rewards_account)
		.build()
		.execute_with(|| {
			let initial_collator_balance = Balances::free_balance(&collator);
			assert!(BlockRewardsSource::rewards_account().is_some());
			assert_eq!(
				BlockRewardsSource::payout_collator_reward(0, collator, amount),
				weights::Weight::zero()
			);
			assert_eq!(Balances::free_balance(&collator), initial_collator_balance);
		});
}

#[test]
fn payout_works() {
	let collator = 0;
	let rewards_account = 1;
	let amount = 100;
	ExtBuilder::default()
		.with_rewards_account(rewards_account)
		.with_balances(vec![(rewards_account, 1000)])
		.build()
		.execute_with(|| {
			let initial_collator_balance = Balances::free_balance(&collator);
			assert!(BlockRewardsSource::rewards_account().is_some());
			assert_ne!(
				BlockRewardsSource::payout_collator_reward(0, collator, amount),
				weights::Weight::zero()
			);
			assert_eq!(Balances::free_balance(&rewards_account), 1000 - amount);
			assert_eq!(Balances::free_balance(&collator), initial_collator_balance + amount);
		});
}

#[test]
fn deposit_into_existing_when_rewards_account_does_not_exist() {
	let collator = 0;
	let amount = 100;
	ExtBuilder::default().build().execute_with(|| {
		let initial_collator_balance = Balances::free_balance(&collator);
		assert!(BlockRewardsSource::rewards_account().is_none());
		assert_eq!(BlockRewardsSource::deposit_into_existing(&collator, amount).unwrap(), 0);
		assert_eq!(Balances::free_balance(&collator), initial_collator_balance);
	});
}

#[test]
fn deposit_into_existing_when_rewards_account_has_no_enough_balance() {
	let collator = 0;
	let rewards_account: u64 = 1;
	let amount = 100;
	ExtBuilder::default()
		.with_rewards_account(rewards_account)
		.build()
		.execute_with(|| {
			let initial_collator_balance = Balances::free_balance(&collator);
			assert!(BlockRewardsSource::rewards_account().is_some());
			assert_eq!(BlockRewardsSource::deposit_into_existing(&collator, amount).unwrap(), 0);
			assert_eq!(Balances::free_balance(&collator), initial_collator_balance);
		});
}

#[test]
fn deposit_into_existing_works() {
	let collator = 0;
	let rewards_account = 1;
	let amount = 100;
	ExtBuilder::default()
		.with_rewards_account(rewards_account)
		.with_balances(vec![(rewards_account, 1000)])
		.build()
		.execute_with(|| {
			let initial_collator_balance = Balances::free_balance(&collator);
			assert!(BlockRewardsSource::rewards_account().is_some());
			assert_eq!(
				BlockRewardsSource::deposit_into_existing(&collator, amount).unwrap(),
				amount
			);
			assert_eq!(Balances::free_balance(&rewards_account), 1000 - amount);
			assert_eq!(Balances::free_balance(&collator), initial_collator_balance + amount);
		});
}
