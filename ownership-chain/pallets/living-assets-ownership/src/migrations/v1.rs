use crate::{Config, Pallet};
use frame_support::{
	log,
	traits::{Get, GetStorageVersion, OnRuntimeUpgrade, PalletInfoAccess, StorageVersion},
};

/// The log target of this pallet.
pub const LOG_TARGET: &str = "runtime::living_assets_ownership";

/// Unchecked version migration logic.
// TODO: we should use `frame_support::migrations::VersionedMigration` when the following PR is
// included in release. https://github.com/paritytech/polkadot-sdk/pull/1503
pub mod version_unchecked {
	use frame_support::{storage::migration::clear_storage_prefix, weights::Weight};

	use super::*;

	/// Migrate from [`StorageVersion`] 0 to 1.
	pub struct MigrateV0ToV1<T>(sp_std::marker::PhantomData<T>);

	impl<T: Config> OnRuntimeUpgrade for MigrateV0ToV1<T> {
		/// Clear LivingAssetsOwnership pallet storage.
		///
		/// Since we lost the `collection_id` information, we can't migrate the storage items.
		/// So we just purge all storage items.
		fn on_runtime_upgrade() -> Weight {
			log::info!(target: LOG_TARGET, "Running MigrateV0ToV1");

			if Pallet::<T>::on_chain_storage_version() < 1 {
				assert_eq!(<Pallet<T>>::name(), "LivingAssetsOwnership");
				let res = clear_storage_prefix(
					<Pallet<T>>::name().as_bytes(),
					b"AssetOwner",
					b"",
					None,
					None,
				);

				if res.unique == 0 {
					log::error!(
						target: LOG_TARGET,
						"No storage entries are cleared from 'LivingAssetsOwnership_AssetOwner' storage prefix.",
					);
				}

				log::info!(
					target: LOG_TARGET,
					"Cleared '{}' entries from 'LivingAssetsOwnership_AssetOwner' storage prefix",
					res.unique
				);

				if res.maybe_cursor.is_some() {
					log::error!(
						target: LOG_TARGET,
						"Storage prefix 'LivingAssetsOwnership_AssetOwner' is not completely cleared."
					);
				}

				// Update storage version.
				StorageVersion::new(1).put::<Pallet<T>>();

				T::DbWeight::get().writes(res.unique.into())
			} else {
				log::debug!(
					target: LOG_TARGET,
					"Migration v0_to_v1 is skipped"
				);
				Weight::zero()
			}
		}
	}
}
