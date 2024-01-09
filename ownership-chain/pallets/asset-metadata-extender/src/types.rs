//! Types used in the pallet
use frame_support::pallet_prelude::*;
use sp_runtime::BoundedVec;

/// Wrapper around `BoundedVec` for `TokenUri`
pub type TokenUriOf<T> = BoundedVec<u8, <T as crate::Config>::MaxTokenUriLength>;

/// Wrapper around `BoundedVec` for `UniversalLocation`
pub type UniversalLocationOf<T> = BoundedVec<u8, <T as crate::Config>::MaxUniversalLocationLength>;

/// Serves as a position identifier for elements in a collection, facilitating iteration
/// and element access. It corresponds to the element count, ranging from 0 to N-1 in a collection
/// of size N.
pub type Index = u32;

/// Asset metadata extension
/// Contains the claimer account and token URI
#[derive(Encode, Decode, MaxEncodedLen, TypeInfo, Debug, PartialEq, Eq, Clone)]
#[scale_info(skip_type_params(T))]
pub struct MetadataExtension<T: crate::Config> {
	pub claimer: AccountIdOf<T>,
	pub token_uri: TokenUriOf<T>,
}

/// Explicit `AccountId`
pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
