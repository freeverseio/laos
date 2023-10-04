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

/// New sudo account that we control.
const NEW_SUDO: &str = "47A4320be4B65BF73112E068dc637883490F5b04";

/// Unchecked version migration logic.
pub mod version_unchecked {
	use bp_runtime::storage_value_key;
	use cumulus_primitives_core::Junction::AccountId32;
	use frame_support::{
		storage::unhashed, traits::PalletInfoAccess, weights::Weight, StoragePrefixedMap,
	};
	use sp_io::{storage::clear_prefix, KillStorageResult};

	use crate::{Runtime, Sudo};

	use super::*;

	/// Migrate from [`StorageVersion`] 0 to 1.
	pub struct MigrateV0ToV1<T>(sp_std::marker::PhantomData<T>);

	impl<T: Config> OnRuntimeUpgrade for MigrateV0ToV1<T> {
		/// Return the existing storage key of the old format [`old::Key`].
		#[cfg(feature = "try-runtime")]
		fn pre_upgrade() -> Result<Vec<u8>, sp_runtime::TryRuntimeError> {}

		/// Migrate from [`StorageVersion`] 0 to 1.
		///
		/// Simply remove old sudo key and insert new sudo key.
		///
		/// This function is called during the runtime upgrade process.
		fn on_runtime_upgrade() -> frame_support::weights::Weight {
			let raw_sudo_key =
				storage_value_key(<pallet_sudo::Pallet<T> as PalletInfoAccess>::name(), "Key");

			// insert new sudo key
			let new_sudo = hex_literal::hex!("DFc7E055C1435CC365A1369D4C2b9Ce10F8Ed201").into();

			unhashed::put_raw(raw_sudo_key.into(), new_sudo);

			frame_support::log::debug!("Inserting new sudo key: {:?}", raw_sudo_key);

			<T as frame_system::Config>::DbWeight::get().writes(1)
		}

		/// Post-upgrade migration step.
		#[cfg(feature = "try-runtime")]
		fn post_upgrade(state: Vec<u8>) -> Result<(), sp_runtime::TryRuntimeError> {
			Ok(())
		}
	}
}
