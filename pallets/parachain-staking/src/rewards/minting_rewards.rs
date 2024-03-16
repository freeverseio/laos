use crate::{traits::PayoutReward, BalanceOf, RoundIndex};
use frame_support::{
	pallet_prelude::Weight,
	traits::{tokens::currency::Currency, Imbalance},
};
use sp_runtime::DispatchError;

pub struct MintingRewards;
impl<Runtime: crate::Config> PayoutReward<Runtime, BalanceOf<Runtime>> for MintingRewards {
	fn payout_collator_rewards(
		for_round: RoundIndex,
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
	use frame_support::{assert_err, assert_ok};

	#[test]
	fn test_payout_to_dead_account_fails() {
		ExtBuilder::default().build().execute_with(|| {
			let delegator = 10;
			let amount = 100;

			assert_err!(
				<MintingRewards as PayoutReward<Test, Balance>>::payout(&delegator, amount),
				pallet_balances::Error::<Test>::DeadAccount
			);
		});
	}

	#[test]
	fn test_payout_with_zero_amount_succeeds() {
		ExtBuilder::default().build().execute_with(|| {
			let delegator = 10;
			let amount = 0;

			assert_ok!(
				<MintingRewards as PayoutReward<Test, Balance>>::payout(&delegator, amount),
				0
			);
		});
	}
}
