mod minting_rewards;
mod transfer_from_rewards_account;

pub use minting_rewards::MintingRewards;
pub use transfer_from_rewards_account::TransferFromRewardsAccount;

use crate::{
	traits::PayoutReward, BalanceOf, Config, Event, Pallet, RewardsAccount, RoundIndex, WeightInfo,
};
use frame_support::{
	pallet_prelude::Weight,
	traits::{Currency, ExistenceRequirement},
};

impl<T: Config> Pallet<T> {
	/// Mint a specified reward amount to the collator's account. Emits the [Rewarded] event.
	pub(crate) fn mint_collator_reward(
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
		// if RewardAccount is not set then return
		if RewardsAccount::<T>::get().is_none() {
			return Weight::zero(); // TODO RewardsAccount should not be an Option
		}

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
