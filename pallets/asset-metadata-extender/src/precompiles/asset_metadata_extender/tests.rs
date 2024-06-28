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

use super::*;
use fp_evm::{Context, PrecompileSet};
use mock::*;
use precompile_utils::{
	prelude::log3,
	testing::{Alice, MockHandle, Precompile1, PrecompileTesterExt},
};
use sp_core::U256;
use sp_io::hashing::keccak_256;

/// Get precompiles from the mock.
fn precompiles() -> LaosPrecompiles<Test> {
	PrecompilesInstance::get()
}

/// Utility function to extend an universal location.
///
/// Note: this function is used instead of `PrecompileTesterExt::execute_returns` because the latter
/// does not return the output of the precompile. And `PrecompileTester::execute` is a private
/// function.
fn extend(universal_location: UnboundedString, token_uri: UnboundedString) {
	let mut handle = MockHandle::new(
		Precompile1.into(),
		Context { address: Precompile1.into(), caller: Alice.into(), apparent_value: U256::zero() },
	);
	handle.input = PrecompileCall::extend { universal_location, token_uri }.into();

	precompiles().execute(&mut handle).unwrap().unwrap();
}

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
fn selectors() {
	assert!(PrecompileCall::extend_selectors().contains(&0xA5FBDF1D));
	assert!(PrecompileCall::update_selectors().contains(&0xCD79C745));
	assert!(PrecompileCall::balance_of_selectors().contains(&0x7B65DED5));
	assert!(PrecompileCall::claimer_by_index_selectors().contains(&0xA565BB04));
	assert!(PrecompileCall::extension_by_index_selectors().contains(&0xB2B7C05A));
}

#[test]
fn create_token_uri_extension_should_emit_log() {
	new_test_ext().execute_with(|| {
		let universal_location: UnboundedString = "my_awesome_universal_location".into();
		let token_uri: UnboundedString = "ciao".into();

		precompiles()
			.prepare_test(
				Alice,
				Precompile1,
				PrecompileCall::extend {
					universal_location: universal_location.clone(),
					token_uri: token_uri.clone(),
				},
			)
			.expect_log(log3(
				Precompile1,
				SELECTOR_LOG_EXTENDED_UL_WITH_EXTERNAL_URI,
				Alice,
				keccak_256(universal_location.as_bytes()),
				solidity::encode_event_data((universal_location, token_uri)),
			))
			.execute_some();
	});
}

#[test]
fn create_token_uri_extension_reverts_when_ul_exceeds_length() {
	new_test_ext().execute_with(|| {
		let unallowed_size = (MaxUniversalLocationLength::get() + 10).try_into().unwrap();
		let universal_location: UnboundedString = vec![b'a'; unallowed_size].into();
		let token_uri: UnboundedString = "ciao".into();

		precompiles()
			.prepare_test(
				Alice,
				Precompile1,
				PrecompileCall::extend {
					universal_location: universal_location.clone(),
					token_uri: token_uri.clone(),
				},
			)
			.execute_reverts(|r| r == b"invalid universal location length");
	});
}

#[test]
fn create_token_uri_extension_reverts_when_token_uri_exceeds_length() {
	new_test_ext().execute_with(|| {
		let unallowed_size = (MaxTokenUriLength::get() + 10).try_into().unwrap();
		let token_uri: UnboundedString = vec![b'a'; unallowed_size].into();
		let universal_location: UnboundedString = "ciao".into();
		precompiles()
			.prepare_test(
				Alice,
				Precompile1,
				PrecompileCall::extend {
					universal_location: universal_location.clone(),
					token_uri: token_uri.clone(),
				},
			)
			.execute_reverts(|r| r == b"invalid token uri length");
	});
}

#[test]
fn create_token_uri_extension_reverts_when_claimer_already_has_metadata_extension_for_universal_location(
) {
	new_test_ext().execute_with(|| {
		let universal_location: UnboundedString = "my_awesome_universal_location".into();
		let token_uri: UnboundedString = "ciao".into();

		precompiles()
			.prepare_test(
				Alice,
				Precompile1,
				PrecompileCall::extend {
					universal_location: universal_location.clone(),
					token_uri: token_uri.clone(),
				},
			)
			.execute_returns(());

		precompiles()
			.prepare_test(
				Alice,
				Precompile1,
				PrecompileCall::extend {
					universal_location: universal_location.clone(),
					token_uri: token_uri.clone(),
				},
			)
			.execute_reverts(|r| r == b"ExtensionAlreadyExists");
	});
}

#[test]
fn create_token_uri_extension_on_mock_with_nonzero_value_fails() {
	new_test_ext().execute_with(|| {
		let universal_location: UnboundedString = "my_awesome_universal_location".into();
		let token_uri: UnboundedString = "ciao".into();

		precompiles()
			.prepare_test(
				Alice,
				Precompile1,
				PrecompileCall::extend {
					universal_location: universal_location.clone(),
					token_uri: token_uri.clone(),
				},
			)
			.with_value(U256::from(1))
			.execute_reverts(|r| r == b"Function is not payable");
	});
}

#[test]
fn create_token_uri_extension_records_cost() {
	new_test_ext().execute_with(|| {
		let universal_location: UnboundedString = "my_awesome_universal_location".into();
		let token_uri: UnboundedString = "ciao".into();

		// Expected weight of the precompile call implementation.
		// Since benchmarking precompiles is not supported yet, we are benchmarking
		// functions that precompile calls internally.
		//
		// Following `cost` is calculated as:
		// `create_token_uri_extension` weight + log cost
		precompiles()
			.prepare_test(
				Alice,
				Precompile1,
				PrecompileCall::extend {
					universal_location: universal_location.clone(),
					token_uri: token_uri.clone(),
				},
			)
			.expect_cost(390243253) // [`WeightToGas`] set to 1:1 in mock
			.execute_some();
	})
}

#[test]
fn update_inexistent_extension_should_fail() {
	new_test_ext().execute_with(|| {
		let universal_location: UnboundedString = "my_awesome_universal_location".into();
		let token_uri: UnboundedString = "ciao".into();

		precompiles()
			.prepare_test(
				Alice,
				Precompile1,
				PrecompileCall::update {
					universal_location: universal_location.clone(),
					token_uri: token_uri.clone(),
				},
			)
			.execute_reverts(|r| r == b"ExtensionDoesNotExist");
	});
}

#[test]
fn update_of_extension_should_succeed() {
	new_test_ext().execute_with(|| {
		let universal_location: UnboundedString = "my_awesome_universal_location".into();
		let token_uri: UnboundedString = "my_awesome_token_uri".into();
		extend(universal_location.clone(), token_uri.clone());

		let new_token_uri: UnboundedString = "my_awesome_new_token_uri".into();

		precompiles()
			.prepare_test(
				Alice,
				Precompile1,
				PrecompileCall::update {
					universal_location: universal_location.clone(),
					token_uri: new_token_uri.clone(),
				},
			)
			.execute_returns(());
	});
}

#[test]
fn update_token_uri_extension_records_cost() {
	new_test_ext().execute_with(|| {
		let universal_location: UnboundedString = "my_awesome_universal_location".into();
		let token_uri: UnboundedString = "my_awesome_token_uri".into();
		extend(universal_location.clone(), token_uri.clone());

		let new_token_uri: UnboundedString = "my_awesome_new_token_uri".into();

		// Expected weight of the precompile call implementation.
		// Since benchmarking precompiles is not supported yet, we are benchmarking
		// functions that precompile calls internally.
		//
		// Following `cost` is calculated as:
		// `create_token_uri_extension` weight + log cost
		precompiles()
			.prepare_test(
				Alice,
				Precompile1,
				PrecompileCall::update {
					universal_location: universal_location.clone(),
					token_uri: new_token_uri.clone(),
				},
			)
			.expect_cost(163452882) // [`WeightToGas`] set to 1:1 in mock
			.execute_some();
	})
}

#[test]
fn update_of_extension_should_emit_a_log() {
	new_test_ext().execute_with(|| {
		let universal_location: UnboundedString = "my_awesome_universal_location".into();
		let token_uri: UnboundedString = "my_awesome_token_uri".into();
		extend(universal_location.clone(), token_uri.clone());

		let new_token_uri: UnboundedString = "my_awesome_new_token_uri".into();

		precompiles()
			.prepare_test(
				Alice,
				Precompile1,
				PrecompileCall::update {
					universal_location: universal_location.clone(),
					token_uri: new_token_uri.clone(),
				},
			)
			.expect_log(log3(
				Precompile1,
				SELECTOR_LOG_UPDATED_EXTENDED_UL_WITH_EXTERNAL_URI,
				Alice,
				keccak_256(universal_location.as_bytes()),
				solidity::encode_event_data((universal_location, new_token_uri)),
			))
			.execute_returns(());
	});
}

#[test]
fn claimer_by_index_invalid_index_fails() {
	new_test_ext().execute_with(|| {
		let universal_location: UnboundedString = "my_awesome_universal_location".into();

		precompiles()
			.prepare_test(
				Alice,
				Precompile1,
				PrecompileCall::claimer_by_index {
					universal_location: universal_location.clone(),
					index: 2u32,
				},
			)
			.execute_reverts(|r| r == b"invalid index");

		extend(universal_location.clone(), "ciao".into());

		precompiles()
			.prepare_test(
				Alice,
				Precompile1,
				PrecompileCall::claimer_by_index {
					universal_location: universal_location.clone(),
					index: 1u32,
				},
			)
			.execute_reverts(|r| r == b"invalid index");
	});
}

#[test]
fn claimer_by_index_works() {
	new_test_ext().execute_with(|| {
		let universal_location: UnboundedString = "my_awesome_universal_location".into();
		extend(universal_location.clone(), "ciao".into());

		precompiles()
			.prepare_test(
				Alice,
				Precompile1,
				PrecompileCall::claimer_by_index {
					universal_location: universal_location.clone(),
					index: 0u32,
				},
			)
			.execute_returns(Address(Alice.into()));
	});
}

#[test]
fn extension_by_index_works() {
	new_test_ext().execute_with(|| {
		let universal_location: UnboundedString = "my_awesome_universal_location".into();
		let token_uri: UnboundedString = "ciao".into();
		extend(universal_location.clone(), token_uri.clone());

		precompiles()
			.prepare_test(
				Alice,
				Precompile1,
				PrecompileCall::extension_by_index {
					universal_location: universal_location.clone(),
					index: 0u32,
				},
			)
			.execute_returns(token_uri);
	});
}

#[test]
fn extension_by_index_invalid_ul_and_index_fails() {
	new_test_ext().execute_with(|| {
		let universal_location: UnboundedString = "my_awesome_universal_location".into();
		let token_uri: UnboundedString = "ciao".into();

		precompiles()
			.prepare_test(
				Alice,
				Precompile1,
				PrecompileCall::extension_by_index {
					universal_location: universal_location.clone(),
					index: 0u32,
				},
			)
			.execute_reverts(|r| r == b"invalid index");

		// now create an extension
		extend(universal_location.clone(), token_uri.clone());

		// now try to get an extension with an invalid index
		let other_universal_location: UnboundedString = "some_other_ul".into();

		// reverts
		precompiles()
			.prepare_test(
				Alice,
				Precompile1,
				PrecompileCall::extension_by_index {
					universal_location: other_universal_location.clone(),
					index: 0u32,
				},
			)
			.execute_reverts(|r| r == b"invalid index");

		// now try to get an extension with an invalid index
		precompiles()
			.prepare_test(
				Alice,
				Precompile1,
				PrecompileCall::extension_by_index {
					universal_location: universal_location.clone(),
					index: 1u32,
				},
			)
			.execute_reverts(|r| r == b"invalid index");
	});
}

#[test]
fn balance_of_works() {
	new_test_ext().execute_with(|| {
		let universal_location: UnboundedString = "my_awesome_universal_location".into();

		// default balance is 0
		precompiles()
			.prepare_test(
				Alice,
				Precompile1,
				PrecompileCall::balance_of { universal_location: universal_location.clone() },
			)
			.execute_returns(0_u32);

		extend(universal_location.clone(), "ciao".into());

		precompiles()
			.prepare_test(
				Alice,
				Precompile1,
				PrecompileCall::balance_of { universal_location: universal_location.clone() },
			)
			.execute_returns(1_u32);
	});
}

#[test]
fn extension_by_location_and_claimer_works() {
	new_test_ext().execute_with(|| {
		let universal_location: UnboundedString = "my_awesome_universal_location".into();
		let token_uri: UnboundedString = "ciao".into();

		extend(universal_location.clone(), token_uri.clone());

		precompiles()
			.prepare_test(
				Alice,
				Precompile1,
				PrecompileCall::extension_by_location_and_claimer {
					universal_location: universal_location.clone(),
					claimer: Address(Alice.into()),
				},
			)
			.execute_returns(token_uri);
	});
}

#[test]
fn extension_by_location_and_claimer_of_unexistent_claim_reverts() {
	new_test_ext().execute_with(|| {
		let universal_location: UnboundedString = "my_awesome_universal_location".into();

		precompiles()
			.prepare_test(
				Alice,
				Precompile1,
				PrecompileCall::extension_by_location_and_claimer {
					universal_location: universal_location.clone(),
					claimer: Address(Alice.into()),
				},
			)
			.execute_reverts(|r| r == b"invalid ul");
	});
}

#[test]
fn has_extension_by_claim_of_existent_claim_returns_true() {
	new_test_ext().execute_with(|| {
		let universal_location: UnboundedString = "my_awesome_universal_location".into();
		let token_uri: UnboundedString = "ciao".into();

		extend(universal_location.clone(), token_uri.clone());

		precompiles()
			.prepare_test(
				Alice,
				Precompile1,
				PrecompileCall::has_extension_by_claimer {
					universal_location: universal_location.clone(),
					claimer: Address(Alice.into()),
				},
			)
			.execute_returns(true);
	});
}

#[test]
fn has_extension_by_claimer_of_unexistent_claim_returns_false() {
	new_test_ext().execute_with(|| {
		let universal_location: UnboundedString = "my_awesome_universal_location".into();

		precompiles()
			.prepare_test(
				Alice,
				Precompile1,
				PrecompileCall::has_extension_by_claimer {
					universal_location: universal_location.clone(),
					claimer: Address(Alice.into()),
				},
			)
			.execute_returns(false);
	});
}
