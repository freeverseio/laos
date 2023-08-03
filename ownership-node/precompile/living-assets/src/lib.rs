//! Living Assets precompile module.

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(test, feature(assert_matches))]
use fp_evm::{
	ExitError, ExitSucceed, Precompile, PrecompileFailure, PrecompileHandle, PrecompileOutput,
};
use pallet_living_assets_ownership::LivingAssetsOwnership;
use parity_scale_codec::Encode;
use precompile_utils::{
	succeed, Address, EvmDataWriter, EvmResult, FunctionModifier, PrecompileHandleExt,
};
use sp_arithmetic::traits::BaseArithmetic;
use sp_runtime::{DispatchError, SaturatedConversion};

use scale_info::prelude::format;
use sp_std::{fmt::Debug, marker::PhantomData};

#[precompile_utils_macro::generate_function_selector]
#[derive(Debug, PartialEq)]
pub enum Action {
	/// Create a new collection
	CreateCollection = "createCollection(uint64,address)",
	/// Get owner of the collection
	OwnerOfCollection = "ownerOfCollection(uint64)",

	CreateCollectionReturnId = "createCollection()",
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
			Action::CreateCollection => FunctionModifier::NonPayable,
			Action::CreateCollectionReturnId => FunctionModifier::NonPayable,
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

				match LivingAssets::create_collection(collection_id, owner) {
					Ok(_) => Ok(succeed(EvmDataWriter::new().write(true).build())),
					Err(DispatchError::Other(err)) => Err(PrecompileFailure::Error {
						exit_status: ExitError::Other(sp_std::borrow::Cow::Borrowed(err)),
					}),
					// for the other errors the type or error is returned
					Err(e) => Err(PrecompileFailure::Error {
						exit_status: ExitError::Other(sp_std::borrow::Cow::Owned(format!(
							"{:?}",
							e
						))),
					}),
				}
			},
			Action::CreateCollectionReturnId => match LivingAssets::create_collection2() {
				Ok(collection_id) => Ok(PrecompileOutput {
					exit_status: ExitSucceed::Returned,
					output: collection_id.saturated_into::<u64>().encode(),
				}),
				Err(err) => Err(PrecompileFailure::Error {
					exit_status: ExitError::Other(sp_std::borrow::Cow::Borrowed(err)),
				}),
			},
		}
	}
}

#[cfg(test)]
mod tests;
