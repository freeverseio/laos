//! Common types used in the bridge clients and relays.

use bp_runtime::StorageMapKeyProvider;
use sp_core::storage::StorageKey;
use sp_runtime::codec::FullCodec;

/// Provides a storage key for account data.
///
/// We need to use this approach when we don't have access to the runtime.
/// The equivalent command to invoke in case full `Runtime` is known is this:
/// `let key = frame_system::Account::<Runtime>::storage_map_final_key(&account_id);`
///
/// NOTE: this is a custom impl for `AccountId = AccountId20`
/// source: [`bp_polkadot_core::AccountInfoStorageMapKeyProvider`]
pub struct AccountInfoStorageMapKeyProvider<AccountId>(sp_std::marker::PhantomData<AccountId>)
where
	AccountId: FullCodec + Send + Sync;

impl<AccountId> StorageMapKeyProvider for AccountInfoStorageMapKeyProvider<AccountId>
where
	AccountId: FullCodec + Send + Sync,
{
	const MAP_NAME: &'static str = "Account";
	type Hasher = frame_support::Blake2_128Concat;
	type Key = AccountId;
	// This should actually be `AccountInfo`, but we don't use this property in order to decode the
	// data. So we use `Vec<u8>` as if we would work with encoded data.
	type Value = Vec<u8>;
}

impl<AccountId> AccountInfoStorageMapKeyProvider<AccountId>
where
	AccountId: FullCodec + Send + Sync,
{
	/// Name of the system pallet.
	const PALLET_NAME: &'static str = "System";

	/// Return storage key for given account data.
	pub fn final_key(id: &AccountId) -> StorageKey {
		<Self as StorageMapKeyProvider>::final_key(Self::PALLET_NAME, id)
	}
}
