//! Types used in the pallet
use sp_core::U256;
use sp_runtime::BoundedVec;

/// Collection id type
pub type CollectionId = u64;

/// Explicit `AccountId`
pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

/// Wrapper around `BoundedVec` for `tokenUri`
pub type TokenUriOf<T> = BoundedVec<u8, <T as crate::Config>::MaxTokenUriLength>;

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
