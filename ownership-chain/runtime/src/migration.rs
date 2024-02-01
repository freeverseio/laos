use frame_support::{
	ensure,
	traits::{Get, OnRuntimeUpgrade},
	weights::Weight,
};
use pallet_evm::Pallet as Evm;
use sp_core::H160;

#[cfg(feature = "try-runtime")]
use sp_std::vec::Vec;

pub struct AddPrecompileBytecode<T>(sp_std::marker::PhantomData<T>);

fn is_bytecode_stored<T: pallet_evm::Config>(address: H160) -> bool {
	Evm::<T>::account_code_metadata(address).size != 0
}

impl<T> OnRuntimeUpgrade for AddPrecompileBytecode<T>
where
	T: pallet_evm::Config,
{
	#[cfg(feature = "try-runtime")]
	fn pre_upgrade() -> Result<Vec<u8>, sp_runtime::TryRuntimeError> {
		let asset_metadata_extender_address: H160 = H160::from_low_u64_be(1029);
		ensure!(
			is_bytecode_stored::<T>(asset_metadata_extender_address),
			"account code metadata is not zero, i.e. bytecode is already stored"
		);
		Ok(Vec::new())
	}

	fn on_runtime_upgrade() -> Weight {
		let asset_metadata_extender_address = H160::from_low_u64_be(1029);

		// early return if bytecode is already stored, it prevents from running migration twice
		if is_bytecode_stored::<T>(asset_metadata_extender_address) {
			log::info!(target: "runtime::evm", "AddPrecompileBytecode migration already executed");
			return Default::default();
		}

		let db_weight = <T as frame_system::Config>::DbWeight::get();
		let mut consumed_weight = Default::default();

		Evm::<T>::create_account(
			asset_metadata_extender_address,
			pallet_evm_evolution_collection_factory::REVERT_BYTECODE.into(),
		);
		consumed_weight += db_weight.writes(1);
		log::info!(target: "runtime::evm", "AddPrecompileBytecode migration executed successfully");

		consumed_weight
	}

	#[cfg(feature = "try-runtime")]
	fn post_upgrade(_state: Vec<u8>) -> Result<(), sp_runtime::TryRuntimeError> {
		let asset_metadata_extender_address = H160::from_low_u64_be(1029);
		ensure!(
			is_bytecode_stored::<T>(asset_metadata_extender_address),
			"account code metadata is zero, i.e. bytecode is not stored"
		);
		Ok(())
	}
}
