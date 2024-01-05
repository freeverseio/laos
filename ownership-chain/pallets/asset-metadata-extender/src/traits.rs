use sp_runtime::DispatchError;

/// `AssetMetadataExtender` trait for managing asset metadata extensions
pub trait AssetMetadataExtender<AccountId, TokenUri, UniversalLocation> {
	/// Creates new asset metadata extension
	fn create_metadata_extension(
		claimer: AccountId,
		universal_location: UniversalLocation,
		token_uri: TokenUri,
	) -> Result<(), DispatchError>;
}
