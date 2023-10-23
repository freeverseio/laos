//! Living assets precompile tests.

//TODO: remove this and fix clippy issues
#![allow(clippy::redundant_closure_call)]

use super::*;
use frame_support::assert_ok;
use precompile_utils::{
	revert, succeed,
	testing::{create_mock_handle, create_mock_handle_from_input},
};
use sp_core::{H160, U256};
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
		hex::encode(SELECTOR_LOG_NEW_COLLECTION),
		"6eb24fd767a7bcfa417f3fe25a2cb245d2ae52293d3c4a8f8c6450a09795d289"
	);
}

#[test]
fn function_selectors() {
	assert_eq!(Action::CreateCollection as u32, 0x2069E953);
	assert_eq!(Action::OwnerOfCollection as u32, 0xFB34AE53);
	assert_eq!(Action::TokenURI as u32, 0xC8A3F102);
	assert_eq!(Action::Mint as u32, 0x3B8EF7A4);
}

#[test]
fn failing_create_collection_should_return_error() {
	impl_precompile_mock_simple!(
		Mock,
		Err(DispatchError::Other("this is an error")),
		None,
		Ok(0.into()),
		None
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
	impl_precompile_mock_simple!(Mock, Ok(0), None, Ok(0.into()), None);

	let input = EvmDataWriter::new_with_selector(Action::CreateCollection)
		.write(Address(H160([1u8; 20])))
		.build();
	let mut handle = create_mock_handle_from_input(input);

	let result = Mock::execute(&mut handle);
	assert_ok!(result, succeed(H256::from_low_u64_be(0)));
}

#[test]
fn create_collection_should_generate_log() {
	impl_precompile_mock_simple!(Mock, Ok(0), None, Ok(0.into()), None);

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
	assert_eq!(logs[0].topics[0], SELECTOR_LOG_NEW_COLLECTION.into());
	assert_eq!(logs[0].topics[1], H256::from_low_u64_be(0));
	assert_eq!(logs[0].data, Vec::<u8>::new());
}

#[test]
fn create_collection_on_mock_with_nonzero_value_fails() {
	impl_precompile_mock_simple!(Mock, Ok(5), None, Ok(0.into()), None);

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
		}, // Closure for create_collection result
		|_| { None }, // Closure for collection_owner result
		|_, _, _, _, _| { Ok(0.into()) }, // Closure for mint result
		|_, _| { None }  // Closure for token_uri result
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
	impl_precompile_mock_simple!(Mock, Ok(0), None, Ok(0.into()), None);

	let nonexistent_selector =
		hex::decode("fb24ae530000000000000000000000000000000000000000000000000000000000000000")
			.unwrap();
	let mut handle = create_mock_handle_from_input(nonexistent_selector);
	let result = Mock::execute(&mut handle);
	assert_eq!(result.unwrap_err(), revert("unknown selector"));
}

#[test]
fn call_owner_of_non_existent_collection() {
	impl_precompile_mock_simple!(Mock, Ok(0), None, Ok(0.into()), None);

	let input = EvmDataWriter::new_with_selector(Action::OwnerOfCollection)
		.write(U256::from(0))
		.build();
	let mut handle = create_mock_handle_from_input(input);
	let result = Mock::execute(&mut handle);
	assert_eq!(result.unwrap_err(), revert("collection does not exist"));
}

#[test]
fn call_owner_of_collection_works() {
	impl_precompile_mock_simple!(
		Mock,
		Ok(0),
		Some(H160::from_low_u64_be(0x1234)),
		Ok(0.into()),
		None
	);

	let owner = H160::from_low_u64_be(0x1234);

	let input = EvmDataWriter::new_with_selector(Action::OwnerOfCollection)
		.write(Address(owner))
		.build();

	let mut handle = create_mock_handle_from_input(input);
	let result = Mock::execute(&mut handle).unwrap();
	assert_eq!(result, succeed(EvmDataWriter::new().write(Address(owner.into())).build()));
}

#[test]
fn token_uri_returns_nothing_when_source_token_uri_is_none() {
	impl_precompile_mock_simple!(Mock, Ok(0), None, Ok(0.into()), None);

	let input = EvmDataWriter::new_with_selector(Action::TokenURI)
		.write(0_u64)
		.write(TokenId::from(0))
		.build();

	let mut handle = create_mock_handle_from_input(input);
	let result = Mock::execute(&mut handle);
	assert_eq!(result.unwrap(), succeed(EvmDataWriter::new().write(Bytes(Vec::new())).build()));
}

#[test]
fn token_uri_returns_the_result_from_source() {
	impl_precompile_mock_simple!(Mock, Ok(0), None, Ok(0.into()), Some(vec![1_u8, 10]));

	let input = EvmDataWriter::new_with_selector(Action::TokenURI)
		.write(0_u64)
		.write(TokenId::from(0))
		.build();

	let mut handle = create_mock_handle_from_input(input);
	let result = Mock::execute(&mut handle);
	assert_eq!(result.unwrap(), succeed(EvmDataWriter::new().write(Bytes(vec![1_u8, 10])).build()));
}

#[test]
fn mint_works() {
	impl_precompile_mock_simple!(
		Mock,
		Ok(0),
		Some(H160::from_low_u64_be(0x1234)),
		Ok(1.into()),
		None
	);

	let to = H160::from_low_u64_be(1);

	let input = EvmDataWriter::new_with_selector(Action::Mint)
		.write(U256::from(0))
		.write(U256::from(1))
		.write(Address(to))
		.write(Bytes([1u8; 20].to_vec()))
		.build();

	let mut handle = create_mock_handle_from_input(input);
	let result = Mock::execute(&mut handle).unwrap();

	assert_eq!(result, succeed(EvmDataWriter::new().write(H256::from_low_u64_be(1)).build()));
}

#[test]
fn failing_mint_should_return_error() {
	impl_precompile_mock_simple!(
		Mock,
		Ok(0),
		Some(H160::from_low_u64_be(0x1234)),
		Err(DispatchError::Other("this is error")),
		None
	);

	let to = H160::from_low_u64_be(1);

	let input = EvmDataWriter::new_with_selector(Action::Mint)
		.write(U256::from(0))
		.write(U256::from(1))
		.write(Address(to))
		.write(Bytes([1u8; 20].to_vec()))
		.build();

	let mut handle = create_mock_handle_from_input(input);
	let result = Mock::execute(&mut handle).unwrap_err();

	assert_eq!(result, revert("this is error"));
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
	/// * `$collection_owner_result`: An expression that evaluates to an `Option<AccountId>`.
	///
	/// # Example
	///
	/// ```
	/// impl_precompile_mock_simple!(Mock, Ok(0), Some(H160::zero()));
	/// ```
	#[macro_export]
	macro_rules! impl_precompile_mock {
		($name:ident, $create_collection_result:expr, $collection_owner_result:expr, $mint_result:expr, $token_uri_result:expr ) => {
			use pallet_laos_evolution::types::*;
			use sp_runtime::DispatchError;
			type TokenUri = Vec<u8>;

			struct LaosEvolutionMock;

			impl pallet_laos_evolution::traits::LaosEvolution<AccountId, TokenUri>
				for LaosEvolutionMock
			{
				fn create_collection(owner: AccountId) -> Result<CollectionId, DispatchError> {
					($create_collection_result)(owner)
				}

				fn mint_with_external_uri(
					who: AccountId,
					collection_id: CollectionId,
					slot: Slot,
					to: AccountId,
					token_uri: TokenUri,
				) -> Result<TokenId, DispatchError> {
					($mint_result)(who, collection_id, slot, to, token_uri)
				}

				fn collection_owner(collection_id: CollectionId) -> Option<AccountId> {
					($collection_owner_result)(collection_id)
				}

				fn token_uri(_collection_id: CollectionId, _token_id: TokenId) -> Option<TokenUri> {
					($token_uri_result)(_collection_id, _token_id)
				}
			}

			type $name =
				LaosEvolutionPrecompile<AddressMapping, AccountId, TokenUri, LaosEvolutionMock>;
		};
	}

	/// Macro to define a precompile mock for testing.
	///
	/// This macro creates mock implementations of the `CollectionManager` trait,
	/// allowing you to test how your code interacts with the precompiled contracts.
	/// The mock type is named `Mock`, and the implementation uses the provided ehttps://meet.google.com/ntw-cgcg-fjfxpressions.
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
		($name:ident, $create_collection_result:expr, $collection_owner_result:expr, $mint_result:expr, $token_uri_result:expr) => {
			impl_precompile_mock!(
				$name,
				|_owner| { $create_collection_result },
				|_collection_id| { $collection_owner_result },
				|_who, _collection_id, _slot, _to, _token_uri| { $mint_result },
				|_collection_id, _token_id| { $token_uri_result }
			);
		};
	}
}
