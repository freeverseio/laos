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

use crate::{
	mock::*,
	traits::AssetMetadataExtender as _,
	types::{AccountIdOf, TokenUriOf, UniversalLocationOf},
	Error, Event,
};
use frame_support::{assert_noop, assert_ok};
use sp_core::{bounded_vec, H160};

// UL stands for Universal Location

fn create_token_uri_extension(
	claimer: AccountIdOf<Test>,
	universal_location: UniversalLocationOf<Test>,
	token_uri: TokenUriOf<Test>,
) {
	assert_ok!(AssetMetadataExtender::create_token_uri_extension(
		claimer,
		universal_location,
		token_uri
	));
}

#[test]
fn create_token_uri_extension_works() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		let claimer = H160::zero();
		let universal_location: UniversalLocationOf<Test> = bounded_vec![1; 10];
		let token_uri: TokenUriOf<Test> = bounded_vec![2; 10];

		create_token_uri_extension(claimer, universal_location.clone(), token_uri.clone());

		assert_eq!(
			AssetMetadataExtender::token_uris_by_claimer_and_location(
				claimer,
				universal_location.clone()
			)
			.unwrap(),
			token_uri
		);

		System::assert_last_event(
			Event::ExtensionCreated { universal_location, claimer, token_uri }.into(),
		);
	});
}

#[test]
fn claimer_cannot_create_multiple_extensions_per_ul() {
	new_test_ext().execute_with(|| {
		let claimer = H160::zero();
		let universal_location: UniversalLocationOf<Test> = bounded_vec![1; 10];
		let token_uri: TokenUriOf<Test> = bounded_vec![2; 10];

		create_token_uri_extension(claimer, universal_location.clone(), token_uri.clone());
		assert_noop!(
			AssetMetadataExtender::create_token_uri_extension(
				claimer,
				universal_location,
				token_uri
			),
			Error::<Test>::ExtensionAlreadyExists
		);
	});
}

#[test]
fn create_token_uri_extension_increases_counter() {
	new_test_ext().execute_with(|| {
		let claimer = H160::zero();
		let universal_location: UniversalLocationOf<Test> = bounded_vec![1; 10];
		let token_uri: TokenUriOf<Test> = bounded_vec![2; 10];

		// create first extension for the given UL
		assert_eq!(AssetMetadataExtender::extensions_counter(universal_location.clone()), 0);

		create_token_uri_extension(claimer, universal_location.clone(), token_uri.clone());
		assert_eq!(AssetMetadataExtender::extensions_counter(universal_location.clone()), 1);

		// check that no other UL has been affected
		let another_universal_location: UniversalLocationOf<Test> = bounded_vec![1; 1];
		assert_eq!(AssetMetadataExtender::extensions_counter(another_universal_location), 0);

		// create another extension for the same UL with another claimer
		let another_claimer = H160::from_low_u64_be(1);
		create_token_uri_extension(another_claimer, universal_location.clone(), token_uri);
		assert_eq!(AssetMetadataExtender::extensions_counter(universal_location), 2);
	});
}

#[test]
fn get_all_token_uris_and_claimers_from_extensions_works() {
	new_test_ext().execute_with(|| {
		let universal_location: UniversalLocationOf<Test> = bounded_vec![1; 10];
		let token_uri_expected: TokenUriOf<Test> = bounded_vec![2; 10];

		let n = 1000;
		for i in 0..n {
			let claimer = H160::from_low_u64_be(i);
			create_token_uri_extension(
				claimer,
				universal_location.clone(),
				token_uri_expected.clone(),
			);
		}

		for i in 0..n {
			let claimer = AssetMetadataExtender::claimers_by_location_and_index(
				universal_location.clone(),
				i as u32,
			)
			.unwrap();
			let token_uri = AssetMetadataExtender::token_uris_by_claimer_and_location(
				claimer,
				universal_location.clone(),
			)
			.unwrap();
			assert_eq!(token_uri, token_uri_expected);
			assert_eq!(claimer, H160::from_low_u64_be(i));
		}
	});
}

#[test]
fn get_token_uris_by_claimer_and_location_works() {
	new_test_ext().execute_with(|| {
		let claimer = H160::zero();
		let universal_location: UniversalLocationOf<Test> = bounded_vec![1; 10];
		let token_uri: TokenUriOf<Test> = bounded_vec![2; 10];

		create_token_uri_extension(claimer, universal_location.clone(), token_uri.clone());
		assert_eq!(
			AssetMetadataExtender::token_uris_by_claimer_and_location(
				claimer,
				universal_location.clone()
			)
			.unwrap(),
			token_uri
		);
	});
}

#[test]
fn update_extension_works() {
	new_test_ext().execute_with(|| {
		let claimer = H160::zero();
		let universal_location: UniversalLocationOf<Test> = bounded_vec![1; 10];
		let token_uri: TokenUriOf<Test> = bounded_vec![2; 10];
		let new_token_uri: TokenUriOf<Test> = bounded_vec![3; 10];

		create_token_uri_extension(claimer, universal_location.clone(), token_uri.clone());
		assert_eq!(
			AssetMetadataExtender::token_uris_by_claimer_and_location(
				claimer,
				universal_location.clone()
			)
			.unwrap(),
			token_uri
		);

		assert_ok!(AssetMetadataExtender::update_token_uri_extension(
			claimer,
			universal_location.clone(),
			new_token_uri.clone()
		));
		assert_eq!(
			AssetMetadataExtender::token_uris_by_claimer_and_location(
				claimer,
				universal_location.clone()
			)
			.unwrap(),
			new_token_uri
		);
	});
}

#[test]
fn after_update_extension_it_returns_the_new_value() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		let claimer = H160::zero();
		let universal_location: UniversalLocationOf<Test> = bounded_vec![1; 10];
		let token_uri: TokenUriOf<Test> = bounded_vec![2; 10];
		let new_token_uri: TokenUriOf<Test> = bounded_vec![3; 10];

		create_token_uri_extension(claimer, universal_location.clone(), token_uri.clone());
		assert_eq!(
			AssetMetadataExtender::token_uris_by_claimer_and_location(
				claimer,
				universal_location.clone()
			)
			.unwrap(),
			token_uri
		);

		assert_ok!(AssetMetadataExtender::update_token_uri_extension(
			claimer,
			universal_location.clone(),
			new_token_uri.clone()
		));
		assert_eq!(
			AssetMetadataExtender::token_uris_by_claimer_and_location(
				claimer,
				universal_location.clone()
			)
			.unwrap(),
			new_token_uri
		);

		System::assert_last_event(
			Event::ExtensionUpdated { universal_location, claimer, token_uri: new_token_uri }
				.into(),
		);
	});
}

#[test]
fn update_extension_fails_if_it_does_not_exist() {
	new_test_ext().execute_with(|| {
		let claimer = H160::zero();
		let universal_location: UniversalLocationOf<Test> = bounded_vec![1; 10];
		let token_uri: TokenUriOf<Test> = bounded_vec![2; 10];

		assert_noop!(
			AssetMetadataExtender::update_token_uri_extension(
				claimer,
				universal_location.clone(),
				token_uri.clone()
			),
			Error::<Test>::ExtensionDoesNotExist
		);
	});
}

#[test]

fn after_update_extension_counter_does_not_increase() {
	new_test_ext().execute_with(|| {
		let claimer = H160::zero();
		let universal_location: UniversalLocationOf<Test> = bounded_vec![1; 10];
		let token_uri: TokenUriOf<Test> = bounded_vec![2; 10];
		let new_token_uri: TokenUriOf<Test> = bounded_vec![3; 10];

		create_token_uri_extension(claimer, universal_location.clone(), token_uri.clone());
		assert_eq!(AssetMetadataExtender::extensions_counter(universal_location.clone()), 1);
		assert_ok!(AssetMetadataExtender::update_token_uri_extension(
			claimer,
			universal_location.clone(),
			new_token_uri.clone()
		));
		assert_eq!(AssetMetadataExtender::extensions_counter(universal_location.clone()), 1);
	});
}

#[test]
fn balance_of_works() {
	new_test_ext().execute_with(|| {
		let universal_location: UniversalLocationOf<Test> = bounded_vec![1; 10];
		let token_uri: TokenUriOf<Test> = bounded_vec![2; 10];

		assert_eq!(AssetMetadataExtender::balance_of(universal_location.clone()), 0);
		create_token_uri_extension(H160::zero(), universal_location.clone(), token_uri.clone());
		assert_eq!(AssetMetadataExtender::balance_of(universal_location.clone()), 1);
	});
}

#[test]
fn claimer_by_index_works() {
	new_test_ext().execute_with(|| {
		let universal_location: UniversalLocationOf<Test> = bounded_vec![1; 10];

		assert_eq!(AssetMetadataExtender::claimer_by_index(universal_location.clone(), 0), None);

		let token_uri: TokenUriOf<Test> = bounded_vec![2; 10];

		let n = 10_u32;
		for i in 0..n {
			let claimer = H160::from_low_u64_be(i as u64);
			create_token_uri_extension(claimer, universal_location.clone(), token_uri.clone());
		}

		for i in 0..n {
			let claimer = AssetMetadataExtender::claimer_by_index(universal_location.clone(), i);
			assert_eq!(claimer, Some(H160::from_low_u64_be(i as u64)));
		}
	});
}

#[test]
fn token_uri_extension_by_index_works() {
	new_test_ext().execute_with(|| {
		let universal_location: UniversalLocationOf<Test> = bounded_vec![1; 10];

		assert_eq!(
			AssetMetadataExtender::token_uri_extension_by_index(universal_location.clone(), 0),
			None
		);

		let token_uri_expected: TokenUriOf<Test> = bounded_vec![2; 10];

		let n = 10_u32;
		for i in 0..n {
			let claimer = H160::from_low_u64_be(i as u64);
			create_token_uri_extension(
				claimer,
				universal_location.clone(),
				token_uri_expected.clone(),
			);
		}

		for i in 0..n {
			let token_uri =
				AssetMetadataExtender::token_uri_extension_by_index(universal_location.clone(), i);
			assert_eq!(token_uri, Some(token_uri_expected.clone()));
		}
	});
}

#[test]
fn get_unexistent_extension_by_location_and_claimer_fails() {
	new_test_ext().execute_with(|| {
		let claimer = H160::zero();
		let universal_location: UniversalLocationOf<Test> = bounded_vec![1; 10];

		assert_eq!(
			AssetMetadataExtender::extension_by_location_and_claimer(
				universal_location.clone(),
				claimer
			),
			None
		);
	});
}

#[test]
fn get_extension_by_location_and_claimer_works() {
	new_test_ext().execute_with(|| {
		let claimer = H160::zero();
		let universal_location: UniversalLocationOf<Test> = bounded_vec![1; 10];
		let token_uri: TokenUriOf<Test> = bounded_vec![2; 10];

		create_token_uri_extension(claimer, universal_location.clone(), token_uri.clone());
		assert_eq!(
			AssetMetadataExtender::extension_by_location_and_claimer(
				universal_location.clone(),
				claimer
			)
			.unwrap(),
			token_uri
		);
	});
}

#[test]
fn has_extension_should_return_true_if_it_exists() {
	new_test_ext().execute_with(|| {
		let claimer = H160::zero();
		let universal_location: UniversalLocationOf<Test> = bounded_vec![1; 10];
		let token_uri: TokenUriOf<Test> = bounded_vec![2; 10];

		create_token_uri_extension(claimer, universal_location.clone(), token_uri.clone());
		assert!(AssetMetadataExtender::has_extension(universal_location, claimer));
	});
}

#[test]
fn has_extension_should_return_false_if_it_does_not_exist() {
	new_test_ext().execute_with(|| {
		let claimer = H160::zero();
		let universal_location: UniversalLocationOf<Test> = bounded_vec![1; 10];

		assert!(!AssetMetadataExtender::has_extension(universal_location, claimer));
	});
}
