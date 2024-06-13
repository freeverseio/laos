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

use frame_support::DefaultNoBound;
use pallet_evm::{GasWeightMapping, Pallet as Evm};
use pallet_laos_evolution::{
	collection_id_to_address,
	traits::EvolutionCollectionFactory as EvolutionCollectionFactoryT,
	weights::{SubstrateWeight as LaosEvolutionWeights, WeightInfo},
	Pallet as LaosEvolution,
};
use parity_scale_codec::Encode;
use precompile_utils::{
	prelude::{keccak256, log2, solidity, Address, EvmResult, LogExt, PrecompileHandle},
	substrate::TryDispatchError,
};
use sp_core::{Get, H160};
use sp_runtime::traits::PhantomData;

/// Solidity selector of the CreateCollection log, which is the Keccak of the Log signature.
pub const SELECTOR_LOG_NEW_COLLECTION: [u8; 32] = keccak256!("NewCollection(address,address)");

// This is the simplest bytecode to revert without returning any data.
// We will pre-deploy it under all of our precompiles to ensure they can be called from
// within contracts.
// (PUSH1 0x00 PUSH1 0x00 REVERT)
pub const REVERT_BYTECODE: [u8; 5] = [0x60, 0x00, 0x60, 0x00, 0xFD];

#[derive(Clone, DefaultNoBound)]

pub struct EvolutionCollectionFactoryPrecompile<R>(PhantomData<R>);

impl<R> EvolutionCollectionFactoryPrecompile<R> {
	pub fn new() -> Self {
		Self(PhantomData)
	}
}

#[precompile_utils::precompile]
impl<Runtime> EvolutionCollectionFactoryPrecompile<Runtime>
where
	Runtime: pallet_evm::Config,
	LaosEvolution<Runtime>: EvolutionCollectionFactoryT<Runtime::AccountId>,
	Runtime::AccountId: From<H160> + Into<H160> + Encode,
{
	#[precompile::public("createCollection(address)")]
	fn create_collection(handle: &mut impl PrecompileHandle, owner: Address) -> EvmResult<Address> {
		match LaosEvolution::<Runtime>::create_collection(owner.0.into()) {
			Ok(collection_id) => {
				// TODO this weights are not the actual from runtime
				let mut consumed_weight = LaosEvolutionWeights::<Runtime>::create_collection();

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
				); // TODO sustitute by handler.record_db_cost?

				let consumed_gas = <Runtime as pallet_evm::Config>::GasWeightMapping::weight_to_gas(
					consumed_weight,
				);

				log2(
					handle.context().address,
					SELECTOR_LOG_NEW_COLLECTION,
					owner.0,
					solidity::encode_event_data(Address(collection_address)),
				)
				.record(handle)?;

				// record EVM cost
				handle.record_cost(consumed_gas)?;

				// Record Substrate related costs
				// TODO: Add `ref_time` when precompiles are benchmarked
				handle.record_external_cost(None, Some(consumed_weight.proof_size()))?;

				Ok(Address(collection_address))
			},
			Err(err) => Err(TryDispatchError::Substrate(err).into()),
		}
	}
}

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;
