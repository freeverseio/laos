// Copyright 2023-2024 LAOS Chain Foundation
// This file is part of LAOS.

// LAOS is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// LAOS is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with LAOS.  If not, see <http://www.gnu.org/licenses/>.

//! LAOS precompile module.

#![cfg_attr(not(feature = "std"), no_std)]
use fp_evm::{Precompile, PrecompileHandle, PrecompileOutput};
use laos_precompile_utils::{
	keccak256, revert_dispatch_error, succeed, Address, Bytes, EvmDataWriter, EvmResult,
	FunctionModifier, GasCalculator, LogExt, LogsBuilder, PrecompileHandleExt,
};
use pallet_laos_evolution::{
	address_to_collection_id,
	traits::EvolutionCollection as EvolutionCollectionT,
	weights::{SubstrateWeight as LaosEvolutionWeights, WeightInfo},
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
/// Solidity selector of the EnabledPublicMinting log, which is the Keccak of the Log signature.
pub const SELECTOR_LOG_ENABLED_PUBLIC_MINTING: [u8; 32] = keccak256!("EnabledPublicMinting()");
/// Solidity selector of the DisabledPublicMinting log, which is the Keccak of the Log signature.
pub const SELECTOR_LOG_DISABLED_PUBLIC_MINTING: [u8; 32] = keccak256!("DisabledPublicMinting()");

/// Solidity selector of the `OwnershipTransferred` log, which is the Keccak of the Log signature.
pub const SELECTOR_LOG_OWNERSHIP_TRANSFERRED: [u8; 32] =
	keccak256!("OwnershipTransferred(address,address)");

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
	/// Transfer ownership of the collection
	TransferOwnership = "transferOwnership(address)",
	/// Enable public minting
	EnablePublicMinting = "enablePublicMinting()",
	/// Disable public minting
	DisablePublicMinting = "disablePublicMinting()",
	/// Check if public minting is enabled
	IsPublicMintingEnabled = "isPublicMintingEnabled()",
}

impl<Runtime> Precompile for EvolutionCollectionPrecompile<Runtime>
where
	Runtime: pallet_laos_evolution::Config + pallet_evm::Config,
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
			Action::EnablePublicMinting => FunctionModifier::NonPayable,
			Action::DisablePublicMinting => FunctionModifier::NonPayable,
			Action::IsPublicMintingEnabled => FunctionModifier::View,
			Action::TransferOwnership => FunctionModifier::NonPayable,
		})?;

		match selector {
			Action::Owner => Self::owner(handle),
			Action::TokenURI => Self::token_uri(handle),
			Action::Mint => Self::mint(handle),
			Action::Evolve => Self::evolve(handle),
			Action::EnablePublicMinting => Self::enable_public_minting(handle),
			Action::DisablePublicMinting => Self::disable_public_minting(handle),
			Action::IsPublicMintingEnabled => Self::is_public_minting_enabled(handle),
			Action::TransferOwnership => Self::transfer_ownership(handle),
		}
	}
}

pub struct EvolutionCollectionPrecompile<Runtime>(PhantomData<Runtime>)
where
	Runtime: pallet_laos_evolution::Config;

impl<Runtime> EvolutionCollectionPrecompile<Runtime>
where
	Runtime: pallet_laos_evolution::Config + pallet_evm::Config,
	Runtime::AccountId: From<H160> + Into<H160> + Encode + Debug,
	LaosEvolution<Runtime>: EvolutionCollectionT<Runtime::AccountId, TokenUriOf<Runtime>>,
{
	fn owner(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		let context = handle.context();

		// collection id is encoded into the contract address
		let collection_id = address_to_collection_id(context.address)
			.map_err(|_| revert("invalid collection address"))?;

		if let Some(owner) = LaosEvolution::<Runtime>::collection_owner(collection_id) {
			handle.record_cost(GasCalculator::<Runtime>::db_read_gas_cost(1))?;

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
			let consumed_gas: u64 = GasCalculator::<Runtime>::db_read_gas_cost(1);
			handle.record_cost(consumed_gas)?;
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
				let consumed_weight = LaosEvolutionWeights::<Runtime>::mint_with_external_uri(
					token_uri_raw.len() as u32,
				);

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

				// Record EVM cost
				handle.record_cost(GasCalculator::<Runtime>::weight_to_gas(consumed_weight))?;

				// Record Substrate related costs
				// TODO: Add `ref_time` when precompiles are benchmarked
				handle.record_external_cost(None, Some(consumed_weight.proof_size()))?;

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
				let consumed_weight = LaosEvolutionWeights::<Runtime>::evolve_with_external_uri(
					token_uri_raw.len() as u32,
				);

				let mut token_id_bytes = [0u8; 32];
				token_id.to_big_endian(&mut token_id_bytes);

				LogsBuilder::new(context.address)
					.log2(
						SELECTOR_LOG_EVOLVED_WITH_EXTERNAL_TOKEN_URI,
						token_id_bytes,
						EvmDataWriter::new().write(Bytes(token_uri_raw)).build(),
					)
					.record(handle)?;

				// Record EVM cost
				handle.record_cost(GasCalculator::<Runtime>::weight_to_gas(consumed_weight))?;

				// Record Substrate related costs
				// TODO: Add `ref_time` when precompiles are benchmarked
				handle.record_external_cost(None, Some(consumed_weight.proof_size()))?;

				Ok(succeed(sp_std::vec![]))
			},
			Err(err) => Err(revert_dispatch_error(err)),
		}
	}

	fn enable_public_minting(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		let input = handle.read_input()?;
		input.expect_arguments(0)?;
		let context = handle.context();
		let caller = context.caller;

		// collection id is encoded into the contract address
		let collection_id = address_to_collection_id(context.address)
			.map_err(|_| revert("invalid collection address"))?;

		match LaosEvolution::<Runtime>::enable_public_minting(caller.into(), collection_id) {
			Ok(()) => {
				let consumed_weight = LaosEvolutionWeights::<Runtime>::enable_public_minting();

				LogsBuilder::new(context.address)
					.log1(SELECTOR_LOG_ENABLED_PUBLIC_MINTING, sp_std::vec![])
					.record(handle)?;

				// Record EVM cost
				handle.record_cost(GasCalculator::<Runtime>::weight_to_gas(consumed_weight))?;

				// Record Substrate related costs
				// TODO: Add `ref_time` when precompiles are benchmarked
				handle.record_external_cost(None, Some(consumed_weight.proof_size()))?;

				Ok(succeed(EvmDataWriter::new().build()))
			},
			Err(err) => Err(revert_dispatch_error(err)),
		}
	}

	fn disable_public_minting(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		let input = handle.read_input()?;
		input.expect_arguments(0)?;
		let context = handle.context();
		let caller = context.caller;

		// collection id is encoded into the contract address
		let collection_id = address_to_collection_id(context.address)
			.map_err(|_| revert("invalid collection address"))?;

		match LaosEvolution::<Runtime>::disable_public_minting(caller.into(), collection_id) {
			Ok(()) => {
				let consumed_weight = LaosEvolutionWeights::<Runtime>::enable_public_minting();

				LogsBuilder::new(context.address)
					.log1(SELECTOR_LOG_DISABLED_PUBLIC_MINTING, sp_std::vec![])
					.record(handle)?;

				// Record EVM cost
				handle.record_cost(GasCalculator::<Runtime>::weight_to_gas(consumed_weight))?;

				// Record Substrate related costs
				// TODO: Add `ref_time` when precompiles are benchmarked
				handle.record_external_cost(None, Some(consumed_weight.proof_size()))?;

				Ok(succeed(EvmDataWriter::new().build()))
			},
			Err(err) => Err(revert_dispatch_error(err)),
		}
	}

	fn is_public_minting_enabled(
		handle: &mut impl PrecompileHandle,
	) -> EvmResult<PrecompileOutput> {
		let input = handle.read_input()?;
		input.expect_arguments(0)?;
		let context = handle.context();

		// collection id is encoded into the contract address
		let collection_id = address_to_collection_id(context.address)
			.map_err(|_| revert("invalid collection address"))?;

		let is_enabled = LaosEvolution::<Runtime>::is_public_minting_enabled(collection_id);
		let consumed_gas: u64 = GasCalculator::<Runtime>::db_read_gas_cost(1);
		handle.record_cost(consumed_gas)?;
		Ok(succeed(EvmDataWriter::new().write(is_enabled).build()))
	}

	fn transfer_ownership(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		let context = handle.context();
		let caller = context.caller;

		// collection id is encoded into the contract address
		let collection_id = address_to_collection_id(context.address)
			.map_err(|_| revert("invalid collection address"))?;

		let mut input = handle.read_input()?;
		input.expect_arguments(1)?;

		let to = input.read::<Address>()?.0;

		LaosEvolution::<Runtime>::transfer_ownership(caller.into(), to.into(), collection_id)
			.map_err(revert_dispatch_error)?;

		let consumed_weight = LaosEvolutionWeights::<Runtime>::transfer_ownership();

		LogsBuilder::new(context.address)
			.log3(SELECTOR_LOG_OWNERSHIP_TRANSFERRED, caller, to, EvmDataWriter::new().build())
			.record(handle)?;

		// Record EVM cost
		handle.record_cost(GasCalculator::<Runtime>::weight_to_gas(consumed_weight))?;

		// Record Substrate related costs
		handle.record_external_cost(None, Some(consumed_weight.proof_size()))?;

		Ok(succeed(EvmDataWriter::new().build()))
	}
}

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;
