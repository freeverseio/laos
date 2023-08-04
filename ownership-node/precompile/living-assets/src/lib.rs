//! Living Assets precompile module.

#![cfg_attr(not(feature = "std"), no_std)]
use fp_evm::{
	ExitError, Precompile, PrecompileFailure, PrecompileHandle, PrecompileOutput,
};
use pallet_living_assets_ownership::{traits::CollectionManager, CollectionId};
use parity_scale_codec::Encode;
use precompile_utils::{
	keccak256, succeed, EvmResult, FunctionModifier, LogExt, LogsBuilder, PrecompileHandleExt,
};
use sp_runtime::SaturatedConversion;

use sp_core::H160;
use sp_std::{fmt::Debug, marker::PhantomData, vec::Vec};

/// Solidity selector of the CreateCollection log, which is the Keccak of the Log signature.
pub const SELECTOR_LOG_CREATE_COLLECTION: [u8; 32] = keccak256!("CreateCollection(address)");

#[precompile_utils_macro::generate_function_selector]
#[derive(Debug, PartialEq)]
pub enum Action {
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
			Action::CreateCollection => FunctionModifier::NonPayable,
		})?;

		match selector {
			Action::CreateCollection => {
				let caller = handle.context().caller;
				let owner = AddressMapping::into_account_id(caller);

				match LivingAssets::create_collection(owner) {
					Ok(collection_id) => {
						let collection_address = collection_id_to_address(
							collection_id.saturated_into::<CollectionId>(),
						);

						LogsBuilder::new(handle.context().address)
							.log2(SELECTOR_LOG_CREATE_COLLECTION, collection_address, Vec::new())
							.record(handle)?;

						Ok(succeed(collection_address.encode()))
					},
					Err(err) => Err(PrecompileFailure::Error {
						exit_status: ExitError::Other(sp_std::borrow::Cow::Borrowed(err)),
					}),
				}
			},
		}
	}
}

/// Converts a `u64` collection ID to an `H160` address.
///
/// This function takes a `u64` collection ID and converts it into an `H160` address by using the
/// `from_low_u64_be` method to convert the `u64` value into the lower 64 bits of the `H160`.
/// Additionally, the function sets the first bit of the resulting `H160` to 1, which can be used to
/// distinguish addresses created by this function from other addresses.
pub fn collection_id_to_address(collection_id: CollectionId) -> H160 {
	let mut address = H160::from_low_u64_be(collection_id);
	address.0[0] |= 0x80; // Set the first bit to 1
	address
}

#[cfg(test)]
mod tests;
