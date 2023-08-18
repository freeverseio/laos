use core::str::FromStr;

use crate::{
	address_to_collection_id, collection_id_to_address, is_collection_address, mock::*,
	CollectionError, Event,
};
use frame_support::assert_ok;
use sp_core::H160;

type AccountId = <Test as frame_system::Config>::AccountId;
type BaseURI = crate::BaseURI<Test>;

const ALICE: AccountId = 0x1234;

#[test]
fn base_uri_unexistent_collection_is_none() {
	new_test_ext().execute_with(|| {
		assert_eq!(LivingAssetsModule::collection_base_uri(0), None);
		assert_eq!(LivingAssetsModule::collection_base_uri(1), None);
	});
}

#[test]
fn create_new_collection_should_create_sequential_collections() {
	new_test_ext().execute_with(|| {
		// Check initial condition
		assert_eq!(LivingAssetsModule::collection_base_uri(0), None);

		let base_uri = BaseURI::try_from("https://example.com/".as_bytes().to_vec()).unwrap();

		// Iterate through the collections to be created
		for i in 0..3 {
			// Create the collection
			assert_ok!(LivingAssetsModule::create_collection(
				RuntimeOrigin::signed(ALICE),
				base_uri.clone()
			));

			// Assert that the collection was created with the expected URI
			assert_eq!(LivingAssetsModule::collection_base_uri(i).unwrap(), base_uri);
		}
	});
}

#[test]
fn should_set_base_uri_when_creating_new_collection() {
	let base_uri = BaseURI::try_from("https://example.com/".as_bytes().to_vec()).unwrap();

	new_test_ext().execute_with(|| {
		assert_ok!(LivingAssetsModule::create_collection(
			RuntimeOrigin::signed(ALICE),
			base_uri.clone()
		));
		assert_eq!(LivingAssetsModule::collection_base_uri(0).unwrap(), base_uri);
	});
}

#[test]
fn create_new_collections_should_emit_events_with_collection_id_consecutive() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		assert_ok!(LivingAssetsModule::create_collection(
			RuntimeOrigin::signed(ALICE),
			BaseURI::default()
		));
		System::assert_last_event(Event::CollectionCreated { collection_id: 0, who: ALICE }.into());
		assert_ok!(LivingAssetsModule::create_collection(
			RuntimeOrigin::signed(ALICE),
			BaseURI::default()
		));
		System::assert_last_event(Event::CollectionCreated { collection_id: 1, who: ALICE }.into());
		assert_ok!(LivingAssetsModule::create_collection(
			RuntimeOrigin::signed(ALICE),
			BaseURI::default()
		));
		System::assert_last_event(Event::CollectionCreated { collection_id: 2, who: ALICE }.into());
		assert_ok!(LivingAssetsModule::create_collection(
			RuntimeOrigin::signed(ALICE),
			BaseURI::default()
		));
		System::assert_last_event(Event::CollectionCreated { collection_id: 3, who: ALICE }.into());
	});
}

#[test]
fn test_collection_id_to_address() {
	let collection_id = 5;
	let expected_address = H160::from_str("ffffffffffffffffffffffff0000000000000005").unwrap();
	assert_eq!(collection_id_to_address(collection_id), expected_address);
}

#[test]
fn invalid_collection_address_should_error() {
	let address = H160::from_str("8000000000000000000000000000000000000005").unwrap();
	let error = address_to_collection_id(address).unwrap_err();
	assert_eq!(error, CollectionError::InvalidPrefix);
}

#[test]
fn valid_collection_address_should_return_collection_id() {
	let address = H160::from_str("ffffffffffffffffffffffff0000000000000005").unwrap();
	let collection_id = address_to_collection_id(address).unwrap();
	assert_eq!(collection_id, 5);
}

#[test]
fn test_is_collection_address_valid() {
	let collection_id = 1234567890;
	let address = collection_id_to_address(collection_id);

	assert!(is_collection_address(address));
}

#[test]
fn test_is_collection_address_invalid() {
	let invalid_address = H160([0u8; 20]);

	assert!(!is_collection_address(invalid_address));
}

mod traits {
	use super::*;
	use crate::{
		traits::{CollectionManager, Erc721},
		Error, Event,
	};
	use frame_support::{assert_err, assert_ok};

	#[test]
	fn base_uri_of_unexistent_collection_is_none() {
		new_test_ext().execute_with(|| {
			assert_eq!(<LivingAssetsModule as CollectionManager>::base_uri(0), None);
			assert_eq!(<LivingAssetsModule as CollectionManager>::base_uri(1), None);
		});
	}

	#[test]
	fn create_new_collection_should_emit_an_event() {
		new_test_ext().execute_with(|| {
			// Go past genesis block so events get deposited
			System::set_block_number(1);

			assert_ok!(<LivingAssetsModule as CollectionManager>::create_collection(
				ALICE,
				BaseURI::default(),
			));
			System::assert_last_event(
				Event::CollectionCreated { collection_id: 0, who: ALICE }.into(),
			);
		});
	}

	#[test]
	fn living_assets_ownership_trait_id_of_new_collection_should_be_consecutive() {
		new_test_ext().execute_with(|| {
			assert_eq!(
				<LivingAssetsModule as CollectionManager>::create_collection(
					ALICE,
					BaseURI::default()
				)
				.unwrap(),
				0
			);
			assert_eq!(
				<LivingAssetsModule as CollectionManager>::create_collection(
					ALICE,
					BaseURI::default()
				)
				.unwrap(),
				1
			);
			assert_eq!(
				<LivingAssetsModule as CollectionManager>::create_collection(
					ALICE,
					BaseURI::default()
				)
				.unwrap(),
				2
			);
			assert_eq!(
				<LivingAssetsModule as CollectionManager>::create_collection(
					ALICE,
					BaseURI::default()
				)
				.unwrap(),
				3
			);
			assert_eq!(
				<LivingAssetsModule as CollectionManager>::create_collection(
					ALICE,
					BaseURI::default()
				)
				.unwrap(),
				4
			);
			assert_eq!(
				<LivingAssetsModule as CollectionManager>::create_collection(
					ALICE,
					BaseURI::default()
				)
				.unwrap(),
				5
			);
		});
	}

	#[test]
	fn living_assets_ownership_trait_should_set_base_uri_when_creating_new_collection() {
		let base_uri = BaseURI::try_from("https://example.com/".as_bytes().to_vec()).unwrap();

		new_test_ext().execute_with(|| {
			assert_ok!(<LivingAssetsModule as CollectionManager>::create_collection(
				ALICE,
				base_uri.clone()
			));
			assert_eq!(LivingAssetsModule::collection_base_uri(0).unwrap(), base_uri);
		});
	}

	#[test]
	fn owner_of_asset_of_unexistent_collection_should_error() {
		new_test_ext().execute_with(|| {
			let result = <LivingAssetsModule as Erc721>::owner_of(0, 2.into());
			assert_err!(result, Error::UnexistentCollection);
		});
	}

	#[test]
	fn erc721_owner_of_asset_of_collection() {
		new_test_ext().execute_with(|| {
			let collection_id = <LivingAssetsModule as CollectionManager>::create_collection(
				ALICE,
				BaseURI::default(),
			)
			.unwrap();
			assert_eq!(
				<LivingAssetsModule as Erc721>::owner_of(collection_id, 2.into()).unwrap(),
				H160::from_low_u64_be(0x0000000000000002)
			);
		});
	}
}
