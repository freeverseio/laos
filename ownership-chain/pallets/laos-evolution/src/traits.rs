//! Traits for this pallet

use crate::types::{CollectionId, Slot, TokenId};
use frame_support::pallet_prelude::DispatchResult;
use sp_runtime::DispatchError;

/// `EvolutionCollectionFactory` trait for managing collections
pub trait EvolutionCollectionFactory<AccountId> {
	/// Creates new collection
	fn create_collection(owner: AccountId) -> Result<CollectionId, DispatchError>;
}

/// `EvolutionCollection` trait for managing living assets within a collection
pub trait EvolutionCollection<AccountId, TokenUri> {
	/// Mint new token with external URI
	fn mint_with_external_uri(
		who: AccountId,
		collection_id: CollectionId,
		slot: Slot,
		to: AccountId,
		token_uri: TokenUri,
	) -> Result<TokenId, DispatchError>;

	/// Get owner of the collection
	fn collection_owner(collection_id: CollectionId) -> Option<AccountId>;

	/// Get token URI of a token in a collection
	fn token_uri(collection_id: CollectionId, token_id: TokenId) -> Option<TokenUri>;

	/// Evolve token with external URI
	fn evolve_with_external_uri(
		who: AccountId,
		collection_id: CollectionId,
		token_id: TokenId,
		token_uri: TokenUri,
	) -> Result<(), DispatchError>;

	/// Transfer ownership of the collection
	fn transfer_ownership(
		from: AccountId,
		to: AccountId,
		collection_id: CollectionId,
	) -> DispatchResult;

	/// Enables public minting for a specific collection.
	fn enable_public_minting(who: AccountId, collection_id: CollectionId) -> DispatchResult;

	/// Disables public minting for a specific collection.
	fn disable_public_minting(who: AccountId, collection_id: CollectionId) -> DispatchResult;

	/// Checks if public minting is enabled for a specific collection.
	fn is_public_minting_enabled(collection_id: CollectionId) -> bool;
}
