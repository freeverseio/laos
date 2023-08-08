//! Living assets precompile tests.

//TODO: remove this and fix clippy issues
#![allow(clippy::redundant_closure_call)]

use super::*;
use precompile_utils::{
	revert, succeed,
	testing::{create_mock_handle, create_mock_handle_from_input},
};
use sp_core::H160;
use sp_std::vec::Vec;

type AccountId = H160;
type AddressMapping = pallet_evm::IdentityAddressMapping;

const CREATE_COLLECTION: &str = "647f1a9c";

#[test]
fn check_selectors() {
	assert_eq!(Action::CreateCollection as u32, 0x647F1A9C);
}

#[test]
fn check_log_selectors() {
	assert_eq!(
		hex::encode(SELECTOR_LOG_CREATE_COLLECTION),
		"18896a5e5f9fd6b9d74f89291fe4640722c8dc4d6a1025ccf047607f3e6954ee"
	);
}

#[test]
fn failing_create_collection_should_return_error() {
	impl_precompile_mock_simple!(Mock, Err("spaghetti code"), Some(H160::zero()));

	let mut handle = create_mock_handle_from_input(hex::decode(CREATE_COLLECTION).unwrap());
	let result = Mock::execute(&mut handle);
	assert_eq!(result.unwrap_err(), revert("spaghetti code"));
}

#[test]
fn create_collection_should_return_address() {
	impl_precompile_mock_simple!(Mock, Ok(5), Some(H160::zero()));

	let mut handle = create_mock_handle_from_input(hex::decode(CREATE_COLLECTION).unwrap());
	let result = Mock::execute(&mut handle);
	assert!(result.is_ok());
	assert_eq!(
		result.unwrap(),
		succeed(
			hex::decode("000000000000000000000000ffffffffffffffffffffffff0000000000000005")
				.unwrap()
		)
	);
}

#[test]
fn create_collection_should_generate_log() {
	impl_precompile_mock_simple!(Mock, Ok(0xffff), Some(H160::zero()));

	let mut handle = create_mock_handle_from_input(hex::decode(CREATE_COLLECTION).unwrap());
	let result = Mock::execute(&mut handle);
	assert!(result.is_ok());
	let logs = handle.logs;
	assert_eq!(logs.len(), 1);
	assert_eq!(logs[0].address, H160::zero());
	assert_eq!(logs[0].topics.len(), 2);
	assert_eq!(logs[0].topics[0], SELECTOR_LOG_CREATE_COLLECTION.into());
	assert_eq!(
		hex::encode(logs[0].topics[1]),
		"000000000000000000000000ffffffffffffffffffffffff000000000000ffff"
	);
	assert_eq!(logs[0].data, Vec::<u8>::new());
}

#[test]
fn create_collection_on_mock_with_nonzero_value_fails() {
	impl_precompile_mock_simple!(Mock, Ok(5), Some(H160::zero()));
	let mut handle =
		create_mock_handle(hex::decode(CREATE_COLLECTION).unwrap(), 0, 1, H160::zero());
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
		}, // Closure for create_collection result
		|_| { Some(H160::zero()) }  // Closure for owner_of_collection result
	);

	let mut handle = create_mock_handle(
		hex::decode(CREATE_COLLECTION).unwrap(),
		0,
		0,
		H160::from_low_u64_be(0x1234),
	);
	let result = Mock::execute(&mut handle);
	assert!(result.is_ok());
}

#[test]
fn call_unexistent_selector_should_fail() {
	impl_precompile_mock_simple!(Mock, Ok(0), Some(H160::from_low_u64_be(0x1234)));

	let unexistent_selector =
		hex::decode("fb24ae530000000000000000000000000000000000000000000000000000000000000000")
			.unwrap();
	let mut handle = create_mock_handle_from_input(unexistent_selector);
	let result = Mock::execute(&mut handle);
	assert_eq!(result.unwrap_err(), revert("unknown selector"));
}

mod helpers {
	/// Macro to define a precompile mock for testing.
	///
	/// This macro creates mock implementations of the `CollectionManager` trait,
	/// allowing you to test how your code interacts with the precompiled contracts.
	/// The mock type is named `Mock`, and the implementation uses the provided expressions.
	///
	/// # Arguments
	///
	/// * `$name`: An identifier to name the precompile mock type.
	/// * `$create_collection_result`: An expression that evaluates to a `Result<CollectionId, &'static str>`.
	/// * `$owner_of_collection_result`: An expression that evaluates to an `Option<AccountId>`.
	///
	/// # Example
	///
	/// ```
	/// impl_precompile_mock_simple!(Mock, Ok(0), Some(H160::zero()));
	/// ```
	#[macro_export]
	macro_rules! impl_precompile_mock {
		($name:ident, $create_collection_result:expr, $owner_of_collection_result:expr) => {
			struct CollectionManagerMock;

			impl pallet_living_assets_ownership::traits::CollectionManager<AccountId>
				for CollectionManagerMock
			{
				fn create_collection(owner: AccountId) -> Result<CollectionId, &'static str> {
					($create_collection_result)(owner)
				}

				fn owner_of_collection(collection_id: CollectionId) -> Option<AccountId> {
					($owner_of_collection_result)(collection_id)
				}
			}

			type $name =
				CollectionManagerPrecompile<AddressMapping, AccountId, CollectionManagerMock>;
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
	/// impl_precompile_mock_simple!(Mock, Ok(0), Some(H160::zero()));
	/// ```
	#[macro_export]
	macro_rules! impl_precompile_mock_simple {
		($name:ident, $create_collection_result:expr, $owner_of_collection_result:expr) => {
			impl_precompile_mock!(
				$name,
				|_owner| { $create_collection_result },
				|_collection_id| { $owner_of_collection_result }
			);
		};
	}
}
