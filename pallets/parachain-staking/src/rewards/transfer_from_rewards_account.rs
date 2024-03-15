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
		_for_round: crate::RoundIndex,
		collator_id: Runtime::AccountId,
		amount: crate::BalanceOf<Runtime>,
	) -> Weight {
		let rewards_account = RewardsAccount::<Runtime>::get().unwrap();
		crate::Pallet::<Runtime>::send_collator_reward(rewards_account, collator_id, amount)
	}

	fn payout(
		delegator_id: &Runtime::AccountId,
		amount: crate::BalanceOf<Runtime>,
	) -> Result<crate::BalanceOf<Runtime>, DispatchError> {
		// Early return if amount is zero or rewards account is not set.
		if amount.is_zero() || RewardsAccount::<Runtime>::get().is_none() {
			return Ok(Zero::zero());
		}

		// Ensure the destination account exists.
		ensure!(
			frame_system::Account::<Runtime>::contains_key(delegator_id),
			"Destination Account does not exist"
		);

		// Unwrap is safe here because we checked is_none() above.
		let rewards_account = RewardsAccount::<Runtime>::get().unwrap();

		// Attempt to transfer from rewards to delegator, handling the specific underflow error.
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

// tests
#[cfg(test)]
mod tests {
	use super::*;
	use crate::mock::*;
	use frame_support::{assert_err, assert_ok};
	use sp_runtime::TokenError;

	#[test]
	fn payout_to_account_0_fails() {
		ExtBuilder::default().build().execute_with(|| {
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
	fn payout_0_amount_succeed() {
		ExtBuilder::default().build().execute_with(|| {
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
	fn payout_100_amount_succeed() {
		ExtBuilder::default().build().execute_with(|| {
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
	fn payout_100_with_no_funds_in_rewards_account_should_succeed() {
		ExtBuilder::default().with_rewards_account_balance(0).build().execute_with(|| {
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

	#[test]
	fn payout_100_with_rewards_account_should_succeed() {
		ExtBuilder::default().with_rewards_account_balance(0).build().execute_with(|| {
			let delegator = 0;
			let amount = 100;

			let _ = pallet_balances::Pallet::<Test>::deposit_creating(&delegator, 1);
			RewardsAccount::<Test>::kill();

			assert_ok!(
				<TransferFromRewardsAccount as PayoutReward<Test, Balance>>::payout(
					&delegator, amount
				),
				0
			);
		});
	}
}
