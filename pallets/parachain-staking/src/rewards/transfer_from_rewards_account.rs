use crate::{
	traits::PayoutReward, BalanceOf, Config, Error, Event, Pallet, RewardsAccount, RoundIndex,
};
use frame_support::{
	ensure,
	pallet_prelude::Weight,
	traits::tokens::{currency::Currency, ExistenceRequirement},
};
use sp_runtime::{traits::Zero, ArithmeticError, DispatchError};

pub struct TransferFromRewardsAccount;
impl<Runtime: crate::Config> PayoutReward<Runtime> for TransferFromRewardsAccount {
	fn payout_collator_rewards(
		for_round: crate::RoundIndex,
		collator_id: Runtime::AccountId,
		amount: crate::BalanceOf<Runtime>,
	) -> Weight {
		crate::Pallet::<Runtime>::send_collator_rewards(for_round, collator_id, amount)
	}

	fn payout(
		delegator_id: &Runtime::AccountId,
		amount: crate::BalanceOf<Runtime>,
	) -> Result<crate::BalanceOf<Runtime>, DispatchError> {
		// Early return if amount is zero,
		if amount.is_zero() {
			return Ok(Zero::zero());
		}

		// Early return if RewardsAccount is not set.
		let rewards_account = match RewardsAccount::<Runtime>::get() {
			Some(account) => account,
			None => return Ok(Zero::zero()),
		};

		// Ensure the destination account exists with a clearer error message.
		ensure!(
			frame_system::Account::<Runtime>::contains_key(delegator_id),
			Error::<Runtime>::DeadAccount
		);

		// Directly handle the result of the transfer, making use of match
		// for clearer error handling.
		match Runtime::Currency::transfer(
			&rewards_account,
			&delegator_id,
			amount,
			ExistenceRequirement::KeepAlive,
		) {
			Ok(_) => Ok(amount),
			Err(DispatchError::Arithmetic(ArithmeticError::Underflow)) => Ok(Zero::zero()),
			Err(e) => Err(e),
		}
	}
}

impl<T: Config> Pallet<T> {
	pub fn send_collator_rewards(
		_round_idx: RoundIndex,
		collator_id: T::AccountId,
		amount: BalanceOf<T>,
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
			amount,
			ExistenceRequirement::KeepAlive,
		);

		if let Err(e) = transfer_result {
			log::error!("💥 Failed to send rewards to collator: {:?}, amount: {:?}", e, amount);
		} else {
			Self::deposit_event(Event::Rewarded { account: collator_id, rewards: amount });
		}

		Weight::zero() // TODO
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::mock::*;
	use frame_support::{assert_ok, assert_storage_noop};

	#[test]
	fn test_payout_collator_without_rewards_account() {
		ExtBuilder::default().build().execute_with(|| {
			let collator = 10;
			let amount = 8;
			let round_index = 0;

			let _ = pallet_balances::Pallet::<Test>::deposit_creating(&collator, 1);

			<TransferFromRewardsAccount as PayoutReward<Test>>::payout_collator_rewards(
				round_index,
				collator,
				amount,
			);

			assert_eq!(pallet_balances::Pallet::<Test>::free_balance(&collator), 1);

			assert_no_events!();
		});
	}

	#[test]
	fn test_payout_collator_with_not_enough_funds_in_rewards_account() {
		ExtBuilder::default().with_rewards_account(999, 7).build().execute_with(|| {
			let collator = 10;
			let amount = 8;
			let round_index = 0;

			let _ = pallet_balances::Pallet::<Test>::deposit_creating(&collator, 1);

			<TransferFromRewardsAccount as PayoutReward<Test>>::payout_collator_rewards(
				round_index,
				collator,
				amount,
			);

			assert_eq!(pallet_balances::Pallet::<Test>::free_balance(&collator), 1);

			assert_no_events!();
		});
	}

	#[test]
	fn test_payout_with_no_rewards_account_should_do_nothing() {
		ExtBuilder::default().build().execute_with(|| {
			let delegator = 0;
			let amount = 100;

			let _ = pallet_balances::Pallet::<Test>::deposit_creating(&delegator, 1);

			assert_storage_noop!(<TransferFromRewardsAccount as PayoutReward<Test>>::payout(
				&delegator, amount
			));
		});
	}

	#[test]
	fn test_payout_with_insufficient_rewards_account_funds_succeeds() {
		ExtBuilder::default().with_rewards_account(999, 10).build().execute_with(|| {
			let delegator = 0;
			let amount = 100;

			let _ = pallet_balances::Pallet::<Test>::deposit_creating(&delegator, 1);

			assert_ok!(
				<TransferFromRewardsAccount as PayoutReward<Test>>::payout(&delegator, amount),
				0
			);
		});
	}

	#[test]
	fn test_send_reward_to_nonexistent_collator_does_not_emit_event() {
		ExtBuilder::default().with_rewards_account(2, 100).build().execute_with(|| {
			let collator = 1;

			Pallet::<Test>::send_collator_rewards(0, collator, 100);

			assert_no_events!()
		})
	}

	#[test]
	fn test_send_zero_rewards_to_collator_emits_rewarded_event() {
		ExtBuilder::default().with_rewards_account(2, 1).build().execute_with(|| {
			let collator = 1;
			let amount = 0;

			let _ = pallet_balances::Pallet::<Test>::deposit_creating(&collator, 1);
			Pallet::<Test>::send_collator_rewards(0, collator, amount);

			assert_events_eq_match!(Event::Rewarded { account: 1, rewards: 0 },);
		})
	}

	#[test]
	fn test_send_rewards_to_existing_collator_emits_rewarded_event() {
		ExtBuilder::default().with_rewards_account(2, 100).build().execute_with(|| {
			let collator = 1;

			assert_eq!(System::events().len(), 0);

			let _ = pallet_balances::Pallet::<Test>::deposit_creating(&collator, 1);

			Pallet::<Test>::send_collator_rewards(0, collator, 100);

			assert_events_eq_match!(Event::Rewarded { account: 1, rewards: 100 },);
		})
	}
}
