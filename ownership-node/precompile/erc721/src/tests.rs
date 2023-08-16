use core::str::FromStr;

use super::*;
use pallet_living_assets_ownership::{traits::Erc721Error, CollectionId};
use precompile_utils::testing::create_mock_handle_from_input;
use sp_core::{H160, U256};

type AccountId = H160;
type AddressMapping = pallet_evm::IdentityAddressMapping;

#[test]
fn check_selectors() {
	assert_eq!(Action::OwnerOf as u32, 0x6352211E);
	assert_eq!(Action::TokenURI as u32, 0xC87B56DD);
}

#[test]
fn owner_of_asset_should_return_an_address() {
	impl_precompile_mock_simple!(
		Mock,
		Ok(H160::from_str("ff00000000000000000000000000000012345678").unwrap())
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
	impl_precompile_mock_simple!(Mock, Err(Erc721Error::UnexistentCollection));

	let owner_of_asset_4 =
		hex::decode("6352211e0000000000000000000000000000000000000000000000000000000000000004")
			.unwrap();
	let mut handle = create_mock_handle_from_input(owner_of_asset_4);
	handle.code_address = H160::from_str("ffffffffffffffffffffffff0000000000000005").unwrap();
	let result = Mock::execute(&mut handle);
	assert!(result.is_err());
	assert_eq!(result.unwrap_err(), revert(Erc721Error::UnexistentCollection));
}

#[test]
fn invalid_contract_address_should_error() {
	impl_precompile_mock_simple!(Mock, Ok(H160::zero()));

	let mut handle = create_mock_handle_from_input(Vec::new());
	handle.code_address = H160::zero();
	let result = Mock::execute(&mut handle);
	assert!(result.is_err());
	assert_eq!(result.unwrap_err(), revert("tried to parse selector out of bounds"),);
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
		($name:ident, $owner_of_collection:expr) => {
			struct Erc721Mock;

			impl pallet_living_assets_ownership::traits::Erc721 for Erc721Mock {
				fn owner_of(
					collectio_id: CollectionId,
					asset_id: U256,
				) -> Result<AccountId, Erc721Error> {
					($owner_of_collection)(collectio_id, asset_id)
				}
			}

			type $name = Erc721Precompile<AddressMapping, AccountId, Erc721Mock>;
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
	/// * `$owner_of_collection`: An expression that evaluates to a `Result<AccountId, &'static str>`.
	///
	/// # Example
	///
	/// ```
	/// impl_precompile_mock_simple!(Mock, Ok(AccountId::default()));
	/// ```
	#[macro_export]
	macro_rules! impl_precompile_mock_simple {
		($name:ident, $owner_of_collection:expr) => {
			impl_precompile_mock!($name, |_asset_id, _collection_id| { $owner_of_collection });
		};
	}
}
