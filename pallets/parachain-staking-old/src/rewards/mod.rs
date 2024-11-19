// Copyright 2023-2024 Freeverse.io
// This file is part of LAOS.

// LAOS is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// LAOS is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with LAOS.  If not, see <http://www.gnu.org/licenses/>.

mod mint_rewards;
mod transfer_from_rewards_account;

pub use mint_rewards::MintingRewards;
pub use transfer_from_rewards_account::TransferFromRewardsAccount;

// These tests aim to verify the PayoutReward trait's behavior through its concrete implementations,
// ensuring they function as anticipated.
#[cfg(test)]
mod tests {
	use super::*;
	use crate::{mock::*, Error, Event, PayoutReward, RoundIndex};
	use frame_support::{
		assert_err, assert_ok, pallet_prelude::Weight, traits::tokens::currency::Currency,
	};
	use sp_runtime::DispatchError;

	fn paying_collator_rewards<T: PayoutReward<Test>>(
		round_index: RoundIndex,
		collator: AccountId,
		amount: Balance,
	) -> Weight {
		T::payout_collator_rewards(round_index, collator, amount)
	}

	fn paying<T: PayoutReward<Test>>(
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

			assert_eq!(pallet_balances::Pallet::<Test>::free_balance(collator), 0);

			assert_no_events!();
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

			assert_eq!(pallet_balances::Pallet::<Test>::free_balance(collator), 17);

			assert_events_eq_match!(
				Event::Rewarded { account: 10, rewards: 8 },
				Event::Rewarded { account: 10, rewards: 8 },
			);
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

			assert_eq!(pallet_balances::Pallet::<Test>::free_balance(collator), 1);

			assert_events_eq_match!(
				Event::Rewarded { account: 10, rewards: 0 },
				Event::Rewarded { account: 10, rewards: 0 },
			);
		});
	}

	#[test]
	fn test_payout_nonexistent_account_fails() {
		ExtBuilder::default().with_rewards_account(999, 100).build().execute_with(|| {
			let delegator = 9;
			let amount = 100;

			assert_err!(
				paying::<MintingRewards>(delegator, amount),
				pallet_balances::Error::<Test>::DeadAccount
			);
			assert_err!(
				paying::<TransferFromRewardsAccount>(delegator, amount),
				Error::<Test>::DeadAccount
			);
		});
	}

	#[test]
	fn test_payout_zero_amount() {
		ExtBuilder::default().with_rewards_account(999, 100).build().execute_with(|| {
			let delegator = 9;
			let amount = 0;

			let _ = pallet_balances::Pallet::<Test>::deposit_creating(&delegator, 1);

			assert_ok!(paying::<MintingRewards>(delegator, amount), 0);
			assert_ok!(paying::<TransferFromRewardsAccount>(delegator, amount), 0);

			assert_eq!(pallet_balances::Pallet::<Test>::free_balance(delegator), 1);
		});
	}

	#[test]
	fn test_payout_non_zero_amount() {
		ExtBuilder::default().with_rewards_account(999, 100).build().execute_with(|| {
			let delegator = 9;
			let amount = 100;

			let _ = pallet_balances::Pallet::<Test>::deposit_creating(&delegator, 1);

			assert_ok!(paying::<MintingRewards>(delegator, amount), 100);
			assert_ok!(paying::<TransferFromRewardsAccount>(delegator, amount), 100);

			assert_eq!(pallet_balances::Pallet::<Test>::free_balance(delegator), 201);
		});
	}
}
