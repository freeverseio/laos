/// Migration v1 for the runtime.
///
/// This migration is responsible for migrating accounts from AccountId32 to AccountId20.
/// It only migrates [`pallet_sudo::Key`].

/// Unchecked version migration logic.
pub mod version_unchecked {
	use frame_support::{
		storage::{storage_prefix, unhashed},
		traits::{Get, OnRuntimeUpgrade, PalletInfoAccess},
	};
	#[cfg(feature = "try-runtime")]
	use sp_std::vec::Vec;

	/// Migrate Sudo key
	pub struct MigrateSudo<T>(sp_std::marker::PhantomData<T>);

	impl<T> OnRuntimeUpgrade for MigrateSudo<T>
	where
		T: pallet_sudo::Config,
	{
		/// This simply asserts that it's not possible to read the old sudo key, since it is
		/// `AccountId32`.
		#[cfg(feature = "try-runtime")]
		fn pre_upgrade() -> Result<Vec<u8>, sp_runtime::TryRuntimeError> {
			// make sure we can't read current sudo key
			let raw_sudo_key = storage_prefix(
				<pallet_sudo::Pallet<T> as PalletInfoAccess>::name().as_bytes(),
				b"Key",
			);
			let maybe_sudo =
				unhashed::get_raw(raw_sudo_key.as_ref()).expect("Sudo key should be set!; qed");

			frame_support::log::debug!("Old sudo key: {:?}", maybe_sudo);

			// old sudo key has length of 32
			assert_eq!(maybe_sudo.len(), 32);

			Ok(Vec::new())
		}

		/// Migrate sudo key.
		///
		/// Simply remove old sudo key and insert new sudo key.
		///
		/// This function is called during the runtime upgrade process.
		///
		/// NOTE: storage version of `pallet_sudo` can not be updated because it's an external
		/// pallet, i.e we have to set `#[pallet::storage_version(VERSION)]` attribute.
		fn on_runtime_upgrade() -> frame_support::weights::Weight {
			let raw_sudo_key = storage_prefix(
				<pallet_sudo::Pallet<T> as PalletInfoAccess>::name().as_bytes(),
				b"Key",
			);

			let new_sudo: [u8; 20] =
				hex_literal::hex!("47A4320be4B65BF73112E068dc637883490F5b04").into();

			// insert new sudo key
			unhashed::put_raw(raw_sudo_key.as_ref(), new_sudo.as_ref());

			frame_support::log::debug!("Inserting new sudo key: {:?}", new_sudo);

			<T as frame_system::Config>::DbWeight::get().reads_writes(1, 1)
		}

		/// This checks that the new sudo key is set.
		#[cfg(feature = "try-runtime")]
		fn post_upgrade(_state: Vec<u8>) -> Result<(), sp_runtime::TryRuntimeError> {
			// there should be new sudo key
			let expected_sudo: [u8; 20] =
				hex_literal::hex!("47A4320be4B65BF73112E068dc637883490F5b04").into();

			let raw_sudo_key = storage_prefix(
				<pallet_sudo::Pallet<T> as PalletInfoAccess>::name().as_bytes(),
				b"Key",
			);

			let current_sudo = unhashed::get_raw(raw_sudo_key.as_ref()).ok_or(
				sp_runtime::TryRuntimeError::Other(
					"Sudo key wasn't set! THIS SHOULD NEVER HAPPEN!",
				),
			)?;

			assert_eq!(current_sudo, expected_sudo.to_vec());

			Ok(())
		}
	}
}
