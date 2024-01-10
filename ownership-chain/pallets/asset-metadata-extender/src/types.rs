//! Types used in the pallet
use sp_runtime::BoundedVec;

/// Wrapper around `BoundedVec` for `TokenUri`
pub type TokenUriOf<T> = BoundedVec<u8, <T as crate::Config>::MaxTokenUriLength>;

/// Wrapper around `BoundedVec` for `UniversalLocation`
pub type UniversalLocationOf<T> = BoundedVec<u8, <T as crate::Config>::MaxUniversalLocationLength>;

/// Serves as a position identifier for elements in a collection, facilitating iteration
/// and element access. It corresponds to the element count, ranging from 0 to N-1 in a collection
/// of size N.
pub type Index = u32;

/// Explicit `AccountId`
pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
