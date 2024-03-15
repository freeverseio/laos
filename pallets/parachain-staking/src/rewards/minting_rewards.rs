use crate::{traits::PayoutReward, BalanceOf};
use frame_support::{
	pallet_prelude::Weight,
	traits::{tokens::currency::Currency, Imbalance},
};
use sp_runtime::DispatchError;

pub struct MintingRewards;
impl<Runtime: crate::Config> PayoutReward<Runtime, BalanceOf<Runtime>> for MintingRewards {
	fn payout_collator_rewards(
		for_round: crate::RoundIndex,
		collator_id: Runtime::AccountId,
		amount: crate::BalanceOf<Runtime>,
	) -> Weight {
		crate::Pallet::<Runtime>::mint_collator_reward(for_round, collator_id, amount)
	}

	fn payout(
		delegator_id: &Runtime::AccountId,
		amount: crate::BalanceOf<Runtime>,
	) -> Result<crate::BalanceOf<Runtime>, DispatchError> {
		Runtime::Currency::deposit_into_existing(delegator_id, amount)
			.map(|imbalance| imbalance.peek())
	}
}

// tests
#[cfg(test)]
mod tests {
	use super::*;
	use crate::mock::*;
	use frame_support::assert_err;

	#[test]
	fn payout_collator_rewards_should_not_panic() {
		ExtBuilder::default().build().execute_with(|| {
			let collator = 678;
			let amount = 100;
			let round_index = 1;

			// check not panic
			<MintingRewards as PayoutReward<Test, Balance>>::payout_collator_rewards(
				round_index,
				collator,
				amount,
			);
		});
	}

	#[test]
	fn payout_should_error_id_delegator_account_do_not_exist() {
		ExtBuilder::default().build().execute_with(|| {
			let delegator = 678;
			let amount = 100;

			assert_err!(
				<MintingRewards as PayoutReward<Test, Balance>>::payout(&delegator, amount),
				pallet_balances::Error::<Test>::DeadAccount
			);
		});
	}

	#[test]
	fn payout_should_return_amount_transferred() {
		ExtBuilder::default().build().execute_with(|| {
			let delegator = 678;
			let amount = 100;
			let _ = pallet_balances::Pallet::<Test>::deposit_creating(&delegator, amount);

			assert_eq!(
				<MintingRewards as PayoutReward<Test, Balance>>::payout(&delegator, amount),
				Ok(amount)
			);

			let _ = pallet_balances::Pallet::<Test>::deposit_creating(&delegator, amount);
			assert_eq!(
				<MintingRewards as PayoutReward<Test, Balance>>::payout(&delegator, amount),
				Ok(amount)
			);
		});
	}

	// test when delegator is 0
	#[test]
	fn payout_should_return_zero() {
		ExtBuilder::default().with_rewards_account_balance(0).build().execute_with(|| {
			let delegator = 0;
			let amount = 100;
			let _ = pallet_balances::Pallet::<Test>::deposit_creating(&delegator, amount);

			assert_eq!(
				<MintingRewards as PayoutReward<Test, Balance>>::payout(&delegator, amount),
				Ok(100)
			);

			let _ = pallet_balances::Pallet::<Test>::deposit_creating(&delegator, amount);
			assert_eq!(
				<MintingRewards as PayoutReward<Test, Balance>>::payout(&delegator, amount),
				Ok(amount)
			);
		});
	}

	// // test when amount is 0
	// #[test]
	// fn payout_should_return_zero_amount() {
	// 	ExtBuilder::default().build().execute_with(|| {
	// 		let delegator = 100;
	// 		let amount = 0;
	// 		let _ = pallet_balances::Pallet::<Test>::deposit_creating(&delegator, amount);

	// 		assert_eq!(
	// 			<MintingRewards as PayoutReward<Test, Balance>>::payout(&delegator, amount),
	// 			Ok(0)
	// 		);

	// 		assert_err!(
	// 			<TransferFromRewardsAccount as PayoutReward<Test, Balance>>::payout(
	// 				&delegator, amount
	// 			),
	// 			TokenError::FundsUnavailable
	// 		);

	// 		let _ = pallet_balances::Pallet::<Test>::deposit_creating(&delegator, amount);
	// 		assert_eq!(
	// 			<MintingRewards as PayoutReward<Test, Balance>>::payout(&delegator, amount),
	// 			Ok(0)
	// 		);

	// 		RewardsAccount::<Test>::kill();
	// 		// if RewardAccount is not set then Error
	// 		assert_err!(
	// 			<TransferFromRewardsAccount as PayoutReward<Test, Balance>>::payout(
	// 				&delegator, amount
	// 			),
	// 			"RewardAccount is not set"
	// 		);
	// 	});
	// }
}
