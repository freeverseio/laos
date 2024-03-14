use crate::{traits::PayoutReward, BalanceOf, *};
use frame_support::{
	pallet_prelude::Weight,
	traits::{
		tokens::{currency::Currency, ExistenceRequirement},
		Get, Imbalance,
	},
};
use sp_runtime::DispatchError;

pub struct MintingRewards;

/// Defines the default behavior for paying out the collator's reward. The amount is directly
/// deposited into the collator's account.
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

impl<T: Config> Pallet<T> {
	/// Mint a specified reward amount to the collator's account. Emits the [Rewarded] event.
	fn mint_collator_reward(
		_paid_for_round: RoundIndex,
		collator_id: T::AccountId,
		amt: BalanceOf<T>,
	) -> Weight {
		if let Ok(amount_transferred) = T::PayoutReward::payout(&collator_id, amt) {
			Self::deposit_event(Event::Rewarded {
				account: collator_id.clone(),
				rewards: amount_transferred,
			});
		}
		T::WeightInfo::mint_collator_reward()
	}

	pub fn send_collator_reward(
		_paid_for_round: RoundIndex,
		collator_id: T::AccountId,
		amt: BalanceOf<T>,
	) -> Weight {
		if let Ok(()) = T::Currency::transfer(
			&T::RewardsAccount::get(),
			&collator_id,
			amt,
			ExistenceRequirement::KeepAlive,
		) {
			Self::deposit_event(Event::Rewarded { account: collator_id.clone(), rewards: amt });
		}
		Weight::zero() // TODO: weight
	}
}
