//! Living Assets precompile module.

#![cfg_attr(not(feature = "std"), no_std)]
use fp_evm::{Precompile, PrecompileHandle, PrecompileOutput};
use pallet_living_assets_ownership::{
	collection_id_to_address, traits::CollectionManager, BaseURIOf, CollectionId,
};
use parity_scale_codec::Encode;
use precompile_utils::{
	keccak256, revert, succeed, Address, Bytes, EvmDataWriter, EvmResult, FunctionModifier, LogExt,
	LogsBuilder, PrecompileHandleExt,
};
use sp_runtime::SaturatedConversion;

use sp_std::{fmt::Debug, marker::PhantomData, vec::Vec};

/// Solidity selector of the CreateCollection log, which is the Keccak of the Log signature.
pub const SELECTOR_LOG_CREATE_COLLECTION: [u8; 32] = keccak256!("CreateCollection(address)");

#[precompile_utils_macro::generate_function_selector]
#[derive(Debug, PartialEq)]
pub enum Action {
	/// Create collection
	CreateCollection = "createCollection(string)",
}

/// Wrapper for the precompile function.
pub struct CollectionManagerPrecompile<AddressMapping, AccountId, LivingAssets>(
	PhantomData<(AddressMapping, AccountId, LivingAssets)>,
)
where
	AddressMapping: pallet_evm::AddressMapping<AccountId>,
	AccountId: Encode + Debug,
	LivingAssets: pallet_living_assets_ownership::Config
		+ CollectionManager<AccountId, BaseURIOf<LivingAssets>>;

impl<AddressMapping, AccountId, LivingAssets> Precompile
	for CollectionManagerPrecompile<AddressMapping, AccountId, LivingAssets>
where
	AddressMapping: pallet_evm::AddressMapping<AccountId>,
	AccountId: Encode + Debug,
	LivingAssets: pallet_living_assets_ownership::Config
		+ CollectionManager<AccountId, BaseURIOf<LivingAssets>>,
{
	fn execute(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		let selector = handle.read_selector()?;

		handle.check_function_modifier(match selector {
			Action::CreateCollection => FunctionModifier::NonPayable,
		})?;

		match selector {
			Action::CreateCollection => {
				let mut input = handle.read_input()?;
				input.expect_arguments(1)?;

				let base_uri_bytes: Vec<u8> = match input.read::<Bytes>() {
					Ok(bytes) => bytes.into(),
					Err(e) => return Err(e),
				};

				let base_uri = match base_uri_bytes.try_into() {
					Ok(value) => value,
					Err(_) => return Err(revert("base_uri too long")),
				};

				let caller = handle.context().caller;
				let owner = AddressMapping::into_account_id(caller);

				match LivingAssets::create_collection(owner, base_uri) {
					Ok(collection_id) => {
						let collection_address = collection_id_to_address(
							collection_id.saturated_into::<CollectionId>(),
						);

						LogsBuilder::new(handle.context().address)
							.log2(SELECTOR_LOG_CREATE_COLLECTION, collection_address, Vec::new())
							.record(handle)?;

						Ok(succeed(EvmDataWriter::new().write(Address(collection_address)).build()))
					},
					Err(err) => Err(revert(err)),
				}
			},
		}
	}
}

#[cfg(test)]
mod tests;
