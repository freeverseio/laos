use sp_runtime::DispatchError;

/// `AssetMetadataExtender` trait for managing asset extensions
pub trait AssetMetadataExtender {
	/// Creates new extension
	fn create_extension(
		claimer: u32,
		universal_location: u32,
		token_uri: u32,
	) -> Result<(), DispatchError>;
}
