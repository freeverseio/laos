use crate::{mock::*, traits::AssetMetadataExtender as _, Error, Event};
use frame_support::{assert_noop, assert_ok};
use sp_core::{bounded_vec, H160};
use sp_runtime::BoundedVec;

// UL stands for Universal Location

fn create_extension(
	claimer: AccountId,
	universal_location: UniversalLocation,
	token_uri: TokenUri,
) {
	assert_ok!(AssetMetadataExtender::create_extension(claimer, universal_location, token_uri));
}

#[test]
fn given_an_ul_and_token_uri_i_can_create_asset_extension() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		let claimer = H160::zero();
		let universal_location: BoundedVec<u8, MaxUniversalLocationLength> = bounded_vec![0; 1];
		let token_uri: BoundedVec<u8, MaxTokenUriLength> = bounded_vec![0; 1];

		create_extension(claimer, universal_location.clone(), token_uri.clone());

		System::assert_last_event(
			Event::ExtensionCreated { universal_location, claimer, token_uri }.into(),
		);
	});
}

#[test]
fn given_an_ul_and_token_uri_i_cannot_create_twice_asset_extension_with_same_claimer() {
	new_test_ext().execute_with(|| {
		let claimer = H160::zero();
		let universal_location: BoundedVec<u8, MaxUniversalLocationLength> = bounded_vec![0; 1];
		let token_uri: BoundedVec<u8, MaxTokenUriLength> = bounded_vec![0; 1];

		create_extension(claimer, universal_location.clone(), token_uri.clone());
		assert_noop!(
			AssetMetadataExtender::create_extension(claimer, universal_location, token_uri),
			Error::<Test>::ExtensionAlreadyExists
		);
	});
}

#[test]
fn given_an_universal_location_after_creating_extension_counter_increases() {
	new_test_ext().execute_with(|| {
		let claimer = H160::zero();
		let universal_location: BoundedVec<u8, MaxUniversalLocationLength> = bounded_vec![0; 1];
		let token_uri: BoundedVec<u8, MaxTokenUriLength> = bounded_vec![0; 1];

		// create first extension for the given UL
		assert_eq!(
			AssetMetadataExtender::universal_location_extensions_counter(
				universal_location.clone()
			),
			0
		);

		create_extension(claimer, universal_location.clone(), token_uri.clone());
		assert_eq!(
			AssetMetadataExtender::universal_location_extensions_counter(
				universal_location.clone()
			),
			1
		);

		// check that no other UL has been affected
		let another_universal_location: BoundedVec<u8, MaxUniversalLocationLength> =
			bounded_vec![1; 1];
		assert_eq!(
			AssetMetadataExtender::universal_location_extensions_counter(
				another_universal_location
			),
			0
		);

		// create another extension for the same UL with another claimer
		let another_claimer = H160::from_low_u64_be(1);
		create_extension(another_claimer, universal_location.clone(), token_uri);
		assert_eq!(
			AssetMetadataExtender::universal_location_extensions_counter(universal_location),
			2
		);
	});
}

#[test]
fn given_an_ul_i_can_get_all_its_extensions() {
	new_test_ext().execute_with(|| {
		let universal_location: BoundedVec<u8, MaxUniversalLocationLength> = bounded_vec![0; 1];
		let token_uri: BoundedVec<u8, MaxTokenUriLength> = bounded_vec![0; 1];

		let n = 1000;
		for i in 0..n {
			let claimer = H160::from_low_u64_be(i);
			create_extension(claimer, universal_location.clone(), token_uri.clone());
		}

		for i in 0..n {
			assert_eq!(
				AssetMetadataExtender::universal_location_extensions(universal_location.clone(), i)
					.unwrap()
					.token_uri,
				token_uri
			);
			let expected_claimer = H160::from_low_u64_be(i);
			assert_eq!(
				AssetMetadataExtender::universal_location_extensions(universal_location.clone(), i)
					.unwrap()
					.claimer,
				expected_claimer
			);
		}
	});
}

#[test]
fn given_a_claimer_and_ul_i_can_get_the_extension() {
	new_test_ext().execute_with(|| {
		let claimer = H160::zero();
		let universal_location: BoundedVec<u8, MaxUniversalLocationLength> = bounded_vec![0; 1];
		let token_uri: BoundedVec<u8, MaxTokenUriLength> = bounded_vec![0; 1];

		create_extension(claimer, universal_location.clone(), token_uri);
		assert_eq!(
			AssetMetadataExtender::claimer_extensions(claimer, universal_location.clone()).unwrap(),
			universal_location
		);
	});
}
