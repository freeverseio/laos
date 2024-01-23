use super::*;
use crate::mock::*;
use core::str::FromStr;
use fp_evm::Log;
use laos_precompile_utils::EvmDataWriter;
use precompile_utils::{solidity::codec::BoundedBytes, testing::PrecompileTesterExt};
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
		hex::encode(SELECTOR_LOG_EXTENDED_UL_WITH_EXTERNAL_URI),
		"f744da499cb735a8fc987aa2a331a1cbeca79e449e4c04eeccfe57c538e79070"
	);
	assert_eq!(
		hex::encode(SELECTOR_LOG_UPDATED_EXTENDED_UL_WITH_EXTERNAL_URI),
		"e7ebe38355126fe0c3eab0ec03eb1b94ff501458a80713c9eb8b737334a651ff"
	);
}

#[test]
fn function_selectors() {
	assert_eq!(Action::Extend as u32, 0xA5FBDF1D);
	assert_eq!(Action::Balance as u32, 0x7B65DED5);
	assert_eq!(Action::Claimer as u32, 0xA565BB04);
	assert_eq!(Action::Extension as u32, 0xB2B7C05A);
	assert_eq!(Action::Update as u32, 0xCD79C745);
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
				SELECTOR_LOG_EXTENDED_UL_WITH_EXTERNAL_URI.into(),
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
fn update_inexistent_extension_should_fail() {
	new_test_ext().execute_with(|| {
		let universal_location = Bytes("my_awesome_universal_location".as_bytes().to_vec());
		let token_uri = Bytes("my_awesome_token_uri".as_bytes().to_vec());

		let input = EvmDataWriter::new_with_selector(Action::Update)
			.write(universal_location)
			.write(token_uri)
			.build();

		precompiles()
			.prepare_test(H160::from_str(TEST_CLAIMER).unwrap(), H160(PRECOMPILE_ADDRESS), input)
			.execute_reverts(|r| r == b"ExtensionDoesNotExist");
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
			.expect_cost(364560357) // [`WeightToGas`] set to 1:1 in mock
			.execute_some();
	})
}

#[test]
fn update_token_uri_extension_records_cost() {
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

		let new_token_uri = Bytes("my_awesome_new_token_uri".as_bytes().to_vec());
		let input = EvmDataWriter::new_with_selector(Action::Update)
			.write(universal_location)
			.write(new_token_uri)
			.build();

		// Expected weight of the precompile call implementation.
		// Since benchmarking precompiles is not supported yet, we are benchmarking
		// functions that precompile calls internally.
		//
		// Following `cost` is calculated as:
		// `create_token_uri_extension` weight + log cost
		precompiles()
			.prepare_test(H160::from_str(TEST_CLAIMER).unwrap(), H160(PRECOMPILE_ADDRESS), input)
			.expect_cost(136659074) // [`WeightToGas`] set to 1:1 in mock
			.execute_some();
	})
}

#[test]
fn update_of_extension_should_succeed() {
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

		let new_token_uri = Bytes("my_awesome_new_token_uri".as_bytes().to_vec());

		let input = EvmDataWriter::new_with_selector(Action::Update)
			.write(universal_location)
			.write(new_token_uri)
			.build();

		precompiles()
			.prepare_test(H160::from_str(TEST_CLAIMER).unwrap(), H160(PRECOMPILE_ADDRESS), input)
			.execute_returns_raw(vec![]);
	});
}

#[test]
fn update_of_extension_should_emit_a_log() {
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

		let new_token_uri = Bytes("my_awesome_new_token_uri".as_bytes().to_vec());

		let input = EvmDataWriter::new_with_selector(Action::Update)
			.write(universal_location)
			.write(new_token_uri)
			.build();

		let expected_log = Log {
			address: H160(PRECOMPILE_ADDRESS),
			topics: vec![
				SELECTOR_LOG_UPDATED_EXTENDED_UL_WITH_EXTERNAL_URI.into(),
				H160::from_str(TEST_CLAIMER).unwrap().into(),
				keccak256!("my_awesome_universal_location").into(),
			],
			data: hex::decode("00000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000000000000000080000000000000000000000000000000000000000000000000000000000000001D6D795F617765736F6D655F756E6976657273616C5F6C6F636174696F6E00000000000000000000000000000000000000000000000000000000000000000000186D795F617765736F6D655F6E65775F746F6B656E5F7572690000000000000000")
				.unwrap(),
		};

		precompiles()
			.prepare_test(H160::from_str(TEST_CLAIMER).unwrap(), H160(PRECOMPILE_ADDRESS), input)
			.expect_log(expected_log)
			.execute_returns_raw(vec![]);
	});
}

#[test]
fn claimer_by_index_invalid_index_fails() {
	new_test_ext().execute_with(|| {
		let universal_location = Bytes("my_awesome_universal_location".as_bytes().to_vec());
		let input = EvmDataWriter::new_with_selector(Action::Claimer)
			.write(universal_location.clone())
			.write(2u32)
			.build();

		precompiles()
			.prepare_test(
				H160::from_str(TEST_CLAIMER).unwrap(),
				H160(PRECOMPILE_ADDRESS),
				input.clone(),
			)
			.execute_reverts(|r| r == b"invalid index");

		let input = EvmDataWriter::new_with_selector(Action::Extend)
			.write(universal_location.clone())
			.write(Bytes(vec![1u8; 10]))
			.build();

		precompiles()
			.prepare_test(
				H160::from_str(TEST_CLAIMER).unwrap(),
				H160(PRECOMPILE_ADDRESS),
				input.clone(),
			)
			.execute_returns_raw(sp_std::vec![]);

		let input = EvmDataWriter::new_with_selector(Action::Claimer)
			.write(universal_location)
			.write(1u32)
			.build();

		precompiles()
			.prepare_test(H160::from_str(TEST_CLAIMER).unwrap(), H160(PRECOMPILE_ADDRESS), input)
			.execute_reverts(|r| r == b"invalid index");
	});
}

#[test]
fn claimer_by_index_works() {
	new_test_ext().execute_with(|| {
		let universal_location = Bytes("some_universal_location".as_bytes().to_vec());

		let input = EvmDataWriter::new_with_selector(Action::Extend)
			.write(universal_location.clone())
			.write(Bytes("some_token_uri".as_bytes().to_vec()))
			.build();

		precompiles()
			.prepare_test(H160::from_str(TEST_CLAIMER).unwrap(), H160(PRECOMPILE_ADDRESS), input)
			.execute_returns_raw(vec![]);

		let input = EvmDataWriter::new_with_selector(Action::Claimer)
			.write(universal_location.clone())
			.write(0u32)
			.build();

		precompiles()
			.prepare_test(
				H160::from_str(TEST_CLAIMER).unwrap(),
				H160(PRECOMPILE_ADDRESS),
				input.clone(),
			)
			.execute_returns(precompile_utils::prelude::Address(
				H160::from_str(TEST_CLAIMER).unwrap(),
			));
	});
}

#[test]
fn extension_by_index_works() {
	new_test_ext().execute_with(|| {
		let universal_location = Bytes("some_universal_location".as_bytes().to_vec());

		let input = EvmDataWriter::new_with_selector(Action::Extend)
			.write(universal_location.clone())
			.write(Bytes(vec![1u8; 10]))
			.build();

		precompiles()
			.prepare_test(
				H160::from_str(TEST_CLAIMER).unwrap(),
				H160(PRECOMPILE_ADDRESS),
				input.clone(),
			)
			.execute_returns_raw(sp_std::vec![]);

		let input = EvmDataWriter::new_with_selector(Action::Extension)
			.write(universal_location.clone())
			.write(0_u32)
			.build();

		precompiles()
			.prepare_test(H160::from_str(TEST_CLAIMER).unwrap(), H160(PRECOMPILE_ADDRESS), input)
			.execute_returns(BoundedBytes::<MaxTokenUriLength>::from(vec![1u8; 10]));
	});
}

#[test]
fn extension_by_index_invalid_ul_and_index_fails() {
	new_test_ext().execute_with(|| {
		let universal_location = Bytes("invalid_universal_location".as_bytes().to_vec());

		let input = EvmDataWriter::new_with_selector(Action::Extension)
			.write(universal_location.clone())
			.write(0_u32)
			.build();

		precompiles()
			.prepare_test(
				H160::from_str(TEST_CLAIMER).unwrap(),
				H160(PRECOMPILE_ADDRESS),
				input.clone(),
			)
			.execute_reverts(|r| r == b"invalid index");

		// now create an extension
		let input = EvmDataWriter::new_with_selector(Action::Extend)
			.write(universal_location.clone())
			.write(Bytes(vec![1u8; 10]))
			.build();

		precompiles()
			.prepare_test(
				H160::from_str(TEST_CLAIMER).unwrap(),
				H160(PRECOMPILE_ADDRESS),
				input.clone(),
			)
			.execute_returns_raw(sp_std::vec![]);

		// now try to get an extension with an invalid index
		let input = EvmDataWriter::new_with_selector(Action::Extension)
			.write(Bytes("some_other_ul".as_bytes().to_vec()).clone())
			.write(0_u32)
			.build();

		// reverts
		precompiles()
			.prepare_test(
				H160::from_str(TEST_CLAIMER).unwrap(),
				H160(PRECOMPILE_ADDRESS),
				input.clone(),
			)
			.execute_reverts(|r| r == b"invalid index");

		// now try to get an extension with an invalid index
		let input = EvmDataWriter::new_with_selector(Action::Extension)
			.write(universal_location)
			.write(1_u32)
			.build();

		// reverts
		precompiles()
			.prepare_test(H160::from_str(TEST_CLAIMER).unwrap(), H160(PRECOMPILE_ADDRESS), input)
			.execute_reverts(|r| r == b"invalid index");
	});
}

#[test]
fn balance_of_works() {
	new_test_ext().execute_with(|| {
		let universal_location = Bytes("some_universal_location".as_bytes().to_vec());

		let input = EvmDataWriter::new_with_selector(Action::Balance)
			.write(universal_location.clone())
			.build();

		// default balance is 0
		precompiles()
			.prepare_test(
				H160::from_str(TEST_CLAIMER).unwrap(),
				H160(PRECOMPILE_ADDRESS),
				input.clone(),
			)
			.execute_returns(0_u32);

		let input = EvmDataWriter::new_with_selector(Action::Extend)
			.write(universal_location.clone())
			.write(Bytes(vec![1u8; 10]))
			.build();

		precompiles()
			.prepare_test(
				H160::from_str(TEST_CLAIMER).unwrap(),
				H160(PRECOMPILE_ADDRESS),
				input.clone(),
			)
			.execute_returns_raw(sp_std::vec![]);

		let input = EvmDataWriter::new_with_selector(Action::Balance)
			.write(universal_location.clone())
			.build();

		precompiles()
			.prepare_test(H160::from_str(TEST_CLAIMER).unwrap(), H160(PRECOMPILE_ADDRESS), input)
			.execute_returns(1_u32);
	});
}

#[test]
fn extension_by_location_and_claimer_works() {
	new_test_ext().execute_with(|| {
		let universal_location = Bytes("some_universal_location".as_bytes().to_vec());
		let claimer = H160::from_str(TEST_CLAIMER).unwrap();
		let claim = Bytes(vec![1u8; 10]);

		let input = EvmDataWriter::new_with_selector(Action::Extend)
			.write(universal_location.clone())
			.write(claim.clone())
			.build();

		precompiles()
			.prepare_test(claimer.clone(), H160(PRECOMPILE_ADDRESS), input.clone())
			.execute_returns_raw(sp_std::vec![]);

		let input = EvmDataWriter::new_with_selector(Action::ExtensionOfULByClaimer)
			.write(universal_location.clone())
			.write(Address::from(claimer.clone()))
			.build();

		precompiles()
			.prepare_test(claimer, H160(PRECOMPILE_ADDRESS), input)
			.execute_returns(BoundedBytes::<MaxTokenUriLength>::from(vec![1u8; 10]));
	});
}

#[test]
fn extension_by_location_and_claimer_of_unexistent_claim_reverts() {
	new_test_ext().execute_with(|| {
		let universal_location = Bytes("some_universal_location".as_bytes().to_vec());
		let claimer = H160::from_str(TEST_CLAIMER).unwrap();

		let input = EvmDataWriter::new_with_selector(Action::ExtensionOfULByClaimer)
			.write(universal_location.clone())
			.write(Address::from(claimer.clone()))
			.build();

		precompiles()
			.prepare_test(claimer, H160(PRECOMPILE_ADDRESS), input)
			.execute_reverts(|r| r == b"invalid ul");
	});
}

#[test]
fn has_extension_by_claim_of_existent_claim_returns_true() {
	new_test_ext().execute_with(|| {
		let universal_location = Bytes("some_universal_location".as_bytes().to_vec());
		let claimer = H160::from_str(TEST_CLAIMER).unwrap();
		let claim = Bytes(vec![1u8; 10]);

		let input = EvmDataWriter::new_with_selector(Action::Extend)
			.write(universal_location.clone())
			.write(claim.clone())
			.build();

		precompiles()
			.prepare_test(claimer.clone(), H160(PRECOMPILE_ADDRESS), input.clone())
			.execute_returns_raw(sp_std::vec![]);

		let input = EvmDataWriter::new_with_selector(Action::HasExtension)
			.write(universal_location.clone())
			.write(Address::from(claimer.clone()))
			.build();

		precompiles()
			.prepare_test(claimer, H160(PRECOMPILE_ADDRESS), input)
			.execute_returns(true);
	});
}

#[test]
fn has_extension_by_claimer_of_unexistent_claim_returns_false() {
	new_test_ext().execute_with(|| {
		let universal_location = Bytes("some_universal_location".as_bytes().to_vec());
		let claimer = H160::from_str(TEST_CLAIMER).unwrap();

		let input = EvmDataWriter::new_with_selector(Action::HasExtension)
			.write(universal_location.clone())
			.write(Address::from(claimer.clone()))
			.build();

		precompiles()
			.prepare_test(claimer, H160(PRECOMPILE_ADDRESS), input)
			.execute_returns(false);
	});
}
