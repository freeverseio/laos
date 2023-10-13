#![cfg_attr(not(feature = "std"), no_std)]
use fp_evm::{Precompile, PrecompileHandle, PrecompileOutput};
use frame_support::traits::tokens::nonfungibles_v2::{Create, Inspect, Mutate, Transfer};
use pallet_living_assets_ownership::address_to_collection_id;
use precompile_utils::{
	keccak256, revert, revert_dispatch_error, succeed, Address, Bytes, EvmDataWriter, EvmResult,
	FunctionModifier, LogExt, LogsBuilder, PrecompileHandleExt,
};

use ownership_parachain_primitives::nfts::*;
use parity_scale_codec::alloc::string::ToString;
use sp_core::{H160, H256, U256};
use sp_runtime::traits::Convert;
use sp_std::{fmt::Debug, marker::PhantomData, vec, vec::Vec};

/// Represents a mapping between `AssetId` and `AccountId`.
/// This struct provides functionalities to convert an `AssetId` (represented by `U256`) into an
/// `AccountId`.
pub struct AssetIdToInitialOwner;
impl Convert<U256, H160> for AssetIdToInitialOwner {
	fn convert(asset_id: U256) -> H160 {
		let mut bytes = [0u8; 20];
		let asset_id_bytes: [u8; 32] = asset_id.into();
		bytes.copy_from_slice(&asset_id_bytes[asset_id_bytes.len() - 20..]);

		bytes.into()
	}
}

/// Solidity selector of the TransferFrom log, which is the Keccak of the Log signature.
pub const SELECTOR_LOG_TRANSFER_FROM: [u8; 32] = keccak256!("Transfer(address,address,uint256)");

#[precompile_utils_macro::generate_function_selector]
#[derive(Debug, PartialEq)]
pub enum Action {
	/// Get token URI
	TokenURI = "tokenURI(uint256)",
	/// Owner of
	OwnerOf = "ownerOf(uint256)",
	/// Transfer from
	TransferFrom = "transferFrom(address,address,uint256)",
}

/// Wrapper for the precompile function.
pub struct Erc721Precompile<AccountId, CollectionId, ItemId, AssetManager>(
	PhantomData<(AccountId, CollectionId, ItemId, AssetManager)>,
);

impl<AccountId, CollectionId, ItemId, AssetManager> Precompile
	for Erc721Precompile<AccountId, CollectionId, ItemId, AssetManager>
where
	AccountId: Into<H160> + From<H160> + Into<[u8; 20]> + PartialEq,
	CollectionId: From<u64> + Into<u64>,
	ItemId: From<U256> + Into<U256>,
	AssetManager: Create<AccountId, CollectionConfig>
		+ Mutate<AccountId, ItemConfig>
		+ Inspect<AccountId, ItemId = ItemId, CollectionId = CollectionId>
		+ Transfer<AccountId>,
{
	fn execute(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		// collection id is encoded into the contract address
		let collection_id = address_to_collection_id(handle.code_address())
			.map_err(|_| revert("invalid collection address"))?
			.into();

		let selector = handle.read_selector()?;

		handle.check_function_modifier(match selector {
			Action::TokenURI => FunctionModifier::View,
			Action::OwnerOf => FunctionModifier::View,
			Action::TransferFrom => FunctionModifier::NonPayable,
		})?;

		match selector {
			Action::TokenURI => Self::token_uri(collection_id, handle),
			Action::OwnerOf => Self::owner_of(collection_id, handle),
			Action::TransferFrom => Self::transfer_from(collection_id, handle),
		}
	}
}

impl<AccountId, CollectionId, ItemId, AssetManager>
	Erc721Precompile<AccountId, CollectionId, ItemId, AssetManager>
where
	AccountId: Into<H160> + From<H160> + Into<[u8; 20]> + PartialEq,
	CollectionId: From<u64> + Into<u64>,
	ItemId: From<U256> + Into<U256>,
	AssetManager: Create<AccountId, CollectionConfig>
		+ Mutate<AccountId, ItemConfig>
		+ Inspect<AccountId, ItemId = ItemId, CollectionId = CollectionId>
		+ Transfer<AccountId>,
{
	fn owner_of(
		collection_id: CollectionId,
		handle: &mut impl PrecompileHandle,
	) -> EvmResult<PrecompileOutput> {
		let mut input = handle.read_input()?;
		input.expect_arguments(1)?;

		let asset_id: U256 = input.read()?;

		let owner = if let Some(owner) = AssetManager::owner(&collection_id, &asset_id.into()) {
			owner
		} else {
			AssetIdToInitialOwner::convert(asset_id.clone()).into()
		};
		Ok(succeed(EvmDataWriter::new().write(Address(owner.into())).build()))
	}

	fn token_uri(
		collection_id: CollectionId,
		handle: &mut impl PrecompileHandle,
	) -> EvmResult<PrecompileOutput> {
		let mut input = handle.read_input()?;
		input.expect_arguments(1)?;

		let asset_id: U256 = input.read()?;

		let base_uri = AssetManager::collection_attribute(&collection_id, b"baseURI")
			.ok_or(revert("no base URI set"))?;

		// concatenate base_uri with asset_id
		let mut token_uri = base_uri.to_vec();
		token_uri.push(b'/');
		token_uri.extend_from_slice(asset_id.to_string().as_bytes());

		Ok(succeed(EvmDataWriter::new().write(Bytes(token_uri)).build()))
	}

	fn transfer_from(
		collection_id: CollectionId,
		handle: &mut impl PrecompileHandle,
	) -> EvmResult<PrecompileOutput> {
		// get input data
		let mut input = handle.read_input()?;
		input.expect_arguments(3)?;
		let from = input.read::<Address>()?;
		let to = input.read::<Address>()?;
		let asset_id: U256 = input.read()?;
		let mut asset_id_big_endian = [0u8; 32];
		asset_id.to_big_endian(&mut asset_id_big_endian);

		// validate dest
		if to.0 == H160::zero() {
			return Err(revert("cannot transfer to zero address"))
		}
		if to.0 == from.0 {
			return Err(revert("cannot transfer to self"))
		}

		// validate owner here
		let who = handle.context().caller.into();
		let owner = if let Some(owner) = AssetManager::owner(&collection_id, &asset_id.into()) {
			owner
		} else {
			AssetIdToInitialOwner::convert(asset_id.clone()).into()
		};

		if owner == who {
			AssetManager::transfer(&collection_id, &asset_id.into(), &to.0.into())
				.map_err(revert_dispatch_error)?;

			LogsBuilder::new(handle.context().address)
				.log4(
					SELECTOR_LOG_TRANSFER_FROM,
					from.0,
					to.0,
					H256::from_slice(asset_id_big_endian.as_slice()),
					Vec::new(),
				)
				.record(handle)?;

			Ok(succeed(vec![]))
		} else {
			Err(revert("not owner"))
		}
	}
}

#[cfg(test)]
mod tests;
