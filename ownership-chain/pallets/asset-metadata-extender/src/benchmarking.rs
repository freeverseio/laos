//! Benchmarking setup for pallet-living-assets-evolution
#![cfg(feature = "runtime-benchmarks")]
use super::*;

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
	fn create_metadata_extension(
		t: Linear<0, { <T as Config>::MaxTokenUriLength::get() }>,
		u: Linear<0, { <T as Config>::MaxUniversalLocationLength::get() }>,
	) {
		let claimer: T::AccountId = whitelisted_caller();
		let index = 0;

		#[block]
		{
			AssetMetadataExtender::<T>::create_metadata_extension(
				claimer.clone(),
				vec![1u8; t as usize].try_into().unwrap(),
				vec![1u8; u as usize].try_into().unwrap(),
			)
			.unwrap();

			// TODO uncomment and fix error
			// assert_eq!(
			// 	AssetMetadataExtender::<T>::indexed_metadata_extensions(
			// 		vec![1u8; u as usize].try_into().unwrap(),
			// 		index
			// 	),
			// 	Some(MetadataExtensionDetails {
			// 		claimer: claimer.clone(),
			// 		token_uri: vec![1u8; t as usize].try_into().unwrap()
			// 	})
			// );
		};
	}
}
