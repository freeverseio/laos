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

use crate::precompiles::LaosPrecompiles;

use crate::{configs::laos_evolution::REVERT_BYTECODE, Runtime};
use core::str::FromStr;
use fp_evm::Context;
use frame_support::assert_noop;
use pallet_evm::{AccountCodes, ExitRevert, PrecompileFailure, PrecompileSet};
use pallet_laos_evolution::precompiles::evolution_collection_factory::EvolutionCollectionFactoryPrecompileCall;
use precompile_utils::{
	prelude::Address,
	testing::{Alice, MockHandle, PrecompileTesterExt},
};
use sp_core::H160;

use super::ExtBuilder;

fn hash(a: u64) -> H160 {
	H160::from_low_u64_be(a)
}

#[test]
fn execute_delegate_call_on_custom_precompile_should_fail() {
	ExtBuilder::default().build().execute_with(|| {
		let p = LaosPrecompiles::<Runtime>::new();

		let code_address = hash(1027);
		let context_address = hash(123456);

		// Setup the mock handle for the delegate call
		let mut handle = MockHandle::new(
			code_address,
			Context { address: context_address, caller: H160::zero(), apparent_value: 0.into() },
		);

		// Execute the precompile with the delegate call
		let result = p.execute(&mut handle);

		// Verify that the execution failed due to a delegate call to a custom precompile
		assert!(
			matches!(result, Some(Err(PrecompileFailure::Revert { exit_status: ExitRevert::Reverted, output })) if String::from_utf8_lossy(&output).contains("Cannot be called with DELEGATECALL or CALLCODE"))
		);
	});
}

#[test]
fn call_unknown_address_does_not_revert() {
	ExtBuilder::default().build().execute_with(|| {
		let dummy_contract = H160::from_str("0xe4BdA39B4E2730a578D5E2461A0Cc74FCAa64d62").unwrap();
		let p = LaosPrecompiles::<Runtime>::new();

		// call data for `mint_with_external_uri`
		let mint_with_external_uri_input = "0xfd024566000000000000000000000000f24ff3a9cf04c71dbc94d0b566f7a27b94566cac000000000000000000000000000000000000000000000000000000000000007b0000000000000000000000000000000000000000000000000000000000000060000000000000000000000000000000000000000000000000000000000000000e746573742d746f6b656e2d757269000000000000000000000000000000000000";
		// call data for `evolve_with_external_uri`
		let evolve_with_external_uri = "0x2fd38f4d000000000000000000000000f24ff3a9cf04c71dbc94d0b566f7a27b94566cac00000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000000000000000012616b6a647368666a6b616866646b6c6164660000000000000000000000000000";

		let mut handle = MockHandle::new(
			dummy_contract,
			Context {
				address: dummy_contract,
				caller: H160([1u8;20]),
				apparent_value: sp_core::U256::zero(),
			},
		);

		handle.input = mint_with_external_uri_input.as_bytes().to_vec();

		// now dispatch it again and check it is none
		let result = p.execute(&mut handle);

		assert!(result.is_none());

		assert_ne!(
			result, Some(Err(PrecompileFailure::Revert { exit_status: ExitRevert::Reverted, output: vec![] })
		));

		handle.input = evolve_with_external_uri.as_bytes().to_vec();

		let result = p.execute(&mut handle);

		assert!(result.is_none());

		assert_ne!(
			result, Some(Err(PrecompileFailure::Revert { exit_status: ExitRevert::Reverted, output: vec![] })
		));
	});
}

#[test]
fn call_unknown_address_is_noop() {
	ExtBuilder::default().build().execute_with(|| {
		let dummy_contract = H160::from_str("0x80fc115869ba344BBd6Baf14a8b089b48e870AaD").unwrap();

		let mut handle = MockHandle::new(
			dummy_contract,
			Context {
				address: dummy_contract,
				caller: H160([1u8;20]),
				apparent_value: sp_core::U256::zero(),
			},
		);

		// call data for `mint_with_external_uri`
		let mint_with_external_uri_input = "0xfd024566000000000000000000000000f24ff3a9cf04c71dbc94d0b566f7a27b94566cac000000000000000000000000000000000000000000000000000000000000007b0000000000000000000000000000000000000000000000000000000000000060000000000000000000000000000000000000000000000000000000000000000e746573742d746f6b656e2d757269000000000000000000000000000000000000";
		// call data for `evolve_with_external_uri`
		let evolve_with_external_uri = "0x2fd38f4d000000000000000000000000f24ff3a9cf04c71dbc94d0b566f7a27b94566cac00000000000000000000000000000000000000000000000000000000000000400000000000000000000000000000000000000000000000000000000000000012616b6a647368666a6b616866646b6c6164660000000000000000000000000000";

		handle.input = evolve_with_external_uri.as_bytes().to_vec();

		let p = LaosPrecompiles::<Runtime>::new();

		assert_noop!(
			p.execute(&mut handle).ok_or("returned None"),
			"returned None"
		);

		handle.input = mint_with_external_uri_input.as_bytes().to_vec();

		assert_noop!(
			p.execute(&mut handle).ok_or("returned None"),
			"returned None"
		);
	});
}

#[test]
fn create_collection_inserts_bytecode_to_address() {
	ExtBuilder::default().build().execute_with(|| {
		let expected_collection_address =
			H160::from_str("fffffffffffffffffffffffe0000000000000000").unwrap();

		let precompiles = <Runtime as pallet_evm::Config>::PrecompilesValue::get();
		precompiles
			.prepare_test(
				Alice,
				hash(1027),
				EvolutionCollectionFactoryPrecompileCall::<Runtime>::create_collection {
					owner: Address(Alice.into()),
				},
			)
			.execute_returns(Address(expected_collection_address));

		// Address is not empty
		assert!(!pallet_evm::Pallet::<Runtime>::is_account_empty(&expected_collection_address));

		// Address has correct code
		assert!(AccountCodes::<Runtime>::get(expected_collection_address) == REVERT_BYTECODE);
	});
}
