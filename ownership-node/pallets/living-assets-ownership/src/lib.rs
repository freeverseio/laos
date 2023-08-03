#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

mod functions;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::{OptionQuery, *};
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// Collection id type
		type CollectionId: Member + Parameter + MaxEncodedLen + Copy;
	}

	/// Mapping from collection id to owner
	#[pallet::storage]
	#[pallet::getter(fn owner_of_collection)]
	pub(super) type OwnerOfCollection<T: Config> =
		StorageMap<_, Blake2_128Concat, T::CollectionId, T::AccountId, OptionQuery>;

	/// Pallet events
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Collection created
		/// parameters. [collection_id, who]
		CollectionCreated { collection_id: T::CollectionId, who: T::AccountId },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Collection already exists
		CollectionAlreadyExists,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())] // TODO set proper weight
		pub fn create_collection(
			origin: OriginFor<T>,
			collection_id: T::CollectionId,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::do_create_collection(collection_id, who)
		}
	}

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
		fn create_collection(collection_id: CollectionId, who: AccountId) -> DispatchResult;
		fn create_collection2(owner: AccountId) -> Result<CollectionId, &'static str>;
	}

	impl<T: Config> LivingAssetsOwnership<T::AccountId, T::CollectionId> for Pallet<T> {
		fn owner_of_collection(collection_id: T::CollectionId) -> Option<T::AccountId> {
			OwnerOfCollection::<T>::get(collection_id)
		}

		fn create_collection(collection_id: T::CollectionId, who: T::AccountId) -> DispatchResult {
			Self::do_create_collection(collection_id, who)
		}

		fn create_collection2(_owner: T::AccountId) -> Result<T::CollectionId, &'static str> {
			todo!();
		}
	}
}
