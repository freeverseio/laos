use crate::{traits::PayoutReward, BalanceOf, RewardsAccount};
use frame_support::{
	ensure,
	pallet_prelude::Weight,
	traits::tokens::{currency::Currency, ExistenceRequirement},
};
use sp_runtime::{traits::Zero, ArithmeticError, DispatchError};

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
			"Destination Account does not exist"
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

#[cfg(test)]
mod tests {
	use super::*;
	use crate::mock::*;
	use frame_support::{assert_err, assert_ok};

	#[test]
	fn test_payout_collator_rewards_without_rewards_account_does_not_panic() {
		ExtBuilder::default().build().execute_with(|| {
			let collator = 1;
			let amount = 100;

			<TransferFromRewardsAccount as PayoutReward<Test, Balance>>::payout_collator_rewards(
				0, collator, amount,
			);
		});
	}

	#[test]
	fn test_payout_to_nonexistent_account_fails() {
		ExtBuilder::default().with_rewards_account(999, 100).build().execute_with(|| {
			let delegator = 0;
			let amount = 100;

			assert_err!(
				<TransferFromRewardsAccount as PayoutReward<Test, Balance>>::payout(
					&delegator, amount
				),
				"Destination Account does not exist"
			);
		});
	}

	#[test]
	fn test_payout_with_zero_amount_succeeds() {
		ExtBuilder::default().with_rewards_account(999, 100).build().execute_with(|| {
			let delegator = 0;
			let amount = 0;

			assert_ok!(
				<TransferFromRewardsAccount as PayoutReward<Test, Balance>>::payout(
					&delegator, amount
				),
				0
			);
		});
	}

	#[test]
	fn test_payout_with_nonzero_amount_succeeds() {
		ExtBuilder::default().with_rewards_account(999, 100).build().execute_with(|| {
			let delegator = 0;
			let amount = 100;

			let _ = pallet_balances::Pallet::<Test>::deposit_creating(&delegator, 1);

			assert_ok!(
				<TransferFromRewardsAccount as PayoutReward<Test, Balance>>::payout(
					&delegator, amount
				),
				100
			);
		});
	}

	#[test]
	fn test_payout_with_insufficient_rewards_account_funds_succeeds() {
		ExtBuilder::default().with_rewards_account(999, 10).build().execute_with(|| {
			let delegator = 0;
			let amount = 100;

			let _ = pallet_balances::Pallet::<Test>::deposit_creating(&delegator, 1);

			assert_ok!(
				<TransferFromRewardsAccount as PayoutReward<Test, Balance>>::payout(
					&delegator, amount
				),
				0
			);
		});
	}
}
