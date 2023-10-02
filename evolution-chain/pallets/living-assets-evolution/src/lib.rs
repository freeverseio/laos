#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
mod types;
pub mod weights;

use types::*;
pub use weights::*;

use sp_core::H160;
use sp_runtime::traits::Convert;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{pallet_prelude::*, sp_runtime::traits::One};
	use frame_system::pallet_prelude::*;
	use sp_runtime::ArithmeticError;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// Limit for the length of `token_uri`
		#[pallet::constant]
		type MaxTokenUriLength: Get<u32>;
		/// Type representing the weight of this pallet
		type WeightInfo: WeightInfo;
		/// Converts [`AccountId`] to [`H160`]
		type AccountIdToH160: Convert<AccountIdOf<Self>, H160>;
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
			to: SlotOwnerId,
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
		/// [`Slot`] overflow
		SlotOverflow,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// The `create_collection` extrinsic allows users to create a new collection.
		///
		/// # Parameters
		///
		/// - `origin`: The origin account sending the extrinsic.
		/// - `owner`: The account that will be set as the owner of the new collection.
		///
		/// # Storage Changes
		///
		/// - [`CollectionOwner`](`CollectionOwner`): Inserts a new mapping from the generated
		///   `collection_id` to the `owner` account.
		/// - [`CollectionCounter`](`CollectionCounter`): Updates the counter for the next available
		///   `collection_id`.
		///
		/// # Events
		///
		/// Emits a [`CollectionCreated`](`Event::<T>::CollectionCreated`) event upon successful
		/// execution.
		///
		/// # Errors
		///
		/// - Returns [`Overflow`](`ArithmeticError::<T>::Overflow`) if incrementing the
		///   `collection_id` counter would result in an overflow.
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::create_collection())]
		pub fn create_collection(origin: OriginFor<T>, owner: T::AccountId) -> DispatchResult {
			ensure_signed(origin)?;

			let collection_id = Self::collection_counter();

			CollectionOwner::<T>::insert(collection_id, owner.clone());

			// Attempt to increment the collection counter by 1. If this operation
			// would result in an overflow, return early with an error
			let counter = collection_id.checked_add(One::one()).ok_or(ArithmeticError::Overflow)?;
			CollectionCounter::<T>::put(counter);

			// Emit an event.
			Self::deposit_event(Event::CollectionCreated { collection_id, owner });

			// Return a successful DispatchResult
			Ok(())
		}

		/// Mint new asset with external URI
		///
		/// This function performs the minting of a new asset with setting its external URI.
		///
		/// NOTE: This function will panic if the `slot` has a value greater than `2^96 - 1`
		/// This will be fixed in the future https://github.com/freeverseio/laos-evolution-node/issues/77
		///
		/// # Errors
		///
		///  This function returns a dispatch error in the following cases:
		///
		/// * [`NoPermission`](`Error::<T>::NoPermission`) - if the caller is not the owner of the
		///   collection
		/// * [`CollectionDoesNotExist`](`Error::<T>::CollectionDoesNotExist`) - if the collection
		///   does not exist
		/// * [`AlreadyMinted`](`Error::<T>::AlreadyMinted`) - if the asset is already minted
		/// * [`Overflow`](`ArithmeticError::<T>::Overflow`) - if the `slot` is greater than `2^96 -
		///   1`
		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::mint_with_external_uri())]
		pub fn mint_with_external_uri(
			origin: OriginFor<T>,
			collection_id: CollectionId,
			slot: Slot,
			to: SlotOwnerId,
			token_uri: TokenUriOf<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(
				CollectionOwner::<T>::contains_key(collection_id),
				Error::<T>::CollectionDoesNotExist
			);

			ensure!(
				CollectionOwner::<T>::get(collection_id) == Some(who),
				Error::<T>::NoPermission
			);

			// compose asset_id	from slot and owner
			let token_id = Self::slot_and_owner_to_token_id((slot, to.clone()))?;

			ensure!(
				TokenURI::<T>::get(collection_id, token_id).is_none(),
				Error::<T>::AlreadyMinted
			);

			TokenURI::<T>::insert(collection_id, token_id, token_uri.clone());

			Self::deposit_event(Event::MintedWithExternalTokenURI {
				collection_id,
				slot,
				to,
				token_id,
				token_uri,
			});

			Ok(())
		}
	}
}

impl<T: Config> Pallet<T> {
	// Utility functions
	/// A struct responsible for converting `Slot` and `AccountId` to `TokenId`
	///
	/// Every slot is identified by a unique `token_id` where `token_id = concat(slot #,
	/// owner_address)`
	fn slot_and_owner_to_token_id(
		slot_and_owner: (Slot, SlotOwnerId),
	) -> Result<TokenId, pallet::Error<T>> {
		let (slot, owner) = slot_and_owner;

		// Check if slot is larger than 96 bits
		if slot >= (1_u128 << 96) {
			return Err(Error::<T>::SlotOverflow)
		}

		let mut bytes = [0u8; 32];

		let slot_bytes = slot.to_be_bytes();

		// we also use the last 12 bytes of the slot, since the first 4 bytes are always 0
		bytes[..12].copy_from_slice(&slot_bytes[4..]);

		let account_id_bytes = owner.as_fixed_bytes();

		bytes[12..].copy_from_slice(account_id_bytes);

		Ok(TokenId::from(bytes))
	}
}
