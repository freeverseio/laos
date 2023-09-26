use crate::{
	mock::*,
	types::{TokenId, TokenUriOf, MAX_U96},
	CollectionId, Error, Event,
};
use frame_support::{assert_noop, assert_ok};
use sp_runtime::traits::Convert;

/// Utility function to create a collection and return its ID
fn create_collection(owner: u64) -> CollectionId {
	let collection_id = LivingAssets::collection_counter();
	assert_ok!(LivingAssets::create_collection(RuntimeOrigin::signed(owner)));
	collection_id
}

#[test]
fn owner_of_inexistent_collection() {
	new_test_ext().execute_with(|| {
		let collection_id: CollectionId = 0;
		assert_eq!(LivingAssets::collection_owner(collection_id), None);
	});
}

#[test]
fn create_collection_works() {
	new_test_ext().execute_with(|| {
		let collection_id: CollectionId = 0;
		assert_eq!(LivingAssets::collection_owner(collection_id), None);
		assert_ok!(LivingAssets::create_collection(RuntimeOrigin::signed(1)));
		assert_eq!(LivingAssets::collection_owner(collection_id), Some(1));
		let collection_id: CollectionId = 1;
		assert_eq!(LivingAssets::collection_owner(collection_id), None);
		assert_ok!(LivingAssets::create_collection(RuntimeOrigin::signed(2)));
		assert_eq!(LivingAssets::collection_owner(collection_id), Some(2));
	});
}

#[test]
fn counter_of_collection_increases() {
	new_test_ext().execute_with(|| {
		assert_eq!(LivingAssets::collection_counter(), 0);
		assert_ok!(LivingAssets::create_collection(RuntimeOrigin::signed(1)));
		assert_eq!(LivingAssets::collection_counter(), 1);
	})
}

#[test]
fn create_collection_emits_event() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		let collection_id = create_collection(1);

		// Assert that the correct event was deposited
		System::assert_last_event(Event::CollectionCreated { collection_id, owner: 1 }.into());
	});
}

#[test]
fn mint_with_external_uri_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		let collection_id = create_collection(1);
		let token_uri: TokenUriOf<Test> =
			vec![1, MaxTokenUriLength::get() as u8].try_into().unwrap();
		let slot = 0;
		let owner = 1;

		assert_ok!(LivingAssets::mint_with_external_uri(
			RuntimeOrigin::signed(1),
			collection_id,
			slot,
			owner,
			token_uri.clone()
		));

		let expected_token_id = {
			let mut buf = [0u8; 32];
			buf[..12].copy_from_slice(&slot.to_be_bytes()[4..]);
			buf[12..].copy_from_slice(
				<Test as crate::Config>::AccountIdToH160::convert(owner).as_fixed_bytes(),
			);

			TokenId::from(buf)
		};

		let token_id = LivingAssets::slot_and_owner_to_token_id((slot, owner));

		assert_eq!(token_id, expected_token_id);
		assert_eq!(LivingAssets::token_uri(collection_id, token_id), Some(token_uri.clone()));

		System::assert_has_event(
			Event::MintedWithExternalTokenURI {
				collection_id,
				slot,
				to: owner,
				token_id: expected_token_id,
				token_uri,
			}
			.into(),
		);
	});
}

#[test]
fn slot_and_owner_to_asset_id_works() {
	let slot = 0;
	let owner = 0;

	let expected_token_id = [0u8; 32].into();

	assert_eq!(LivingAssets::slot_and_owner_to_token_id((slot, owner)), expected_token_id);

	let mut expected_token_id_bytes = [0; 32];

	let slot = 1_u128;
	let owner: AccountId = 1;
	let mut owner_bytes = [0u8; 20];
	owner_bytes[0..8].copy_from_slice(&owner.to_be_bytes());
	expected_token_id_bytes[..12].copy_from_slice(&slot.to_be_bytes()[4..]);
	expected_token_id_bytes[12..].copy_from_slice(&owner_bytes);

	assert_eq!(
		LivingAssets::slot_and_owner_to_token_id((slot, owner)),
		TokenId::from(expected_token_id_bytes)
	);
}

#[test]
fn mint_with_external_uri_non_owner() {
	new_test_ext().execute_with(|| {
		let collection_id = create_collection(1);
		let token_uri: TokenUriOf<Test> =
			vec![1, MaxTokenUriLength::get() as u8].try_into().unwrap();

		assert_noop!(
			LivingAssets::mint_with_external_uri(
				RuntimeOrigin::signed(2),
				collection_id,
				0,
				1,
				token_uri.clone()
			),
			Error::<Test>::NoPermission
		);
	});
}

#[test]
fn mint_with_external_uri_collection_does_not_exist() {
	new_test_ext().execute_with(|| {
		// simply use the collection counter as collection id, do not create the collection
		let collection_id = LivingAssets::collection_counter();

		let token_uri: TokenUriOf<Test> =
			vec![1, MaxTokenUriLength::get() as u8].try_into().unwrap();

		assert_noop!(
			LivingAssets::mint_with_external_uri(
				RuntimeOrigin::signed(1),
				collection_id,
				0,
				1,
				token_uri.clone()
			),
			Error::<Test>::CollectionDoesNotExist
		);
	});
}

#[test]
fn mint_with_external_uri_asset_already_minted() {
	new_test_ext().execute_with(|| {
		let collection_id = LivingAssets::collection_counter();
		let token_uri: TokenUriOf<Test> =
			vec![1, MaxTokenUriLength::get() as u8].try_into().unwrap();

		assert_ok!(LivingAssets::create_collection(RuntimeOrigin::signed(1)));
		assert_ok!(LivingAssets::mint_with_external_uri(
			RuntimeOrigin::signed(1),
			collection_id,
			0,
			1,
			token_uri.clone()
		));

		assert_noop!(
			LivingAssets::mint_with_external_uri(
				RuntimeOrigin::signed(1),
				collection_id,
				0,
				1,
				token_uri.clone()
			),
			Error::<Test>::AlreadyMinted
		);
	});
}

#[test]
fn slot_overflow() {
	new_test_ext().execute_with(|| {
		let collection_id = create_collection(1);
		let token_uri: TokenUriOf<Test> =
			vec![1, MaxTokenUriLength::get() as u8].try_into().unwrap();

		assert_noop!(
			LivingAssets::mint_with_external_uri(
				RuntimeOrigin::signed(1),
				collection_id,
				MAX_U96 + 1, // pass a value greater than 2^96 - 1
				1,
				token_uri.clone()
			),
			sp_runtime::ArithmeticError::Overflow
		);
	});
}
