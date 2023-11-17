//! LAOS precompile module.

#![cfg_attr(not(feature = "std"), no_std)]
use fp_evm::{Precompile, PrecompileHandle, PrecompileOutput};
use laos_precompile_utils::{
	keccak256, revert_dispatch_error, succeed, Address, Bytes, EvmDataReader, EvmDataWriter,
	EvmResult, FunctionModifier, LogExt, LogsBuilder, PrecompileHandleExt,
};
use pallet_laos_evolution::{
	address_to_collection_id, traits::EvolutionCollection as EvolutionCollectionT,
	Pallet as LaosEvolution, Slot, TokenId, TokenUriOf,
};
use parity_scale_codec::Encode;
use precompile_utils::solidity::revert::revert;

use sp_core::H160;
use sp_std::{fmt::Debug, marker::PhantomData};

/// Solidity selector of the MintedWithExternalURI log, which is the Keccak of the Log signature.
pub const SELECTOR_LOG_MINTED_WITH_EXTERNAL_TOKEN_URI: [u8; 32] =
	keccak256!("MintedWithExternalURI(address,uint96,uint256,string)");
/// Solidity selector of the EvolvedWithExternalURI log, which is the Keccak of the Log signature.
pub const SELECTOR_LOG_EVOLVED_WITH_EXTERNAL_TOKEN_URI: [u8; 32] =
	keccak256!("EvolvedWithExternalURI(uint256,string)");

#[laos_precompile_utils_macro::generate_function_selector]
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

impl<Runtime> Precompile for EvolutionCollectionPrecompile<Runtime>
where
	Runtime: pallet_laos_evolution::Config,
	Runtime::AccountId: From<H160> + Into<H160> + Encode + Debug,
	LaosEvolution<Runtime>: EvolutionCollectionT<Runtime::AccountId, TokenUriOf<Runtime>>,
{
	fn execute(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		let selector = handle.read_selector()?;

		handle.check_function_modifier(match selector {
			Action::Mint => FunctionModifier::NonPayable,
			Action::Owner => FunctionModifier::View,
			Action::TokenURI => FunctionModifier::View,
			Action::Evolve => FunctionModifier::NonPayable,
		})?;

		match selector {
			Action::Owner => Self::owner(handle),
			Action::TokenURI => Self::token_uri(handle),
			Action::Mint => Self::mint(handle),
			Action::Evolve => Self::evolve(handle),
		}
	}
}

/// Wrapper for the precompile function.
pub struct EvolutionCollectionPrecompile<Runtime>(PhantomData<Runtime>)
where
	Runtime: pallet_laos_evolution::Config;

impl<Runtime> EvolutionCollectionPrecompile<Runtime>
where
	Runtime: pallet_laos_evolution::Config,
	Runtime::AccountId: From<H160> + Into<H160> + Encode + Debug,
	LaosEvolution<Runtime>: EvolutionCollectionT<Runtime::AccountId, TokenUriOf<Runtime>>,
{
	fn owner(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		let context = handle.context();

		// collection id is encoded into the contract address
		let collection_id = address_to_collection_id(context.address)
			.map_err(|_| revert("invalid collection address"))?;

		if let Some(owner) = LaosEvolution::<Runtime>::collection_owner(collection_id) {
			Ok(succeed(EvmDataWriter::new().write(Address(owner.into())).build()))
		} else {
			Err(revert("collection does not exist"))
		}
	}

	fn token_uri(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		let mut input = handle.read_input()?;
		let context = handle.context();
		input.expect_arguments(1)?;

		// collection id is encoded into the contract address
		let collection_id = address_to_collection_id(context.address)
			.map_err(|_| revert("invalid collection address"))?;
		let token_id = input.read::<TokenId>()?;

		if let Some(token_uri) = LaosEvolution::<Runtime>::token_uri(collection_id, token_id) {
			Ok(succeed(EvmDataWriter::new().write(Bytes(token_uri.into())).build()))
		} else {
			Err(revert("asset does not exist"))
		}
	}

	fn mint(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		let mut input = handle.read_input()?;
		let context = handle.context();
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

		match LaosEvolution::<Runtime>::mint_with_external_uri(
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
	}

	fn evolve(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		let mut input = handle.read_input()?;
		let context = handle.context();

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

		match LaosEvolution::<Runtime>::evolve_with_external_uri(
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
	}
}

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;
