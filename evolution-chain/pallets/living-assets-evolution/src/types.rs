//! Types used in the pallet

use codec::{Decode, Encode};
use frame_support::traits::Get;
use scale_info::TypeInfo;
use sp_core::{U256, H160};
use sp_runtime::BoundedVec;

/// Collection id type
pub type CollectionId = u64;

/// Explicit `AccountId`
pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

/// Wrapper around `BoundedVec` for `tokenUri`
pub type TokenUriOf<T> = BoundedVec<u8, <T as crate::Config>::MaxTokenUriLength>;

/// OwnerId type
pub type SlotOwnerId = H160;

/// TokenId type
/// every slot is identified by a unique `asset_id = concat(slot #, owner_address)`
pub type TokenId = U256;

/// Slot type - 96-bit unsigned integer
///
/// NOTE: `u128` is used since there is no native support for 96-bit integers in Rust and using
/// `[u8;12]` is bad for UX Maybe in the future we can use a custom type for this
pub type Slot = u128;

/// Max value of `Slot`, used for validation
pub const MAX_U96: Slot = (1 << 96) - 1;

/// Asset metadata
#[derive(Encode, Decode, Clone, PartialEq, Eq, sp_core::RuntimeDebug, TypeInfo)]
pub enum AssetMetadata<MaxTokenUriLength: Get<u32>> {
	/// Explicit token URI
	External {
		/// Token URI
		token_uri: BoundedVec<u8, MaxTokenUriLength>,
	},
}
