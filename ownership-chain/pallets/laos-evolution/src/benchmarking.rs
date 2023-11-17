//! Benchmarking setup for pallet-living-assets-evolution
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as LaosEvolution;
use frame_benchmarking::v2::*;

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn create_collection() {
		let caller: T::AccountId = whitelisted_caller();
		let owner = caller.clone();

		#[block]
		{
			LaosEvolution::<T>::create_collection(owner.clone()).unwrap();
		}
		assert_eq!(CollectionOwner::<T>::get(0), Some(caller));
	}

	impl_benchmark_test_suite!(LaosEvolution, crate::mock::new_test_ext(), crate::mock::Test);
}
