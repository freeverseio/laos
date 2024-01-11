use sp_runtime::DispatchResult;

use crate::{
	types::{AccountIdOf, TokenUriOf, UniversalLocationOf},
	Config,
};

/// `AssetMetadataExtender` trait for managing asset metadata extensions
pub trait AssetMetadataExtender<T: Config> {
	fn create_token_uri_extension(
		claimer: AccountIdOf<T>,
		universal_location: UniversalLocationOf<T>,
		token_uri: TokenUriOf<T>,
	) -> DispatchResult;

	fn update_token_uri_extension(
		claimer: AccountIdOf<T>,
		universal_location: UniversalLocationOf<T>,
		token_uri: TokenUriOf<T>,
	) -> DispatchResult;

	// fn balance_of_universal_location(universal_location: UniversalLocationOf<T>) -> u32;

	// fn indexed_metadata_extensions(
	// 	universal_location: UniversalLocationOf<T>,
	// 	index: Index,
	// ) -> Option<(AccountIdOf<T>, TokenUriOf<T>)>;
}
