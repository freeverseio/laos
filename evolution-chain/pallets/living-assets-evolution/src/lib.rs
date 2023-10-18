#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::ensure;
pub use pallet::*;
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

mod traits;
mod types;

use frame_support::pallet_prelude::*;
use sp_core::H160;
use sp_runtime::{
	traits::{Convert, One},
	ArithmeticError, DispatchError,
};

use traits::LivingAssetsEvolution;
use types::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// Converts `Self::AccountId` to `H160`
		type AccountIdToH160: Convert<Self::AccountId, H160>;
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
		MintedWithExternalTokenURI {
			collection_id: CollectionId,
			slot: Slot,
			to: AccountIdOf<T>,
			token_uri: TokenUriOf<T>,
			token_id: TokenId,
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
		/// This happens when `Slot` is larger than 96 bits
		SlotOverflow,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {}
}

impl<T: Config> LivingAssetsEvolution<AccountIdOf<T>, TokenUriOf<T>> for Pallet<T> {
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
		ensure!(CollectionOwner::<T>::get(collection_id) == Some(who), Error::<T>::NoPermission);

		let to_as_h160 = T::AccountIdToH160::convert(to.clone());
		// compose asset_id	from slot and owner
		let token_id =
			slot_and_owner_to_token_id(slot, to_as_h160).ok_or(Error::<T>::SlotOverflow)?;

		ensure!(TokenURI::<T>::get(collection_id, token_id).is_none(), Error::<T>::AlreadyMinted);

		TokenURI::<T>::insert(collection_id, token_id, token_uri.clone());

		Self::deposit_event(Event::MintedWithExternalTokenURI {
			collection_id,
			slot,
			to,
			token_id,
			token_uri,
		});

		Ok(token_id)
	}
}

/// Converts `Slot` and `H160` to `TokenId`
///
/// Every slot is identified by a unique `token_id` where `token_id = concat(slot #,
/// owner_address)`
///
/// Returns `None` if `Slot` is larger than 96 bits
fn slot_and_owner_to_token_id(slot: Slot, owner: H160) -> Option<TokenId> {
	// Check if slot is larger than 96 bits
	if slot > MAX_U96 {
		return None
	}

	let mut bytes = [0u8; 32];

	let slot_bytes = slot.to_be_bytes();

	// we also use the last 12 bytes of the slot, since the first 4 bytes are always 0
	bytes[..12].copy_from_slice(&slot_bytes[4..]);
	bytes[12..].copy_from_slice(&owner.0);

	Some(TokenId::from(bytes))
}
