//! Living assets precompile tests.

//TODO: remove this and fix clippy issues
#![allow(clippy::redundant_closure_call)]

use core::str::FromStr;

use crate::tests::helpers::PrecompileMockParams;

use super::*;
use frame_support::assert_ok;
use laos_precompile_utils::{
	revert, succeed,
	testing::{create_mock_handle, create_mock_handle_from_input},
};
use sp_core::{H160, H256};

type AccountId = H160;
type AddressMapping = pallet_evm::IdentityAddressMapping;

const ALICE: &str = "0xf24FF3a9CF04c71Dbc94D0b566f7A27B94566cac";

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
fn failing_create_collection_should_return_error() {
	impl_precompile_mock_simple!(
		Mock,
		PrecompileMockParams {
			create_collection_result: Err(DispatchError::Other("this is an error")),
			..Default::default()
		}
	);

	let input = EvmDataWriter::new_with_selector(Action::CreateCollection)
		.write(Address(H160([1u8; 20])))
		.build();

	let mut handle = create_mock_handle_from_input(input);

	let result = Mock::execute(&mut handle);
	assert_eq!(
		result.unwrap_err(),
		revert_dispatch_error(DispatchError::Other("this is an error"))
	);
}

#[test]
fn create_collection_should_return_collection_id() {
	impl_precompile_mock_simple!(Mock, PrecompileMockParams::default());

	let input = EvmDataWriter::new_with_selector(Action::CreateCollection)
		.write(Address(H160([1u8; 20])))
		.build();
	let mut handle = create_mock_handle_from_input(input);

	let result = Mock::execute(&mut handle);
	assert_ok!(
		result,
		succeed(H256::from_slice(&[
			0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
			255, 254, 0, 0, 0, 0, 0, 0, 0, 0
		]))
	);
}

#[test]
fn create_collection_should_generate_log() {
	impl_precompile_mock_simple!(
		Mock,
		PrecompileMockParams { create_collection_result: Ok(123), ..Default::default() }
	);

	let input = EvmDataWriter::new_with_selector(Action::CreateCollection)
		.write(Address(H160::from_str(ALICE).unwrap()))
		.build();
	let mut handle = create_mock_handle_from_input(input);

	let result = Mock::execute(&mut handle);
	assert!(result.is_ok());
	let logs = handle.logs;
	assert_eq!(logs.len(), 1);
	assert_eq!(logs[0].address, H160::zero());
	assert_eq!(logs[0].topics.len(), 2);
	assert_eq!(logs[0].topics[0], SELECTOR_LOG_NEW_COLLECTION.into());
	assert_eq!(
		logs[0].topics[1],
		H256::from_str("0x000000000000000000000000f24ff3a9cf04c71dbc94d0b566f7a27b94566cac")
			.unwrap()
	);
	assert_eq!(
		logs[0].data,
		vec![
			0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
			255, 254, 0, 0, 0, 0, 0, 0, 0, 123
		]
	);
}

#[test]
fn create_collection_on_mock_with_nonzero_value_fails() {
	impl_precompile_mock_simple!(
		Mock,
		PrecompileMockParams { create_collection_result: Ok(5), ..Default::default() }
	);

	let input = EvmDataWriter::new_with_selector(Action::CreateCollection)
		.write(Address(H160([1u8; 20])))
		.build();
	let mut handle = create_mock_handle(input, 0, 1, H160::zero());

	let result = Mock::execute(&mut handle);
	assert!(result.is_err());
	assert_eq!(result.unwrap_err(), revert("function is not payable"));
}

#[test]
fn create_collection_assign_collection_to_caller() {
	impl_precompile_mock!(
		Mock, // name of the defined precompile
		|owner| {
			assert_eq!(owner, H160::from_low_u64_be(0x1234));
			Ok(0)
		}  // Closure for create_collection result
	);

	let input = EvmDataWriter::new_with_selector(Action::CreateCollection)
		.write(Address(H160::from_low_u64_be(0x1234)))
		.build();

	let mut handle = create_mock_handle(input, 0, 0, H160::from_low_u64_be(0x1234));
	let result = Mock::execute(&mut handle);
	assert!(result.is_ok());
}

#[test]
fn call_unexistent_selector_should_fail() {
	impl_precompile_mock_simple!(Mock, PrecompileMockParams::default());

	let nonexistent_selector =
		hex::decode("fb24ae530000000000000000000000000000000000000000000000000000000000000000")
			.unwrap();
	let mut handle = create_mock_handle_from_input(nonexistent_selector);
	let result = Mock::execute(&mut handle);
	assert_eq!(result.unwrap_err(), revert("unknown selector"));
}

mod helpers {
	use pallet_laos_evolution::CollectionId;
	use sp_runtime::DispatchError;

	/// Macro to define a precompile mock for testing.
	///
	/// This macro creates mock implementations of the `CollectionManager` trait,
	/// allowing you to test how your code interacts with the precompiled contracts.
	/// The mock type is named `Mock`, and the implementation uses the provided expressions.
	///
	/// # Arguments
	///
	/// * `$name`: An identifier to name the precompile mock type.
	/// * `$create_collection_result`: An expression that evaluates to a `Result<CollectionId,
	///   &'static str>`.
	///
	/// # Example
	///
	/// ```
	/// impl_precompile_mock_simple!(Mock, Ok(0)));
	/// ```
	#[macro_export]
	macro_rules! impl_precompile_mock {
		($name:ident, $create_collection_result:expr) => {
			use pallet_laos_evolution::types::*;
			use sp_runtime::DispatchError;

			struct EvolutionCollectionFactoryMock;

			impl pallet_laos_evolution::traits::EvolutionCollectionFactory<AccountId>
				for EvolutionCollectionFactoryMock
			{
				fn create_collection(owner: AccountId) -> Result<CollectionId, DispatchError> {
					($create_collection_result)(owner)
				}
			}

			type $name = EvolutionCollectionFactoryPrecompile<
				AddressMapping,
				AccountId,
				EvolutionCollectionFactoryMock,
			>;
		};
	}

	/// Macro to define a precompile mock for testing.
	///
	/// This macro creates mock implementations of the `CollectionManager` trait,
	/// allowing you to test how your code interacts with the precompiled contracts.
	/// The mock type is named `Mock`, and the implementation uses the provided expressions.
	///
	/// # Arguments
	///
	/// * `$create_collection_result`: An expression that evaluates to a `Result`.
	/// * `$owner_of_collection_result`: An expression that evaluates to an `Option<AccountId>`.
	///
	/// # Example
	///
	/// ```
	/// impl_precompile_mock_simple!(Mock, Ok(0), Some(BaseURI::new());
	/// ```
	#[macro_export]
	macro_rules! impl_precompile_mock_simple {
		($name:ident, $params:expr) => {
			impl_precompile_mock!($name, |_owner| { $params.create_collection_result });
		};
	}

	pub struct PrecompileMockParams {
		pub create_collection_result: Result<CollectionId, DispatchError>,
	}

	impl Default for PrecompileMockParams {
		fn default() -> Self {
			Self { create_collection_result: Ok(0) }
		}
	}
}
