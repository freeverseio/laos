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

use crate::precompiles;
use frame_support::{
	traits::{Get, OnRuntimeUpgrade},
	weights::Weight,
};
use pallet_evm::Pallet as Evm;

#[cfg(feature = "try-runtime")]
use frame_support::ensure;
#[cfg(feature = "try-runtime")]
use sp_std::vec::Vec;

/// This struct is used to inject precompiled bytecode into the EVM for Asset Metadata Extender
/// precompile during a runtime upgrade.
pub struct InjectDamePrecompileBytecode<T>(sp_std::marker::PhantomData<T>);

impl<T> OnRuntimeUpgrade for InjectDamePrecompileBytecode<T>
where
	T: pallet_evm::Config,
{
	#[cfg(feature = "try-runtime")]
	fn pre_upgrade() -> Result<Vec<u8>, sp_runtime::TryRuntimeError> {
		let asset_metadata_extender_address = precompiles::hash(1029);
		ensure!(
			Evm::<T>::is_account_empty(&asset_metadata_extender_address),
			"account es not empty, i.e. bytecode is already stored"
		);
		Ok(Vec::new())
	}

	fn on_runtime_upgrade() -> Weight {
		let mut consumed_weight = Default::default();
		let asset_metadata_extender_address = precompiles::hash(1029);

		// early return if bytecode is already stored, it prevents from running migration twice
		if !Evm::<T>::is_account_empty(&asset_metadata_extender_address) {
			log::info!(target: "runtime::evm", "InjectDamePrecompileBytecode migration already executed");
			return consumed_weight;
		}

		let db_weight = <T as frame_system::Config>::DbWeight::get();

		Evm::<T>::create_account(
			asset_metadata_extender_address,
			pallet_evm_evolution_collection_factory::REVERT_BYTECODE.into(),
		);
		consumed_weight += db_weight.reads_writes(2, 2);
		log::info!(target: "runtime::evm", "InjectDamePrecompileBytecode migration executed successfully");

		consumed_weight
	}

	#[cfg(feature = "try-runtime")]
	fn post_upgrade(_state: Vec<u8>) -> Result<(), sp_runtime::TryRuntimeError> {
		let asset_metadata_extender_address = precompiles::hash(1029);
		ensure!(
			!Evm::<T>::is_account_empty(&asset_metadata_extender_address),
			"account is empty, i.e. bytecode is not stored"
		);
		Ok(())
	}
}
