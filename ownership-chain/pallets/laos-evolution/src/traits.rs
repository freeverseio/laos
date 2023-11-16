//! Traits for this pallet

use crate::types::{CollectionId, Slot, TokenId};
use sp_runtime::DispatchError;

/// `EvolutionCollectionFactory` trait for creating collections
pub trait EvolutionCollectionFactory<AccountId> {
	/// Creates new collection
	fn create_collection(owner: AccountId) -> Result<CollectionId, DispatchError>;
}

/// `CollectionEvolverAndMinter` trait for managing living assets within a collection
pub trait CollectionEvolverAndMinter<AccountId, TokenUri> {
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
}
