use crate::{Config, Pallet};
use frame_support::{
	log, storage_alias,
	traits::{Get, OnRuntimeUpgrade},
};

#[cfg(feature = "try-runtime")]
use sp_std::vec::Vec;

/// Collection of storage item formats from the previous storage version.
mod old {
	use frame_support::{pallet_prelude::OptionQuery, Blake2_128Concat};
	use sp_core::U256;

	use super::*;

	/// V0 type for [`crate::Value`].
	#[storage_alias]
	pub(super) type AssetOwner<T: Config> = StorageMap<
		Pallet<T>,
		Blake2_128Concat,
		U256,
		<T as frame_system::Config>::AccountId,
		OptionQuery,
	>;
}

/// Unchecked version migration logic.
pub mod version_unchecked {
	use frame_support::{weights::Weight, StoragePrefixedMap};
	use sp_io::{storage::clear_prefix, KillStorageResult};

	use super::*;

	/// Migrate from [`StorageVersion`] 0 to 1.
	pub struct MigrateV0ToV1<T>(sp_std::marker::PhantomData<T>);

	impl<T: Config> OnRuntimeUpgrade for MigrateV0ToV1<T> {
		/// Return the existing `AssetOwner` storage item.
		#[cfg(feature = "try-runtime")]
		fn pre_upgrade() -> Result<Vec<u8>, sp_runtime::TryRuntimeError> {
			use parity_scale_codec::Encode;

			// access the storage item
			let old_keys = old::AssetOwner::<T>::iter_keys().collect::<Vec<_>>();

			// frame_support::log::debug!("old_keys: {:?}", old_keys);
			log::info!("old_keys: {:?}", old_keys);

			// encode the storage item
			Ok(old_keys.encode())
		}

		/// Migrate the [`old::AssetOwner`] storage item to the new format [`crate::AssetOwner`]
		///
		/// Since we lost the `collection_id` information, we can't migrate the storage items.
		/// So we just purge all storage items.
		///
		/// This function is called during the runtime upgrade process.
		fn on_runtime_upgrade() -> frame_support::weights::Weight {
			log::info!(
				target: "aaaaaaaaaaaa",
				"Migration did not execute. This probably should be removed"
			);
			let db_weight = <T as frame_system::Config>::DbWeight::get();

			// simply purge all storage items
			let mut weight_consumed = Weight::default();

			// TODO: no limit is dangerous
			match clear_prefix(&old::AssetOwner::<T>::final_prefix(), None) {
				KillStorageResult::AllRemoved(all) => {
					weight_consumed += db_weight.writes(all.into());
				},
				KillStorageResult::SomeRemaining(some) => {
					weight_consumed += db_weight.writes(some.into());
				},
			}

			frame_support::log::debug!("weight_consumed: {:?}", weight_consumed);

			weight_consumed
		}

		/// Post-upgrade migration step.
		#[cfg(feature = "try-runtime")]
		fn post_upgrade(state: Vec<u8>) -> Result<(), sp_runtime::TryRuntimeError> {
			use frame_support::ensure;
			use parity_scale_codec::Decode;

			let old_keys = Vec::<sp_core::U256>::decode(&mut &state[..])
				.map_err(|_| sp_runtime::TryRuntimeError::Other("Failed to decode old_keys"))?;

			frame_support::log::debug!("old_keys: {:?}", old_keys);

			// check the old storage items
			for old_key in old_keys {
				let old_asset_owner = old::AssetOwner::<T>::get(old_key);
				ensure!(old_asset_owner.is_none(), "WARNING: old_asset_owner is not None");
			}

			Ok(())
		}
	}
}

// Public module containing *version checked* migration logic.
//
// This is the only module that should be exported from this module.
//
// TODO: we should use `frame_support::migrations::VersionedMigration` when the following PR is
// included in release. https://github.com/paritytech/polkadot-sdk/pull/1503
// pub mod versioned {
// 	use super::*;

// 	/// `version_unchecked::MigrateV0ToV1` wrapped in a
// 	/// [`VersionedMigration`](frame_support::migrations::VersionedMigration), which ensures that:
// 	/// - The migration only runs once when the on-chain storage version is 0
// 	/// - The on-chain storage version is updated to `1` after the migration executes
// 	/// - Reads/Writes from checking/settings the on-chain storage version are accounted for
// 	pub type MigrateV0ToV1<T> = frame_support::migrations::VersionedRuntimeUpgrade<
// 		0, // The migration will only execute when the on-chain storage version is 0
// 		1, // The on-chain storage version will be set to 1 after the migration is complete
// 		version_unchecked::MigrateV0ToV1<T>,
// 		crate::pallet::Pallet<T>,
// 		<T as frame_system::Config>::DbWeight,
// 	>;
// }
