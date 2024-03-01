//! Benchmarking setup for pallet-living-assets-evolution
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as BlockRewardsSource;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn set_rewards_account() {
		let rewards_account: T::AccountId = account("rewards_account", 0, 0);

		#[block]
		{
			BlockRewardsSource::<T>::set_rewards_account(
				RawOrigin::Root.into(),
				rewards_account.clone(),
			)
			.unwrap();
		}
		assert_eq!(BlockRewardsSource::<T>::rewards_account(), Some(rewards_account));
	}
}
