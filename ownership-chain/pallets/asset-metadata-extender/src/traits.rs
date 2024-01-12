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

	fn extension_by_location_and_claimer(
		claimer: AccountIdOf<T>,
		universal_location: UniversalLocationOf<T>,
	) -> Option<TokenUriOf<T>>;

	fn has_extension(universal_location: UniversalLocationOf<T>, claimer: AccountIdOf<T>) -> bool;
}
