use sp_runtime::DispatchError;

/// `AssetMetadataExtender` trait for managing asset extensions
pub trait AssetMetadataExtender<AccountId, TokenUri, UniversalLocation> {
	/// Creates new extension
	fn create_extension(
		claimer: AccountId,
		universal_location: UniversalLocation,
		token_uri: TokenUri,
	) -> Result<(), DispatchError>;
}
