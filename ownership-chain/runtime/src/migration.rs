use frame_support::{
	ensure,
	traits::{Get, OnRuntimeUpgrade, StorageVersion},
	weights::Weight,
};
use pallet_evm::Pallet as Evm;
use sp_core::H160;

#[cfg(feature = "try-runtime")]
use sp_std::vec::Vec;

pub struct AddPrecompileDummyCode<T>(sp_std::marker::PhantomData<T>);

impl<T> OnRuntimeUpgrade for AddPrecompileDummyCode<T>
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
		if StorageVersion::get::<Evm<T>>() == 0 {
			let db_weight = <T as frame_system::Config>::DbWeight::get();
			let mut consumed_weight = Default::default();

			let asset_metadata_extender_address = H160::from_low_u64_be(1029); // TODO resuse it
			Evm::<T>::create_account(
				asset_metadata_extender_address,
				pallet_evm_evolution_collection_factory::REVERT_BYTECODE.into(),
			);
			consumed_weight += db_weight.writes(1);
			log::info!(target: "runtime::evm", "AddPrecompileDummyCode migration executed successfully");

			// check new version
			StorageVersion::new(1).put::<Evm<T>>();
			consumed_weight += db_weight.writes(1);

			log::info!(target: "runtime::evm", "Evm migrated to {:#?}", StorageVersion::get::<Evm<T>>());
			consumed_weight += db_weight.reads(1);

			consumed_weight
		} else {
			log::warn!(target: "runtime::evm", "AddPrecompileDummyCode migration already executed");
			T::DbWeight::get().reads(1)
		}
	}

	#[cfg(feature = "try-runtime")]
	fn post_upgrade(_state: Vec<u8>) -> Result<(), sp_runtime::TryRuntimeError> {
		let asset_metadata_extender_address = H160::from_low_u64_be(1029);
		ensure!(
			Evm::<T>::account_code_metadata(asset_metadata_extender_address).size != 0,
			"account code metadata is zero"
		);
		Ok(())
	}
}
