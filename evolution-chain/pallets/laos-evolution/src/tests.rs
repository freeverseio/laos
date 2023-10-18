use core::str::FromStr;

use crate::{
	mock::*,
	slot_and_owner_to_token_id,
	traits::LaosEvolution as _,
	types::{TokenId, TokenUriOf, MAX_U96},
	CollectionId, Error, Event,
};
use codec::Encode;
use frame_support::{assert_noop, assert_ok};
use sp_core::H160;

const ALICE: &str = "0x0000000000000000000000000000000000000005";
const BOB: &str = "0x0000000000000000000000000000000000000006";

/// Utility function to create a collection and return its ID
fn create_collection(owner: &str) -> CollectionId {
	let owner = AccountId::from_str(owner).unwrap();
	let collection_id = LaosEvolution::collection_counter();
	assert_ok!(LaosEvolution::create_collection(owner));
	collection_id
}

#[test]
fn owner_of_inexistent_collection() {
	new_test_ext().execute_with(|| {
		let collection_id: CollectionId = 0;
		assert_eq!(LaosEvolution::collection_owner(collection_id), None);
	});
}

#[test]
fn create_collection_works() {
	new_test_ext().execute_with(|| {
		let collection_id: CollectionId = 0;
		assert_eq!(LaosEvolution::collection_owner(collection_id), None);
		create_collection(ALICE);
		assert_eq!(
			LaosEvolution::collection_owner(collection_id),
			Some(AccountId::from_str(ALICE).unwrap())
		);
		let collection_id: CollectionId = 1;
		assert_eq!(LaosEvolution::collection_owner(collection_id), None);
		create_collection(BOB);
		assert_eq!(
			LaosEvolution::collection_owner(collection_id),
			Some(AccountId::from_str(BOB).unwrap())
		);
	});
}

#[test]
fn counter_of_collection_increases() {
	new_test_ext().execute_with(|| {
		assert_eq!(LaosEvolution::collection_counter(), 0);
		create_collection(ALICE);
		assert_eq!(LaosEvolution::collection_counter(), 1);
	})
}

#[test]
fn create_collection_emits_event() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		let collection_id = create_collection(ALICE);

		// Assert that the correct event was deposited
		System::assert_last_event(
			Event::CollectionCreated { collection_id, owner: AccountId::from_str(ALICE).unwrap() }
				.into(),
		);
	});
}

#[test]
fn mint_with_external_uri_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		let collection_id = create_collection(ALICE);
		let token_uri: TokenUriOf<Test> =
			vec![1, MaxTokenUriLength::get() as u8].try_into().unwrap();
		let slot = 0;
		let owner = AccountId::from_str(ALICE).unwrap();

		assert_ok!(LaosEvolution::mint_with_external_uri(
			owner,
			collection_id,
			slot,
			owner,
			token_uri.clone()
		));

		let expected_token_id = {
			let mut buf = [0u8; 32];
			buf[..12].copy_from_slice(&slot.to_be_bytes()[4..]);
			let owner_bytes = owner.encode();
			buf[12..].copy_from_slice(&owner_bytes[..]);

			TokenId::from(buf)
		};

		let token_id = slot_and_owner_to_token_id(slot, owner).unwrap();

		assert_eq!(token_id, expected_token_id);
		assert_eq!(LaosEvolution::token_uri(collection_id, token_id), Some(token_uri.clone()));

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
		let owner = H160::zero();

		assert!(slot_and_owner_to_token_id(slot, owner).is_some());

		let slot = 1_u128 << 96;
		assert_eq!(slot_and_owner_to_token_id(slot, owner), None);
	});
}
#[test]
fn slot_and_owner_to_asset_id_works() {
	// Helper function to encapsulate the common logic of generating a token_id
	// and comparing it to an expected value.
	fn check_token_id(slot: u128, owner_hex: &str, expected_hex: &str) {
		let owner = AccountId::from_str(owner_hex).unwrap();
		let token_id = slot_and_owner_to_token_id(slot, owner).unwrap();
		assert_eq!(format!("0x{:064x}", token_id), expected_hex);
	}

	check_token_id(
		0_u128,
		"0x0000000000000000000000000000000000000000",
		"0x0000000000000000000000000000000000000000000000000000000000000000",
	);

	check_token_id(
		1_u128,
		"0x0000000000000000000000000000000000000000",
		"0x0000000000000000000000010000000000000000000000000000000000000000",
	);

	check_token_id(
		1_u128,
		"0xe00000000000000000000000000000000000000f",
		"0x000000000000000000000001e00000000000000000000000000000000000000f",
	);

	check_token_id(
		MAX_U96,
		"0xe00000000000000000000000000000000000000f",
		"0xffffffffffffffffffffffffe00000000000000000000000000000000000000f",
	);
}

#[test]
fn mint_with_external_uri_non_owner() {
	new_test_ext().execute_with(|| {
		let collection_id = create_collection(ALICE);
		let token_uri: TokenUriOf<Test> =
			vec![1, MaxTokenUriLength::get() as u8].try_into().unwrap();

		assert_noop!(
			LaosEvolution::mint_with_external_uri(
				AccountId::from_str(BOB).unwrap(),
				collection_id,
				0,
				AccountId::from_str(BOB).unwrap(),
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
		let collection_id = LaosEvolution::collection_counter();

		let token_uri: TokenUriOf<Test> =
			vec![1, MaxTokenUriLength::get() as u8].try_into().unwrap();

		let test_account = AccountId::from_str(ALICE).unwrap();

		assert_noop!(
			LaosEvolution::mint_with_external_uri(
				test_account,
				collection_id,
				0,
				test_account,
				token_uri.clone()
			),
			Error::<Test>::CollectionDoesNotExist
		);
	});
}

#[test]
fn mint_with_external_uri_asset_already_minted() {
	new_test_ext().execute_with(|| {
		let collection_id = LaosEvolution::collection_counter();
		let token_uri: TokenUriOf<Test> =
			vec![1, MaxTokenUriLength::get() as u8].try_into().unwrap();
		let owner = AccountId::from_str(ALICE).unwrap();
		let to = AccountId::from_str("0x0000000000000000000000000000000000000001").unwrap();

		create_collection(ALICE);
		assert_ok!(LaosEvolution::mint_with_external_uri(
			owner,
			collection_id,
			0,
			to,
			token_uri.clone()
		));

		assert_noop!(
			LaosEvolution::mint_with_external_uri(owner, collection_id, 0, to, token_uri.clone()),
			Error::<Test>::AlreadyMinted
		);
	});
}

#[test]
fn slot_overflow() {
	new_test_ext().execute_with(|| {
		let test_account =
			AccountId::from_str("0x0000000000000000000000000000000000000001").unwrap();
		let collection_id = create_collection("0x0000000000000000000000000000000000000001");
		let token_uri: TokenUriOf<Test> =
			vec![1, MaxTokenUriLength::get() as u8].try_into().unwrap();

		assert_noop!(
			LaosEvolution::mint_with_external_uri(
				test_account,
				collection_id,
				MAX_U96 + 1, // pass a value greater than 2^96 - 1
				test_account,
				token_uri.clone()
			),
			Error::<Test>::SlotOverflow
		);
	});
}

#[test]
fn collection_owner_works() {
	new_test_ext().execute_with(|| {
		// non-existent collection
		assert_eq!(
			<LivingAssets as LivingAssetsEvolution<AccountId, TokenUriOf<Test>>>::collection_owner(
				0_u64
			),
			None
		);

		create_collection(ALICE);

		assert_eq!(
			<LivingAssets as LivingAssetsEvolution<AccountId, TokenUriOf<Test>>>::collection_owner(
				0_u64
			),
			Some(AccountId::from_str(ALICE).unwrap())
		);
	})
}
