//! Living assets precompile tests.

//TODO: remove this and fix clippy issues
#![allow(clippy::redundant_closure_call)]

use core::str::FromStr;

use crate::mock::*;

use super::*;
use fp_evm::Log;
use pallet_evm::AccountCodes;
use precompile_utils::testing::PrecompileTesterExt;
use sp_core::{H160, H256, U256};

const ALICE: &str = "0xf24FF3a9CF04c71Dbc94D0b566f7A27B94566cac";

/// Fixed precompile address for testing.
const PRECOMPILE_ADDRESS: [u8; 20] = [5u8; 20];

/// Get precompiles from the mock.
fn precompiles() -> MockPrecompileSet<Test> {
	MockPrecompiles::get()
}

#[test]
fn check_log_selectors() {
	assert_eq!(
		hex::encode(SELECTOR_LOG_NEW_COLLECTION),
		"5b84d9550adb7000df7bee717735ecd3af48ea3f66c6886d52e8227548fb228c"
	);
}

#[test]
fn function_selectors() {
	assert_eq!(Action::CreateCollection as u32, 0x2069E953);
}

#[test]
fn create_collection_returns_address() {
	new_test_ext().execute_with(|| {
		let input = EvmDataWriter::new_with_selector(Action::CreateCollection)
			.write(Address(H160([1u8; 20])))
			.build();

		let expected_address = "fffffffffffffffffffffffe0000000000000000";
		// output is padded with 12 bytes of zeros
		let expected_output =
			H256::from_str(format!("000000000000000000000000{}", expected_address).as_str())
				.unwrap();

		precompiles()
			.prepare_test(H160([1u8; 20]), H160(PRECOMPILE_ADDRESS), input)
			.execute_returns(expected_output);
	})
}

#[test]
fn create_collection_should_generate_log() {
	new_test_ext().execute_with(|| {
		let input = EvmDataWriter::new_with_selector(Action::CreateCollection)
			.write(Address(H160::from_str(ALICE).unwrap()))
			.build();

		let expected_log = Log {
			address: H160(PRECOMPILE_ADDRESS),
			topics: vec![
				SELECTOR_LOG_NEW_COLLECTION.into(),
				H256::from_str(
					"0x000000000000000000000000f24ff3a9cf04c71dbc94d0b566f7a27b94566cac",
				)
				.unwrap(),
			],
			data: vec![
				0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255,
				255, 255, 254, 0, 0, 0, 0, 0, 0, 0, 0,
			],
		};

		precompiles()
			.prepare_test(H160([1u8; 20]), H160(PRECOMPILE_ADDRESS), input)
			.expect_log(expected_log)
			.execute_some();
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
	new_test_ext().execute_with(|| {
		let input = EvmDataWriter::new_with_selector(Action::CreateCollection)
			.write(Address(H160([1u8; 20])))
			.build();

		let expected_address = "fffffffffffffffffffffffe0000000000000000";
		// output is padded with 12 bytes of zeros
		let expected_output =
			H256::from_str(format!("000000000000000000000000{}", expected_address).as_str())
				.unwrap();

		precompiles()
			.prepare_test(H160([1u8; 20]), H160(PRECOMPILE_ADDRESS), input)
			.execute_returns(expected_output);

		assert_eq!(LaosEvolution::<Test>::collection_owner(0), Some(H160([1u8; 20].into())));
	});
}

#[test]
fn create_collection_inserts_bytecode_to_address() {
	new_test_ext().execute_with(|| {
		let input = EvmDataWriter::new_with_selector(Action::CreateCollection)
			.write(Address(H160([1u8; 20])))
			.build();

		let expected_address = "fffffffffffffffffffffffe0000000000000000";
		// output is padded with 12 bytes of zeros
		let expected_output =
			H256::from_str(format!("000000000000000000000000{}", expected_address).as_str())
				.unwrap();

		precompiles()
			.prepare_test(H160([1u8; 20]), H160(PRECOMPILE_ADDRESS), input)
			.execute_returns(expected_output);

		let collection_address = &H160::from_str(expected_address).unwrap();
		// Address is not empty
		assert!(!Evm::<Test>::is_account_empty(&collection_address));

		// Address has correct code
		assert!(AccountCodes::<Test>::get(&collection_address) == REVERT_BYTECODE);
	});
}

#[test]
fn test_expected_cost_create_collection() {
	new_test_ext().execute_with(|| {
		let input = EvmDataWriter::new_with_selector(Action::CreateCollection)
			.write(Address(H160([1u8; 20])))
			.build();

		// Expected weight of the precompile call implementation.
		// Since benchmarking precompiles is not supported yet, we are benchmarking
		// functions that precompile calls internally.
		//
		// The weight of this precompile call is calculated as:
		// `create_collection` weight + insert account bytecode for the collection + log costs
		precompiles()
			.prepare_test(H160([1u8; 20]), H160(PRECOMPILE_ADDRESS), input)
			.expect_cost(486001381) //[`WeightToGas`] set to 1:1 in mock // weight to gas -> 1 to 1 in mock
			.execute_some();
	})
}
