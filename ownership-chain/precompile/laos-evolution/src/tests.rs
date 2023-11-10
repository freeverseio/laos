//! Living assets precompile tests.

//TODO: remove this and fix clippy issues
#![allow(clippy::redundant_closure_call)]

use core::str::FromStr;

use crate::tests::helpers::PrecompileMockParams;

use super::*;
use frame_support::assert_ok;
use precompile_utils::{
	revert, succeed,
	testing::{create_mock_handle, create_mock_handle_from_input},
};
use sp_core::{H160, H256, U256};
use sp_std::vec::Vec;

type AccountId = H160;
type AddressMapping = pallet_evm::IdentityAddressMapping;

const ALICE: &str = "0xf24FF3a9CF04c71Dbc94D0b566f7A27B94566cac";

#[test]
fn check_selectors() {
	assert_eq!(Action::CreateCollection as u32, 0x2069E953);
}

#[test]
fn check_log_selectors() {
	assert_eq!(
		hex::encode(SELECTOR_LOG_NEW_COLLECTION),
		"5b84d9550adb7000df7bee717735ecd3af48ea3f66c6886d52e8227548fb228c"
	);
	assert_eq!(
		hex::encode(SELECTOR_LOG_MINTED_WITH_EXTERNAL_TOKEN_URI),
		"4b3b5da28a351f8bb73b960d7c80b2cef3e3570cb03448234dee173942c74786"
	);
	assert_eq!(
		hex::encode(SELECTOR_LOG_EVOLVED_WITH_EXTERNAL_TOKEN_URI),
		"95c167d04a267f10e6b3f373c7a336dc65cf459caf048854dc32a2d37ab1607c"
	);
}

#[test]
fn function_selectors() {
	assert_eq!(Action::CreateCollection as u32, 0x2069E953);
	assert_eq!(Action::Owner as u32, 0x8DA5CB5B);
	assert_eq!(Action::TokenURI as u32, 0xC8A3F102);
	assert_eq!(Action::Mint as u32, 0xD4AF5BBB);
	assert_eq!(Action::Evolve as u32, 0x0EF2629F);
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
			0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0,
			0, 0, 0
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
			0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0,
			0, 0, 123
		]
	);
}

#[test]
fn mint_with_external_uri_should_generate_log() {
	impl_precompile_mock_simple!(
		Mock,
		PrecompileMockParams {
			mint_result: Ok(U256::from_str("0x010203").unwrap()),
			..Default::default()
		}
	);

	let input = EvmDataWriter::new_with_selector(Action::Mint)
		.write(U256::from(123)) // collection_id
		.write(U256::from(9)) // slot
		.write(Address(H160::from_str(ALICE).unwrap())) // to
		.write(Bytes("ciao".into())) // token_uri
		.build();
	let mut handle = create_mock_handle_from_input(input);

	let result = Mock::execute(&mut handle);
	assert!(result.is_ok());
	let logs = handle.logs;
	assert_eq!(logs.len(), 1);
	assert_eq!(logs[0].address, H160::zero());
	assert_eq!(logs[0].topics.len(), 2);
	assert_eq!(logs[0].topics[0], SELECTOR_LOG_MINTED_WITH_EXTERNAL_TOKEN_URI.into());
	assert_eq!(
		logs[0].topics[1],
		H256::from_str("0x000000000000000000000000f24ff3a9cf04c71dbc94d0b566f7a27b94566cac")
			.unwrap()
	);
	assert_eq!(
		logs[0].data,
		vec![
			0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
			0, 0, 123, // collection id
			0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
			0, 0, 9, // slot
			0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
			0, 0, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
			0, 0, 0, 0, 1, 2, 3, // token id
			0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
			0, 0, 4, // token uri length
			99, 105, 97, 111, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
			0, 0, 0, 0, 0
		] // token uri
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
		}, // Closure for create_collection result
		|_| { None }, // Closure for collection_owner result
		|_, _, _, _, _| { Ok(0.into()) }, // Closure for mint result
		|_, _| { None }, // Closure for token_uri result
		|_, _, _, _| { Ok(()) }  // Closure for evolve result
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

#[test]
fn call_owner_of_non_existent_collection() {
	impl_precompile_mock_simple!(Mock, PrecompileMockParams::default());

	let input = EvmDataWriter::new_with_selector(Action::Owner).build();
	let mut handle = create_mock_handle(input, 0, 0, H160::zero());
	handle.context.address = H160::from_str("0000000000000000000000010000000000000005").unwrap();
	let result = Mock::execute(&mut handle);
	assert_eq!(result.unwrap_err(), revert("collection does not exist"));
}

#[test]
fn call_owner_of_non_invalid_collection() {
	impl_precompile_mock_simple!(Mock, Ok(0), None, Ok(0.into()), None, Ok(()));

	let input = EvmDataWriter::new_with_selector(Action::Owner).write(U256::from(0)).build();
	let mut handle = create_mock_handle(input, 0, 0, H160::zero());
	handle.context.address = H160::from_str("0000000000000000000000000000000000000005").unwrap();
	let result = Mock::execute(&mut handle);
	assert_eq!(result.unwrap_err(), revert("invalid collection address"));
}

#[test]
fn call_owner_of_collection_works() {
	impl_precompile_mock_simple!(
		Mock,
		PrecompileMockParams {
			collection_owner_result: Some(H160::from_low_u64_be(0x1234)),
			..Default::default()
		}
	);

	let owner = H160::from_low_u64_be(0x1234);
	let input = EvmDataWriter::new_with_selector(Action::Owner).build();

	let mut handle = create_mock_handle(input, 0, 0, H160::zero());
	handle.context.address = H160::from_str("0000000000000000000000010000000000000005").unwrap();
	let result = Mock::execute(&mut handle).unwrap();
	assert_eq!(result, succeed(EvmDataWriter::new().write(Address(owner.into())).build()));
}

#[test]
fn token_uri_returns_nothing_when_source_token_uri_is_none() {
	impl_precompile_mock_simple!(Mock, PrecompileMockParams::default());

	let input = EvmDataWriter::new_with_selector(Action::TokenURI)
		.write(0_u64)
		.write(TokenId::from(0))
		.build();

	let mut handle = create_mock_handle_from_input(input);
	let result = Mock::execute(&mut handle);
	assert_eq!(result.unwrap_err(), revert("asset does not exist"));
}

#[test]
fn token_uri_returns_the_result_from_source() {
	impl_precompile_mock_simple!(
		Mock,
		PrecompileMockParams { token_uri_result: Some(vec![1_u8, 10]), ..Default::default() }
	);

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
		PrecompileMockParams {
			collection_owner_result: Some(H160::from_low_u64_be(0x1234)),
			mint_result: Ok(1.into()),
			..Default::default()
		}
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
		PrecompileMockParams {
			collection_owner_result: Some(H160::from_low_u64_be(0x1234)),
			mint_result: Err(DispatchError::Other("this is error")),
			..Default::default()
		}
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

mod evolve {
	use super::*;

	#[test]
	fn happy_path() {
		impl_precompile_mock_simple!(
			Mock,
			PrecompileMockParams {
				collection_owner_result: Some(H160::from_low_u64_be(0x1234)),
				mint_result: Ok(1.into()),
				..Default::default()
			}
		);

		let input = EvmDataWriter::new_with_selector(Action::Evolve)
			.write(U256::from(0))
			.write(U256::from(1))
			.write(Bytes([1u8; 20].to_vec()))
			.build();

		let mut handle = create_mock_handle_from_input(input);
		let result = Mock::execute(&mut handle).unwrap();

		assert_eq!(result, succeed(EvmDataWriter::new().write(H256::from_low_u64_be(1)).build()));
	}

	#[test]
	fn when_succeeds_should_generate_log() {
		impl_precompile_mock_simple!(Mock, PrecompileMockParams::default());

		let collection_id = 2;
		let token_id = 1;
		let token_uri = Bytes([1u8; 20].to_vec());

		let input = EvmDataWriter::new_with_selector(Action::Evolve)
			.write(U256::from(collection_id))
			.write(U256::from(token_id))
			.write(token_uri.clone())
			.build();
		let mut handle = create_mock_handle_from_input(input);

		let result = Mock::execute(&mut handle);
		assert!(result.is_ok());
		let logs = handle.logs;
		assert_eq!(logs.len(), 1);
		assert_eq!(logs[0].address, H160::zero());
		assert_eq!(logs[0].topics.len(), 2);
		assert_eq!(logs[0].topics[0], SELECTOR_LOG_EVOLVED_WITH_EXTERNAL_TOKEN_URI.into());
		assert_eq!(
			logs[0].data,
			vec![
				0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
				0, 0, 0, 2, // collection_id
				0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
				0, 0, 0, 64, // offset
				0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
				0, 0, 0, 20, // lenght of token_uri
				1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0,
				0, 0, 0, 0 // token_uri
			]
		);
	}

	#[test]
	fn when_fails_should_return_error() {
		impl_precompile_mock_simple!(
			Mock,
			PrecompileMockParams {
				collection_owner_result: Some(H160::from_low_u64_be(0x1234)),
				mint_result: Ok(1.into()),
				evolve_result: Err(DispatchError::Other("this is error")),
				..Default::default()
			}
		);

		let input = EvmDataWriter::new_with_selector(Action::Evolve)
			.write(U256::from(0))
			.write(U256::from(1))
			.write(Bytes([1u8; 20].to_vec()))
			.build();

		let mut handle = create_mock_handle_from_input(input);
		let result = Mock::execute(&mut handle).unwrap_err();

		assert_eq!(result, revert("this is error"));
	}
}

mod helpers {
	use super::AccountId;
	use pallet_laos_evolution::{CollectionId, TokenId};
	use sp_runtime::DispatchError;
	type TokenUri = Vec<u8>;

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
		($name:ident, $create_collection_result:expr, $collection_owner_result:expr, $mint_result:expr, $token_uri_result:expr, $evolve_result:expr ) => {
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

				fn evolve_with_external_uri(
					who: AccountId,
					collection_id: CollectionId,
					token_id: TokenId,
					token_uri: TokenUri,
				) -> Result<(), DispatchError> {
					($evolve_result)(who, collection_id, token_id, token_uri)
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
			impl_precompile_mock!(
				$name,
				|_owner| { $params.create_collection_result },
				|_collection_id| { $params.collection_owner_result },
				|_who, _collection_id, _slot, _to, _token_uri| { $params.mint_result },
				|_collection_id, _token_id| { $params.token_uri_result },
				|_who, _collection_id, _token_id, _token_uri| { $params.evolve_result }
			);
		};
	}

	pub struct PrecompileMockParams {
		pub create_collection_result: Result<CollectionId, DispatchError>,
		pub collection_owner_result: Option<AccountId>,
		pub mint_result: Result<TokenId, DispatchError>,
		pub token_uri_result: Option<TokenUri>,
		pub evolve_result: Result<(), DispatchError>,
	}

	impl Default for PrecompileMockParams {
		fn default() -> Self {
			Self {
				create_collection_result: Ok(0),
				collection_owner_result: None,
				mint_result: Ok(0.into()),
				token_uri_result: None,
				evolve_result: Ok(()),
			}
		}
	}
}
