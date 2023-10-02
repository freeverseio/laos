//! Benchmarking setup for pallet-living-assets-evolution
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as LivingAssetsEvo;
use frame_benchmarking::v2::*;
use frame_support::traits::Get;
use frame_system::RawOrigin;

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn create_collection() {
		let caller: T::AccountId = whitelisted_caller();
		let owner = caller.clone();
		#[extrinsic_call]
		create_collection(RawOrigin::Signed(caller.clone()), owner);

		assert_eq!(CollectionOwner::<T>::get(0), Some(caller));
	}

	#[benchmark]
	fn mint_with_external_uri() {
		let caller: T::AccountId = whitelisted_caller();
		let owner = caller.clone();
		LivingAssetsEvo::<T>::create_collection(RawOrigin::Signed(caller.clone()).into(), owner)
			.unwrap();

		let token_uri: TokenUriOf<T> =
			vec![0; T::MaxTokenUriLength::get() as usize].try_into().unwrap();
		let slot = 0;
		let token_id = LivingAssetsEvo::<T>::slot_and_owner_to_token_id(slot, SlotOwnerId::default())
			.unwrap();

		#[extrinsic_call]
		mint_with_external_uri(
			RawOrigin::Signed(caller.clone()),
			0,
			slot,
			SlotOwnerId::default(),
			token_uri.clone(),
		);

		assert_eq!(TokenURI::<T>::get(0, token_id), Some(token_uri));
	}

	impl_benchmark_test_suite!(LivingAssetsEvo, crate::mock::new_test_ext(), crate::mock::Test);
}
