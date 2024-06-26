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

use crate::{
	collection_id_to_address,
	traits::{EvolutionCollectionFactory as EvolutionCollectionFactoryT, OnCreateCollection},
	weights::WeightInfo,
	Pallet as LaosEvolution,
};
use frame_support::DefaultNoBound;
use precompile_utils::prelude::{
	keccak256, log2, revert, solidity, Address, EvmResult, LogExt, PrecompileHandle,
};
use scale_info::prelude::{format, string::String};
use sp_core::H160;
use sp_runtime::{
	traits::{ConvertBack, PhantomData},
	DispatchError,
};

/// Solidity selector of the CreateCollection log, which is the Keccak of the Log signature.
pub const SELECTOR_LOG_NEW_COLLECTION: [u8; 32] = keccak256!("NewCollection(address,address)");

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
	Runtime: crate::Config,
{
	#[precompile::public("createCollection(address)")]
	pub(crate) fn create_collection(
		handle: &mut impl PrecompileHandle,
		owner: Address,
	) -> EvmResult<Address> {
		super::register_cost::<Runtime>(
			handle,
			Runtime::WeightInfo::precompile_create_collection(),
		)?;

		match LaosEvolution::<Runtime>::create_collection(Runtime::AccountIdToH160::convert_back(
			owner.0,
		)) {
			Ok(collection_id) => {
				let collection_address: H160 = collection_id_to_address(collection_id);

				Runtime::OnCreateCollection::on_create_collection(collection_address);

				log2(
					handle.context().address,
					SELECTOR_LOG_NEW_COLLECTION,
					owner.0,
					solidity::encode_event_data(Address(collection_address)),
				)
				.record(handle)?;

				Ok(Address(collection_address))
			},
			Err(err) => Err(revert(convert_dispatch_error_to_string(err))),
		}
	}
}

fn convert_dispatch_error_to_string(err: DispatchError) -> String {
	match err {
		DispatchError::Module(mod_err) => mod_err.message.unwrap_or("Unknown module error").into(),
		_ => format!("{:?}", err),
	}
}

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;
