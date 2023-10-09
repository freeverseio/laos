/// Migration v1 for the runtime.
///
/// This migration is responsible for migrating accounts from AccountId32 to AccountId20.
/// It only migrates [`pallet_sudo::Key`].

mod old {
	/// Old storage types
	use frame_support::{pallet_prelude::ValueQuery, storage_alias, Blake2_128Concat};
	use frame_system::AccountInfo;
	use sp_runtime::AccountId32;

	/// Old account storage with `AccountId32`
	#[storage_alias]
	pub type Account<T: frame_system::Config> = StorageMap<
		frame_system::Pallet<T>,
		Blake2_128Concat,
		AccountId32,
		AccountInfo<<T as frame_system::Config>::Nonce, <T as frame_system::Config>::AccountData>,
		ValueQuery,
	>;
}

/// Unchecked version migration logic.
pub mod version_unchecked {
	use super::*;
	use frame_support::{
		storage::{storage_prefix, unhashed},
		traits::{Currency, Get, OnRuntimeUpgrade, PalletInfoAccess},
	};
	use frame_system::AccountInfo;
	use sp_runtime::{traits::Saturating, AccountId32};
	#[cfg(feature = "try-runtime")]
	use sp_std::vec::Vec;

	/// Migrate Sudo key
	pub struct MigrateSudo<T>(sp_std::marker::PhantomData<T>);

	impl<T> OnRuntimeUpgrade for MigrateSudo<T>
	where
		T: pallet_sudo::Config + pallet_balances::Config,
		T::AccountId: From<[u8; 20]>,
	{
		/// Fetches old sudo account and returns it
		#[cfg(feature = "try-runtime")]
		fn pre_upgrade() -> Result<Vec<u8>, sp_runtime::TryRuntimeError> {
			use parity_scale_codec::Encode;
			// make sure we can't read current sudo key
			let raw_sudo_key = storage_prefix(
				<pallet_sudo::Pallet<T> as PalletInfoAccess>::name().as_bytes(),
				b"Key",
			);
			let old_sudo: AccountId32 =
				unhashed::get(raw_sudo_key.as_ref()).ok_or(sp_runtime::TryRuntimeError::Other(
					"Sudo key wasn't set! THIS SHOULD NEVER HAPPEN!",
				))?;

			// take old sudo balance
			let old_sudo_balance = old::Account::<T>::get(&old_sudo).data;

			// return old sudo account and balance
			Ok((old_sudo, old_sudo_balance).encode())
		}

		/// Migrate sudo key.
		///
		/// This migration does two things:
		///
		/// 1. remove old sudo key and insert new sudo key.
		/// 2. fund new sudo key with some balance.
		///
		/// This function is called during the runtime upgrade process.
		///
		/// NOTE: storage version of `pallet_sudo` can not be updated because it's an external
		/// pallet, i.e we have to set `#[pallet::storage_version(VERSION)]` attribute.
		fn on_runtime_upgrade() -> frame_support::weights::Weight {
			let db_weight = <T as frame_system::Config>::DbWeight::get();
			let mut consumed_weight = Default::default();

			let raw_sudo_key = storage_prefix(
				<pallet_sudo::Pallet<T> as PalletInfoAccess>::name().as_bytes(),
				b"Key",
			);
			let old_sudo: AccountId32 =
				unhashed::get(raw_sudo_key.as_ref()).expect("Sudo key should be set!; qed");
			// take old sudo balance
			let old_sudo_balance = old::Account::<T>::take(&old_sudo).data;

			frame_support::log::debug!(
				"Old sudo account: {:?} balance {:?}",
				old_sudo,
				old_sudo_balance
			);
			consumed_weight += db_weight.reads_writes(2, 1);

			let new_sudo: [u8; 20] =
				hex_literal::hex!("47A4320be4B65BF73112E068dc637883490F5b04").into();
			let new_sudo_account: T::AccountId = new_sudo.into();

			// insert new sudo key (`pallet_sudo::Key` is private)
			frame_support::log::debug!("Inserting new sudo key: {:?}", new_sudo);
			unhashed::put_raw(raw_sudo_key.as_ref(), new_sudo.as_ref());

			consumed_weight += db_weight.writes(1);

			// fund new sudo account
			<pallet_balances::Pallet<T> as Currency<T::AccountId>>::make_free_balance_be(
				&new_sudo_account,
				<T as pallet_balances::Config>::ExistentialDeposit::get()
					.saturating_mul(1_000_000_u32.into()),
			);

			// frame_system::Account::<T>::insert(new_sudo_account, sudo_account_info);
			consumed_weight += db_weight.writes(1);

			consumed_weight
		}

		/// This checks that the new sudo key is set.
		#[cfg(feature = "try-runtime")]
		fn post_upgrade(state: Vec<u8>) -> Result<(), sp_runtime::TryRuntimeError> {
			let old_sudo: AccountId32 = parity_scale_codec::Decode::decode(&mut &state[..])
				.map_err(|_| sp_runtime::TryRuntimeError::Other("Failed to decode state"))?;

			let old_sudo_balance: <T as frame_system::Config>::AccountData =
				parity_scale_codec::Decode::decode(&mut &state[32..])
					.map_err(|_| sp_runtime::TryRuntimeError::Other("Failed to decode state"))?;

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

			let new_sudo_account: T::AccountId = expected_sudo.into();
			let new_sudo_balance = frame_system::Account::<T>::get(&new_sudo_account);
			let old_sudo_new_balance = old::Account::<T>::get(&old_sudo);

			// new sudo account is funded with the same balance as old sudo account
			// assert_eq!(new_sudo_balance.data, old_sudo_balance);
			// sudo account is removed from old storage
			assert_eq!(old_sudo_new_balance.data, Default::default());

			Ok(())
		}
	}
}
