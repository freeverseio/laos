//! Benchmarking setup for pallet-living-assets-evolution
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as LaosEvolution;
use frame_benchmarking::v2::*;
use sp_std::vec;

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

	#[benchmark]
	fn mint_with_external_uri(s: Linear<0, { <T as Config>::MaxTokenUriLength::get() }>) {
		let caller: T::AccountId = whitelisted_caller();
		let owner = caller.clone();
		let collection_id = LaosEvolution::<T>::create_collection(owner.clone()).unwrap();

		#[block]
		{
			let token_id = LaosEvolution::<T>::mint_with_external_uri(
				owner.clone(),
				collection_id,
				s as Slot,
				owner.clone(),
				vec![1u8; s as usize].try_into().unwrap(),
			)
			.unwrap();

			assert_eq!(
				LaosEvolution::<T>::token_uri(collection_id, token_id),
				Some(vec![1u8; s as usize].try_into().unwrap())
			);
		};
	}

	#[benchmark]
	fn evolve_with_external_uri(s: Linear<0, { <T as Config>::MaxTokenUriLength::get() }>) {
		let caller: T::AccountId = whitelisted_caller();
		let owner = caller.clone();
		let collection_id = LaosEvolution::<T>::create_collection(owner.clone()).unwrap();
		let token_id = LaosEvolution::<T>::mint_with_external_uri(
			owner.clone(),
			0,
			s as Slot,
			owner.clone(),
			vec![0u8; s as usize].try_into().unwrap(),
		)
		.unwrap();

		#[block]
		{
			LaosEvolution::<T>::evolve_with_external_uri(
				owner.clone(),
				collection_id,
				token_id,
				vec![1u8; s as usize].try_into().unwrap(),
			)
			.unwrap();
		}

		assert_eq!(
			LaosEvolution::<T>::token_uri(collection_id, token_id),
			Some(vec![1u8; s as usize].try_into().unwrap())
		);
	}
}
