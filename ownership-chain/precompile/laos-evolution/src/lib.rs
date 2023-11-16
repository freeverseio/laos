//! LAOS precompile module.

#![cfg_attr(not(feature = "std"), no_std)]
use fp_evm::{Precompile, PrecompileHandle, PrecompileOutput};
use laos_precompile_utils::{
	keccak256, revert, revert_dispatch_error, succeed, Address, Bytes, EvmDataWriter, EvmResult,
	FunctionModifier, LogExt, LogsBuilder, PrecompileHandleExt,
};
use pallet_evm::Pallet as Evm;
use pallet_laos_evolution::{
	address_to_collection_id, collection_id_to_address, traits::LaosEvolution as LaosEvolutionT,
	Pallet as LaosEvolution, Slot, TokenId, TokenUriOf,
};
use parity_scale_codec::Encode;

use sp_core::H160;
use sp_std::{fmt::Debug, marker::PhantomData};

/// Solidity selector of the CreateCollection log, which is the Keccak of the Log signature.
pub const SELECTOR_LOG_NEW_COLLECTION: [u8; 32] = keccak256!("NewCollection(address,address)");
/// Solidity selector of the Transfer log, which is the Keccak of the Log signature.
pub const SELECTOR_LOG_MINTED_WITH_EXTERNAL_TOKEN_URI: [u8; 32] =
	keccak256!("MintedWithExternalURI(address,uint96,uint256,string)");
pub const SELECTOR_LOG_EVOLVED_WITH_EXTERNAL_TOKEN_URI: [u8; 32] =
	keccak256!("EvolvedWithExternalURI(uint256,string)");

// This is the simplest bytecode to revert without returning any data.
// We will pre-deploy it under all of our precompiles to ensure they can be called from
// within contracts.
// (PUSH1 0x00 PUSH1 0x00 REVERT)
pub const REVERT_BYTECODE: [u8; 5] = [0x60, 0x00, 0x60, 0x00, 0xFD];

#[laos_precompile_utils_macro::generate_function_selector]
#[derive(Debug, PartialEq)]
pub enum Action {
	/// Create collection
	CreateCollection = "createCollection(address)",
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
pub struct LaosEvolutionPrecompile<Runtime>(PhantomData<Runtime>)
where
	Runtime: pallet_evm::Config + pallet_laos_evolution::Config;

impl<Runtime> LaosEvolutionPrecompile<Runtime>
where
	Runtime: pallet_evm::Config + pallet_laos_evolution::Config,
	Runtime::AccountId: From<H160> + Into<H160> + Encode + Debug,
	LaosEvolution<Runtime>: LaosEvolutionT<Runtime::AccountId, TokenUriOf<Runtime>>,
{
	/// Inner execute function.
	fn inner_execute(
		handle: &mut impl PrecompileHandle,
		action: &Action,
	) -> EvmResult<PrecompileOutput> {
		let mut input = handle.read_input()?;
		let context = handle.context();

		match action {
			Action::CreateCollection => {
				input.expect_arguments(1)?;

				let owner = input.read::<Address>()?.0;

				match LaosEvolution::<Runtime>::create_collection(owner.into()) {
					Ok(collection_id) => {
						let collection_address: H160 = collection_id_to_address(collection_id);

						/// Currently, we insert [`REVERT_BYTECODE`] as an
						/// `AccountCode` for the collection address.
						///
						/// This is done to ensure internal calls to the collection address do not
						/// fail.
						Evm::<Runtime>::create_account(*address, REVERT_BYTECODE.into());

						LogsBuilder::new(context.address)
							.log2(
								SELECTOR_LOG_NEW_COLLECTION,
								owner,
								EvmDataWriter::new()
									.write(Address(collection_address.into()))
									.build(),
							)
							.record(handle)?;

						Ok(succeed(
							EvmDataWriter::new().write(Address(collection_address.into())).build(),
						))
					},
					Err(err) => Err(revert_dispatch_error(err)),
				}
			},
			Action::Owner => {
				// collection id is encoded into the contract address
				let collection_id = address_to_collection_id(context.address)
					.map_err(|_| revert("invalid collection address"))?;

				if let Some(owner) = LaosEvolution::<Runtime>::collection_owner(collection_id) {
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

				if let Some(token_uri) =
					LaosEvolution::<Runtime>::token_uri(collection_id, token_id)
				{
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
			},
		}
	}
}

impl<Runtime> Precompile for LaosEvolutionPrecompile<Runtime>
where
	Runtime: pallet_evm::Config + pallet_laos_evolution::Config,
	Runtime::AccountId: From<H160> + Into<H160> + Encode + Debug,
	LaosEvolution<Runtime>: LaosEvolutionT<Runtime::AccountId, TokenUriOf<Runtime>>,
{
	fn execute(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		let selector = handle.read_selector()?;

		handle.check_function_modifier(match selector {
			Action::CreateCollection => FunctionModifier::NonPayable,
			Action::Mint => FunctionModifier::NonPayable,
			Action::Owner => FunctionModifier::View,
			Action::TokenURI => FunctionModifier::View,
			Action::Evolve => FunctionModifier::NonPayable,
		})?;

		Self::inner_execute(handle, &selector)
	}
}

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;
