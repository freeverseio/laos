mod minting_rewards;
mod transfer_from_rewards_account;

pub use minting_rewards::MintingRewards;
pub use transfer_from_rewards_account::TransferFromRewardsAccount;

use crate::{BalanceOf, Config, Event, Pallet, RewardsAccount, RoundIndex, WeightInfo};
use frame_support::{
	pallet_prelude::Weight,
	traits::{Currency, ExistenceRequirement, Imbalance},
};

impl<T: Config> Pallet<T> {
	/// Mint a specified reward amount to the collator's account. Emits the [Rewarded] event.
	pub(crate) fn mint_collator_reward(
		_round_idx: RoundIndex,
		collator_id: T::AccountId,
		amt: BalanceOf<T>,
	) -> Weight {
		if let Ok(amount_transferred) = T::Currency::deposit_into_existing(&collator_id, amt) {
			Self::deposit_event(Event::Rewarded {
				account: collator_id.clone(),
				rewards: amount_transferred.peek(),
			});
		}
		T::WeightInfo::mint_collator_reward()
	}

	pub fn send_collator_reward(
		_round_idx: RoundIndex,
		collator_id: T::AccountId,
		amt: BalanceOf<T>,
	) -> Weight {
		// Check if the collator's account exists; return early if not.
		if !frame_system::Account::<T>::contains_key(&collator_id) {
			return Weight::zero(); // TODO
		}

		// Attempt to get the RewardsAccount and return early if not set.
		let rewards_account = match RewardsAccount::<T>::get() {
			Some(account) => account,
			None => {
				return Weight::zero(); // TODO Adjust with the actual weight for a missing rewards account.
			},
		};

		// Proceed with the transfer and handle the result.
		let transfer_result = T::Currency::transfer(
			&rewards_account,
			&collator_id,
			amt,
			ExistenceRequirement::KeepAlive,
		);

		match transfer_result {
			Ok(_) => {
				Self::deposit_event(Event::Rewarded { account: collator_id, rewards: amt });
			},
			Err(e) =>
				log::error!("💥 Failed to send reward to collator: {:?}, amount: {:?}", e, amt),
		}

		Weight::zero() // TODO
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	use crate::mock::*;

	#[test]
	fn test_mint_reward_for_nonexistent_collator_does_not_emit_event() {
		ExtBuilder::default().build().execute_with(|| {
			let collator = 1;
			let amount = 100;
			System::set_block_number(1);

			Pallet::<Test>::mint_collator_reward(0, collator, amount);

			assert_eq!(System::events().len(), 0);
		})
	}

	#[test]
	fn test_mint_zero_rewards_for_collator_emits_rewarded_event() {
		ExtBuilder::default().build().execute_with(|| {
			let collator = 1;
			System::set_block_number(1);

			let _ = pallet_balances::Pallet::<Test>::deposit_creating(&collator, 1);
			Pallet::<Test>::mint_collator_reward(0, collator, 0);

			assert_events_eq_match!(Event::Rewarded { account: 1, rewards: 0 },);
		})
	}

	#[test]
	fn test_mint_reward_for_existing_collator_emits_rewarded_event() {
		ExtBuilder::default().with_rewards_account(999, 100).build().execute_with(|| {
			let collator = 1;

			System::set_block_number(1);

			assert_eq!(System::events().len(), 0);
			let _ = pallet_balances::Pallet::<Test>::deposit_creating(&collator, 1);

			Pallet::<Test>::mint_collator_reward(0, collator, 100);

			assert_events_eq_match!(Event::Rewarded { account: 1, rewards: 100 },);
		})
	}

	#[test]
	fn test_send_reward_to_nonexistent_collator_does_not_emit_event() {
		ExtBuilder::default().with_rewards_account(2, 100).build().execute_with(|| {
			let collator = 1;
			System::set_block_number(1);

			Pallet::<Test>::send_collator_reward(0, collator, 100);

			assert_eq!(System::events().len(), 0);
		})
	}

	#[test]
	fn test_send_zero_rewards_to_collator_emits_rewarded_event() {
		ExtBuilder::default().with_rewards_account(2, 1).build().execute_with(|| {
			let collator = 1;
			let amount = 0;
			System::set_block_number(1);

			let _ = pallet_balances::Pallet::<Test>::deposit_creating(&collator, 1);
			Pallet::<Test>::send_collator_reward(0, collator, amount);

			assert_events_eq_match!(Event::Rewarded { account: 1, rewards: 0 },);
		})
	}

	#[test]
	fn test_send_rewards_to_existing_collator_emits_rewarded_event() {
		ExtBuilder::default().with_rewards_account(2, 100).build().execute_with(|| {
			let collator = 1;

			System::set_block_number(1);

			assert_eq!(System::events().len(), 0);

			let _ = pallet_balances::Pallet::<Test>::deposit_creating(&collator, 1);

			Pallet::<Test>::send_collator_reward(0, collator, 100);

			assert_events_eq_match!(Event::Rewarded { account: 1, rewards: 100 },);
		})
	}
}
