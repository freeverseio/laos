use crate::{
	address_to_collection_id, collection_id_to_address, is_collection_address, mock::*,
	traits::CollectionManager, AssetOwner, CollectionBaseURI, CollectionError, CollectionId, Event,
};
use core::str::FromStr;
use frame_support::assert_ok;

type BaseURI = crate::BaseURIOf<Test>;
type AccountId = <Test as frame_system::Config>::AccountId;

const ALICE: [u8; 20] = [1u8; 20];
const BOB: [u8; 20] = [2u8; 20];

/// Create a new collection with the given base URI.
fn create_collection(who: AccountId, base_uri: Option<BaseURI>) -> CollectionId {
	<LivingAssetsModule as CollectionManager<AccountId, BaseURI>>::create_collection(
		who,
		base_uri.unwrap_or_default(),
	)
	.unwrap()
}

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
		for _ in 0..3 {
			// Create the collection
			let i = create_collection(ALICE.into(), Some(base_uri.clone()));

			// Assert that the collection was created with the expected URI
			assert_eq!(LivingAssetsModule::collection_base_uri(i).unwrap(), base_uri);
		}
	});
}

#[test]
fn should_set_base_uri_when_creating_new_collection() {
	let base_uri = BaseURI::try_from("https://example.com/".as_bytes().to_vec()).unwrap();

	new_test_ext().execute_with(|| {
		create_collection(ALICE.into(), Some(base_uri.clone()));
		assert_eq!(LivingAssetsModule::collection_base_uri(0).unwrap(), base_uri);
	});
}

#[test]
fn create_new_collections_should_emit_events_with_collection_id_consecutive() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		assert_ok!(LivingAssetsModule::create_collection(ALICE.into(), BaseURI::default()));
		System::assert_last_event(
			Event::CollectionCreated { collection_id: 0, who: ALICE.into() }.into(),
		);
		assert_ok!(LivingAssetsModule::create_collection(ALICE.into(), BaseURI::default()));
		System::assert_last_event(
			Event::CollectionCreated { collection_id: 1, who: ALICE.into() }.into(),
		);
		assert_ok!(LivingAssetsModule::create_collection(ALICE.into(), BaseURI::default()));
		System::assert_last_event(
			Event::CollectionCreated { collection_id: 2, who: ALICE.into() }.into(),
		);
		assert_ok!(LivingAssetsModule::create_collection(ALICE.into(), BaseURI::default()));
		System::assert_last_event(
			Event::CollectionCreated { collection_id: 3, who: ALICE.into() }.into(),
		);
	});
}

#[test]
fn test_collection_id_to_address() {
	let collection_id = 5;
	let expected_address = AccountId::from_str("ffffffffffffffffffffffff0000000000000005").unwrap();
	assert_eq!(collection_id_to_address::<AccountId>(collection_id), expected_address);
}

#[test]
fn invalid_collection_address_should_error() {
	let address = AccountId::from_str("8000000000000000000000000000000000000005").unwrap();
	let error = address_to_collection_id::<AccountId>(address).unwrap_err();
	assert_eq!(error, CollectionError::InvalidPrefix);
}

#[test]
fn valid_collection_address_should_return_collection_id() {
	let address = AccountId::from_str("ffffffffffffffffffffffff0000000000000005").unwrap();
	let collection_id = address_to_collection_id::<AccountId>(address).unwrap();
	assert_eq!(collection_id, 5);
}

#[test]
fn test_is_collection_address_valid() {
	let collection_id = 1234567890;
	let address = collection_id_to_address::<AccountId>(collection_id);

	assert!(is_collection_address::<AccountId>(address));
}

#[test]
fn test_is_collection_address_invalid() {
	let invalid_address = [0u8; 20].into();

	assert!(!is_collection_address::<AccountId>(invalid_address));
}

mod traits {
	use super::*;
	use crate::{
		traits::{CollectionManager, Erc721},
		Error, Event,
	};
	use frame_support::{assert_err, assert_noop, assert_ok};
	use sp_core::{H160, U256};

	#[test]
	fn base_uri_of_unexistent_collection_is_none() {
		new_test_ext().execute_with(|| {
			assert_eq!(LivingAssetsModule::base_uri(0), None);
			assert_eq!(LivingAssetsModule::base_uri(1), None);
		});
	}

	#[test]
	fn create_new_collection_should_emit_an_event() {
		new_test_ext().execute_with(|| {
			// Go past genesis block so events get deposited
			System::set_block_number(1);

			create_collection(ALICE.into(), None);
			System::assert_last_event(
				Event::CollectionCreated { collection_id: 0, who: ALICE.into() }.into(),
			);
		});
	}

	#[test]
	fn living_assets_ownership_trait_id_of_new_collection_should_be_consecutive() {
		new_test_ext().execute_with(|| {
			assert_eq!(create_collection(ALICE.into(), None), 0);

			for i in 0..5 {
				assert_eq!(create_collection(ALICE.into(), None), i + 1,)
			}
		});
	}

	#[test]
	fn living_assets_ownership_trait_should_set_base_uri_when_creating_new_collection() {
		let base_uri = BaseURI::try_from("https://example.com/".as_bytes().to_vec()).unwrap();

		new_test_ext().execute_with(|| {
			let _ = create_collection(ALICE.into(), Some(base_uri.clone()));
			assert_eq!(LivingAssetsModule::collection_base_uri(0).unwrap(), base_uri);
		});
	}

	#[test]
	fn owner_of_asset_of_unexistent_collection_should_error() {
		new_test_ext().execute_with(|| {
			let result = LivingAssetsModule::owner_of(0, 2.into());
			assert_err!(result, Error::CollectionDoesNotExist);
		});
	}

	#[test]
	fn verify_different_owners_for_same_asset_across_different_collections() {
		let asset_id = U256::from(
			hex::decode("03C0F0f4ab324C46e55D02D0033343B4Be8A55532d").unwrap().as_slice(),
		);
		let sender = AccountId::from_str("c0f0f4ab324c46e55d02d0033343b4be8a55532d").unwrap();
		let receiver = BOB.into();
		new_test_ext().execute_with(|| {
			CollectionBaseURI::<Test>::insert(1, BaseURI::default());
			CollectionBaseURI::<Test>::insert(2, BaseURI::default());
			assert_eq!(LivingAssetsModule::owner_of(1, asset_id).unwrap(), sender);
			assert_ok!(LivingAssetsModule::transfer_from(sender, 1, sender, receiver, asset_id,));
			assert_eq!(LivingAssetsModule::owner_of(1, asset_id).unwrap(), receiver);
			assert_eq!(LivingAssetsModule::owner_of(2, asset_id).unwrap(), sender);
		});
	}

	#[test]
	fn erc721_owner_of_asset_of_collection() {
		new_test_ext().execute_with(|| {
			let collection_id = create_collection(ALICE.into(), None);
			assert_eq!(
				LivingAssetsModule::owner_of(collection_id, 2.into()).unwrap(),
				H160::from_low_u64_be(0x0000000000000002).0.into()
			);
		});
	}

	#[test]
	fn caller_is_not_current_owner_should_fail() {
		let asset_id = U256::from(5);
		let sender = AccountId::from_str("0000000000000000000000000000000000000006").unwrap();
		let receiver = BOB.into();
		new_test_ext().execute_with(|| {
			System::set_block_number(1);
			assert!(AssetOwner::<Test>::get(0, asset_id).is_none());
			CollectionBaseURI::<Test>::insert(1, BaseURI::default());
			assert_noop!(
				LivingAssetsModule::transfer_from(ALICE.into(), 1, sender, receiver, asset_id,),
				Error::<Test>::NoPermission
			);
		});
	}

	#[test]
	fn sender_is_not_current_owner_should_fail() {
		let asset_id = U256::from(5);
		let sender = AccountId::from_str("0000000000000000000000000000000000000006").unwrap();
		let receiver = BOB.into();
		new_test_ext().execute_with(|| {
			System::set_block_number(1);
			assert!(AssetOwner::<Test>::get(0, asset_id).is_none());
			CollectionBaseURI::<Test>::insert(1, BaseURI::default());
			assert_noop!(
				LivingAssetsModule::transfer_from(sender, 1, sender, receiver, asset_id,),
				Error::<Test>::NoPermission
			);
		});
	}

	#[test]
	fn same_sender_and_receiver_should_fail() {
		let asset_id = U256::from(5);
		let sender = AccountId::from_str("0000000000000000000000000000000000000005").unwrap();
		new_test_ext().execute_with(|| {
			System::set_block_number(1);
			assert!(AssetOwner::<Test>::get(0, asset_id).is_none());
			CollectionBaseURI::<Test>::insert(1, BaseURI::default());
			assert_noop!(
				LivingAssetsModule::transfer_from(sender, 1, sender, sender, asset_id,),
				Error::<Test>::CannotTransferSelf
			);
		});
	}

	#[test]
	fn receiver_is_the_zero_address_should_fail() {
		let asset_id = U256::from(5);
		let sender = AccountId::from_str("0000000000000000000000000000000000000005").unwrap();
		let receiver = AccountId::from_str("0000000000000000000000000000000000000000").unwrap();
		new_test_ext().execute_with(|| {
			System::set_block_number(1);
			assert!(AssetOwner::<Test>::get(0, asset_id).is_none());
			CollectionBaseURI::<Test>::insert(1, BaseURI::default());
			assert_noop!(
				LivingAssetsModule::transfer_from(sender, 1, sender, receiver, asset_id,),
				Error::<Test>::TransferToNullAddress
			);
		});
	}

	#[test]
	fn unexistent_collection_when_transfer_from_should_fail() {
		let asset_id = U256::from(5);
		let sender = AccountId::from_str("0000000000000000000000000000000000000005").unwrap();
		let receiver = BOB.into();
		new_test_ext().execute_with(|| {
			System::set_block_number(1);
			assert!(AssetOwner::<Test>::get(0, asset_id).is_none());
			assert_noop!(
				LivingAssetsModule::transfer_from(sender, 1, sender, receiver, asset_id,),
				Error::<Test>::CollectionDoesNotExist
			);
		});
	}

	#[test]
	fn successful_transfer_from_trait_should_work() {
		let asset_id = U256::from(
			hex::decode("03C0F0f4ab324C46e55D02D0033343B4Be8A55532d").unwrap().as_slice(),
		);
		let sender = AccountId::from_str("c0f0f4ab324c46e55d02d0033343b4be8a55532d").unwrap();
		let receiver = BOB.into();
		let collection_id = 1;
		new_test_ext().execute_with(|| {
			System::set_block_number(1);
			CollectionBaseURI::<Test>::insert(collection_id, BaseURI::default());
			assert!(AssetOwner::<Test>::get(collection_id, asset_id).is_none());
			assert_eq!(LivingAssetsModule::owner_of(collection_id, asset_id).unwrap(), sender);
			assert_ok!(LivingAssetsModule::transfer_from(
				sender,
				collection_id,
				sender,
				receiver,
				asset_id,
			));
			assert_eq!(AssetOwner::<Test>::get(collection_id, asset_id).unwrap(), BOB.into());
			assert_eq!(LivingAssetsModule::owner_of(collection_id, asset_id).unwrap(), receiver);
			System::assert_last_event(
				Event::AssetTransferred { collection_id, asset_id, to: BOB.into() }.into(),
			);
		});
	}

	#[test]
	fn token_uri_of_unexistent_collection() {
		new_test_ext().execute_with(|| {
			let result = LivingAssetsModule::token_uri(0, 2.into());
			assert_err!(result, Error::CollectionDoesNotExist);
		});
	}

	#[test]
	fn token_uri_should_concatenate_the_base_uri_with_the_token_id() {
		let base_uri = BaseURI::try_from("https://example.com".as_bytes().to_vec()).unwrap();

		new_test_ext().execute_with(|| {
			let collection_id = create_collection(ALICE.into(), Some(base_uri.clone()));
			assert_eq!(
				LivingAssetsModule::token_uri(collection_id, 2.into()).unwrap(),
				"https://example.com/2".as_bytes().to_vec()
			);
		});
	}
}
