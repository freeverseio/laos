use crate::{traits::PayoutReward, BalanceOf, *};
use frame_support::{
	pallet_prelude::Weight,
	traits::{
		tokens::{currency::Currency, ExistenceRequirement},
		Get, Imbalance,
	},
};
use sp_runtime::DispatchError;
use sp_std::marker::PhantomData;

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

pub struct TransferFromRewardsAccount<T> {
	_phantom: PhantomData<T>,
}
impl<Runtime: crate::Config, T> PayoutReward<Runtime, BalanceOf<Runtime>>
	for TransferFromRewardsAccount<T>
where
	T: Get<Runtime::AccountId>,
{
	fn payout_collator_rewards(
		for_round: crate::RoundIndex,
		collator_id: Runtime::AccountId,
		amount: crate::BalanceOf<Runtime>,
	) -> Weight {
		crate::Pallet::<Runtime>::send_collator_reward(for_round, T::get(), collator_id, amount)
		// crate::Pallet::<Runtime>::mint_collator_reward(for_round, collator_id, amount)
	}

	fn payout(
		delegator_id: &Runtime::AccountId,
		amount: crate::BalanceOf<Runtime>,
	) -> Result<crate::BalanceOf<Runtime>, DispatchError> {
		// Runtime::Currency::transfer(
		// 	&Runtime::RewardsAccount::get(),
		// 	&delegator_id,
		// 	amount,
		// 	ExistenceRequirement::KeepAlive,
		// )
		// .map(|_| amount)
		// .or_else(|e| match e {
		// 	DispatchError::Arithmetic(ArithmeticError::Underflow) => Ok(Zero::zero()),
		// 	_ => Err(e),
		// })
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
		source_id: T::AccountId,
		collator_id: T::AccountId,
		amt: BalanceOf<T>,
	) -> Weight {
		if T::Currency::transfer(&source_id, &collator_id, amt, ExistenceRequirement::KeepAlive)
			.is_ok()
		{
			Self::deposit_event(Event::Rewarded { account: collator_id.clone(), rewards: amt });
		}
		Weight::zero() // TODO: weight
	}
}
