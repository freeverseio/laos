use super::*;
use frame_support::{
	ensure, storage_alias,
	traits::{Get, GetStorageVersion, OnRuntimeUpgrade, StorageVersion},
	weights::Weight,
};
use sp_core::H160;

/// Unchecked version migration logic.
use super::*;

#[cfg(feature = "try-runtime")]
use sp_std::vec::Vec;

pub struct VersionUncheckedMigratePrecompileDummyCode<T>(sp_std::marker::PhantomData<T>);

impl<T> OnRuntimeUpgrade for VersionUncheckedMigratePrecompileDummyCode<T>
where
	T: pallet_evm::Config,
{
	#[cfg(feature = "try-runtime")]
	fn pre_upgrade() -> Result<Vec<u8>, sp_runtime::TryRuntimeError> {
		let asset_metadata_extender_address = H160::from_low_u64_be(1029);
		ensure!(
			Evm::<T>::account_code_metadata(asset_metadata_extender_address).size == 0,
			"account code metadata is not zero"
		);
		Ok(Vec::new())
	}

	fn on_runtime_upgrade() -> Weight {
		// TODO check storage version
		// let db_weight = <T as frame_system::Config>::DbWeight::get();
		// let mut consumed_weight = Default::default();

		let asset_metadata_extender_address = H160::from_low_u64_be(1029); // TODO resuse it
																   // consumed_weight += db_weight.writes(1);
		Evm::<T>::create_account(
			asset_metadata_extender_address,
			pallet_evm_evolution_collection_factory::REVERT_BYTECODE.into(),
		);

		log::info!(target: "runtime::evm", "MigratePrecompileDummyCode executed successfully");

		// consumed_weight
		Weight::zero() // TODO return actual weight
	}

	#[cfg(feature = "try-runtime")]
	fn post_upgrade(_state: Vec<u8>) -> Result<(), sp_runtime::TryRuntimeError> {
		let asset_metadata_extender_address = H160::from_low_u64_be(1029);
		ensure!(
			Evm::<T>::account_code_metadata(asset_metadata_extender_address).size != 0, /* check
			                                                                             * real value */
			"account code metadata is zero"
		);
		Ok(())
	}
}

// WIP
pub type VersionCheckeMigratePrecompileDummyCode<T> = frame_support::migrations::VersionedMigration<
	0,
	1,
	VersionUncheckedMigratePrecompileDummyCode<T>,
	frame_system::Config,
	<T as frame_system::Config>::DbWeight,
>;
