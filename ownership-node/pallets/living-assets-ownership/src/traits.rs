use sp_core::{H160, U256};

use crate::CollectionId;

/// The `CollectionManager` trait provides an interface for managing collections in a
/// decentralized and non-fungible asset management system. This system allows for the creation of
/// collections, each of which can be owned by a unique `AccountId`.
///
/// A collection in this context can be thought of as a container for non-fungible assets.
/// Each collection has an associated `collection_id` which is a unique identifier for the collection
/// and can be used to retrieve the owner of the collection.
///
/// # Methods
///
/// - `owner_of_collection(collection_id: CollectionId) -> Option<AccountId>`: This method retrieves the owner
/// of a collection given its `collection_id`. If no collection exists with the provided `collection_id`,
/// the method returns `None`.
///
/// - `create_collection(owner: AccountId) -> Result<CollectionId, &'static str>`: This method creates a
/// new collection and assigns ownership to the provided `AccountId`. The method returns the `collection_id`
/// of the newly created collection.
///
pub trait CollectionManager<AccountId> {
	/// Get owner of collection
	fn owner_of_collection(collection_id: CollectionId) -> Option<AccountId>;

	/// Create collection
	fn create_collection(owner: AccountId) -> Result<CollectionId, &'static str>;
}

/// `Erc721` Trait
///
/// This trait provides an interface for handling ERC721 tokens, a standard for non-fungible tokens on the blockchain.
pub trait Erc721 {
	/// Retrieves the owner of a specific asset in a collection.
	///
	/// # Parameters
	///
	/// * `collection_id`: An identifier for the collection to which the asset belongs.
	/// * `asset_id`: The unique identifier for the asset within the specified collection.
	///
	/// # Returns
	///
	/// * A `Result` which is:
	///   - `Ok(H160)`: Returns the Ethereum address (`H160`) of the owner of the asset.
	///   - `Err(&'static str)`: Returns an error message if the asset owner could not be determined.
	fn owner_of(collection_id: CollectionId, asset_id: U256) -> Result<H160, &'static str>;
}
