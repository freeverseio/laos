use crate::{
	mock::*,
	traits::*,
	types::{TokenId, TokenUriOf, MAX_U96},
	CollectionId, Error, Event,
};
use codec::Encode;
use frame_support::{assert_noop, assert_ok};

const ALICE: AccountId = 5;
const BOB: AccountId = 6;

/// Utility function to create a collection and return its ID
fn create_collection(owner: u64) -> CollectionId {
	let collection_id = LivingAssets::collection_counter();
	assert_ok!(LivingAssets::create_collection(owner));
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
		assert_ok!(LivingAssets::create_collection(ALICE));
		assert_eq!(LivingAssets::collection_owner(collection_id), Some(ALICE));
		let collection_id: CollectionId = 1;
		assert_eq!(LivingAssets::collection_owner(collection_id), None);
		assert_ok!(LivingAssets::create_collection(BOB));
		assert_eq!(LivingAssets::collection_owner(collection_id), Some(BOB));
	});
}

#[test]
fn counter_of_collection_increases() {
	new_test_ext().execute_with(|| {
		assert_eq!(LivingAssets::collection_counter(), 0);
		assert_ok!(LivingAssets::create_collection(ALICE));
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
			1,
			collection_id,
			slot,
			owner,
			token_uri.clone()
		));

		let expected_token_id = {
			let mut buf = [0u8; 32];
			buf[..12].copy_from_slice(&slot.to_be_bytes()[4..]);
			let mut owner_bytes = owner.encode();
			owner_bytes.reverse();
			buf[24..].copy_from_slice(&owner_bytes[..]);

			TokenId::from(buf)
		};

		let token_id = LivingAssets::slot_and_owner_to_token_id(slot, owner).unwrap();

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
fn slot_and_owner_should_fail_if_slot_is_greater_than_96_bits() {
	new_test_ext().execute_with(|| {
		let slot = 1_u128 << 95;
		let owner = 0;

		assert_ok!(LivingAssets::slot_and_owner_to_token_id(slot, owner));

		let slot = 1_u128 << 96;
		assert_noop!(
			LivingAssets::slot_and_owner_to_token_id(slot, owner),
			Error::<Test>::SlotOverflow
		);
	});
}

#[test]
fn slot_and_owner_to_asset_id_works() {
	// Helper function to encapsulate the common logic of generating a token_id
	// and comparing it to an expected value.
	fn check_token_id(slot: u128, owner_hex: &str, expected_hex: &str) {
		let owner = u64::from_str_radix(owner_hex, 16).unwrap();
		let token_id = LivingAssets::slot_and_owner_to_token_id(slot, owner).unwrap();
		assert_eq!(format!("0x{:064x}", token_id), expected_hex);
	}

	check_token_id(
		0_u128,
		"0000000000000000",
		"0x0000000000000000000000000000000000000000000000000000000000000000",
	);

	check_token_id(
		1_u128,
		"0000000000000000",
		"0x0000000000000000000000010000000000000000000000000000000000000000",
	);

	check_token_id(
		1_u128,
		"e00000000000000f",
		"0x000000000000000000000001000000000000000000000000e00000000000000f",
	);

	check_token_id(
		MAX_U96,
		"e00000000000000f",
		"0xffffffffffffffffffffffff000000000000000000000000e00000000000000f",
	);
}

#[test]
fn mint_with_external_uri_non_owner() {
	new_test_ext().execute_with(|| {
		let collection_id = create_collection(1);
		let token_uri: TokenUriOf<Test> =
			vec![1, MaxTokenUriLength::get() as u8].try_into().unwrap();

		assert_noop!(
			LivingAssets::mint_with_external_uri(2, collection_id, 0, 1, token_uri.clone()),
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

		let to = 1;

		assert_noop!(
			LivingAssets::mint_with_external_uri(1, collection_id, 0, to, token_uri.clone()),
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
		let to = 1;

		assert_ok!(LivingAssets::create_collection(ALICE));
		assert_ok!(LivingAssets::mint_with_external_uri(
			ALICE,
			collection_id,
			0,
			to,
			token_uri.clone()
		));

		assert_noop!(
			LivingAssets::mint_with_external_uri(ALICE, collection_id, 0, to, token_uri.clone()),
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
		let to = 1;

		assert_noop!(
			LivingAssets::mint_with_external_uri(
				1,
				collection_id,
				MAX_U96 + 1, // pass a value greater than 2^96 - 1
				to,
				token_uri.clone()
			),
			Error::<Test>::SlotOverflow
		);
	});
}
