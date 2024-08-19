// Copyright 2023-2024 Freeverse.io
// This file is part of LAOS.

// LAOS is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// LAOS is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with LAOS.  If not, see <http://www.gnu.org/licenses/>.

//! Traits for this pallet

use crate::types::{CollectionId, Slot, TokenId};
use frame_support::pallet_prelude::DispatchResult;
use sp_core::H160;
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
}

pub trait OnCreateCollection {
	fn on_create_collection(address: H160);
}

impl OnCreateCollection for () {
	fn on_create_collection(_address: H160) {}
}
