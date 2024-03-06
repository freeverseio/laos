//! Benchmarking setup for pallet-living-assets-evolution
#![cfg(feature = "runtime-benchmarks")]
use super::*;
#[allow(unused)]
use crate::{Pallet as BlockRewardsSource, RewardsAccount};
use frame_benchmarking::v2::*;
use sp_std::vec;

#[benchmarks]
mod benchmarks {
	use super::*;
	#[benchmark]
	fn send_rewards() {
		let rewards_account: T::AccountId = account("rewards_account", 0, 0);
		let destination: T::AccountId = account("destination", 0, 0);
		let amount = 100u32;
		let _ = T::Currency::deposit_creating(&rewards_account, amount.into());
		let initial_destination_balance = T::Currency::free_balance(&destination);
		RewardsAccount::<T>::put(rewards_account.clone());
		#[block]
		{
			BlockRewardsSource::<T>::send_rewards(destination.clone(), amount.into()).unwrap();
		}
		assert_eq!(
			T::Currency::free_balance(&destination),
			initial_destination_balance + amount.into()
		);
	}
}
