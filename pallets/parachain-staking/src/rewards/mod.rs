mod minting_rewards;
mod transfer_from_rewards_account;

pub use minting_rewards::MintingRewards;
pub use transfer_from_rewards_account::TransferFromRewardsAccount;

// tests on the trait
#[cfg(test)]
mod tests {
	use super::*;
	use crate::{mock::*, PayoutReward, RoundIndex};
	use frame_support::{pallet_prelude::Weight, traits::tokens::currency::Currency};
	use sp_runtime::DispatchError;

	fn paying_collator_rewards<T: PayoutReward<Test, Balance>>(
		round_index: RoundIndex,
		collator: AccountId,
		amount: Balance,
	) -> Weight {
		T::payout_collator_rewards(round_index, collator, amount)
	}

	fn paying<T: PayoutReward<Test, Balance>>(
		destination: AccountId,
		amount: Balance,
	) -> Result<Balance, DispatchError> {
		T::payout(&destination, amount)
	}

	#[test]
	fn test_payout_unexistent_collator_does_nothing() {
		ExtBuilder::default().with_rewards_account(999, 100).build().execute_with(|| {
			let collator = 10;
			let amount = 8;
			let round_index = 0;

			paying_collator_rewards::<MintingRewards>(round_index, collator, amount);
			paying_collator_rewards::<TransferFromRewardsAccount>(round_index, collator, amount);

			assert_eq!(pallet_balances::Pallet::<Test>::free_balance(&collator), 0);
		});
	}

	#[test]
	fn test_payout_collator_non_zero_rewards() {
		ExtBuilder::default().with_rewards_account(999, 100).build().execute_with(|| {
			let collator = 10;
			let amount = 8;
			let round_index = 0;

			let _ = pallet_balances::Pallet::<Test>::deposit_creating(&collator, 1);

			paying_collator_rewards::<MintingRewards>(round_index, collator, amount);
			paying_collator_rewards::<TransferFromRewardsAccount>(round_index, collator, amount);

			assert_eq!(pallet_balances::Pallet::<Test>::free_balance(&collator), 17);
		});
	}

	#[test]
	fn test_payout_collator_zero_rewards() {
		ExtBuilder::default().with_rewards_account(999, 100).build().execute_with(|| {
			let collator = 10;
			let amount = 0;
			let round_index = 0;

			let _ = pallet_balances::Pallet::<Test>::deposit_creating(&collator, 1);

			paying_collator_rewards::<MintingRewards>(round_index, collator, amount);
			paying_collator_rewards::<TransferFromRewardsAccount>(round_index, collator, amount);

			assert_eq!(pallet_balances::Pallet::<Test>::free_balance(&collator), 1);
		});
	}
}
