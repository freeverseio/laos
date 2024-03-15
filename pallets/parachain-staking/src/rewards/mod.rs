mod minting_rewards;
mod transfer_from_rewards_account;

pub use minting_rewards::MintingRewards;
pub use transfer_from_rewards_account::TransferFromRewardsAccount;

use crate::{traits::PayoutReward, BalanceOf, Config, Event, Pallet, WeightInfo};
use frame_support::{
	pallet_prelude::Weight,
	traits::{Currency, ExistenceRequirement},
};

impl<T: Config> Pallet<T> {
	/// Mint a specified reward amount to the collator's account. Emits the [Rewarded] event.
	pub(crate) fn mint_collator_reward(collator_id: T::AccountId, amt: BalanceOf<T>) -> Weight {
		if let Ok(amount_transferred) = T::PayoutReward::payout(&collator_id, amt) {
			Self::deposit_event(Event::Rewarded {
				account: collator_id.clone(),
				rewards: amount_transferred,
			});
		}
		T::WeightInfo::mint_collator_reward()
	}

	pub fn send_collator_reward(
		source: T::AccountId,
		collator_id: T::AccountId,
		amt: BalanceOf<T>,
	) -> Weight {
		match T::Currency::transfer(&source, &collator_id, amt, ExistenceRequirement::KeepAlive) {
			Ok(_) => {
				Self::deposit_event(Event::Rewarded { account: collator_id.clone(), rewards: amt });
			},
			Err(e) => log::error!(
				"💥 Failed to send reward to collator: {:?} from: {:?}, to: {:?}, amount: {:?}",
				e,
				source,
				collator_id,
				amt
			),
		}

		Weight::zero() // TODO: weight
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	use crate::mock::*;

	#[test]
	fn mint_collator_rewards_of_unexistent_account_do_not_succeed() {
		ExtBuilder::default().build().execute_with(|| {
			let collator = 1;
			let amount = 100;
			System::set_block_number(1);

			Pallet::<Test>::mint_collator_reward(collator, amount);

			assert_eq!(System::events().len(), 0);
		})
	}

	#[test]
	fn mint_collator_0_rewards_succeed() {
		ExtBuilder::default().build().execute_with(|| {
			let collator = 1;
			System::set_block_number(1);

			let _ = pallet_balances::Pallet::<Test>::deposit_creating(&collator, 1);
			Pallet::<Test>::mint_collator_reward(collator, 0);

			assert_events_eq_match!(Event::Rewarded { account: 1, rewards: 0 },);
		})
	}

	#[test]
	fn mint_collator_rewards_of_existent_account_succeed() {
		ExtBuilder::default().build().execute_with(|| {
			let collator = 1;

			System::set_block_number(1);

			assert_eq!(System::events().len(), 0);
			let _ = pallet_balances::Pallet::<Test>::deposit_creating(&collator, 1);

			Pallet::<Test>::mint_collator_reward(collator, 100);

			assert_events_eq_match!(Event::Rewarded { account: 1, rewards: 100 },);
		})
	}

	#[test]
	fn send_collator_rewards_to_unexistent_account_should_success() {
		ExtBuilder::default().build().execute_with(|| {
			let source = 2;
			let collator = 1;
			System::set_block_number(1);

			Pallet::<Test>::send_collator_reward(source, collator, 100);

			assert_eq!(System::events().len(), 0);
		})
	}

	#[test]
	fn send_collator_0_rewards_succeed() {
		ExtBuilder::default().build().execute_with(|| {
			let source = 2;
			let collator = 1;
			System::set_block_number(1);

			let _ = pallet_balances::Pallet::<Test>::deposit_creating(&source, 1);
			let _ = pallet_balances::Pallet::<Test>::deposit_creating(&collator, 1);
			Pallet::<Test>::send_collator_reward(source, collator, 0);

			assert_events_eq_match!(Event::Rewarded { account: 1, rewards: 0 },);
		})
	}

	#[test]
	fn send_collator_rewards_of_existent_account_succeed() {
		ExtBuilder::default().build().execute_with(|| {
			let source = 2;
			let collator = 1;

			System::set_block_number(1);

			assert_eq!(System::events().len(), 0);

			let _ = pallet_balances::Pallet::<Test>::deposit_creating(&source, 100);
			let _ = pallet_balances::Pallet::<Test>::deposit_creating(&collator, 1);

			Pallet::<Test>::send_collator_reward(source, collator, 100);

			assert_events_eq_match!(Event::Rewarded { account: 1, rewards: 100 },);
		})
	}
}
