//! Living Assets precompile module.

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(test, feature(assert_matches))]
use fp_evm::{ExitError, ExitSucceed, PrecompileFailure, PrecompileHandle, PrecompileOutput, Precompile};
use pallet_living_assets_ownership::LivingAssetsOwnership;
use parity_scale_codec::Encode;
use precompile_utils::{
	succeed, Address, EvmDataWriter, EvmResult, FunctionModifier, PrecompileHandleExt,
};
use sp_arithmetic::traits::BaseArithmetic;
use sp_runtime::SaturatedConversion;

use sp_std::{fmt::Debug, marker::PhantomData};

#[precompile_utils_macro::generate_function_selector]
#[derive(Debug, PartialEq)]
pub enum Action {
	/// Create a new collection
	CreateCollection = "createCollection(uint64,address)",
	/// Get owner of the collection
	OwnerOfCollection = "ownerOfCollection(uint64)",
}

/// Wrapper for the precompile function.
pub struct LivingAssetsOwnershipPrecompile<AddressMapping, AccountId, CollectionId, LivingAssets>(
	PhantomData<(AddressMapping, AccountId, CollectionId, LivingAssets)>,
)
where
	AddressMapping: pallet_evm::AddressMapping<AccountId>,
	AccountId: Encode + Debug,
	CollectionId: BaseArithmetic + Debug,
	LivingAssets: LivingAssetsOwnership<AccountId, CollectionId>;

impl<AddressMapping, AccountId, CollectionId, LivingAssets>
	LivingAssetsOwnershipPrecompile<AddressMapping, AccountId, CollectionId, LivingAssets>
where
	AddressMapping: pallet_evm::AddressMapping<AccountId>,
	AccountId: Encode + Debug,
	CollectionId: BaseArithmetic + Debug,
	LivingAssets: LivingAssetsOwnership<AccountId, CollectionId>,
{
	#[allow(clippy::new_without_default)]
	pub fn new() -> Self {
		Self(PhantomData)
	}
}

impl<AddressMapping, AccountId, CollectionId, LivingAssets> Precompile
	for LivingAssetsOwnershipPrecompile<AddressMapping, AccountId, CollectionId, LivingAssets>
where
	AddressMapping: pallet_evm::AddressMapping<AccountId>,
	AccountId: Encode + Debug,
	CollectionId: BaseArithmetic + Debug,
	LivingAssets: LivingAssetsOwnership<AccountId, CollectionId>,
{
	fn execute(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		let selector = handle.read_selector()?;

		handle.check_function_modifier(match selector {
			Action::OwnerOfCollection => FunctionModifier::View,
			_ => FunctionModifier::NonPayable,
		})?;

		match selector {
			// read storage
			Action::OwnerOfCollection => {
				let mut input = handle.read_input()?;
				input.expect_arguments(1)?;

				if let Some(owner) =
					LivingAssets::owner_of_collection(input.read::<u64>()?.saturated_into())
				{
					Ok(PrecompileOutput {
						exit_status: ExitSucceed::Returned,
						output: owner.encode(),
					})
				} else {
					Ok(PrecompileOutput {
						exit_status: ExitSucceed::Stopped,
						output: sp_std::vec::Vec::new(),
					})
				}
			},
			// write storage
			Action::CreateCollection => {
				let mut input = handle.read_input()?;
				input.expect_arguments(2)?;

				let collection_id = input.read::<u64>()?.saturated_into();
				let owner = AddressMapping::into_account_id(input.read::<Address>()?.0);

				if LivingAssets::create_collection(collection_id, owner).is_err() {
					return Err(PrecompileFailure::Error {
						exit_status: ExitError::Other(sp_std::borrow::Cow::Borrowed(
							"Could net create collection",
						)),
					})
				}

				Ok(succeed(EvmDataWriter::new().write(true).build()))
			},
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use pallet_evm_test_vector_support::{ MockHandle, test_precompile_test_vectors };
	use sp_core::H160;
	use sp_runtime::DispatchResult;
	use evm::Context;

	#[test]
	fn check_selectors() {
		assert_eq!(Action::CreateCollection as u32, 0x1EAF2516);
		assert_eq!(Action::OwnerOfCollection as u32, 0xFB34AE53);
	}

	macro_rules! define_precompile_mock {
    ($create_collection_result:expr, $owner_of_collection_result:expr) => {
        type AccountId = H160;
        type CollectionId = u64;
        type AddressMapping = pallet_evm::IdentityAddressMapping;

        struct CollectionManagerMock;

        impl pallet_living_assets_ownership::LivingAssetsOwnership<AccountId, CollectionId>
            for CollectionManagerMock
        {
            fn create_collection(_collection_id: CollectionId, _who: AccountId) -> DispatchResult {
                $create_collection_result
            }

            fn owner_of_collection(_collection_id: CollectionId) -> Option<AccountId> {
                $owner_of_collection_result
            }
        }

        type PrecompileMock = LivingAssetsOwnershipPrecompile<
            AddressMapping,
            AccountId,
            CollectionId,
            CollectionManagerMock,
        >;
    };
}

	#[test]
	fn check_create_collection() -> Result<(), String> {
		define_precompile_mock!(Ok(()), Some(H160::zero()));
		test_precompile_test_vectors::<PrecompileMock>("testdata/living_assets_ownership.json")?;
		Ok(())
	}

	#[test]
	fn check_owner_of() -> Result<(), String> {
		define_precompile_mock!(Ok(()), Some(H160::zero()));
		test_precompile_test_vectors::<PrecompileMock>("testdata/owner_of.json")?;
		Ok(())
	}

	fn create_mock_handle(input: &str, cost: Option<u64>) -> MockHandle {
		let i: Vec<u8> = hex::decode(input).expect("todo");

		let context: Context = Context {
			address: Default::default(),
			caller: Default::default(),
			apparent_value: From::from(0),
		};

		MockHandle::new(i, cost, context)
	}

	#[test]
	fn test_directly() {
		define_precompile_mock!(Ok(()), Some(H160::zero()));

		let create_collection_0 = "1eaf25160000000000000000000000000000000000000000000000000000000000000000000000000000000000000000b7469c43535c826e29c30d25a9f3a035759cf132";
		let mut handle = create_mock_handle(create_collection_0, None);
		let result = PrecompileMock::execute(&mut handle);
		assert!(result.is_ok());
	}
}
