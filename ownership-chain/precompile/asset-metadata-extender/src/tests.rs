use super::*;
use crate::mock::*;
use core::str::FromStr;
use fp_evm::Log;
use laos_precompile_utils::EvmDataWriter;
use precompile_utils::testing::PrecompileTesterExt;
use sp_core::{H160, H256, U256};

/// Fixed precompile address for testing.
const PRECOMPILE_ADDRESS: [u8; 20] = [5u8; 20];

/// Get precompiles from the mock.
fn precompiles() -> MockPrecompileSet<Test> {
	MockPrecompiles::get()
}

const TEST_CLAIMER: &str = "0xf24FF3a9CF04c71Dbc94D0b566f7A27B94566cac";

#[test]
fn check_log_selectors() {
	assert_eq!(
		hex::encode(SELECTOR_LOG_TOKEN_URI_EXTENDED),
		"f46176c5100037c043791a91ccf3a737b3c8b2d240c695679518d3b5efa72680"
	);
	assert_eq!(
		hex::encode(SELECTOR_LOG_EXTENDED_TOKEN_URI_UPDATED),
		"09caa56717fb6472811ebb17ea40328232e4481d80b2079a6e7578dba4d36e3d"
	);
}

#[test]
fn function_selectors() {
	assert_eq!(Action::Extend as u32, 0xB5A72BFF);
	assert_eq!(Action::Balance as u32, 0x7B65DED5);
	assert_eq!(Action::Claimer as u32, 0xC050D9BE);
	assert_eq!(Action::Extension as u32, 0x4322DE03);
	assert_eq!(Action::Update as u32, 0xC7108F8);
}

#[test]
#[ignore]
fn call_unexistent_selector_should_fail() {
	todo!();
}

#[test]
fn create_token_uri_extension_should_emit_log() {
	new_test_ext().execute_with(|| {
		let universal_location = Bytes("my_awesome_universal_location".as_bytes().to_vec());
		let token_uri = Bytes("ciao".as_bytes().to_vec());

		let input = EvmDataWriter::new_with_selector(Action::Extend)
			.write(universal_location.clone())
			.write(token_uri.clone())
			.build();

		let expected_log = Log {
			address: H160(PRECOMPILE_ADDRESS),
			topics: vec![
				SELECTOR_LOG_TOKEN_URI_EXTENDED.into(),
				H256::from_str(
					format!("000000000000000000000000{}", TEST_CLAIMER.trim_start_matches("0x"))
						.as_str(),
				)
				.unwrap(),
				keccak_256(&universal_location.0).into(),
			],
			// ul is 29 bytes, so it's prepended with 64 bytes of zeros + ul + 3 bytes to make it 32
			data: hex::decode("00000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000000000000000080000000000000000000000000000000000000000000000000000000000000001D6D795F617765736F6D655F756E6976657273616C5F6C6F636174696F6E00000000000000000000000000000000000000000000000000000000000000000000046369616F00000000000000000000000000000000000000000000000000000000")
				.unwrap(),
		};

		precompiles()
			.prepare_test(H160::from_str(TEST_CLAIMER).unwrap(), H160(PRECOMPILE_ADDRESS), input)
			.expect_log(expected_log)
			.execute_some();
	});
}

#[test]
fn create_token_uri_extension_reverts_when_ul_exceeds_length() {
	new_test_ext().execute_with(|| {
		let unallowed_size = (MaxUniversalLocationLength::get() + 10).try_into().unwrap();
		let universal_location = Bytes(vec![b'a'; unallowed_size]);
		let token_uri = Bytes(vec![b'b'; MaxTokenUriLength::get().try_into().unwrap()]);

		let input = EvmDataWriter::new_with_selector(Action::Extend)
			.write(universal_location)
			.write(token_uri)
			.build();

		precompiles()
			.prepare_test(H160::from_str(TEST_CLAIMER).unwrap(), H160(PRECOMPILE_ADDRESS), input)
			.execute_reverts(|r| r == b"invalid universal location length");
	});
}

#[test]
fn create_token_uri_extension_reverts_when_token_uri_exceeds_length() {
	new_test_ext().execute_with(|| {
		let unallowed_size = (MaxTokenUriLength::get() + 1).try_into().unwrap();
		let token_uri = Bytes(vec![b'a'; unallowed_size]);
		let universal_location =
			Bytes(vec![b'b'; MaxUniversalLocationLength::get().try_into().unwrap()]);

		let input = EvmDataWriter::new_with_selector(Action::Extend)
			.write(universal_location)
			.write(token_uri)
			.build();

		precompiles()
			.prepare_test(H160::from_str(TEST_CLAIMER).unwrap(), H160(PRECOMPILE_ADDRESS), input)
			.execute_reverts(|r| r == b"invalid token uri length");
	});
}

#[test]
fn create_token_uri_extension_reverts_when_claimer_already_has_metadata_extension_for_universal_location(
) {
	new_test_ext().execute_with(|| {
		let universal_location = Bytes("my_awesome_universal_location".as_bytes().to_vec());
		let token_uri = Bytes("my_awesome_token_uri".as_bytes().to_vec());

		let input = EvmDataWriter::new_with_selector(Action::Extend)
			.write(universal_location.clone())
			.write(token_uri.clone())
			.build();

		precompiles()
			.prepare_test(H160::from_str(TEST_CLAIMER).unwrap(), H160(PRECOMPILE_ADDRESS), input)
			.execute_returns_raw(vec![]);

		let input = EvmDataWriter::new_with_selector(Action::Extend)
			.write(universal_location)
			.write(token_uri)
			.build();

		precompiles()
			.prepare_test(H160::from_str(TEST_CLAIMER).unwrap(), H160(PRECOMPILE_ADDRESS), input)
			.execute_reverts(|r| r == b"ExtensionAlreadyExists");
	});
}

#[test]
fn create_token_uri_extension_on_mock_with_nonzero_value_fails() {
	new_test_ext().execute_with(|| {
		let universal_location = Bytes("my_awesome_universal_location".as_bytes().to_vec());
		let token_uri = Bytes("my_awesome_token_uri".as_bytes().to_vec());
		let input = EvmDataWriter::new_with_selector(Action::Extend)
			.write(universal_location)
			.write(token_uri)
			.build();

		precompiles()
			.prepare_test(H160::from_str(TEST_CLAIMER).unwrap(), H160(PRECOMPILE_ADDRESS), input)
			.with_value(U256::from(1))
			.execute_reverts(|r| r == b"function is not payable");
	});
}

#[test]
fn create_token_uri_extension_records_cost() {
	new_test_ext().execute_with(|| {
		let universal_location = Bytes("my_awesome_universal_location".as_bytes().to_vec());
		let token_uri = Bytes("my_awesome_token_uri".as_bytes().to_vec());
		let input = EvmDataWriter::new_with_selector(Action::Extend)
			.write(universal_location)
			.write(token_uri)
			.build();

		// Expected weight of the precompile call implementation.
		// Since benchmarking precompiles is not supported yet, we are benchmarking
		// functions that precompile calls internally.
		//
		// Following `cost` is calculated as:
		// `create_token_uri_extension` weight + log cost
		precompiles()
			.prepare_test(H160::from_str(TEST_CLAIMER).unwrap(), H160(PRECOMPILE_ADDRESS), input)
			.expect_cost(364339662) // [`WeightToGas`] set to 1:1 in mock
			.execute_some();
	})
}

#[test]
#[ignore]
fn claimer_of_ul_by_index_given_unexistent_index_returns_empty_address() {
	todo!();
}

#[test]
#[ignore]
fn claimer_of_ul_by_index_returns_claimer() {
	todo!();
}

#[test]
#[ignore]
fn claimer_of_ul_by_index_it_is_expected_to_have_a_cost() {
	todo!();
}

#[test]
#[ignore]
fn extension_of_ul_by_index_it_is_expected_to_have_a_cost() {
	todo!();
}

#[test]
#[ignore]
fn extension_of_ul_by_index_returns_extension() {
	todo!();
}

#[test]
#[ignore]
fn extension_of_ul_by_index_given_unexistent_ul_returns_empty_string() {
	todo!();
}

#[test]
#[ignore]
fn extension_of_ul_by_index_given_unexistent_index_returns_empty_string() {
	todo!();
}

#[test]
#[ignore]

fn reverts_when_ul_does_not_exist() {
	todo!();
}

#[test]
#[ignore]

fn returns_number_of_extensions() {
	todo!();
}

#[test]
#[ignore]

fn it_is_expected_to_have_a_cost() {
	todo!();
}
