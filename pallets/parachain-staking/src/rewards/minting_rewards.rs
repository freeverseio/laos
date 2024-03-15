use crate::{traits::PayoutReward, BalanceOf};
use frame_support::{
	pallet_prelude::Weight,
	traits::{tokens::currency::Currency, Imbalance},
};
use sp_runtime::DispatchError;

pub struct MintingRewards;
impl<Runtime: crate::Config> PayoutReward<Runtime, BalanceOf<Runtime>> for MintingRewards {
	fn payout_collator_rewards(
		_for_round: crate::RoundIndex,
		collator_id: Runtime::AccountId,
		amount: crate::BalanceOf<Runtime>,
	) -> Weight {
		crate::Pallet::<Runtime>::mint_collator_reward(collator_id, amount)
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
	fn payout_to_account_0_fails() {
		ExtBuilder::default().build().execute_with(|| {
			let delegator = 0;
			let amount = 100;

			assert_err!(
				<MintingRewards as PayoutReward<Test, Balance>>::payout(&delegator, amount),
				pallet_balances::Error::<Test>::DeadAccount
			);
		});
	}

	#[test]
	fn payout_0_amount_succeed() {
		ExtBuilder::default().build().execute_with(|| {
			let delegator = 0;
			let amount = 0;

			assert_ok!(
				<MintingRewards as PayoutReward<Test, Balance>>::payout(&delegator, amount),
				0
			);
		});
	}
}
