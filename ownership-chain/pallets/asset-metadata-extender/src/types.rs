//! Types used in the pallet
use sp_core::U256;
use sp_runtime::BoundedVec;

/// Wrapper around `BoundedVec` for `tokenUri`
pub type TokenUriOf<T> = BoundedVec<u8, <T as crate::Config>::MaxTokenUriLength>;
