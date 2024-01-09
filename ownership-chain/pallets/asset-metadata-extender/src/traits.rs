use sp_runtime::DispatchResult;

use crate::{
	types::{AccountIdOf, TokenUriOf, UniversalLocationOf},
	Config,
};

/// `AssetMetadataExtender` trait for managing asset metadata extensions
pub trait AssetMetadataExtender<T: Config> {
	fn create_metadata_extension(
		claimer: AccountIdOf<T>,
		universal_location: UniversalLocationOf<T>,
		token_uri: TokenUriOf<T>,
	) -> DispatchResult;
}
