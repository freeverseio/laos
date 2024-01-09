use super::*;
use crate::mock::*;
use laos_precompile_utils::EvmDataWriter;
use precompile_utils::testing::PrecompileTesterExt;
use sp_core::H160;
/// Fixed precompile address for testing.
const PRECOMPILE_ADDRESS: [u8; 20] = [5u8; 20];

/// Get precompiles from the mock.
fn precompiles() -> MockPrecompileSet<Test> {
	MockPrecompiles::get()
}

const TEST_CLAIMER: H160 = H160([1u8; 20]);

#[test]
fn check_log_selectors() {
	assert_eq!(
		hex::encode(SELECTOR_LOG_TOKEN_URI_EXTENDED),
		"f46176c5100037c043791a91ccf3a737b3c8b2d240c695679518d3b5efa72680"
	);
}

#[test]
fn function_selectors() {
	assert_eq!(Action::Extend as u32, 0xB5A72BFF);
	assert_eq!(Action::Balance as u32, 0x7B65DED5);
	assert_eq!(Action::Claimer as u32, 0xC050D9BE);
	assert_eq!(Action::Extension as u32, 0x4322DE03);
}

#[test]
#[ignore]
fn call_unexistent_selector_should_fail() {
	todo!();
}

fn create_metadata_extension_succeeds(claimer: H160, universal_location: Bytes, token_uri: Bytes) {
	let input = EvmDataWriter::new_with_selector(Action::Extend)
		.write(universal_location)
		.write(token_uri)
		.build();

	precompiles()
		.prepare_test(claimer, H160(PRECOMPILE_ADDRESS), input)
		.execute_returns_raw(vec![])
}

fn create_metadata_extension_reverts(
	claimer: H160,
	universal_location: Bytes,
	token_uri: Bytes,
	revert_reason: &[u8],
) {
	let input = EvmDataWriter::new_with_selector(Action::Extend)
		.write(universal_location)
		.write(token_uri)
		.build();

	precompiles()
		.prepare_test(claimer, H160(PRECOMPILE_ADDRESS), input)
		.execute_reverts(|r| r == revert_reason);
}

mod create_metadata_extension {
	use super::*;

	#[test]
	fn should_generates_log() {
		new_test_ext().execute_with(|| {
			let universal_location = Bytes("my_awesome_universal_location".as_bytes().to_vec());
			let token_uri = Bytes("my_awesome_token_uri".as_bytes().to_vec());
			create_metadata_extension_succeeds(TEST_CLAIMER, universal_location, token_uri);
			// TODO
		});
	}

	#[test]
	fn reverts_when_ul_exceeds_length() {
		new_test_ext().execute_with(|| {
			let unallowed_size = (MaxUniversalLocationLength::get() + 10).try_into().unwrap();
			let universal_location = Bytes(vec![b'a'; unallowed_size]);
			let token_uri = Bytes(vec![b'b'; MaxTokenUriLength::get().try_into().unwrap()]);
			create_metadata_extension_reverts(
				TEST_CLAIMER,
				universal_location,
				token_uri,
				b"invalid universal location length",
			);
		});
	}

	#[test]
	fn reverts_when_token_uri_exceeds_length() {
		new_test_ext().execute_with(|| {
			let unallowed_size = (MaxTokenUriLength::get() + 1).try_into().unwrap();
			let token_uri = Bytes(vec![b'a'; unallowed_size]);
			let universal_location =
				Bytes(vec![b'b'; MaxUniversalLocationLength::get().try_into().unwrap()]);
			create_metadata_extension_reverts(
				TEST_CLAIMER,
				universal_location,
				token_uri,
				b"invalid token uri length",
			);
		});
	}

	#[test]
	#[ignore]
	fn reverts_when_claimer_already_has_metadata_extension_for_universal_location() {
		let universal_location = Bytes("my_awesome_universal_location".as_bytes().to_vec());
		let token_uri = Bytes("my_awesome_token_uri".as_bytes().to_vec());
		create_metadata_extension_succeeds(TEST_CLAIMER, universal_location, token_uri);
		todo!();
	}

	#[test]
	#[ignore]
	fn on_mock_with_nonzero_value_fails() {
		// duda onmock?
		todo!();
	}

	#[test]
	#[ignore]
	fn it_is_expected_to_have_a_cost() {
		todo!();
	}
}

mod balance_of_ul {
	#[test]
	#[ignore]
	fn given_unexistent_ul_returns_zero() {
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
}

mod claimer_of_ul_by_index {
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
}

mod extension_of_ul_by_index {
	#[test]
	#[ignore]
	fn it_is_expected_to_have_a_cost() {
		todo!();
	}

	#[test]
	#[ignore]
	fn returns_extension() {
		todo!();
	}

	#[test]
	#[ignore]
	fn given_unexistent_ul_returns_empty_string() {
		todo!();
	}

	#[test]
	#[ignore]
	fn given_unexistent_index_returns_empty_string() {
		todo!();
	}
}
