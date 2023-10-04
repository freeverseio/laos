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

	/// Migrate from [`StorageVersion`] 0 to 1.
	pub struct MigrateV0ToV1<T>(sp_std::marker::PhantomData<T>);

	impl<T> OnRuntimeUpgrade for MigrateV0ToV1<T>
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
				unhashed::get::<<T as frame_system::Config>::AccountId>(raw_sudo_key.as_ref());

			frame_support::log::debug!("Old sudo key: {:?}", maybe_sudo);
			assert!(maybe_sudo.is_none());

			Ok(Vec::new())
		}

		/// Migrate from [`StorageVersion`] 0 to 1.
		///
		/// Simply remove old sudo key and insert new sudo key.
		///
		/// This function is called during the runtime upgrade process.
		fn on_runtime_upgrade() -> frame_support::weights::Weight {
			let raw_sudo_key = storage_prefix(
				<pallet_sudo::Pallet<T> as PalletInfoAccess>::name().as_bytes(),
				b"Key",
			);

			let new_sudo: [u8; 20] =
				hex_literal::hex!("47A4320be4B65BF73112E068dc637883490F5b04").into();

			// insert new sudo key
			unhashed::put_raw(raw_sudo_key.as_ref(), new_sudo.as_ref());

			frame_support::log::debug!("Inserting new sudo key: {:?}", raw_sudo_key);

			<T as frame_system::Config>::DbWeight::get().writes(1)
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
