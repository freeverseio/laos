//! Living assets precompile tests.

//TODO: remove this and fix clippy issues
#![allow(clippy::redundant_closure_call)]

use core::str::FromStr;

use super::*;
use evm::Context;
use fp_evm::{Log, PrecompileSet};
use mock::*;
use precompile_utils::testing::{execution::PrecompileTesterExt, MockHandle};
use sp_core::{H160, H256, U256};

const ALICE: &str = "0xf24FF3a9CF04c71Dbc94D0b566f7A27B94566cac";

fn precompiles() -> MockPrecompileSet<Test> {
	MockPrecompiles::get()
}

/// Utility function to create a collection
fn create_collection(owner: impl Into<H160>) -> H160 {
	let owner: H160 = owner.into();
	let input = EvmDataWriter::new_with_selector(Action::CreateCollection)
		.write(Address(owner.clone()))
		.build();

	let collection_address = {
		let mut handle = MockHandle::new(
			PRECOMPILE_ADDRESS.into(),
			Context {
				address: PRECOMPILE_ADDRESS.into(),
				caller: owner,
				apparent_value: U256::zero(),
			},
		);

		handle.input = input;

		let res = precompiles().execute(&mut handle).unwrap().unwrap();

		res.output
	};

	H160::from_slice(collection_address.as_slice()[12..].as_ref())
}

/// Fixed precompile address for testing.
const PRECOMPILE_ADDRESS: [u8; 20] = [5u8; 20];

#[test]
fn check_log_selectors() {
	assert_eq!(
		hex::encode(SELECTOR_LOG_NEW_COLLECTION),
		"5b84d9550adb7000df7bee717735ecd3af48ea3f66c6886d52e8227548fb228c"
	);
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
	assert_eq!(Action::CreateCollection as u32, 0x2069E953);
	assert_eq!(Action::Owner as u32, 0x8DA5CB5B);
	assert_eq!(Action::TokenURI as u32, 0xC87B56DD);
	assert_eq!(Action::Mint as u32, 0xFD024566);
	assert_eq!(Action::Evolve as u32, 0x2FD38F4D);
}

#[test]
fn create_collection_returns_address() {
	new_test_ext().execute_with(|| {
		let input = EvmDataWriter::new_with_selector(Action::CreateCollection)
			.write(Address(H160([1u8; 20])))
			.build();

		precompiles()
			.prepare_test(H160([1u8; 20]), H160(PRECOMPILE_ADDRESS), input)
			.execute_returns(1_u64);
	})
}

#[test]
fn create_collection_should_return_collection_id() {
	new_test_ext().execute_with(|| {
		let input = EvmDataWriter::new_with_selector(Action::CreateCollection)
			.write(Address(H160([1u8; 20])))
			.build();

		precompiles()
			.prepare_test(H160([1u8; 20]), H160(PRECOMPILE_ADDRESS), input)
			.execute_returns(H256::from([
				0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0,
				0, 0, 0, 0,
			]));
	})
}

#[test]
fn create_collection_should_generate_log() {
	new_test_ext().execute_with(|| {
		let input = EvmDataWriter::new_with_selector(Action::CreateCollection)
			.write(Address(H160::from_str(ALICE).unwrap()))
			.build();

		let expected_log = Log {
			address: H160::zero(),
			topics: vec![
				SELECTOR_LOG_NEW_COLLECTION.into(),
				H256::from_str(
					"0x000000000000000000000000f24ff3a9cf04c71dbc94d0b566f7a27b94566cac",
				)
				.unwrap(),
			],
			data: vec![
				0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0,
				0, 0, 0, 123,
			],
		};

		let _ = precompiles()
			.prepare_test(H160([1u8; 20]), H160(PRECOMPILE_ADDRESS), input)
			.expect_log(expected_log);
	});
}

#[test]
fn mint_with_external_uri_should_generate_log() {
	new_test_ext().execute_with(|| {
		let alice = H160::from_str(ALICE).unwrap();
		let collection_address = create_collection(alice);

		let input = EvmDataWriter::new_with_selector(Action::Mint)
			.write(Address(alice)) // to
			.write(U256::from(9)) // slot
			.write(Bytes("ciao".into())) // token_uri
			.build();

		let expected_log = Log {
			address: collection_address,
			topics: vec![
				SELECTOR_LOG_MINTED_WITH_EXTERNAL_TOKEN_URI.into(),
				H256::from_str(
					"0x000000000000000000000000f24ff3a9cf04c71dbc94d0b566f7a27b94566cac",
				)
				.unwrap(),
			],
			data: vec![
				0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
				0, 0, 0, 9, // slot
				0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
				0, 1, 2, 3, // token id
				0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
				0, 0, 0, 96, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
				0, 0, 0, 0, 0, 0, 0, 0, 4, // token uri length
				99, 105, 97, 111, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
				0, 0, 0, 0, 0, 0, // token uri
			],
		};

		let _ = precompiles()
			.prepare_test(alice, collection_address, input)
			.expect_log(expected_log);
	});
}

#[test]
fn create_collection_on_mock_with_nonzero_value_fails() {
	new_test_ext().execute_with(|| {
		let input = EvmDataWriter::new_with_selector(Action::CreateCollection)
			.write(Address(H160([1u8; 20])))
			.build();

		precompiles()
			.prepare_test(H160([1u8; 20]), H160(PRECOMPILE_ADDRESS), input)
			.with_value(U256::from(1))
			.execute_reverts(|r| r == b"function is not payable");
	});
}

#[test]
fn create_collection_assign_collection_to_caller() {
	new_test_ext().execute_with(|| {});
}

// #[test]
// fn call_unexistent_selector_should_fail() {
// 	impl_precompile_mock_simple!(Test, Mock, PrecompileMockParams::default());

// 	let nonexistent_selector =
// 		hex::decode("fb24ae530000000000000000000000000000000000000000000000000000000000000000")
// 			.unwrap();
// 	let mut handle = create_mock_handle_from_input(nonexistent_selector);
// 	let result = Mock::execute(&mut handle);
// 	assert_eq!(result.unwrap_err(), revert("unknown selector"));
// }

// #[test]
// fn call_owner_of_non_existent_collection() {
// 	impl_precompile_mock_simple!(Test, Mock, PrecompileMockParams::default());

// 	let collection_address = H160::from_str("0000000000000000000000010000000000000005").unwrap();
// 	let input = EvmDataWriter::new_with_selector(Action::Owner).build();
// 	let mut handle = create_mock_handle(input, 0, 0, H160::zero());
// 	handle.context.address = collection_address;
// 	let result = Mock::execute(&mut handle);
// 	assert_eq!(result.unwrap_err(), revert("collection does not exist"));
// }

// #[test]
// fn call_owner_of_non_invalid_collection() {
// 	impl_precompile_mock_simple!(Test, Mock, PrecompileMockParams { ..Default::default() });

// 	let input = EvmDataWriter::new_with_selector(Action::Owner).write(U256::from(0)).build();
// 	let mut handle = create_mock_handle(input, 0, 0, H160::zero());
// 	handle.context.address = H160::from_str("0000000000000000000000000000000000000005").unwrap();
// 	let result = Mock::execute(&mut handle);
// 	assert_eq!(result.unwrap_err(), revert("invalid collection address"));
// }

// #[test]
// fn call_owner_of_collection_works() {
// 	impl_precompile_mock_simple!(
// 		Test,
// 		Mock,
// 		PrecompileMockParams {
// 			collection_owner_result: Some(H160::from_low_u64_be(0x1234)),
// 			..Default::default()
// 		}
// 	);

// 	let owner = H160::from_low_u64_be(0x1234);
// 	let input = EvmDataWriter::new_with_selector(Action::Owner).build();
// 	let collection_address = H160::from_str("0000000000000000000000010000000000000005").unwrap();

// 	let mut handle = create_mock_handle(input, 0, 0, H160::zero());
// 	handle.context.address = collection_address;
// 	let result = Mock::execute(&mut handle).unwrap();
// 	assert_eq!(result, succeed(EvmDataWriter::new().write(Address(owner.into())).build()));
// }

// #[test]
// fn token_uri_returns_nothing_when_source_token_uri_is_none() {
// 	impl_precompile_mock_simple!(Test, Mock, PrecompileMockParams::default());

// 	let collection_address = H160::from_str("0000000000000000000000010000000000000005").unwrap();
// 	let input = EvmDataWriter::new_with_selector(Action::TokenURI)
// 		.write(TokenId::from(0))
// 		.build();

// 	let mut handle = create_mock_handle(input, 0, 0, H160::zero());
// 	handle.context.address = collection_address;
// 	let result = Mock::execute(&mut handle);
// 	assert_eq!(result.unwrap_err(), revert("asset does not exist"));
// }

// #[test]
// fn token_uri_returns_the_result_from_source() {
// 	impl_precompile_mock_simple!(
// 		Test,
// 		Mock,
// 		PrecompileMockParams { token_uri_result: Some(vec![1_u8, 10]), ..Default::default() }
// 	);

// 	let collection_address = H160::from_str("0000000000000000000000010000000000000005").unwrap();

// 	let input = EvmDataWriter::new_with_selector(Action::TokenURI)
// 		.write(TokenId::from(0))
// 		.build();

// 	let mut handle = create_mock_handle(input, 0, 0, H160::zero());
// 	handle.context.address = collection_address;
// 	let result = Mock::execute(&mut handle);
// 	assert_eq!(result.unwrap(), succeed(EvmDataWriter::new().write(Bytes(vec![1_u8, 10])).build()));
// }

// #[test]
// fn mint_works() {
// 	impl_precompile_mock_simple!(
// 		Test,
// 		Mock,
// 		PrecompileMockParams {
// 			collection_owner_result: Some(H160::from_low_u64_be(0x1234)),
// 			mint_result: Ok(1.into()),
// 			..Default::default()
// 		}
// 	);

// 	let to = H160::from_low_u64_be(1);
// 	let collection_address = H160::from_str("0000000000000000000000010000000000000005").unwrap();

// 	let input = EvmDataWriter::new_with_selector(Action::Mint)
// 		.write(Address(to))
// 		.write(U256::from(1))
// 		.write(Bytes([1u8; 20].to_vec()))
// 		.build();

// 	let mut handle = create_mock_handle(input, 0, 0, H160::zero());
// 	handle.context.address = collection_address;
// 	let result = Mock::execute(&mut handle).unwrap();

// 	assert_eq!(result, succeed(EvmDataWriter::new().write(H256::from_low_u64_be(1)).build()));
// }

// #[test]
// fn failing_mint_should_return_error() {
// 	impl_precompile_mock_simple!(
// 		Test,
// 		Mock,
// 		PrecompileMockParams {
// 			collection_owner_result: Some(H160::from_low_u64_be(0x1234)),
// 			mint_result: Err(DispatchError::Other("this is error")),
// 			..Default::default()
// 		}
// 	);

// 	let to = H160::from_low_u64_be(1);
// 	let collection_address = H160::from_str("0000000000000000000000010000000000000005").unwrap();

// 	let input = EvmDataWriter::new_with_selector(Action::Mint)
// 		.write(Address(to))
// 		.write(U256::from(1))
// 		.write(Bytes([1u8; 20].to_vec()))
// 		.build();

// 	let mut handle = create_mock_handle(input, 0, 0, H160::zero());
// 	handle.context.address = collection_address;
// 	let result = Mock::execute(&mut handle).unwrap_err();

// 	assert_eq!(result, revert("this is error"));
// }

// mod evolve {
// 	use super::*;

// 	#[test]
// 	fn happy_path() {
// 		impl_precompile_mock_simple!(
// 			Test,
// 			Mock,
// 			PrecompileMockParams {
// 				collection_owner_result: Some(H160::from_low_u64_be(0x1234)),
// 				mint_result: Ok(1.into()),
// 				..Default::default()
// 			}
// 		);

// 		let collection_address =
// 			H160::from_str("0000000000000000000000010000000000000005").unwrap();
// 		let input = EvmDataWriter::new_with_selector(Action::Evolve)
// 			.write(U256::from(1))
// 			.write(Bytes([1u8; 20].to_vec()))
// 			.build();

// 		let mut handle = create_mock_handle(input, 0, 0, H160::zero());
// 		handle.context.address = collection_address;
// 		let result = Mock::execute(&mut handle).unwrap();

// 		assert_eq!(result, succeed(EvmDataWriter::new().write(H256::from_low_u64_be(1)).build()));
// 	}

// 	#[test]
// 	fn when_succeeds_should_generate_log() {
// 		impl_precompile_mock_simple!(Test, Mock, PrecompileMockParams::default());

// 		let collection_address =
// 			H160::from_str("0000000000000000000000010000000000000005").unwrap();
// 		let token_id = 1;
// 		let token_uri = Bytes([1u8; 20].to_vec());

// 		let input = EvmDataWriter::new_with_selector(Action::Evolve)
// 			.write(U256::from(token_id))
// 			.write(token_uri.clone())
// 			.build();
// 		let mut handle = create_mock_handle(input, 0, 0, H160::zero());
// 		handle.context.address = collection_address;

// 		let result = Mock::execute(&mut handle);
// 		assert!(result.is_ok());
// 		let logs = handle.logs;
// 		assert_eq!(logs.len(), 1);
// 		assert_eq!(logs[0].address, collection_address);
// 		assert_eq!(logs[0].topics.len(), 2);
// 		assert_eq!(logs[0].topics[0], SELECTOR_LOG_EVOLVED_WITH_EXTERNAL_TOKEN_URI.into());
// 		assert_eq!(
// 			logs[0].data,
// 			vec![
// 				0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
// 				0, 0, 0, 32, // offset
// 				0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
// 				0, 0, 0, 20, // lenght of token_uri
// 				1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0,
// 				0, 0, 0, 0 // token_uri
// 			]
// 		);
// 	}

// 	#[test]
// 	fn when_fails_should_return_error() {
// 		impl_precompile_mock_simple!(
// 			Test,
// 			Mock,
// 			PrecompileMockParams {
// 				collection_owner_result: Some(H160::from_low_u64_be(0x1234)),
// 				mint_result: Ok(1.into()),
// 				evolve_result: Err(DispatchError::Other("this is error")),
// 				..Default::default()
// 			}
// 		);

// 		let collection_address =
// 			H160::from_str("0000000000000000000000010000000000000005").unwrap();
// 		let input = EvmDataWriter::new_with_selector(Action::Evolve)
// 			.write(U256::from(1))
// 			.write(Bytes([1u8; 20].to_vec()))
// 			.build();

// 		let mut handle = create_mock_handle(input, 0, 0, H160::zero());
// 		handle.context.address = collection_address;
// 		let result = Mock::execute(&mut handle).unwrap_err();

// 		assert_eq!(result, revert("this is error"));
// 	}
// }
