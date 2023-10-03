use frame_support::{
	log,
	traits::{Get, GetStorageVersion, OnRuntimeUpgrade, PalletInfoAccess},
};

/// The log target of this pallet.
pub const LOG_TARGET: &str = "runtime::frontier";

/// Unchecked version migration logic.
// TODO: we should use `frame_support::migrations::VersionedMigration` when the following PR is
// included in release. https://github.com/paritytech/polkadot-sdk/pull/1503
pub mod version_unchecked {
	use frame_support::{storage::unhashed::clear_prefix, weights::Weight};

	use super::*;
	use crate::{BaseFee, EVMChainId, Ethereum, LivingAssetsOwnership, EVM};
	use frame_system::Config;

	/// Clear all Frontier related pallets storage.
	pub struct ClearFrontierPalletsStorage<T>(sp_std::marker::PhantomData<T>);

	impl<T: Config> OnRuntimeUpgrade for ClearFrontierPalletsStorage<T> {
		fn on_runtime_upgrade() -> Weight {
			log::info!(target: LOG_TARGET, "Running ClearFrontierPalletsStorage");
			if LivingAssetsOwnership::on_chain_storage_version() < 1 {
				let mut entries = 0;
				entries = self::clear_storage_entries::<Ethereum>().saturating_add(entries);
				entries = self::clear_storage_entries::<EVM>().saturating_add(entries);
				entries = self::clear_storage_entries::<EVMChainId>().saturating_add(entries);
				entries = self::clear_storage_entries::<BaseFee>().saturating_add(entries);

				T::DbWeight::get().writes(entries)
			} else {
				log::debug!(
					target: LOG_TARGET,
					"Clearing frontier pallets storage skipped"
				);
				Weight::zero()
			}
		}
	}

	fn clear_storage_entries<T: PalletInfoAccess>() -> u64 {
		let res = clear_prefix(&sp_io::hashing::twox_128(<T>::name().as_bytes()), None, None);

		if res.unique == 0 {
			log::error!(
				target: LOG_TARGET,
				"No storage entries are cleared from '{}' storage prefix.",
				<T>::name()
			);
		}

		log::info!(
			target: LOG_TARGET,
			"Cleared '{}' entries from '{}' storage prefix",
			res.unique, <T>::name()
		);

		if res.maybe_cursor.is_some() {
			log::error!(
				target: LOG_TARGET,
				"Storage prefix '{}' is not completely cleared.",
				<T>::name()
			);
		}
		res.unique.into()
	}
}
