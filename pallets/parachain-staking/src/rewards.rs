use crate::{traits::PayoutReward, BalanceOf, *};
use frame_support::{
	ensure,
	pallet_prelude::Weight,
	traits::{
		tokens::{currency::Currency, ExistenceRequirement},
		Imbalance,
	},
};
use sp_runtime::{traits::Zero, ArithmeticError, DispatchError};

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

pub struct TransferFromRewardsAccount;
impl<Runtime: crate::Config> PayoutReward<Runtime, BalanceOf<Runtime>>
	for TransferFromRewardsAccount
{
	fn payout_collator_rewards(
		for_round: crate::RoundIndex,
		collator_id: Runtime::AccountId,
		amount: crate::BalanceOf<Runtime>,
	) -> Weight {
		crate::Pallet::<Runtime>::send_collator_reward(for_round, collator_id, amount)
	}

	fn payout(
		delegator_id: &Runtime::AccountId,
		amount: crate::BalanceOf<Runtime>,
	) -> Result<crate::BalanceOf<Runtime>, DispatchError> {
		ensure!(
			frame_system::Account::<Runtime>::contains_key(delegator_id),
			"Account does not exist"
		);

		let rewards_account = RewardsAccount::<Runtime>::get().unwrap();

		Runtime::Currency::transfer(
			&rewards_account,
			&delegator_id,
			amount,
			ExistenceRequirement::KeepAlive,
		)
		.map(|_| amount)
		.or_else(|e| match e {
			DispatchError::Arithmetic(ArithmeticError::Underflow) => Ok(Zero::zero()),
			_ => Err(e),
		})
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
		if T::Currency::transfer(
			&RewardsAccount::<T>::get().unwrap(),
			&collator_id,
			amt,
			ExistenceRequirement::KeepAlive,
		)
		.is_ok()
		{
			Self::deposit_event(Event::Rewarded { account: collator_id.clone(), rewards: amt });
		}
		Weight::zero() // TODO: weight
	}
}
