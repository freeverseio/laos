//! Living assets precompile tests.

//TODO: remove this and fix clippy issues
#![allow(clippy::redundant_closure_call)]

use super::*;
use frame_support::assert_ok;
use precompile_utils::{
	revert, succeed,
	testing::{create_mock_handle, create_mock_handle_from_input},
};
use sp_core::H160;
use sp_std::vec::Vec;

type AccountId = H160;
type AddressMapping = pallet_evm::IdentityAddressMapping;

#[test]
fn check_selectors() {
	assert_eq!(Action::CreateCollection as u32, 0x2069E953);
}

#[test]
fn check_log_selectors() {
	assert_eq!(
		hex::encode(SELECTOR_LOG_CREATE_COLLECTION),
		"0bc7b5823efec35847638ca709f87eb1588b66d062d448e6fb9eb715b103cbb8"
	);
}

#[test]
fn failing_create_collection_should_return_error() {
	impl_precompile_mock_simple!(Mock, Err(DispatchError::Other("this is an error")));

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
	impl_precompile_mock_simple!(Mock, Ok(0));

	let input = EvmDataWriter::new_with_selector(Action::CreateCollection)
		.write(Address(H160([1u8; 20])))
		.build();
	let mut handle = create_mock_handle_from_input(input);

	let result = Mock::execute(&mut handle);
	assert_ok!(result, succeed(H256::from_low_u64_be(0)));
}

#[test]
fn create_collection_should_generate_log() {
	impl_precompile_mock_simple!(Mock, Ok(0));

	let input = EvmDataWriter::new_with_selector(Action::CreateCollection)
		.write(Address(H160([1u8; 20])))
		.build();
	let mut handle = create_mock_handle_from_input(input);

	let result = Mock::execute(&mut handle);
	assert!(result.is_ok());
	let logs = handle.logs;
	assert_eq!(logs.len(), 1);
	assert_eq!(logs[0].address, H160::zero());
	assert_eq!(logs[0].topics.len(), 3);
	assert_eq!(logs[0].topics[0], SELECTOR_LOG_CREATE_COLLECTION.into());
	assert_eq!(logs[0].topics[1], H256::from_low_u64_be(0));
	assert_eq!(logs[0].data, Vec::<u8>::new());
}

#[test]
fn create_collection_on_mock_with_nonzero_value_fails() {
	impl_precompile_mock_simple!(Mock, Ok(5));

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
	impl_precompile_mock_simple!(Mock, Ok(0));

	let nonexistent_selector =
		hex::decode("fb24ae530000000000000000000000000000000000000000000000000000000000000000")
			.unwrap();
	let mut handle = create_mock_handle_from_input(nonexistent_selector);
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
	/// * `$create_collection_result`: An expression that evaluates to a `Result<CollectionId,
	///   &'static str>`.
	/// * `$owner_of_collection_result`: An expression that evaluates to an `Option<AccountId>`.
	///
	/// # Example
	///
	/// ```
	/// impl_precompile_mock_simple!(Mock, Ok(0), Some(BaseURI::new());
	/// ```
	#[macro_export]
	macro_rules! impl_precompile_mock {
		($name:ident, $create_collection_result:expr) => {
			use sp_runtime::DispatchError;
			type TokenUri = Vec<u8>;

			struct LivingAssetsEvolutionMock;

			impl pallet_laos_evolution::traits::LivingAssetsEvolution<AccountId, TokenUri>
				for LivingAssetsEvolutionMock
			{
				fn create_collection(
					owner: AccountId,
				) -> Result<pallet_laos_evolution::types::CollectionId, DispatchError> {
					($create_collection_result)(owner)
				}

				fn mint_with_external_uri(
					_who: AccountId,
					_collection_id: pallet_laos_evolution::types::CollectionId,
					_slot: pallet_laos_evolution::types::Slot,
					_to: AccountId,
					_token_uri: TokenUri,
				) -> Result<pallet_laos_evolution::types::TokenId, DispatchError> {
					unimplemented!()
				}
			}

			type $name = LaosEvolutionPrecompile<
				AddressMapping,
				AccountId,
				TokenUri,
				LivingAssetsEvolutionMock,
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
		($name:ident, $create_collection_result:expr) => {
			impl_precompile_mock!($name, |_owner| { $create_collection_result });
		};
	}
}
