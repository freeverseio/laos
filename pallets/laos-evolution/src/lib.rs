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

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::ensure;
pub use pallet::*;
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

mod benchmarking;
pub mod precompiles;
pub mod traits;
pub mod types;
pub mod weights;

use frame_support::pallet_prelude::*;
use sp_core::H160;
use sp_runtime::{
	traits::{Convert, ConvertBack, One},
	ArithmeticError, DispatchError,
};

use pallet_evm::GasWeightMapping;
pub use traits::{EvolutionCollection, EvolutionCollectionFactory, OnCreateCollection};
pub use types::*;
pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	/// The current storage version.
	const STORAGE_VERSION: StorageVersion = StorageVersion::new(0);

	#[pallet::pallet]
	#[pallet::storage_version(STORAGE_VERSION)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// Converts `Self::AccountId` to `H160`
		type AccountIdToH160: ConvertBack<Self::AccountId, H160>;
		/// The weight information of this pallet.
		type WeightInfo: WeightInfo;
		/// Gas weight mapping
		type GasWeightMapping: GasWeightMapping;
		/// Callback for creating a new collection
		type OnCreateCollection: OnCreateCollection;
		/// Limit for the length of `token_uri`
		#[pallet::constant]
		type MaxTokenUriLength: Get<u32>;
	}

	/// Collection counter
	#[pallet::storage]
	#[pallet::getter(fn collection_counter)]
	pub(super) type CollectionCounter<T: Config> = StorageValue<_, CollectionId, ValueQuery>;

	/// Storage for the ownership of collections
	#[pallet::storage]
	#[pallet::getter(fn collection_owner)]
	pub type CollectionOwner<T: Config> =
		StorageMap<_, Blake2_128Concat, CollectionId, AccountIdOf<T>, OptionQuery>;

	/// Token URI which can override the default URI scheme and set explicitly
	/// This will contain external URI in a raw form
	#[pallet::storage]
	#[pallet::getter(fn token_uri)]
	pub type TokenURI<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		CollectionId,
		Blake2_128Concat,
		TokenId,
		TokenUriOf<T>,
		OptionQuery,
	>;

	/// Events for this pallet.
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Collection created
		/// parameters. [collection_id, who]
		CollectionCreated { collection_id: CollectionId, owner: AccountIdOf<T> },
		/// Asset minted
		/// [collection_id, slot, to, token_uri]
		MintedWithExternalURI {
			collection_id: CollectionId,
			slot: Slot,
			to: AccountIdOf<T>,
			token_uri: TokenUriOf<T>,
			token_id: TokenId,
		},
		/// Asset evolved
		/// [collection_id, token_uri, token_id]
		EvolvedWithExternalURI {
			collection_id: CollectionId,
			token_id: TokenId,
			token_uri: TokenUriOf<T>,
		},
		/// Ownership of collection was transferred
		/// [collection_id, from, to]
		CollectionTransferred {
			collection_id: CollectionId,
			from: AccountIdOf<T>,
			to: AccountIdOf<T>,
		},
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	#[derive(PartialEq)]
	pub enum Error<T> {
		/// Collection does not exist
		CollectionDoesNotExist,
		/// Not the owner of the collection
		NoPermission,
		/// [`Slot`] is already minted
		AlreadyMinted,
		/// Asset does not exist
		AssetDoesNotExist,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {}

	impl<T: Config> Pallet<T> {
		pub fn is_owner(collection_id: CollectionId, who: T::AccountId) -> bool {
			CollectionOwner::<T>::get(collection_id) == Some(who)
		}
	}
}

impl<T: Config> EvolutionCollectionFactory<AccountIdOf<T>> for Pallet<T> {
	fn create_collection(owner: AccountIdOf<T>) -> Result<CollectionId, DispatchError> {
		let collection_id = Self::collection_counter();

		CollectionOwner::<T>::insert(collection_id, owner.clone());

		// Attempt to increment the collection counter by 1. If this operation
		// would result in an overflow, return early with an error
		let counter = collection_id.checked_add(One::one()).ok_or(ArithmeticError::Overflow)?;
		CollectionCounter::<T>::put(counter);

		// Emit an event.
		Self::deposit_event(Event::CollectionCreated { collection_id, owner });

		Ok(collection_id)
	}
}

impl<T: Config> EvolutionCollection<AccountIdOf<T>, TokenUriOf<T>> for Pallet<T> {
	fn mint_with_external_uri(
		who: AccountIdOf<T>,
		collection_id: CollectionId,
		slot: Slot,
		to: AccountIdOf<T>,
		token_uri: TokenUriOf<T>,
	) -> Result<TokenId, DispatchError> {
		ensure!(
			CollectionOwner::<T>::contains_key(collection_id),
			Error::<T>::CollectionDoesNotExist
		);
		ensure!(Self::is_owner(collection_id, who), Error::<T>::NoPermission);

		let to_as_h160 = T::AccountIdToH160::convert(to.clone());
		// compose asset_id	from slot and owner
		let token_id = slot_and_owner_to_token_id(slot, to_as_h160);

		ensure!(TokenURI::<T>::get(collection_id, token_id).is_none(), Error::<T>::AlreadyMinted);

		TokenURI::<T>::insert(collection_id, token_id, token_uri.clone());

		Self::deposit_event(Event::MintedWithExternalURI {
			collection_id,
			slot,
			to,
			token_id,
			token_uri,
		});

		Ok(token_id)
	}

	fn collection_owner(collection_id: CollectionId) -> Option<AccountIdOf<T>> {
		CollectionOwner::<T>::get(collection_id)
	}

	fn token_uri(collection_id: CollectionId, token_id: TokenId) -> Option<TokenUriOf<T>> {
		TokenURI::<T>::get(collection_id, token_id)
	}

	fn evolve_with_external_uri(
		who: AccountIdOf<T>,
		collection_id: CollectionId,
		token_id: TokenId,
		token_uri: TokenUriOf<T>,
	) -> Result<(), DispatchError> {
		ensure!(
			CollectionOwner::<T>::contains_key(collection_id),
			Error::<T>::CollectionDoesNotExist
		);
		ensure!(Self::is_owner(collection_id, who), Error::<T>::NoPermission);
		ensure!(
			TokenURI::<T>::contains_key(collection_id, token_id),
			Error::<T>::AssetDoesNotExist
		);

		TokenURI::<T>::insert(collection_id, token_id, token_uri.clone());

		Self::deposit_event(Event::EvolvedWithExternalURI { collection_id, token_id, token_uri });

		Ok(())
	}

	fn transfer_ownership(
		from: AccountIdOf<T>,
		to: AccountIdOf<T>,
		collection_id: CollectionId,
	) -> DispatchResult {
		CollectionOwner::<T>::mutate(collection_id, |owner| -> DispatchResult {
			ensure!(owner.is_some(), Error::<T>::CollectionDoesNotExist);
			ensure!(owner.clone() == Some(from.clone()), Error::<T>::NoPermission);

			*owner = Some(to.clone());

			Self::deposit_event(Event::CollectionTransferred { collection_id, from, to });

			Ok(())
		})
	}
}

/// Converts `Slot` and `H160` to `TokenId`
///
/// Every slot is identified by a unique `token_id` where `token_id = concat(slot #,
/// owner_address)`
///
/// Returns `Slot`
fn slot_and_owner_to_token_id(slot: Slot, owner: H160) -> TokenId {
	let mut bytes = [0u8; 32];

	let slot_bytes = slot.to_be_bytes();

	// Copy the slot into the first 12 bytes of the array
	bytes[..12].copy_from_slice(&slot_bytes);
	// Copy the owner address bytes into the array
	bytes[12..].copy_from_slice(&owner.0);

	TokenId::from(bytes)
}

/// `ASSET_PRECOMPILE_ADDRESS_PREFIX` is a predefined prefix used to identify collection addresses.
///
/// All addresses that start with this prefix are considered as collection addresses.
/// Since `CollectionId` is represented as a `u64`, it leaves these bits free to be
/// utilized for such a prefix.
///
/// Usage of this prefix provides a consistent and recognizable pattern for distinguishing
/// collection addresses from other types of addresses in the system.
pub const ASSET_PRECOMPILE_ADDRESS_PREFIX: &[u8] =
	&[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xfe];

/// Enum representing possible errors related to collections.
#[derive(Debug, PartialEq)]
pub enum CollectionError {
	/// Error indicating that the provided address does not have the correct prefix.
	InvalidPrefix,
}
/// Converts a `CollectionId` into an `Address`.
///
/// This function takes the given `CollectionId`, which is assumed to be a `u64`,
/// and maps it into an `Address` address, prepending it with the `ASSET_PRECOMPILE_ADDRESS_PREFIX`.
///
/// # Arguments
///
/// * `collection_id`: The ID of the collection to be converted.
///
/// # Returns
///
/// * An `Address` representation of the collection ID.
pub fn collection_id_to_address<Address: From<[u8; 20]>>(collection_id: CollectionId) -> Address {
	let mut bytes = [0u8; 20];
	bytes[12..20].copy_from_slice(&collection_id.to_be_bytes());
	for (i, byte) in ASSET_PRECOMPILE_ADDRESS_PREFIX.iter().enumerate() {
		bytes[i] = *byte;
	}
	Address::from(bytes)
}

/// Converts an `Address` address into a `CollectionId` format.
///
/// This function takes the given `Address` address, checks for the correct prefix, and extracts
/// the `CollectionId` from it. If the prefix is incorrect, it returns a
/// `CollectionError::InvalidPrefix` error.
///
/// # Arguments
///
/// * `address`: The `Address` address to be converted.
///
/// # Returns
///
/// * A `Result` which is either the `CollectionId` or an error indicating the address is invalid.
pub fn address_to_collection_id<Address>(address: Address) -> Result<CollectionId, CollectionError>
where
	Address: Into<[u8; 20]>,
{
	let address_bytes: [u8; 20] = address.into();
	if &address_bytes[0..12] != ASSET_PRECOMPILE_ADDRESS_PREFIX {
		return Err(CollectionError::InvalidPrefix);
	}
	let mut id_bytes = [0u8; 8];
	id_bytes.copy_from_slice(&address_bytes[12..]);

	Ok(CollectionId::from_be_bytes(id_bytes))
}
