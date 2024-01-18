//! Living assets precompile tests.

//TODO: remove this and fix clippy issues
#![allow(clippy::redundant_closure_call)]

use core::str::FromStr;

use super::*;
use evm::Context;
use fp_evm::{Log, PrecompileSet};
use mock::*;
use pallet_evm_evolution_collection_factory::Action as CollectionFactoryAction;
use precompile_utils::{
	solidity::codec::BoundedBytes,
	testing::{execution::PrecompileTesterExt, MockHandle},
};
use sp_core::{H160, H256, U256};

const ALICE: &str = "0xf24FF3a9CF04c71Dbc94D0b566f7A27B94566cac";

/// Get precompiles from the mock.
fn precompiles() -> MockPrecompileSet<Test> {
	MockPrecompiles::get()
}

/// Utility function to create a collection
///
/// Note: this function is used instead of `PrecompileTesterExt::execute_returns` because the latter
/// does not return the output of the precompile. And `PrecompileTester::execute` is a private
/// function.
fn create_collection(owner: impl Into<H160>) -> H160 {
	let owner: H160 = owner.into();
	let input = EvmDataWriter::new_with_selector(CollectionFactoryAction::CreateCollection)
		.write(Address(owner.clone()))
		.build();

	let mut handle = MockHandle::new(
		EVOLUTION_FACTORY_PRECOMPILE_ADDRESS.into(),
		Context {
			address: EVOLUTION_FACTORY_PRECOMPILE_ADDRESS.into(),
			caller: owner,
			apparent_value: U256::zero(),
		},
	);

	handle.input = input;

	let res = precompiles().execute(&mut handle).unwrap().unwrap();

	H160::from_slice(&res.output.as_slice()[12..].as_ref())
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
	token_uri: Vec<u8>,
) -> TokenId {
	let owner: H160 = owner.into();
	let input = EvmDataWriter::new_with_selector(Action::Mint)
		.write(Address(owner.clone()))
		.write(U256::from(slot))
		.write(Bytes(token_uri))
		.build();

	let mut handle = MockHandle::new(
		collection_address,
		Context { address: collection_address, caller: owner, apparent_value: U256::zero() },
	);

	handle.input = input;

	let res = precompiles().execute(&mut handle).unwrap().unwrap();

	TokenId::from(res.output.as_slice())
}

#[test]
fn check_log_selectors() {
	assert_eq!(
		hex::encode(SELECTOR_LOG_MINTED_WITH_EXTERNAL_TOKEN_URI),
		"a7135052b348b0b4e9943bae82d8ef1c5ac225e594ef4271d12f0744cfc98348"
	);
	assert_eq!(
		hex::encode(SELECTOR_LOG_EVOLVED_WITH_EXTERNAL_TOKEN_URI),
		"dde18ad2fe10c12a694de65b920c02b851c382cf63115967ea6f7098902fa1c8"
	);
}

#[test]
fn function_selectors() {
	assert_eq!(Action::Owner as u32, 0x8DA5CB5B);
	assert_eq!(Action::TokenURI as u32, 0xC87B56DD);
	assert_eq!(Action::Mint as u32, 0xFD024566);
	assert_eq!(Action::Evolve as u32, 0x2FD38F4D);
}

#[test]
fn mint_should_generate_log() {
	new_test_ext().execute_with(|| {
		let owner = H160([1u8; 20]);
		let collection_address = create_collection(owner);

		let input = EvmDataWriter::new_with_selector(Action::Mint)
			.write(Address(owner)) // to
			.write(U256::from(9)) // slot
			.write(Bytes("ciao".into())) // token_uri
			.build();

		let expected_log = Log {
			address: collection_address,
			topics: vec![
				SELECTOR_LOG_MINTED_WITH_EXTERNAL_TOKEN_URI.into(),
				H256::from_str(
					"0x0000000000000000000000000101010101010101010101010101010101010101",
				)
				.unwrap(),
			],
			data: vec![
				0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
				0, 0, 0, 9, // slot
				0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
				1, 1, 1, 1, // token id
				0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
				0, 0, 0, 96, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
				0, 0, 0, 0, 0, 0, 0, 0, 4, // token uri length
				99, 105, 97, 111, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
				0, 0, 0, 0, 0, 0, // token uri
			],
		};

		precompiles()
			.prepare_test(owner, collection_address, input)
			.expect_log(expected_log)
			.execute_some();
	});
}

#[test]
fn unexistent_selector_should_revert() {
	new_test_ext().execute_with(|| {
		let input = EvmDataWriter::new_with_selector(0x12345678 as u32).build();

		precompiles()
			.prepare_test(H160([1u8; 20]), H160(EVOLUTION_FACTORY_PRECOMPILE_ADDRESS), input)
			.execute_reverts(|r| r == b"unknown selector");
	});
}

#[test]
fn owner_of_non_existent_collection_should_revert() {
	new_test_ext().execute_with(|| {
		let alice = H160::from_str(ALICE).unwrap();
		let collection_address =
			H160::from_str("fffffffffffffffffffffffe0000000000000000").unwrap();

		let input = EvmDataWriter::new_with_selector(Action::Owner).build();

		precompiles()
			.prepare_test(alice, collection_address, input)
			.execute_reverts(|r| r == b"collection does not exist");
	})
}

#[test]
fn owner_of_invalid_collection_address() {
	new_test_ext().execute_with(|| {
		let _invalid_address = H160::from_str("0000000000000000000000000000000000000005").unwrap();

		let _input = EvmDataWriter::new_with_selector(Action::Owner).build();

		// TODO: Uncomment this when this PR is merged: https://github.com/paritytech/frontier/pull/1248
		// Above PR fixes a bug in `execute_none()`
		// precompiles()
		// 	.prepare_test(H160([1u8; 20]), invalid_address, input)
		// 	.execute_none();
	});
}

#[test]
fn owner_of_collection_works() {
	new_test_ext().execute_with(|| {
		let alice = H160::from_str(ALICE).unwrap();
		let collection_address = create_collection(alice);

		let input = EvmDataWriter::new_with_selector(Action::Owner).build();

		precompiles().prepare_test(alice, collection_address, input).execute_returns(
			H256::from_str(
				format!("000000000000000000000000{}", ALICE.trim_start_matches("0x")).as_str(),
			)
			.unwrap(),
		);
	});
}

#[test]
fn token_uri_reverts_when_asset_does_not_exist() {
	new_test_ext().execute_with(|| {
		let alice = H160::from_str(ALICE).unwrap();
		let collection_address = create_collection(alice);

		let input = EvmDataWriter::new_with_selector(Action::TokenURI)
			.write(TokenId::from(0))
			.build();

		precompiles()
			.prepare_test(alice, collection_address, input)
			.execute_reverts(|r| r == b"asset does not exist");
	});
}

#[test]
fn token_uri_returns_the_result_from_source() {
	new_test_ext().execute_with(|| {
		let alice = H160::from_str(ALICE).unwrap();
		let collection_address = create_collection(alice);
		let token_id = mint(alice, collection_address, 0, Vec::new());

		let input = EvmDataWriter::new_with_selector(Action::TokenURI).write(token_id).build();

		precompiles()
			.prepare_test(alice, collection_address, input)
			.execute_returns(BoundedBytes::<MaxTokenUriLength>::from(Vec::new()));
	});
}

#[test]
fn mint_asset_in_an_existing_collection_works() {
	new_test_ext().execute_with(|| {
		let to = H160::from_str(ALICE).unwrap();
		let collection_address = create_collection(to);

		let input = EvmDataWriter::new_with_selector(Action::Mint)
			.write(Address(to))
			.write(U256::from(1))
			.write(Bytes([1u8; 20].to_vec()))
			.build();

		// concat of `slot` and `owner` is used as token id
		// note: slot is u96, owner is H160
		let expected_token_id =
			format!("{}{}", "000000000000000000000001", ALICE.trim_start_matches("0x"));

		precompiles()
			.prepare_test(to, collection_address, input)
			.execute_returns(H256::from_str(expected_token_id.as_str()).unwrap());
	});
}

#[test]
fn when_mint_reverts_should_return_error() {
	new_test_ext().execute_with(|| {
		let to = H160::from_str(ALICE).unwrap();
		let collection_address = create_collection(to);

		let _occupied_slot_token_id = mint(to, collection_address, 0, Vec::new());

		let input = EvmDataWriter::new_with_selector(Action::Mint)
			.write(Address(to))
			.write(U256::zero())
			.write(Bytes([1u8; 20].to_vec()))
			.build();

		precompiles()
			.prepare_test(to, collection_address, input)
			.execute_reverts(|r| r == b"AlreadyMinted");
	});
}

#[test]
fn evolve_a_minted_asset_works() {
	new_test_ext().execute_with(|| {
		let alice = H160::from_str(ALICE).unwrap();
		let collection_address = create_collection(alice);
		let token_id = mint(alice, collection_address, 0, Vec::new());

		let input = EvmDataWriter::new_with_selector(Action::Evolve)
			.write(token_id)
			.write(Bytes([1u8; 20].to_vec()))
			.build();

		precompiles()
			.prepare_test(alice, collection_address, input)
			.execute_returns_raw(vec![]);
	});
}

#[test]
fn evolve_generates_log() {
	new_test_ext().execute_with(|| {
		let alice = H160::from_str(ALICE).unwrap();
		let collection_address = create_collection(alice);
		let token_id = mint(alice, collection_address, 0, Vec::new());

		let input = EvmDataWriter::new_with_selector(Action::Evolve)
			.write(token_id)
			.write(Bytes([1u8; 20].to_vec()))
			.build();

		let expected_log = Log {
			address: collection_address,
			topics: vec![
				SELECTOR_LOG_EVOLVED_WITH_EXTERNAL_TOKEN_URI.into(),
				H256::from_str(
					"0x000000000000000000000000f24ff3a9cf04c71dbc94d0b566f7a27b94566cac",
				)
				.unwrap(),
			],
			data: vec![
				0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
				0, 0, 0, 32, // offset
				0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
				0, 0, 0, 20, // length of token_uri
				1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0,
				0, 0, 0, 0, // token_uri
			],
		};

		precompiles()
			.prepare_test(alice, collection_address, input)
			.expect_log(expected_log)
			.execute_some();
	});
}

#[test]
fn when_evolve_reverts_should_return_error() {
	new_test_ext().execute_with(|| {
		let alice = H160::from_str(ALICE).unwrap();
		let collection_address = create_collection(alice);
		let token_id = U256::from(1);

		let input = EvmDataWriter::new_with_selector(Action::Evolve)
			.write(token_id)
			.write(Bytes([1u8; 20].to_vec()))
			.build();

		precompiles()
			.prepare_test(alice, collection_address, input)
			.execute_reverts(|r| r == b"AssetDoesNotExist");
	});
}

#[test]
fn enable_public_minting_generates_log() {
	// check also is_public_minting_enabled
	todo!("unimplemented")
}

#[test]
fn when_enable_public_minting_reverts_should_return_error() {
	todo!("unimplemented")
}

#[test]
fn disable_public_minting_generates_log() {
	// check also is_public_minting_enabled
	todo!("unimplemented")
}

#[test]
fn when_disable_public_minting_reverts_should_return_error() {
	todo!("unimplemented")
}

#[test]
fn test_expected_cost_token_uri() {
	new_test_ext().execute_with(|| {
		let alice = H160::from_str(ALICE).unwrap();
		let collection_address = create_collection(alice);
		let token_id = mint(alice, collection_address, 0, Vec::new());

		let input = EvmDataWriter::new_with_selector(Action::TokenURI).write(token_id).build();

		// Expected weight of the precompile call implementation.
		// Since benchmarking precompiles is not supported yet, we are benchmarking
		// functions that precompile calls internally.
		precompiles()
			.prepare_test(alice, collection_address, input)
			.expect_cost(25000000) //  [`WeightToGas`] set to 1:1 in mock
			.execute_some();
	})
}

#[test]
fn enable_public_minting_has_a_cost() {
	todo!("unimplemented")
}

#[test]
fn disable_public_minting_has_a_cost() {
	todo!("unimplemented")
}

#[test]
fn is_public_minting_enabled_has_a_cost() {
	todo!("unimplemented")
}

#[test]
fn test_expected_cost_owner() {
	new_test_ext().execute_with(|| {
		let alice = H160::from_str(ALICE).unwrap();
		let collection_address = create_collection(alice);

		let input = EvmDataWriter::new_with_selector(Action::Owner).build();

		// Expected weight of the precompile call implementation.
		// Since benchmarking precompiles is not supported yet, we are benchmarking
		// functions that precompile calls internally.
		precompiles()
			.prepare_test(alice, collection_address, input)
			.expect_cost(25000000) //  [`WeightToGas`] set to 1:1 in mock
			.execute_some();
	})
}

#[test]
fn test_expected_cost_mint_with_external_uri() {
	new_test_ext().execute_with(|| {
		let owner = H160([1u8; 20]);
		let collection_address = create_collection(owner);

		let token_uri = Bytes("ciao".into());

		let input = EvmDataWriter::new_with_selector(Action::Mint)
			.write(Address(owner)) // to
			.write(U256::from(9)) // slot
			.write(token_uri.clone()) // token_uri
			.build();

		// Expected weight of the precompile call implementation.
		// Since benchmarking precompiles is not supported yet, we are benchmarking
		// functions that precompile calls internally.
		//
		// Following `cost` is calculated as:
		// `mint_with_external_uri` weight + log cost
		precompiles()
			.prepare_test(owner, collection_address, input)
			.expect_cost(190104055) // [`WeightToGas`] set to 1:1 in mock
			.execute_some();
	})
}

#[test]
fn test_expected_cost_evolve_with_external_uri() {
	new_test_ext().execute_with(|| {
		let alice = H160::from_str(ALICE).unwrap();
		let collection_address = create_collection(alice);
		let token_id = mint(alice, collection_address, 0, Vec::new());

		let input = EvmDataWriter::new_with_selector(Action::Evolve)
			.write(token_id)
			.write(Bytes([1u8; 20].to_vec()))
			.build();

		// Expected weight of the precompile call implementation.
		// Since benchmarking precompiles is not supported yet, we are benchmarking
		// functions that precompile calls internally.
		//
		// Following `cost` is calculated as:
		// `evolve_with_external_uri` weight + log cost
		precompiles()
			.prepare_test(alice, collection_address, input)
			.expect_cost(163256735) // [`WeightToGas`] set to 1:1 in mock
			.execute_some();
	})
}
