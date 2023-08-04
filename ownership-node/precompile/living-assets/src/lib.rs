//! Living Assets precompile module.

#![cfg_attr(not(feature = "std"), no_std)]
use fp_evm::{
	ExitError, ExitSucceed, Precompile, PrecompileFailure, PrecompileHandle, PrecompileOutput,
};
use pallet_living_assets_ownership::{traits::CollectionManager, CollectionId};
use parity_scale_codec::Encode;
use precompile_utils::{EvmResult, FunctionModifier, PrecompileHandleExt};
use sp_runtime::SaturatedConversion;

use sp_std::{fmt::Debug, marker::PhantomData};

#[precompile_utils_macro::generate_function_selector]
#[derive(Debug, PartialEq)]
pub enum Action {
	/// Get owner of the collection
	OwnerOfCollection = "ownerOfCollection(uint64)",
	/// Create collection
	CreateCollection = "createCollection()",
}

/// Wrapper for the precompile function.
pub struct CollectionManagerPrecompile<AddressMapping, AccountId, LivingAssets>(
	PhantomData<(AddressMapping, AccountId, LivingAssets)>,
)
where
	AddressMapping: pallet_evm::AddressMapping<AccountId>,
	AccountId: Encode + Debug,
	LivingAssets: CollectionManager<AccountId>;

impl<AddressMapping, AccountId, LivingAssets> Precompile
	for CollectionManagerPrecompile<AddressMapping, AccountId, LivingAssets>
where
	AddressMapping: pallet_evm::AddressMapping<AccountId>,
	AccountId: Encode + Debug,
	LivingAssets: CollectionManager<AccountId>,
{
	fn execute(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		let selector = handle.read_selector()?;

		handle.check_function_modifier(match selector {
			Action::OwnerOfCollection => FunctionModifier::View,
			Action::CreateCollection => FunctionModifier::NonPayable,
		})?;

		match selector {
			// read storage
			Action::OwnerOfCollection => {
				let mut input = handle.read_input()?;
				input.expect_arguments(1)?;

				// TODO check: maybe we won't saturate
				if let Some(owner) = LivingAssets::owner_of_collection(
					input.read::<CollectionId>()?.saturated_into(),
				) {
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
			Action::CreateCollection => {
				let caller = handle.context().caller;
				let owner = AddressMapping::into_account_id(caller);

				match LivingAssets::create_collection(owner) {
					Ok(collection_id) => Ok(PrecompileOutput {
						exit_status: ExitSucceed::Returned,
						// TODO check if this is correct: maybe we won't saturate
						output: collection_id.saturated_into::<CollectionId>().encode(),
					}),
					Err(err) => Err(PrecompileFailure::Error {
						exit_status: ExitError::Other(sp_std::borrow::Cow::Borrowed(err)),
					}),
				}
			},
		}
	}
}

#[cfg(test)]
mod tests;
