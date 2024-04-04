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

//! traits for parachain-staking

use crate::{weights::WeightInfo, BalanceOf};
use frame_support::{dispatch::PostDispatchInfo, pallet_prelude::Weight};
use sp_runtime::{DispatchError, DispatchErrorWithPostInfo};

pub trait OnCollatorPayout<AccountId, Balance> {
	fn on_collator_payout(
		for_round: crate::RoundIndex,
		collator_id: AccountId,
		amount: Balance,
	) -> Weight;
}
impl<AccountId, Balance> OnCollatorPayout<AccountId, Balance> for () {
	fn on_collator_payout(
		_for_round: crate::RoundIndex,
		_collator_id: AccountId,
		_amount: Balance,
	) -> Weight {
		Weight::zero()
	}
}

pub trait OnNewRound {
	fn on_new_round(round_index: crate::RoundIndex) -> Weight;
}
impl OnNewRound for () {
	fn on_new_round(_round_index: crate::RoundIndex) -> Weight {
		Weight::zero()
	}
}

/// Defines the behavior to payout the block producer reward.
pub trait PayoutReward<Runtime: crate::Config> {
	/// Send amount to the balance of the specified account (collator).
	/// and corresponding weight consumed is returned.
	fn payout_collator_rewards(
		round_index: crate::RoundIndex,
		collator: Runtime::AccountId,
		amount: BalanceOf<Runtime>,
	) -> Weight;

	/// Send amount to the free balance of the specified account (destination).
	/// If the account (destination) does not exist, the operation is not carried out,
	/// and an error is returned instead.
	fn payout(
		destination: &Runtime::AccountId,
		amount: BalanceOf<Runtime>,
	) -> Result<BalanceOf<Runtime>, DispatchError>;
}

pub trait OnInactiveCollator<Runtime: crate::Config> {
	fn on_inactive_collator(
		collator_id: Runtime::AccountId,
		round: crate::RoundIndex,
	) -> Result<Weight, DispatchErrorWithPostInfo<PostDispatchInfo>>;
}

impl<Runtime: crate::Config> OnInactiveCollator<Runtime> for () {
	fn on_inactive_collator(
		collator_id: <Runtime>::AccountId,
		_round: crate::RoundIndex,
	) -> Result<Weight, DispatchErrorWithPostInfo<PostDispatchInfo>> {
		crate::Pallet::<Runtime>::go_offline_inner(collator_id)?;
		Ok(<Runtime as crate::Config>::WeightInfo::go_offline(crate::MAX_CANDIDATES))
	}
}
