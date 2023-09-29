use core::str::FromStr;

use super::*;
use frame_support::assert_ok;
use pallet_living_assets_ownership::CollectionId;
use precompile_utils::testing::create_mock_handle_from_input;
use sp_core::{H160, U256};

type AccountId = H160;

#[test]
fn check_selectors() {
	assert_eq!(Action::OwnerOf as u32, 0x6352211E);
	assert_eq!(Action::TokenURI as u32, 0xC87B56DD);
	assert_eq!(Action::TransferFrom as u32, 0x23b872dd);
}

#[test]
fn owner_of_asset_should_return_an_address() {
	impl_precompile_mock_simple!(
		Mock,
		Ok(H160::from_str("ff00000000000000000000000000000012345678").unwrap()),
		Ok(vec![]),
		Ok(())
	);

	let owner_of_asset_4 =
		hex::decode("6352211e0000000000000000000000000000000000000000000000000000000000000004")
			.unwrap();
	let mut handle = create_mock_handle_from_input(owner_of_asset_4);
	handle.code_address = H160::from_str("ffffffffffffffffffffffff0000000000000005").unwrap();
	let result = Mock::execute(&mut handle);
	assert!(result.is_ok());
	assert_eq!(
		result.unwrap(),
		succeed(
			hex::decode("000000000000000000000000ff00000000000000000000000000000012345678")
				.unwrap()
		),
	);
}

#[test]
fn if_mock_fails_should_return_the_error() {
	impl_precompile_mock_simple!(Mock, Err("this is an error"), Ok(vec![]), Ok(()));

	let owner_of_asset_4 =
		hex::decode("6352211e0000000000000000000000000000000000000000000000000000000000000004")
			.unwrap();
	let mut handle = create_mock_handle_from_input(owner_of_asset_4);
	handle.code_address = H160::from_str("ffffffffffffffffffffffff0000000000000005").unwrap();
	let result = Mock::execute(&mut handle);
	assert!(result.is_err());
	assert_eq!(result.unwrap_err(), revert("this is an error"));
}

#[test]
fn invalid_contract_address_should_error() {
	impl_precompile_mock_simple!(Mock, Ok(H160::zero()), Ok(vec![]), Ok(()));

	let mut handle = create_mock_handle_from_input(Vec::new());
	handle.code_address = H160::zero();
	let result = Mock::execute(&mut handle);
	assert!(result.is_err());
	assert_eq!(result.unwrap_err(), revert("invalid collection address"));
}

#[test]
fn token_owners_should_have_at_least_token_id_as_argument() {
	impl_precompile_mock_simple!(Mock, Ok(H160::zero()), Ok(vec![]), Ok(()));

	let owner_of_with_2_arguments: Vec<u8> =
		hex::decode("6352211e00000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000004")
			.unwrap();
	let mut handle = create_mock_handle_from_input(owner_of_with_2_arguments);
	handle.code_address = H160::from_str("ffffffffffffffffffffffff0000000000000005").unwrap();
	let result = Mock::execute(&mut handle);
	assert!(result.is_ok());

	let owner_of_with_0_arguments: Vec<u8> = hex::decode("6352211e").unwrap();
	let mut handle = create_mock_handle_from_input(owner_of_with_0_arguments);
	handle.code_address = H160::from_str("ffffffffffffffffffffffff0000000000000005").unwrap();
	let result = Mock::execute(&mut handle);
	assert!(result.is_err());
	assert_eq!(result.unwrap_err(), revert("input doesn't match expected length"));
}

mod transfer_from {
	use super::*;
	use frame_support::assert_ok;
	use precompile_utils::testing::create_mock_handle;

	#[test]
	fn send_value_as_money_should_fail() {
		impl_precompile_mock_simple!(
			Mock,
			// owner_of result
			Ok(H160::from_str("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap()),
			// token_uri result
			Ok(vec![]),
			// transfer_from result
			Ok(())
		);

		// test data
		let from = H160::repeat_byte(0xAA);
		let to = H160::repeat_byte(0x0);
		let asset_id = 4;
		let contract_address = H160::from_str("ffffffffffffffffffffffff0000000000000005");

		let input_data = EvmDataWriter::new_with_selector(Action::TransferFrom)
			.write(Address(from))
			.write(Address(to))
			.write(U256::from(asset_id))
			.build();

		let mut handle = create_mock_handle(input_data, 0, 1, H160::zero());
		handle.code_address = contract_address.unwrap();
		let result = Mock::execute(&mut handle);
		assert!(result.is_err());
		assert_eq!(result.unwrap_err(), revert("function is not payable"));
	}

	#[test]
	fn sucessful_transfer_should_work() {
		impl_precompile_mock_simple!(
			Mock,
			// owner_of result
			Ok(H160::from_str("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap()),
			// token_uri result
			Ok(vec![]),
			// transfer_from result
			Ok(())
		);

		// test data
		let from = H160::repeat_byte(0xAA);
		let to = H160::repeat_byte(0xBB);
		let asset_id = 4;
		let contract_address = H160::from_str("ffffffffffffffffffffffff0000000000000005");

		let input_data = EvmDataWriter::new_with_selector(Action::TransferFrom)
			.write(Address(from))
			.write(Address(to))
			.write(U256::from(asset_id))
			.build();

		let mut handle = create_mock_handle(
			input_data,
			0,
			0,
			H160::from_str("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap(),
		);
		handle.code_address = contract_address.unwrap();
		assert_ok!(Mock::execute(&mut handle));
	}
	#[test]
	fn unsucessful_transfer_should_fail() {
		impl_precompile_mock_simple!(
			Mock,
			// owner_of result
			Ok(H160::from_str("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap()),
			// token_uri result
			Ok(vec![]),
			// transfer_from result
			Err("this is an error")
		);

		// test data
		let from = H160::repeat_byte(0xAA);
		let to = H160::repeat_byte(0xBB);
		let asset_id = 4;
		let contract_address = H160::from_str("ffffffffffffffffffffffff0000000000000005");

		let input_data = EvmDataWriter::new_with_selector(Action::TransferFrom)
			.write(Address(from))
			.write(Address(to))
			.write(U256::from(asset_id))
			.build();

		let mut handle = create_mock_handle(
			input_data,
			0,
			0,
			H160::from_str("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap(),
		);
		handle.code_address = contract_address.unwrap();
		let result = Mock::execute(&mut handle);
		assert!(result.is_err());
		assert_eq!(result.unwrap_err(), revert("this is an error"));
	}

	#[test]
	fn check_log_selectors() {
		assert_eq!(
			hex::encode(SELECTOR_LOG_TRANSFER_FROM),
			"ddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef"
		);
	}

	#[test]
	fn transfer_from_should_generate_log() {
		impl_precompile_mock_simple!(
			Mock,
			// owner_of result
			Ok(H160::from_str("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap()),
			// token_uri result
			Ok(vec![]),
			// transfer_from result
			Ok(())
		);

		// test data
		let from = H160::repeat_byte(0xAA);
		let to = H160::repeat_byte(0xBB);
		let asset_id = 4;
		let contract_address = H160::from_str("ffffffffffffffffffffffff0000000000000005");

		let input_data = EvmDataWriter::new_with_selector(Action::TransferFrom)
			.write(Address(from))
			.write(Address(to))
			.write(U256::from(asset_id))
			.build();

		let mut handle = create_mock_handle(
			input_data,
			0,
			0,
			H160::from_str("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap(),
		);
		handle.code_address = contract_address.unwrap();
		assert_ok!(Mock::execute(&mut handle));

		let logs = handle.logs;
		assert_eq!(logs.len(), 1);
		assert_eq!(logs[0].address, H160::zero());
		assert_eq!(logs[0].topics.len(), 4);
		assert_eq!(logs[0].topics[0], SELECTOR_LOG_TRANSFER_FROM.into());
		assert_eq!(
			hex::encode(logs[0].topics[1]),
			"000000000000000000000000aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
		);
		assert_eq!(
			hex::encode(logs[0].topics[2]),
			"000000000000000000000000bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
		);
		assert_eq!(
			hex::encode(logs[0].topics[3]),
			"0000000000000000000000000000000000000000000000000000000000000004"
		);
		assert_eq!(logs[0].data, Vec::<u8>::new());
	}
}
#[test]
fn token_uri_should_return_a_string() {
	impl_precompile_mock_simple!(
		Mock,
		Ok(H160::zero()),
		Ok("This is the token URI".to_string().into_bytes()),
		Ok(())
	);

	let input = EvmDataWriter::new_with_selector(Action::TokenURI).write(U256::from(4)).build();
	let mut handle = create_mock_handle_from_input(input);
	handle.code_address = H160::from_str("ffffffffffffffffffffffff0000000000000005").unwrap();

	let result = Mock::execute(&mut handle);
	assert_ok!(
		result,
		succeed(
			EvmDataWriter::new()
				.write(Bytes("This is the token URI".to_string().into_bytes()))
				.build()
		)
	);
}

mod helpers {
	/// Macro to define a precompile mock with custom closures for testing.
	///
	/// This macro creates mock implementations of the `Erc721` trait,
	/// allowing you to test how your code interacts with the precompiled contracts.
	/// You can define a custom closure for the owner_of function.
	///
	/// # Arguments
	///
	/// * `$name`: An identifier to name the precompile mock type.
	/// * `$owner_of_collection`: A closure that takes `collection_id` and `asset_id` and returns a `Result<AccountId, &'static str>`.
	///
	/// # Example
	///
	/// ```
	/// impl_precompile_mock!(
	///     MyMock,
	///     |collection_id, asset_id| { Ok(AccountId::default()) }
	/// );
	/// ```
	#[macro_export]
	macro_rules! impl_precompile_mock {
		($name:ident, $owner_of_collection:expr, $token_uri:expr, $transfer_from:expr) => {
			struct Erc721Mock;

			impl pallet_living_assets_ownership::traits::Erc721<AccountId> for Erc721Mock {
				type Error = &'static str;

				fn owner_of(
					collection_id: CollectionId,
					asset_id: U256,
				) -> Result<AccountId, Self::Error> {
					($owner_of_collection)(collection_id, asset_id)
				}

				fn token_uri(
					collectio_id: CollectionId,
					asset_id: U256,
				) -> Result<Vec<u8>, Self::Error> {
					($token_uri)(collectio_id, asset_id)
				}

				fn transfer_from(
					origin: AccountId,
					collection_id: CollectionId,
					from: AccountId,
					to: AccountId,
					asset_id: U256,
				) -> Result<(), Self::Error> {
					($transfer_from)(origin, collection_id, from, to, asset_id)
				}
			}

			type $name = Erc721Precompile<Erc721Mock>;
		};
	}

	/// Macro to define a precompile mock for testing.
	///
	/// This macro creates mock implementations of the `Erc721` trait,
	/// allowing you to test how your code interacts with the precompiled contracts.
	/// The mock type is named based on the provided identifier, and the implementation uses the provided expression.
	///
	/// # Arguments
	///
	/// * `$name`: An identifier to name the precompile mock type.
	/// * `$owner_of`: An expression that evaluates to a `Result<AccountId, &'static str>`.
	///
	/// # Example
	///
	/// ```
	/// impl_precompile_mock_simple!(Mock, Ok(AccountId::default()));
	/// ```
	#[macro_export]
	macro_rules! impl_precompile_mock_simple {
		($name:ident, $owner_of:expr, $token_uri:expr, $transfer_from:expr) => {
			impl_precompile_mock!(
				$name,
				|_asset_id, _collection_id| { $owner_of },
				|_asset_id, _collection_id| { $token_uri },
				|_origin, _collection_id, _from, _to, _asset_id| { $transfer_from }
			);
		};
	}
}
