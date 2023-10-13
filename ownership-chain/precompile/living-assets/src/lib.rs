//! Living Assets precompile module.

#![cfg_attr(not(feature = "std"), no_std)]
use fp_evm::{Precompile, PrecompileHandle, PrecompileOutput};
use frame_support::traits::tokens::nonfungibles_v2::{Create, Mutate};
use ownership_parachain_primitives::nfts::*;
use pallet_living_assets_ownership::collection_id_to_address;
use pallet_nfts::{CollectionSettings, ItemSettings, MintSettings};
use parity_scale_codec::Encode;
use precompile_utils::{
	keccak256, revert_dispatch_error, succeed, Address, Bytes, EvmDataWriter, EvmResult,
	FunctionModifier, LogExt, LogsBuilder, PrecompileHandleExt,
};

use sp_core::U256;
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
pub struct CollectionManagerPrecompile<
	AddressMapping,
	AccountId,
	CollectionId,
	ItemId,
	CollectionManager,
>(PhantomData<(AddressMapping, AccountId, CollectionId, ItemId, CollectionManager)>)
where
	AddressMapping: pallet_evm::AddressMapping<AccountId>,
	AccountId: Encode + Debug,
	CollectionId: From<u64> + Into<u64>,
	ItemId: From<U256> + Into<U256>,
	CollectionManager: Create<AccountId, CollectionConfig, CollectionId = CollectionId, ItemId = ItemId>
		+ Mutate<AccountId, ItemConfig>;

impl<AddressMapping, AccountId, CollectionId, ItemId, CollectionManager> Precompile
	for CollectionManagerPrecompile<AddressMapping, AccountId, CollectionId, ItemId, CollectionManager>
where
	AddressMapping: pallet_evm::AddressMapping<AccountId>,
	AccountId: Encode + Debug,
	CollectionId: From<u64> + Into<u64>,
	ItemId: From<U256> + Into<U256>,
	CollectionManager: Create<AccountId, CollectionConfig, CollectionId = CollectionId, ItemId = ItemId>
		+ Mutate<AccountId, ItemConfig>,
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
					Ok(bytes) => bytes.0,
					Err(e) => return Err(e),
				};

				let caller = handle.context().caller;
				let owner = AddressMapping::into_account_id(caller);

				// Customize the collection config as we need.
				let config = CollectionConfig {
					settings: CollectionSettings::all_enabled(),
					max_supply: None,
					mint_settings: MintSettings {
						mint_type: pallet_nfts::MintType::Public,
						price: None,
						start_block: None,
						end_block: None,
						default_item_settings: ItemSettings::all_enabled(),
					},
				};

				match CollectionManager::create_collection(&owner, &owner, &config) {
					Ok(collection_id) => {
						match CollectionManager::set_collection_attribute(
							&collection_id,
							b"baseURI",
							&base_uri_bytes,
						) {
							Ok(_) => {
								let collection_address =
									collection_id_to_address(collection_id.into());

								LogsBuilder::new(handle.context().address)
									.log2(
										SELECTOR_LOG_CREATE_COLLECTION,
										collection_address,
										Vec::new(),
									)
									.record(handle)?;

								Ok(succeed(
									EvmDataWriter::new().write(Address(collection_address)).build(),
								))
							},
							Err(err) => Err(revert_dispatch_error(err)),
						}
					},
					Err(err) => Err(revert_dispatch_error(err)),
				}
			},
		}
	}
}

#[cfg(test)]
mod tests;
