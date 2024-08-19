// Copyright 2023-2024 Freeverse.io
// This file is part of LAOS.

// LAOS is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// LAOS is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with LAOS.  If not, see <http://www.gnu.org/licenses/>.

use core::str::FromStr;

use crate::{
	mock::*,
	slot_and_owner_to_token_id,
	traits::{EvolutionCollection, EvolutionCollectionFactory},
	types::{Slot, TokenId, TokenUriOf},
	CollectionId, Error, Event,
};
use frame_support::{assert_noop, assert_ok};
use sp_core::U256;

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
fn transfer_collection_works() {
	new_test_ext().execute_with(|| {
		// non-existent collection
		assert_noop!(
			LaosEvolution::transfer_ownership(
				AccountId::from_str(ALICE).unwrap(),
				AccountId::from_str(BOB).unwrap(),
				0_u64
			),
			Error::<Test>::CollectionDoesNotExist
		);

		let collection_id = create_collection(ALICE);
		let new_owner = AccountId::from_str(BOB).unwrap();

		// Non-owner cannot transfer collection
		assert_noop!(
			LaosEvolution::transfer_ownership(
				AccountId::from_str(BOB).unwrap(),
				new_owner,
				collection_id
			),
			Error::<Test>::NoPermission
		);

		assert_ok!(LaosEvolution::transfer_ownership(
			AccountId::from_str(ALICE).unwrap(),
			new_owner,
			collection_id
		));

		assert_eq!(LaosEvolution::collection_owner(collection_id), Some(new_owner));
	});
}

#[test]
fn transfer_collection_emits_event() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		let collection_id = create_collection(ALICE);
		let new_owner = AccountId::from_str(BOB).unwrap();

		assert_ok!(LaosEvolution::transfer_ownership(
			AccountId::from_str(ALICE).unwrap(),
			new_owner,
			collection_id
		));

		// Assert that the correct event was deposited
		System::assert_has_event(
			Event::CollectionTransferred {
				collection_id,
				from: AccountId::from_str(ALICE).unwrap(),
				to: new_owner,
			}
			.into(),
		);
	});
}

#[test]
fn slot_and_owner_to_token_id_works() {
	let slot = Slot::MAX_SLOT;
	let owner = AccountId::from_str("0x8000000000000000000000000000000000000001").unwrap();
	let token_id = slot_and_owner_to_token_id(slot, owner);
	assert_eq!(
		format!("0x{:064x}", token_id),
		"0xffffffffffffffffffffffff8000000000000000000000000000000000000001"
	);
}

#[test]
fn mint_with_external_uri_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		let collection_id = create_collection(ALICE);
		let token_uri: TokenUriOf<Test> =
			vec![1, MaxTokenUriLength::get() as u8].try_into().unwrap();
		let slot = Slot::try_from(0).unwrap();
		let owner = AccountId::from_str(ALICE).unwrap();

		assert_ok!(LaosEvolution::mint_with_external_uri(
			owner,
			collection_id,
			slot,
			owner,
			token_uri.clone()
		));

		let token_id = slot_and_owner_to_token_id(slot, owner);

		assert_eq!(LaosEvolution::token_uri(collection_id, token_id), Some(token_uri.clone()));

		System::assert_has_event(
			Event::MintedWithExternalURI { collection_id, slot, to: owner, token_id, token_uri }
				.into(),
		);
	});
}

#[test]
fn slot_and_owner_to_asset_id_works() {
	// Helper function to encapsulate the common logic of generating a token_id
	// and comparing it to an expected value.
	fn check_token_id(slot: Slot, owner_hex: &str, expected_hex: &str) {
		let owner = AccountId::from_str(owner_hex).unwrap();
		let token_id = slot_and_owner_to_token_id(slot, owner);
		assert_eq!(format!("0x{:064x}", token_id), expected_hex);
	}

	check_token_id(
		Slot::try_from(0).unwrap(),
		"0x0000000000000000000000000000000000000000",
		"0x0000000000000000000000000000000000000000000000000000000000000000",
	);

	check_token_id(
		Slot::try_from(1).unwrap(),
		"0x0000000000000000000000000000000000000000",
		"0x0000000000000000000000010000000000000000000000000000000000000000",
	);

	check_token_id(
		Slot::try_from(1).unwrap(),
		"0xe00000000000000000000000000000000000000f",
		"0x000000000000000000000001e00000000000000000000000000000000000000f",
	);

	check_token_id(
		Slot::MAX_SLOT,
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
				0.try_into().unwrap(),
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
				0.try_into().unwrap(),
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
			0.try_into().unwrap(),
			to,
			token_uri.clone()
		));

		assert_noop!(
			LaosEvolution::mint_with_external_uri(
				owner,
				collection_id,
				0.try_into().unwrap(),
				to,
				token_uri.clone()
			),
			Error::<Test>::AlreadyMinted
		);
	});
}

#[test]
fn collection_owner_works() {
	new_test_ext().execute_with(|| {
		// non-existent collection
		assert_eq!(LaosEvolution::collection_owner(0_u64), None);

		create_collection(ALICE);

		assert_eq!(
			LaosEvolution::collection_owner(0_u64),
			Some(AccountId::from_str(ALICE).unwrap())
		);
	})
}

#[test]
fn token_uri_of_unexistent_collection_returns_none() {
	new_test_ext().execute_with(|| {
		let tocken_id: U256 = 0_u64.into();
		assert_eq!(LaosEvolution::token_uri(0_u64, tocken_id), None);
	});
}

#[test]
fn token_uri_of_unexistent_token_returns_none() {
	new_test_ext().execute_with(|| {
		let collection_id = create_collection(ALICE);
		let tocken_id: TokenId = 0_u64.into();
		assert_eq!(LaosEvolution::token_uri(collection_id, tocken_id), None);
	});
}

#[test]
fn token_uri_of_existent_token_returns_correct_token_uri() {
	new_test_ext().execute_with(|| {
		let who = AccountId::from_str(ALICE).unwrap();
		let collection_id = create_collection(ALICE);
		let slot = Slot::try_from(1).unwrap();
		let to = AccountId::from_str(BOB).unwrap();
		let token_uri: TokenUriOf<Test> =
			vec![1, MaxTokenUriLength::get() as u8].try_into().unwrap();
		let token_id =
			LaosEvolution::mint_with_external_uri(who, collection_id, slot, to, token_uri.clone())
				.unwrap();

		assert_eq!(LaosEvolution::token_uri(collection_id, token_id), Some(token_uri));
	});
}

#[test]
fn evolve_with_external_uri_when_unexistent_collection_id_should_fail() {
	new_test_ext().execute_with(|| {
		let who = AccountId::from_str(ALICE).unwrap();
		let collection_id = LaosEvolution::collection_counter();
		let slot = Slot::try_from(0).unwrap();
		let owner = AccountId::from_str(ALICE).unwrap();
		let token_id = slot_and_owner_to_token_id(slot, owner);
		let new_token_uri: TokenUriOf<Test> =
			vec![1, MaxTokenUriLength::get() as u8].try_into().unwrap();

		assert_noop!(
			LaosEvolution::evolve_with_external_uri(who, collection_id, token_id, new_token_uri),
			Error::<Test>::CollectionDoesNotExist
		);
	});
}

#[test]
fn evolve_with_external_uri_when_sender_is_not_collection_owner_should_fail() {
	new_test_ext().execute_with(|| {
		let who = AccountId::from_str(ALICE).unwrap();
		let owner = AccountId::from_str(BOB).unwrap();
		let collection_id = create_collection(BOB);
		let slot = Slot::try_from(0).unwrap();
		let token_id = slot_and_owner_to_token_id(slot, owner);
		let new_token_uri: TokenUriOf<Test> =
			vec![1, MaxTokenUriLength::get() as u8].try_into().unwrap();

		assert_noop!(
			LaosEvolution::evolve_with_external_uri(who, collection_id, token_id, new_token_uri),
			Error::<Test>::NoPermission
		);
	});
}

#[test]
fn evolve_with_external_uri_when_asset_doesnt_exist_should_fail() {
	new_test_ext().execute_with(|| {
		let who = AccountId::from_str(ALICE).unwrap();
		let owner = AccountId::from_str(BOB).unwrap();
		let collection_id = create_collection(ALICE);
		let slot = Slot::try_from(0).unwrap();
		let token_id = slot_and_owner_to_token_id(slot, owner);
		let new_token_uri: TokenUriOf<Test> =
			vec![1, MaxTokenUriLength::get() as u8].try_into().unwrap();

		assert_noop!(
			LaosEvolution::evolve_with_external_uri(who, collection_id, token_id, new_token_uri),
			Error::<Test>::AssetDoesNotExist
		);
	});
}

#[test]
fn evolve_with_external_uri_happy_path() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		let owner = AccountId::from_str(BOB).unwrap();
		let collection_id = create_collection(BOB);
		let slot = Slot::try_from(0).unwrap();
		let token_id = slot_and_owner_to_token_id(slot, owner);
		let token_uri: TokenUriOf<Test> =
			vec![1, MaxTokenUriLength::get() as u8].try_into().unwrap();
		let new_token_uri: TokenUriOf<Test> =
			vec![1, MaxTokenUriLength::get() as u8].try_into().unwrap();

		assert_ok!(LaosEvolution::mint_with_external_uri(
			owner,
			collection_id,
			slot,
			owner,
			token_uri.clone()
		));
		// token uri is set
		assert_eq!(LaosEvolution::token_uri(collection_id, token_id), Some(token_uri.clone()));

		assert_eq!(
			LaosEvolution::evolve_with_external_uri(
				owner,
				collection_id,
				token_id,
				new_token_uri.clone()
			),
			Ok(())
		);

		// token uri is updated and event is emitted
		assert_eq!(LaosEvolution::token_uri(collection_id, token_id), Some(new_token_uri.clone()));
		System::assert_has_event(
			Event::EvolvedWithExternalURI {
				token_id,
				collection_id,
				token_uri: new_token_uri.clone(),
			}
			.into(),
		);
	});
}

mod collection_id_conversion {
	use core::str::FromStr;

	use frame_support::assert_err;

	use crate::{
		address_to_collection_id, collection_id_to_address, mock::AccountId, CollectionError,
	};

	#[test]
	fn given_a_collection_id_from_id_to_address_works() {
		let collection_id = 5;
		let expected_address =
			AccountId::from_str("fffffffffffffffffffffffe0000000000000005").unwrap();
		assert_eq!(collection_id_to_address::<AccountId>(collection_id), expected_address);
	}

	#[test]
	fn given_invalid_format_from_address_to_id_fails() {
		let address = AccountId::from_str("0010000000000000000000010000000000000005").unwrap();
		assert_err!(address_to_collection_id::<AccountId>(address), CollectionError::InvalidPrefix);
	}

	#[test]
	fn given_valid_address_from_address_to_id_works() {
		let address = AccountId::from_str("fffffffffffffffffffffffe0000000000000005").unwrap();
		let collection_id = address_to_collection_id::<AccountId>(address).unwrap();
		assert_eq!(collection_id, 5);
	}
}
