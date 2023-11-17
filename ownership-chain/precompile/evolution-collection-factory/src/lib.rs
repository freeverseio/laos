//! LAOS precompile module.

#![cfg_attr(not(feature = "std"), no_std)]
use fp_evm::{Precompile, PrecompileHandle, PrecompileOutput};
use laos_precompile_utils::{
	keccak256, revert_dispatch_error, succeed, Address, EvmDataWriter, EvmResult, FunctionModifier,
	LogExt, LogsBuilder, PrecompileHandleExt,
};
use pallet_evm::Pallet as Evm;
use pallet_laos_evolution::{
	collection_id_to_address, traits::EvolutionCollectionFactory as EvolutionCollectionFactoryT,
	Pallet as LaosEvolution,
};
use parity_scale_codec::Encode;

use sp_core::H160;
use sp_std::{fmt::Debug, marker::PhantomData};

/// Solidity selector of the CreateCollection log, which is the Keccak of the Log signature.
pub const SELECTOR_LOG_NEW_COLLECTION: [u8; 32] = keccak256!("NewCollection(address,address)");

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
}

/// Wrapper for the precompile function.
pub struct EvolutionCollectionFactoryPrecompile<Runtime>(PhantomData<Runtime>)
where
	Runtime: pallet_evm::Config + pallet_laos_evolution::Config,
	Runtime::AccountId: From<H160> + Into<H160> + Encode + Debug,
	LaosEvolution<Runtime>: EvolutionCollectionFactoryT<Runtime::AccountId>;

impl<Runtime> Precompile for EvolutionCollectionFactoryPrecompile<Runtime>
where
	Runtime: pallet_evm::Config + pallet_laos_evolution::Config,
	Runtime::AccountId: From<H160> + Into<H160> + Encode + Debug,
	LaosEvolution<Runtime>: EvolutionCollectionFactoryT<Runtime::AccountId>,
{
	fn execute(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		let selector = handle.read_selector()?;

		handle.check_function_modifier(match selector {
			Action::CreateCollection => FunctionModifier::NonPayable,
		})?;

		let mut input = handle.read_input()?;
		let context = handle.context();

		match selector {
			Action::CreateCollection => {
				input.expect_arguments(1)?;

				let owner = input.read::<Address>()?.0;

				match LaosEvolution::<Runtime>::create_collection(owner.into()) {
					Ok(collection_id) => {
						let collection_address: H160 = collection_id_to_address(collection_id);

						// Currently, we insert [`REVERT_BYTECODE`] as an
						// `AccountCode` for the collection address.
						//
						// This is done to ensure internal calls to the collection address do not
						// fail.
						Evm::<Runtime>::create_account(collection_address, REVERT_BYTECODE.into());

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
		}
	}
}

#[cfg(test)]
mod tests;

#[cfg(test)]
mod mock;
