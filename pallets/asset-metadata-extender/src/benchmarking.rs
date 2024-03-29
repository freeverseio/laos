//! Benchmarking setup for pallet-living-assets-evolution
#![cfg(feature = "runtime-benchmarks")]
use super::*;

use crate::traits::AssetMetadataExtender as AssetMetadataExtenderT;
#[allow(unused)]
use crate::Pallet as AssetMetadataExtender;
use frame_benchmarking::v2::*;
use sp_std::vec;

#[benchmarks]
mod benchmarks {
	use super::*;

	impl_benchmark_test_suite!(
		AssetMetadataExtender,
		crate::mock::new_test_ext(),
		crate::mock::Test
	);

	#[benchmark]
	fn create_token_uri_extension(
		t: Linear<0, { <T as Config>::MaxTokenUriLength::get() }>,
		u: Linear<0, { <T as Config>::MaxUniversalLocationLength::get() }>,
	) {
		let claimer: T::AccountId = whitelisted_caller();

		let ul: UniversalLocationOf<T> = vec![1u8; u as usize].try_into().unwrap();
		let token_uri: TokenUriOf<T> = vec![1u8; t as usize].try_into().unwrap();

		#[block]
		{
			AssetMetadataExtender::<T>::create_token_uri_extension(
				claimer.clone(),
				ul.clone(),
				token_uri.clone(),
			)
			.unwrap();
		};

		assert_eq!(
			AssetMetadataExtender::<T>::token_uris_by_claimer_and_location(claimer, ul,),
			Some(token_uri)
		);
	}

	#[benchmark]
	fn update_token_uri_extension(
		t: Linear<0, { <T as Config>::MaxTokenUriLength::get() }>,
		u: Linear<0, { <T as Config>::MaxUniversalLocationLength::get() }>,
	) {
		let claimer: T::AccountId = whitelisted_caller();
		let universal_location: UniversalLocationOf<T> = vec![1u8; u as usize].try_into().unwrap();
		let token_uri: TokenUriOf<T> = vec![1u8; t as usize].try_into().unwrap();

		{
			AssetMetadataExtender::<T>::create_token_uri_extension(
				claimer.clone(),
				universal_location.clone(),
				token_uri,
			)
			.unwrap();
		};

		let new_token_uri: TokenUriOf<T> = vec![2u8; t as usize].try_into().unwrap();

		#[block]
		{
			AssetMetadataExtender::<T>::update_token_uri_extension(
				claimer,
				universal_location,
				new_token_uri,
			)
			.unwrap();
		};
	}
}
