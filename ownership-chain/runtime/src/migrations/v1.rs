/// Migration v1 for the runtime.
///
/// This migration is responsible for migrating accounts from AccountId32 to AccountId20.
/// It only migrates [`pallet_sudo::Key`].
use super::*;
use frame_support::{
	storage_alias,
	traits::{Get, OnRuntimeUpgrade},
};

#[cfg(feature = "try-runtime")]
use sp_std::vec::Vec;

/// Collection of storage item formats from the previous storage version.
mod old {
	use frame_support::{pallet_prelude::OptionQuery, Blake2_128Concat, StorageValue};
	use sp_core::U256;
	use sp_runtime::AccountId32;

	use super::*;

	type OldAccountId = AccountId32;

	/// V0 type for [`pallet_sudo::Key`].
	/// The `AccountId` of the sudo key.
	#[storage_alias]
	pub(super) type Key<T: _> = StorageValue<_, AccountId32, OptionQuery>;
}

/// Unchecked version migration logic.
pub mod version_unchecked {
	use frame_support::{weights::Weight, StoragePrefixedMap};
	use sp_io::{storage::clear_prefix, KillStorageResult};

	use crate::{Runtime, Sudo};

	use super::*;

	/// Migrate from [`StorageVersion`] 0 to 1.
	pub struct MigrateV0ToV1<T>(sp_std::marker::PhantomData<T>);

	impl<T: Config> OnRuntimeUpgrade for MigrateV0ToV1<T> {
		/// Return the existing storage key of the old format [`old::Key`].
		#[cfg(feature = "try-runtime")]
		fn pre_upgrade() -> Result<Vec<u8>, sp_runtime::TryRuntimeError> {
			use parity_scale_codec::Encode;

			Ok(old::Key::<T>::final_prefix().encode())
		}

		/// Migrate from [`StorageVersion`] 0 to 1.
		///
		/// Simply remove old sudo key and insert new sudo key.
		///
		/// This function is called during the runtime upgrade process.
		fn on_runtime_upgrade() -> frame_support::weights::Weight {
			let db_weight = <T as frame_system::Config>::DbWeight::get();

			// simply purge all storage items
			let mut weight_consumed = Weight::default();

			// remove old sudo key
			weight_consumed += db_weight.reads_writes(1, 1).saturating_add(db_weight.deletes(1));

			let old_key = old::Key::<T>::take();

			// insert new sudo key
			let new_sudo_key = hex_literal::hex!("992E92d707944b9f13BC1f6c49e57a5D8ce46cb8");
			pallet_sudo::<Runtime>::put(new_sudo_key);

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
