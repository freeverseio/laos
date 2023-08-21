#![cfg_attr(not(feature = "std"), no_std)]
use fp_evm::{Precompile, PrecompileHandle, PrecompileOutput};
use pallet_living_assets_ownership::{address_to_collection_id, CollectionId};
use precompile_utils::{
	revert, succeed, Address, Bytes, EvmDataWriter, EvmResult, FunctionModifier,
	PrecompileHandleExt,
};
use sp_core::U256;
use sp_std::{fmt::Debug, marker::PhantomData};

#[precompile_utils_macro::generate_function_selector]
#[derive(Debug, PartialEq)]
pub enum Action {
	/// Get token URI
	TokenURI = "tokenURI(uint256)",
	/// Owner of
	OwnerOf = "ownerOf(uint256)",
}

/// Wrapper for the precompile function.
pub struct Erc721Precompile<AssetManager>(PhantomData<AssetManager>);

impl<AssetManager> Precompile for Erc721Precompile<AssetManager>
where
	AssetManager: pallet_living_assets_ownership::traits::Erc721,
{
	fn execute(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		// collection id is encoded into the contract address
		let collection_id = address_to_collection_id(handle.code_address())
			.map_err(|_| revert("invalid collection address"))?;

		let selector = handle.read_selector()?;

		handle.check_function_modifier(match selector {
			Action::TokenURI => FunctionModifier::View,
			Action::OwnerOf => FunctionModifier::View,
		})?;

		match selector {
			Action::TokenURI => Self::token_uri(collection_id, handle),
			Action::OwnerOf => Self::owner_of(collection_id, handle),
		}
	}
}

impl<AssetManager> Erc721Precompile<AssetManager>
where
	AssetManager: pallet_living_assets_ownership::traits::Erc721,
{
	fn owner_of(
		collection_id: CollectionId,
		handle: &mut impl PrecompileHandle,
	) -> EvmResult<PrecompileOutput> {
		let mut input = handle.read_input()?;
		input.expect_arguments(1)?;

		let asset_id: U256 = input.read()?;

		let owner = AssetManager::owner_of(collection_id, asset_id).map_err(|err| revert(err))?;
		Ok(succeed(EvmDataWriter::new().write(Address(owner)).build()))
	}

	fn token_uri(
		collection_id: CollectionId,
		handle: &mut impl PrecompileHandle,
	) -> EvmResult<PrecompileOutput> {
		let mut input = handle.read_input()?;
		input.expect_arguments(1)?;

		let asset_id: U256 = input.read()?;

		let uri = AssetManager::token_uri(collection_id, asset_id).map_err(|err| revert(err))?;
		Ok(succeed(EvmDataWriter::new().write(Bytes(uri)).build()))
	}
}

#[cfg(test)]
mod tests;
