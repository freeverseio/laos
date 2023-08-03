use super::*;
use evm::ExitRevert;
use helpers::*;
use sp_core::H160;

type CollectionId = u64;
type AccountId = H160;
type AddressMapping = pallet_evm::IdentityAddressMapping;

const OWNER_OF_COLLECTION_0: &str =
	"fb34ae530000000000000000000000000000000000000000000000000000000000000000";
const CREATE_COLLECTION2: &str = "647f1a9c";

#[test]
fn check_selectors() {
	assert_eq!(Action::OwnerOfCollection as u32, 0xFB34AE53);
	assert_eq!(Action::CreateCollection as u32, 0x647F1A9C);
}

#[test]
fn failing_create_collection_should_return_error() {
	impl_precompile_mock_simple!(Mock, Err("spaghetti code"), Some(H160::zero()));

	let mut handle = create_mock_handle_from_input(CREATE_COLLECTION2);
	let result = Mock::execute(&mut handle);
	assert_eq!(
		result.unwrap_err(),
		PrecompileFailure::Error {
			exit_status: ExitError::Other(sp_std::borrow::Cow::Borrowed("spaghetti code"))
		}
	);
}

#[test]
fn create_collection_should_return_id() {
	impl_precompile_mock_simple!(Mock, Ok(5), Some(H160::zero()));

	let mut handle = create_mock_handle_from_input(CREATE_COLLECTION2);
	let result = Mock::execute(&mut handle);
	assert!(result.is_ok());
	// check that the output is the collection id 0
	assert_eq!(result.unwrap().output, 5u64.encode());
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

	let mut handle = create_mock_handle(CREATE_COLLECTION2, 0, 0, H160::from_low_u64_be(0x1234));
	let result = Mock::execute(&mut handle);
	assert!(result.is_ok());
}

#[test]
fn owner_of_with_nonzero_transfer_should_fail() {
	impl_precompile_mock_simple!(Mock, Ok(0), None);

	let mut handle = create_mock_handle("", 0, 1, H160::zero());
	let result = Mock::execute(&mut handle);
	assert!(result.is_err());
}

#[test]
fn owner_of_on_no_owner_should_return_null() {
	impl_precompile_mock_simple!(Mock, Ok(0), None);

	let mut handle = create_mock_handle_from_input(OWNER_OF_COLLECTION_0);
	let result = Mock::execute(&mut handle);
	assert_eq!(result.unwrap().output, Vec::<u8>::new());
}

#[test]
fn owner_of_should_return_owner_of_mock() {
	impl_precompile_mock_simple!(Mock, Ok(0), Some(H160::from_low_u64_be(0x1234)));

	let mut handle = create_mock_handle_from_input(OWNER_OF_COLLECTION_0);
	let result = Mock::execute(&mut handle);
	assert!(result.is_ok());
	assert_eq!(result.unwrap().output, H160::from_low_u64_be(0x1234).encode());
}

#[test]
fn call_unexistent_selector_should_fail() {
	impl_precompile_mock_simple!(Mock, Ok(0), Some(H160::from_low_u64_be(0x1234)));

	// unexistent selector
	let input = "fb24ae530000000000000000000000000000000000000000000000000000000000000000";
	let mut handle = create_mock_handle_from_input(input);
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

mod helpers {
	use evm::Context;
	use pallet_evm_test_vector_support::MockHandle;
	use sp_core::H160;

	/// Macro to define a precompile mock with custom closures for testing.
	///
	/// This macro creates mock implementations of the `CollectionManager` trait,
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
	/// impl_precompile_mock!(
	///     MyMock,
	///     |collection_id, who| { Ok(()) },
	///     |collection_id| { Some(H160::zero()) }
	/// );
	/// ```
	#[macro_export]
	macro_rules! impl_precompile_mock {
		($name:ident, $create_collection_result:expr, $owner_of_collection_result:expr) => {
			struct CollectionManagerMock;

			impl pallet_living_assets_ownership::traits::CollectionManager<AccountId, CollectionId>
				for CollectionManagerMock
			{
				fn create_collection(owner: AccountId) -> Result<CollectionId, &'static str> {
					($create_collection_result)(owner)
				}

				fn owner_of_collection(collection_id: CollectionId) -> Option<AccountId> {
					($owner_of_collection_result)(collection_id)
				}
			}

			type $name = CollectionManagerPrecompile<
				AddressMapping,
				AccountId,
				CollectionId,
				CollectionManagerMock,
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
	/// * `$create_collection_result`: An expression that evaluates to a `DispatchResult`.
	/// * `$owner_of_collection_result`: An expression that evaluates to an `Option<AccountId>`.
	///
	/// # Example
	///
	/// ```
	/// impl_precompile_mock_simple!(Mock,On(0), Some(H160::zero()));
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
	pub fn create_mock_handle(input: &str, cost: u64, value: u64, caller: H160) -> MockHandle {
		let i: Vec<u8> = hex::decode(input).expect("invalid input");

		let context: Context =
			Context { address: Default::default(), caller, apparent_value: From::from(value) };

		MockHandle::new(i, Some(cost), context)
	}

	/// Create a mock handle for testing precompiled contracts without a specific cost or value.
	///
	/// This function takes an input string representing the data to be sent to the precompiled contract
	/// and returns a `MockHandle` that can be used for testing.
	///
	/// # Arguments
	///
	/// * `input` - The input data as a hexadecimal string.
	///
	/// # Example
	///
	/// ```
	/// let handle = create_mock_handle_from_input("68656c6c6f");
	/// ```
	pub fn create_mock_handle_from_input(input: &str) -> MockHandle {
		create_mock_handle(input, 0, 0, H160::zero())
	}
}
