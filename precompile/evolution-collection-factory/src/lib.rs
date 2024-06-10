// Copyright 2023-2024 Freeverse.io
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
	keccak256, revert_dispatch_error, succeed, Address, EvmDataWriter, EvmResult, FunctionModifier,
	GasCalculator, LogExt, LogsBuilder, PrecompileHandleExt,
};
use pallet_evm::Pallet as Evm;
use pallet_laos_evolution::{
	collection_id_to_address,
	traits::EvolutionCollectionFactory as EvolutionCollectionFactoryT,
	weights::{SubstrateWeight as LaosEvolutionWeights, WeightInfo},
	Pallet as LaosEvolution,
};
use parity_scale_codec::Encode;

use sp_core::{Get, H160};
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
						// TODO default weights are being used here and the ones created calculated using runtime should be used
						let mut consumed_weight =
							LaosEvolutionWeights::<Runtime>::create_collection();

						let collection_address: H160 = collection_id_to_address(collection_id);

						// Currently, we insert [`REVERT_BYTECODE`] as an
						// `AccountCode` for the collection address.
						//
						// This is done to ensure internal calls to the collection address do not
						// fail.
						Evm::<Runtime>::create_account(collection_address, REVERT_BYTECODE.into());

						// `AccountCode` -> 1 write, 1 read
						// `Suicided` -> 1 read
						// `AccountMetadata` -> 1 write
						consumed_weight = consumed_weight.saturating_add(
							<Runtime as frame_system::Config>::DbWeight::get().reads_writes(2, 2),
						);

						let consumed_gas = GasCalculator::<Runtime>::weight_to_gas(consumed_weight);

						LogsBuilder::new(context.address)
							.log2(
								SELECTOR_LOG_NEW_COLLECTION,
								owner,
								EvmDataWriter::new().write(Address(collection_address)).build(),
							)
							.record(handle)?;

						// record EVM cost
						handle.record_cost(consumed_gas)?;

						// Record Substrate related costs
						// TODO: Add `ref_time` when precompiles are benchmarked
						handle.record_external_cost(
							Some(consumed_weight.ref_time()), // TODO why this addition doesn't change anything at all? also comment the whole line doesn't have any effect
							Some(consumed_weight.proof_size()),
						)?;

						Ok(succeed(EvmDataWriter::new().write(Address(collection_address)).build()))
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
