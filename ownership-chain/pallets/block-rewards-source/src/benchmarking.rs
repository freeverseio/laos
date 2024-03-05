//! Benchmarking setup for pallet-living-assets-evolution
#![cfg(feature = "runtime-benchmarks")]
use super::*;
#[allow(unused)]
use crate::{Pallet as BlockRewardsSource, RewardsAccount};
use frame_benchmarking::v2::*;
use pallet_parachain_staking::PayoutCollatorReward;
use sp_std::vec;

#[benchmarks(where
    T: Send + Sync + crate::Config + pallet_parachain_staking::Config
)]
#[benchmarks]
mod benchmarks {
	use super::*;
	#[benchmark]
	fn payout_collator_reward() {
		let rewards_account: T::AccountId = account("rewards_account", 0, 0);
		let collator: T::AccountId = account("collator", 0, 0);
		let amount = 100u32;
		<T as pallet_parachain_staking::Config>::Currency::deposit_creating(
			&rewards_account,
			amount.into(),
		);
		let initial_collator_balance =
			<T as pallet_parachain_staking::Config>::Currency::free_balance(&collator);
		RewardsAccount::<T>::put(rewards_account.clone());
		#[block]
		{
			BlockRewardsSource::<T>::payout_collator_reward(0, collator.clone(), amount.into());
		}
		assert_eq!(
			<T as pallet_parachain_staking::Config>::Currency::free_balance(&collator),
			initial_collator_balance + amount.into()
		);
	}
}
