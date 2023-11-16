//! LAOS precompile module.

#![cfg_attr(not(feature = "std"), no_std)]
use fp_evm::{Precompile, PrecompileHandle, PrecompileOutput};
use pallet_laos_evolution::{
	address_to_collection_id, traits::EvolutionCollection as EvolutionCollectionT, Slot, TokenId,
};
use parity_scale_codec::Encode;
use precompile_utils::{
	keccak256, revert, revert_dispatch_error, succeed, Address, Bytes, EvmDataWriter, EvmResult,
	FunctionModifier, LogExt, LogsBuilder, PrecompileHandleExt,
};

use sp_core::H160;
use sp_std::{fmt::Debug, marker::PhantomData, vec::Vec};

/// Solidity selector of the MintedWithExternalURI log, which is the Keccak of the Log signature.
pub const SELECTOR_LOG_MINTED_WITH_EXTERNAL_TOKEN_URI: [u8; 32] =
	keccak256!("MintedWithExternalURI(address,uint96,uint256,string)");
/// Solidity selector of the EvolvedWithExternalURI log, which is the Keccak of the Log signature.
pub const SELECTOR_LOG_EVOLVED_WITH_EXTERNAL_TOKEN_URI: [u8; 32] =
	keccak256!("EvolvedWithExternalURI(uint256,string)");

#[precompile_utils_macro::generate_function_selector]
#[derive(Debug, PartialEq)]
pub enum Action {
	/// Get owner of the collection
	Owner = "owner()",
	/// Get tokenURI of the token in collection
	TokenURI = "tokenURI(uint256)",
	/// Mint token
	Mint = "mintWithExternalURI(address,uint96,string)",
	/// Evolve token
	Evolve = "evolveWithExternalURI(uint256,string)",
}

/// Wrapper for the precompile function.
pub struct EvolutionCollectionPrecompile<AddressMapping, AccountId, TokenUri, LaosEvolution>(
	PhantomData<(AddressMapping, AccountId, TokenUri, LaosEvolution)>,
)
where
	AddressMapping: pallet_evm::AddressMapping<AccountId>,
	AccountId: Encode + Debug,
	TokenUri: TryFrom<Vec<u8>>,
	LaosEvolution: EvolutionCollectionT<AccountId, TokenUri>;

impl<AddressMapping, AccountId, TokenUri, LaosEvolution>
	EvolutionCollectionPrecompile<AddressMapping, AccountId, TokenUri, LaosEvolution>
where
	AddressMapping: pallet_evm::AddressMapping<AccountId>,
	AccountId: From<H160> + Into<H160> + Encode + Debug,
	TokenUri: TryFrom<Vec<u8>> + Into<Vec<u8>>,
	LaosEvolution: EvolutionCollectionT<AccountId, TokenUri>,
{
	fn inner_execute(
		handle: &mut impl PrecompileHandle,
		action: &Action,
	) -> EvmResult<PrecompileOutput> {
		let mut input = handle.read_input()?;
		let context = handle.context();

		match action {
			Action::Owner => {
				// collection id is encoded into the contract address
				let collection_id = address_to_collection_id(context.address)
					.map_err(|_| revert("invalid collection address"))?;

				if let Some(owner) = LaosEvolution::collection_owner(collection_id) {
					Ok(succeed(EvmDataWriter::new().write(Address(owner.into())).build()))
				} else {
					Err(revert("collection does not exist"))
				}
			},
			Action::TokenURI => {
				let mut input = handle.read_input()?;
				input.expect_arguments(1)?;

				// collection id is encoded into the contract address
				let collection_id = address_to_collection_id(context.address)
					.map_err(|_| revert("invalid collection address"))?;
				let token_id = input.read::<TokenId>()?;

				if let Some(token_uri) = LaosEvolution::token_uri(collection_id, token_id) {
					Ok(succeed(EvmDataWriter::new().write(Bytes(token_uri.into())).build()))
				} else {
					Err(revert("asset does not exist"))
				}
			},
			Action::Mint => {
				let caller = context.caller;

				input.expect_arguments(3)?;

				// collection id is encoded into the contract address
				let collection_id = address_to_collection_id(context.address)
					.map_err(|_| revert("invalid collection address"))?;
				let to = input.read::<Address>()?.0;
				let slot = input.read::<Slot>()?;
				let token_uri_raw = input.read::<Bytes>()?.0;
				let token_uri = token_uri_raw
					.clone()
					.try_into()
					.map_err(|_| revert("invalid token uri length"))?;

				match LaosEvolution::mint_with_external_uri(
					caller.into(),
					collection_id,
					slot,
					to.into(),
					token_uri,
				) {
					Ok(token_id) => {
						LogsBuilder::new(context.address)
							.log2(
								SELECTOR_LOG_MINTED_WITH_EXTERNAL_TOKEN_URI,
								to,
								EvmDataWriter::new()
									.write(slot)
									.write(token_id)
									.write(Bytes(token_uri_raw))
									.build(),
							)
							.record(handle)?;

						Ok(succeed(EvmDataWriter::new().write(token_id).build()))
					},
					Err(err) => Err(revert_dispatch_error(err)),
				}
			},
			Action::Evolve => {
				let caller = context.caller;

				input.expect_arguments(3)?;

				// collection id is encoded into the contract address
				let collection_id = address_to_collection_id(context.address)
					.map_err(|_| revert("invalid collection address"))?;
				let token_id = input.read::<TokenId>()?;
				let token_uri_raw = input.read::<Bytes>()?.0;
				let token_uri = token_uri_raw
					.clone()
					.try_into()
					.map_err(|_| revert("invalid token uri length"))?;

				match LaosEvolution::evolve_with_external_uri(
					caller.into(),
					collection_id,
					token_id,
					token_uri,
				) {
					Ok(()) => {
						let mut token_id_bytes = [0u8; 32];
						token_id.to_big_endian(&mut token_id_bytes);

						LogsBuilder::new(context.address)
							.log2(
								SELECTOR_LOG_EVOLVED_WITH_EXTERNAL_TOKEN_URI,
								token_id_bytes,
								EvmDataWriter::new().write(Bytes(token_uri_raw)).build(),
							)
							.record(handle)?;

						Ok(succeed(EvmDataWriter::new().write(token_id).build()))
					},
					Err(err) => Err(revert_dispatch_error(err)),
				}
			},
		}
	}
}

impl<AddressMapping, AccountId, TokenUri, LaosEvolution> Precompile
	for EvolutionCollectionPrecompile<AddressMapping, AccountId, TokenUri, LaosEvolution>
where
	AddressMapping: pallet_evm::AddressMapping<AccountId>,
	AccountId: From<H160> + Into<H160> + Encode + Debug,
	TokenUri: TryFrom<Vec<u8>> + Into<Vec<u8>>,
	LaosEvolution: EvolutionCollectionT<AccountId, TokenUri>,
{
	fn execute(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		let selector = handle.read_selector()?;

		handle.check_function_modifier(match selector {
			Action::Mint => FunctionModifier::NonPayable,
			Action::Owner => FunctionModifier::View,
			Action::TokenURI => FunctionModifier::View,
			Action::Evolve => FunctionModifier::NonPayable,
		})?;

		Self::inner_execute(handle, &selector)
	}
}

#[cfg(test)]
mod tests;
