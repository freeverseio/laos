//! Types used in LivingAssetsOwnership pallet.
use crate::Config;
use sp_core::U256;
use sp_runtime::BoundedVec;

/// Explicit AccountId of the pallet.
pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

/// AssetId type.
pub type AssetId = U256;

/// Collection id type
pub type CollectionId = u64;

/// Base URI type
pub type BaseURIOf<T> = BoundedVec<u8, <T as Config>::BaseURILimit>;
