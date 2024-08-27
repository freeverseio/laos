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

use crate::{traits::PayoutReward, BalanceOf, Config, Event, Pallet, RoundIndex, WeightInfo};
use frame_support::{
	pallet_prelude::Weight,
	traits::{tokens::currency::Currency, Imbalance},
};
use sp_runtime::DispatchError;

pub struct MintingRewards;
impl<Runtime: crate::Config> PayoutReward<Runtime> for MintingRewards {
	fn payout_collator_rewards(
		for_round: RoundIndex,
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

impl<T: Config> Pallet<T> {
	/// Mint a specified reward amount to the collator's account. Emits the [Rewarded] event.
	pub(crate) fn mint_collator_reward(
		_round_idx: RoundIndex,
		collator_id: T::AccountId,
		amt: BalanceOf<T>,
	) -> Weight {
		match T::Currency::deposit_into_existing(&collator_id, amt) {
			Ok(amount_transferred) => {
				Self::deposit_event(Event::Rewarded {
					account: collator_id.clone(),
					rewards: amount_transferred.peek(),
				});
			},
			Err(e) => {
				log::error!(
					"Failed to deposit reward of {:?} to collator {:?}: {:?}",
					amt,
					collator_id,
					e
				);
			},
		}
		T::WeightInfo::mint_collator_reward()
	}
}

// tests
#[cfg(test)]
mod tests {
	use super::*;
	use crate::mock::*;

	#[test]
	fn test_mint_reward_for_nonexistent_collator_does_not_emit_event() {
		ExtBuilder::default().build().execute_with(|| {
			let collator = 1;
			let amount = 100;

			Pallet::<Test>::mint_collator_reward(0, collator, amount);

			assert_eq!(System::events().len(), 0);
		})
	}

	#[test]
	fn test_mint_zero_rewards_for_collator_emits_rewarded_event() {
		ExtBuilder::default().build().execute_with(|| {
			let collator = 1;

			let _ = pallet_balances::Pallet::<Test>::deposit_creating(&collator, 1);
			Pallet::<Test>::mint_collator_reward(0, collator, 0);

			assert_events_eq_match!(Event::Rewarded { account: 1, rewards: 0 },);
		})
	}

	#[test]
	fn test_mint_reward_for_existing_collator_emits_rewarded_event() {
		ExtBuilder::default().with_rewards_account(999, 100).build().execute_with(|| {
			let collator = 1;

			assert_eq!(System::events().len(), 0);
			let _ = pallet_balances::Pallet::<Test>::deposit_creating(&collator, 1);

			Pallet::<Test>::mint_collator_reward(0, collator, 100);

			assert_events_eq_match!(Event::Rewarded { account: 1, rewards: 100 },);
		})
	}
}
