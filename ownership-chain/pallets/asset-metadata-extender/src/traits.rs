use sp_runtime::DispatchResult;

use crate::{
	types::{AccountIdOf, TokenUriOf, UniversalLocationOf},
	Config,
};

/// `AssetMetadataExtender` trait for managing asset metadata extensions
pub trait AssetMetadataExtender<T: Config> {
	/// Create the token uri extension for a given universal location
	fn create_token_uri_extension(
		claimer: AccountIdOf<T>,
		universal_location: UniversalLocationOf<T>,
		token_uri: TokenUriOf<T>,
	) -> DispatchResult;

	/// Update the token uri extension of a given universal location
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

	/// Get the number of extensions for a given universal location
	fn balance_of(universal_location: UniversalLocationOf<T>) -> u32;

	/// Get claimer of a given universal location using indexation
	fn claimer_by_index(
		universal_location: UniversalLocationOf<T>,
		index: u32,
	) -> Option<AccountIdOf<T>>;

	/// Get the token uri of a given universal location using indexation
	fn token_uri_extension_by_index(
		universal_location: UniversalLocationOf<T>,
		index: u32,
	) -> Option<TokenUriOf<T>>;
}
