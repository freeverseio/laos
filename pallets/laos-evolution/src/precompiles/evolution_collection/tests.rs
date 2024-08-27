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

//! Living assets precompile tests.
use super::*;
use crate::TokenId;
use fp_evm::{Context, PrecompileSet};
use mock::*;
use precompile_utils::testing::*;
use solidity::codec::Writer;
use sp_core::{H160, U256};
use std::str::FromStr;

const ALICE: &str = "0xf24FF3a9CF04c71Dbc94D0b566f7A27B94566cac";

/// Get precompiles from the mock.
fn precompiles() -> LaosPrecompiles<Test> {
	PrecompilesInstance::get()
}

/// Utility function to create a collection
///
/// Note: this function is used instead of `PrecompileTesterExt::execute_returns` because the latter
/// does not return the output of the precompile. And `PrecompileTester::execute` is a private
/// function.
fn create_collection(owner: impl Into<H160>) -> H160 {
	let owner: H160 = owner.into();

	let mut handle = MockHandle::new(
		Precompile1.into(),
		Context { address: Precompile1.into(), caller: owner, apparent_value: U256::zero() },
	);
	handle.input = FactoryPrecompileCall::create_collection { owner: Address(owner) }.into();

	let res = precompiles().execute(&mut handle).unwrap().unwrap();

	H160::from_slice(res.output.as_slice()[12..].as_ref())
}

/// Utility function to mint a token with external token uri
///
/// Note: this function is used instead of `PrecompileTesterExt::execute_returns` because the latter
/// does not return the output of the precompile. And `PrecompileTester::execute` is a private
/// function.
fn mint(
	owner: impl Into<H160>,
	collection_address: H160,
	slot: Slot,
	token_uri: UnboundedString,
) -> TokenId {
	let owner: H160 = owner.into();

	let mut handle = MockHandle::new(
		collection_address,
		Context { address: collection_address, caller: owner, apparent_value: U256::zero() },
	);

	handle.input =
		PrecompileCall::mint { to: Address(owner), slot, token_uri: token_uri.clone() }.into();

	let res = precompiles().execute(&mut handle).unwrap().unwrap();

	TokenId::from(res.output.as_slice())
}

#[test]
fn selectors() {
	assert!(PrecompileCall::owner_selectors().contains(&0x8DA5CB5B));
	assert!(PrecompileCall::mint_selectors().contains(&0xFD024566));
	assert!(PrecompileCall::evolve_selectors().contains(&0x2FD38F4D));
	assert!(PrecompileCall::transfer_ownership_selectors().contains(&0xF2FDE38B));
	assert!(PrecompileCall::token_uri_selectors().contains(&0xC87B56DD));
}

#[test]
fn unexistent_selector_should_revert() {
	new_test_ext().execute_with(|| {
		let collection_address = create_collection(Alice);
		let input = Writer::new_with_selector(0x12345678_u32).build();

		precompiles()
			.prepare_test(H160([1u8; 20]), collection_address, input)
			.execute_reverts(|r| r == b"Unknown selector");
	});
}

#[test]
fn check_log_selectors() {
	assert_eq!(
		hex::encode(SELECTOR_LOG_MINTED_WITH_EXTERNAL_TOKEN_URI),
		"a7135052b348b0b4e9943bae82d8ef1c5ac225e594ef4271d12f0744cfc98348"
	);
}

#[test]
fn owner_of_non_existent_collection_should_revert() {
	new_test_ext().execute_with(|| {
		let collection_address =
			H160::from_str("fffffffffffffffffffffffe0000000000000000").unwrap();

		precompiles()
			.prepare_test(Alice, collection_address, PrecompileCall::owner {})
			.execute_reverts(|r| r == b"collection does not exist");
	})
}

#[test]
fn owner_of_invalid_collection_address() {
	new_test_ext().execute_with(|| {
		let _invalid_address = H160::from_str("0000000000000000000000000000000000000005").unwrap();

		// let _input = EvmDataWriter::new_with_selector(Action::Owner).build();

		// TODO: Uncomment this when this PR is merged: https://github.com/paritytech/frontier/pull/1248
		// Above PR fixes a bug in `execute_none()`
		// precompiles()
		// 	.prepare_test(H160([1u8; 20]), invalid_address, PrecompileCall::owner {})
		// 	.execute_none();
	});
}

#[test]
fn owner_of_collection_works() {
	new_test_ext().execute_with(|| {
		let collection_address = create_collection(Alice);
		precompiles()
			.prepare_test(Alice, collection_address, PrecompileCall::owner {})
			.execute_returns(Address(Alice.into()));
	});
}

#[test]
fn mint_should_generate_log() {
	new_test_ext().execute_with(|| {
		let collection_address = create_collection(Alice);
		let slot: Slot = 9.try_into().unwrap();
		let token_uri: UnboundedString = "ciao".into();
		let expected_token_id =
			U256::from_str("0000000000000000000000090101010101010101010101010101010101010101")
				.unwrap();
		let owner = H160([1u8; 20]);

		precompiles()
			.prepare_test(
				Alice,
				collection_address,
				PrecompileCall::mint { to: Address(owner), slot, token_uri: token_uri.clone() },
			)
			.expect_log(log2(
				collection_address,
				SELECTOR_LOG_MINTED_WITH_EXTERNAL_TOKEN_URI,
				owner,
				solidity::encode_event_data((slot, expected_token_id, token_uri)),
			))
			.execute_some();
	});
}

#[test]
fn mint_asset_in_an_existing_collection_works() {
	new_test_ext().execute_with(|| {
		let to = H160::from_str(ALICE).unwrap();
		let collection_address = create_collection(to);
		let slot = 1.try_into().unwrap();
		let token_uri: UnboundedString = [1u8; 20].into();

		// concat of `slot` and `owner` is used as token id
		// note: slot is u96, owner is H160
		let expected_token_id = U256::from_str(&format!(
			"{}{}",
			"000000000000000000000001",
			ALICE.trim_start_matches("0x")
		))
		.unwrap();

		precompiles()
			.prepare_test(
				to,
				collection_address,
				PrecompileCall::mint { to: Address(to), slot, token_uri: token_uri.clone() },
			)
			.execute_returns(expected_token_id);
	});
}

#[test]
fn when_mint_reverts_should_return_error() {
	new_test_ext().execute_with(|| {
		let to = H160::from_str(ALICE).unwrap();
		let collection_address = create_collection(to);
		let slot = 0.try_into().unwrap();
		let token_uri: UnboundedString = Vec::new().into();

		let _occupied_slot_token_id = mint(to, collection_address, slot, token_uri.clone());

		precompiles()
			.prepare_test(
				to,
				collection_address,
				PrecompileCall::mint { to: Address(to), slot, token_uri },
			)
			.execute_reverts(|r| r == b"AlreadyMinted");
	});
}

#[test]
fn token_uri_reverts_when_asset_does_not_exist() {
	new_test_ext().execute_with(|| {
		let alice = H160::from_str(ALICE).unwrap();
		let collection_address = create_collection(alice);

		precompiles()
			.prepare_test(
				alice,
				collection_address,
				PrecompileCall::token_uri { token_id: TokenId::from(0) },
			)
			.execute_reverts(|r| r == b"asset does not exist");
	});
}

#[test]
fn token_uri_returns_the_result_from_source() {
	new_test_ext().execute_with(|| {
		let alice = H160::from_str(ALICE).unwrap();
		let collection_address = create_collection(alice);
		let token_uri: UnboundedString = "ciao".into();
		let token_id = mint(alice, collection_address, 0.try_into().unwrap(), token_uri.clone());

		precompiles()
			.prepare_test(alice, collection_address, PrecompileCall::token_uri { token_id })
			.execute_returns(token_uri);
	});
}

#[test]
fn evolve_a_minted_asset_works() {
	new_test_ext().execute_with(|| {
		let alice = H160::from_str(ALICE).unwrap();
		let collection_address = create_collection(alice);
		let token_uri: UnboundedString = Vec::new().into();
		let token_id = mint(alice, collection_address, 0.try_into().unwrap(), token_uri.clone());

		precompiles()
			.prepare_test(alice, collection_address, PrecompileCall::evolve { token_id, token_uri })
			.execute_returns_raw(vec![]);
	});
}

#[test]
fn evolve_generates_log() {
	new_test_ext().execute_with(|| {
		let alice = H160::from_str(ALICE).unwrap();
		let collection_address = create_collection(alice);
		let token_uri: UnboundedString = Vec::new().into();
		let token_id = mint(alice, collection_address, 0.try_into().unwrap(), token_uri.clone());

		let mut token_id_bytes = [0u8; 32];
		token_id.to_big_endian(&mut token_id_bytes);

		precompiles()
			.prepare_test(
				alice,
				collection_address,
				PrecompileCall::evolve { token_id, token_uri: token_uri.clone() },
			)
			.expect_log(log2(
				collection_address,
				SELECTOR_LOG_EVOLVED_WITH_EXTERNAL_TOKEN_URI,
				token_id_bytes,
				solidity::encode_event_data(token_uri),
			))
			.execute_some();
	});
}

#[test]
fn when_evolve_reverts_should_return_error() {
	new_test_ext().execute_with(|| {
		let alice = H160::from_str(ALICE).unwrap();
		let collection_address = create_collection(alice);
		let token_uri: UnboundedString = Vec::new().into();
		let token_id = U256::from(1);

		precompiles()
			.prepare_test(alice, collection_address, PrecompileCall::evolve { token_id, token_uri })
			.execute_reverts(|r| r == b"AssetDoesNotExist");
	});
}

// #[test]
// fn test_expected_cost_token_uri() {
// 	new_test_ext().execute_with(|| {
// 		let alice = H160::from_str(ALICE).unwrap();
// 		let collection_address = create_collection(alice);
// 		let token_id = mint(alice, collection_address, 0, Vec::new());

// 		let input = EvmDataWriter::new_with_selector(Action::TokenURI).write(token_id).build();

// 		// Expected weight of the precompile call implementation.
// 		// Since benchmarking precompiles is not supported yet, we are benchmarking
// 		// functions that precompile calls internally.
// 		precompiles()
// 			.prepare_test(alice, collection_address, input)
// 			.expect_cost(25229000) //  [`WeightToGas`] set to 1:1 in mock
// 			.execute_some();
// 	})
// }

#[test]
fn expected_cost_owner() {
	new_test_ext().execute_with(|| {
		let alice = H160::from_str(ALICE).unwrap();
		let collection_address = create_collection(alice);

		// Expected weight of the precompile call implementation.
		// Since benchmarking precompiles is not supported yet, we are benchmarking
		// functions that precompile calls internally.
		precompiles()
			.prepare_test(alice, collection_address, PrecompileCall::owner {})
			.expect_cost(57599000) //  [`WeightToGas`] set to 1:1 in mock
			.execute_some();
	})
}

#[test]
fn expected_cost_mint_with_external_uri() {
	new_test_ext().execute_with(|| {
		let owner = H160([1u8; 20]);
		let collection_address = create_collection(owner);

		// Expected weight of the precompile call implementation.
		// Since benchmarking precompiles is not supported yet, we are benchmarking
		// functions that precompile calls internally.
		//
		// Following `cost` is calculated as:
		// `mint_with_external_uri` weight + log cost
		precompiles()
			.prepare_test(
				owner,
				collection_address,
				PrecompileCall::mint {
					to: Address(owner),
					slot: 9.try_into().unwrap(),
					token_uri: "ciao".into(),
				},
			)
			.expect_cost(197820463) // [`WeightToGas`] set to 1:1 in mock
			.execute_some();
	})
}

#[test]
fn expected_cost_evolve_with_external_uri() {
	new_test_ext().execute_with(|| {
		let alice = H160::from_str(ALICE).unwrap();
		let collection_address = create_collection(alice);
		let token_id = mint(alice, collection_address, 0.try_into().unwrap(), Vec::new().into());

		// Expected weight of the precompile call implementation.
		// Since benchmarking precompiles is not support.try_into().unwrap()ed yet, we are
		// benchmarking functions that precompile calls internally.
		//
		// Following `cost` is calculated as:
		// `evolve_with_external_uri` weight + log cost
		precompiles()
			.prepare_test(
				alice,
				collection_address,
				PrecompileCall::evolve { token_id, token_uri: Vec::new().into() },
			)
			.expect_cost(196356684) // [`WeightToGas`] set to 1:1 in mock
			.execute_some();
	})
}

#[test]
fn collection_transfer_of_ownership_works() {
	new_test_ext().execute_with(|| {
		let alice = H160::from_str(ALICE).unwrap();
		let bob = H160([2u8; 20]);

		let collection_address = create_collection(alice);

		precompiles()
			.prepare_test(
				alice,
				collection_address,
				PrecompileCall::transfer_ownership { to: bob.into() },
			)
			.execute_some();
	});
}

#[test]
fn non_existent_collection_cannot_be_transferred() {
	new_test_ext().execute_with(|| {
		let alice = H160::from_str(ALICE).unwrap();
		let bob = H160([2u8; 20]);

		// non existing collection address
		let non_existing_collection_address =
			H160::from_str("fffffffffffffffffffffffe0000000000000000").unwrap();

		precompiles()
			.prepare_test(
				alice,
				non_existing_collection_address,
				PrecompileCall::transfer_ownership { to: bob.into() },
			)
			.execute_reverts(|r| r == b"CollectionDoesNotExist");
	})
}

#[test]
fn non_owner_cannot_transfer_collection_ownership() {
	new_test_ext().execute_with(|| {
		let alice = H160::from_str(ALICE).unwrap();
		let bob = H160([2u8; 20]);

		let collection_address = create_collection(alice);

		precompiles()
			.prepare_test(
				bob,
				collection_address,
				PrecompileCall::transfer_ownership { to: alice.into() },
			)
			.execute_reverts(|r| r == b"NoPermission");
	});
}

#[test]
fn collection_transfer_of_ownership_emits_log() {
	new_test_ext().execute_with(|| {
		let alice = H160::from_str(ALICE).unwrap();
		let bob = H160([2u8; 20]);
		let collection_address = create_collection(alice);

		precompiles()
			.prepare_test(
				alice,
				collection_address,
				PrecompileCall::transfer_ownership { to: bob.into() },
			)
			.expect_log(log3(
				collection_address,
				SELECTOR_LOG_OWNERSHIP_TRANSFERRED,
				alice,
				bob,
				vec![],
			))
			.execute_some();
	});
}

#[test]
fn collection_transfer_of_ownership_records_costs() {
	new_test_ext().execute_with(|| {
		let alice = H160::from_str(ALICE).unwrap();
		let bob = H160([2u8; 20]);
		let collection_address = create_collection(alice);

		// 1 read and 1 write
		precompiles()
			.prepare_test(
				alice,
				collection_address,
				PrecompileCall::transfer_ownership { to: bob.into() },
			)
			.expect_cost(162126000) //  [`WeightToGas`] set to 1:1 in mock
			.execute_some();
	});
}
