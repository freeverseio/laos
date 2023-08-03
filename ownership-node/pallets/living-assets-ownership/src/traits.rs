/// The `LivingAssetsOwnership` trait provides an interface for managing collections in a
/// decentralized and non-fungible asset management system. This system allows for the creation of
/// collections, each of which can be owned by a unique `AccountId`.
///
/// A collection in this context can be thought of as a container for non-fungible assets.
/// Each collection has an associated `collection_id` which is a unique identifier for the collection
/// and can be used to retrieve the owner of the collection.
///
/// # Methods
///
/// - `owner_of_collection(collection_id: T::CollectionId) -> Option<AccountId>`: This method retrieves the owner
/// of a collection given its `collection_id`. If no collection exists with the provided `collection_id`,
/// the method returns `None`.
///
/// - `create_collection(collection_id: T::CollectionId, who: AccountId) -> DispatchResult`: This method creates a
/// new collection with the specified `collection_id` and assigns ownership to the provided `AccountId`.
/// If a collection already exists with the provided `collection_id`, the method will return an error.
///
/// # Errors
///
/// - `CollectionAlreadyExists`: This error is returned by the `create_collection` method when a collection
/// with the provided `collection_id` already exists.
///
pub trait LivingAssetsOwnership<AccountId, CollectionId> {
	/// Get owner of collection
	fn owner_of_collection(collection_id: CollectionId) -> Option<AccountId>;

	/// Create collection
	fn create_collection(owner: AccountId) -> Result<CollectionId, &'static str>;
}
