use super::*;
use evm::ExitRevert;
use helpers::*;
use sp_core::H160;
use sp_runtime::{DispatchError, DispatchResult};

type CollectionId = u64;
type AccountId = H160;
type AddressMapping = pallet_evm::IdentityAddressMapping;

#[test]
fn check_selectors() {
	assert_eq!(Action::CreateCollection as u32, 0x1EAF2516);
	assert_eq!(Action::OwnerOfCollection as u32, 0xFB34AE53);
}

#[test]
fn create_collection_on_mock_succeed_should_succeed() {
	define_precompile_mock!(Mock, Ok(()), Some(H160::zero()));

	let input = "1eaf25160000000000000000000000000000000000000000000000000000000000000000000000000000000000000000b7469c43535c826e29c30d25a9f3a035759cf132";
	let mut handle = create_mock_handle(input, 0, 0);
	let result = Mock::execute(&mut handle);
	assert!(result.is_ok());
}

#[test]
fn create_collection_on_mock_fail_with_other_error() {
	define_precompile_mock!(
		Mock,
		Err(DispatchError::Other("pizza error")),
		Some(H160::zero())
	);

	let input = "1eaf25160000000000000000000000000000000000000000000000000000000000000000000000000000000000000000b7469c43535c826e29c30d25a9f3a035759cf132";
	let mut handle = create_mock_handle(input, 0, 0);
	let result = Mock::execute(&mut handle);
	assert!(result.is_err());
	assert_eq!(
		result.unwrap_err(),
		PrecompileFailure::Error {
			exit_status: ExitError::Other(sp_std::borrow::Cow::Borrowed("pizza error"))
		}
	);
}

#[test]
fn create_collection_on_mock_with_nonzero_value_fails() {
	define_precompile_mock!(Mock, Ok(()), Some(H160::zero()));

	let input = "1eaf25160000000000000000000000000000000000000000000000000000000000000000000000000000000000000000b7469c43535c826e29c30d25a9f3a035759cf132";
	let mut handle = create_mock_handle(input, 0, 1);
	let result = Mock::execute(&mut handle);
	assert!(result.is_err());
}

#[test]
fn create_collection_on_mock_fail_with_corruption_error() {
	define_precompile_mock!(Mock, Err(DispatchError::Corruption), Some(H160::zero()));

	let input = "1eaf25160000000000000000000000000000000000000000000000000000000000000000000000000000000000000000b7469c43535c826e29c30d25a9f3a035759cf132";
	let mut handle = create_mock_handle(input, 0, 0);
	let result = Mock::execute(&mut handle);
	assert!(result.is_err());
	assert_eq!(
		result.unwrap_err(),
		PrecompileFailure::Error {
			exit_status: ExitError::Other(sp_std::borrow::Cow::Borrowed("Corruption"))
		}
	);
}

#[test]
fn owner_of_with_nonzero_transfer_should_fail() {
	define_precompile_mock!(Mock, Ok(()), None);

	let input = "fb34ae530000000000000000000000000000000000000000000000000000000000000000";
	let mut handle = create_mock_handle(input, 0, 1);
	let result = Mock::execute(&mut handle);
	assert!(result.is_err());
}

#[test]
fn owner_of_on_no_owner_should_return_null() {
	define_precompile_mock!(Mock, Ok(()), None);

	let input = "fb34ae530000000000000000000000000000000000000000000000000000000000000000";
	let mut handle = create_mock_handle(input, 0, 0);
	let result = Mock::execute(&mut handle);
	assert_eq!(result.unwrap().output, Vec::<u8>::new());
}

#[test]
fn owner_of_should_return_owner_of_mock() {
	define_precompile_mock!(Mock, Ok(()), Some(H160::from_low_u64_be(0x1234)));

	let input = "fb34ae530000000000000000000000000000000000000000000000000000000000000000";
	let mut handle = create_mock_handle(input, 0, 0);
	let result = Mock::execute(&mut handle);
	assert!(result.is_ok());
	assert_eq!(result.unwrap().output, H160::from_low_u64_be(0x1234).encode());
}

#[test]
fn call_unexistent_selector_should_fail() {
	define_precompile_mock!(Mock, Ok(()), Some(H160::from_low_u64_be(0x1234)));

	let input = "fb24ae530000000000000000000000000000000000000000000000000000000000000000";
	let mut handle = create_mock_handle(input, 0, 0);
	let result = Mock::execute(&mut handle);
	assert_eq!(
		result.unwrap_err(),
		PrecompileFailure::Revert {
			exit_status: ExitRevert::Reverted,
			output: [117, 110, 107, 110, 111, 119, 110, 32, 115, 101, 108, 101, 99, 116, 111, 114]
				.to_vec()
		}
	);
}

#[test]
fn create_collection_with_max_id() {
	define_precompile_mock_closures!(
		Mock, // name of the defined precompile
		|collection_id, _| {
			assert_eq!(collection_id, CollectionId::max_value());
			Ok(())
		}, // Closure for create_collection result
		|_| { Some(H160::zero()) }  // Closure for owner_of_collection result
	);

	let input = "1eaf2516000000000000000000000000000000000000000000000000ffffffffffffffff000000000000000000000000b7469c43535c826e29c30d25a9f3a035759cf132";
	let mut handle = create_mock_handle(input, 0, 0);
	let result = Mock::execute(&mut handle);
	assert!(result.is_ok());
}

mod helpers {
	use evm::Context;
	use pallet_evm_test_vector_support::MockHandle;

	/// Macro to define a precompile mock with custom closures for testing.
	///
	/// This macro creates mock implementations of the `LivingAssetsOwnership` trait,
	/// allowing you to test how your code interacts with the precompiled contracts.
	/// You can define custom closures for the create_collection and owner_of_collection functions.
	///
	/// # Arguments
	///
	/// * `$name`: An identifier to name the precompile mock type.
	/// * `$create_collection_result`: A closure that takes `collection_id` and `who` and returns a `DispatchResult`.
	/// * `$owner_of_collection_result`: A closure that takes `collection_id` and returns an `Option<AccountId>`.
	///
	/// # Example
	///
	/// ```
	/// define_precompile_mock_closures!(
	///     MyMock,
	///     |collection_id, who| { Ok(()) },
	///     |collection_id| { Some(H160::zero()) }
	/// );
	/// ```
	#[macro_export]
	macro_rules! define_precompile_mock_closures {
		($name:ident, $create_collection_result:expr, $owner_of_collection_result:expr) => {
			struct CollectionManagerMock;

			impl pallet_living_assets_ownership::LivingAssetsOwnership<AccountId, CollectionId>
				for CollectionManagerMock
			{
				fn create_collection(
					collection_id: CollectionId,
					who: AccountId,
				) -> DispatchResult {
					($create_collection_result)(collection_id, who)
				}

				fn owner_of_collection(collection_id: CollectionId) -> Option<AccountId> {
					($owner_of_collection_result)(collection_id)
				}
			}

			type $name = LivingAssetsOwnershipPrecompile<
				AddressMapping,
				AccountId,
				CollectionId,
				CollectionManagerMock,
			>;
		};
	}

	/// Macro to define a precompile mock for testing.
	///
	/// This macro creates mock implementations of the `LivingAssetsOwnership` trait,
	/// allowing you to test how your code interacts with the precompiled contracts.
	/// The mock type is named `Mock`, and the implementation uses the provided expressions.
	///
	/// # Arguments
	///
	/// * `$create_collection_result`: An expression that evaluates to a `DispatchResult`.
	/// * `$owner_of_collection_result`: An expression that evaluates to an `Option<AccountId>`.
	///
	/// # Example
	///
	/// ```
	/// define_precompile_mock!(Mock,Ok(()), Some(H160::zero()));
	/// ```
	#[macro_export]
	macro_rules! define_precompile_mock {
		($name:ident, $create_collection_result:expr, $owner_of_collection_result:expr) => {
			define_precompile_mock_closures!(
				$name,
				|_collection_id, _who| { $create_collection_result },
				|_collection_id| { $owner_of_collection_result }
			);
		};
	}

	/// Create a mock handle for testing precompiled contracts.
	///
	/// This function takes an input string representing the data to be sent to the precompiled contract
	/// and a cost value, returning a `MockHandle` that can be used for testing.
	///
	/// # Arguments
	///
	/// * `input` - The input data as a hexadecimal string.
	/// * `cost` - A cost value as u64.
	/// * `value` - The amount of coins transferred as u64.
	///
	/// # Example
	///
	/// ```
	/// let handle = create_mock_handle("68656c6c6f", 0, 0);
	/// ```
	pub fn create_mock_handle(input: &str, cost: u64, value: u64) -> MockHandle {
		let i: Vec<u8> = hex::decode(input).expect("todo");

		let context: Context = Context {
			address: Default::default(),
			caller: Default::default(),
			apparent_value: From::from(value),
		};

		MockHandle::new(i, Some(cost), context)
	}
}
