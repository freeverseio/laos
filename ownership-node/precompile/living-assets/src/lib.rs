//! Living Assets precompile module.

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(test, feature(assert_matches))]
use fp_evm::{
	ExitError, ExitSucceed, Precompile, PrecompileFailure, PrecompileHandle, PrecompileOutput,
};
use pallet_living_assets_ownership::traits::CollectionManager;
use parity_scale_codec::Encode;
use precompile_utils::{EvmResult, FunctionModifier, PrecompileHandleExt};
use sp_arithmetic::traits::BaseArithmetic;
use sp_runtime::SaturatedConversion;

use sp_core::H160;
use sp_std::{fmt::Debug, marker::PhantomData};

#[precompile_utils_macro::generate_function_selector]
#[derive(Debug, PartialEq)]
pub enum Action {
	/// Create collection
	CreateCollection = "createCollection()",
}

/// Wrapper for the precompile function.
pub struct CollectionManagerPrecompile<AddressMapping, AccountId, CollectionId, LivingAssets>(
	PhantomData<(AddressMapping, AccountId, CollectionId, LivingAssets)>,
)
where
	AddressMapping: pallet_evm::AddressMapping<AccountId>,
	AccountId: Encode + Debug,
	CollectionId: BaseArithmetic + Debug,
	LivingAssets: CollectionManager<AccountId, CollectionId>;

impl<AddressMapping, AccountId, CollectionId, LivingAssets> Precompile
	for CollectionManagerPrecompile<AddressMapping, AccountId, CollectionId, LivingAssets>
where
	AddressMapping: pallet_evm::AddressMapping<AccountId>,
	AccountId: Encode + Debug,
	CollectionId: BaseArithmetic + Debug,
	LivingAssets: CollectionManager<AccountId, CollectionId>,
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
					Ok(collection_id) => Ok(PrecompileOutput {
						exit_status: ExitSucceed::Returned,
						output: collection_id_to_address(collection_id.saturated_into::<u64>())
							.encode(),
					}),
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
pub fn collection_id_to_address(collection_id: u64) -> H160 {
	let mut address = H160::from_low_u64_be(collection_id);
	address.0[0] |= 0x80; // Set the first bit to 1
	address
}

#[cfg(test)]
mod tests;
