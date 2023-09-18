#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_support::sp_runtime::traits::One;
	use frame_system::pallet_prelude::*;

	/// Collection id type
	pub type CollectionId = u64;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// Type representing the weight of this pallet
		type WeightInfo: WeightInfo;
	}

	/// Collection counter
	#[pallet::storage]
	#[pallet::getter(fn collection_counter)]
	pub(super) type CollectionCounter<T: Config> = StorageValue<_, CollectionId, ValueQuery>;

	// storage for the ownership of collections
	#[pallet::storage]
	#[pallet::getter(fn collection_owner)]
	pub type CollectionOwner<T: Config> =
		StorageMap<_, Blake2_128Concat, CollectionId, T::AccountId, OptionQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Collection created
		/// parameters. [collection_id, who]
		CollectionCreated { collection_id: CollectionId, who: T::AccountId },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// The collection ID counter has overflowed
		CollectionIdOverflow,
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
		/// - `origin`: The origin account sending the extrinsic, which will be set as the owner of the new collection.
		///
		/// # Storage Changes
		///
		/// - `CollectionOwner`: Inserts a new mapping from the generated `collection_id` to the `origin` account.
		/// - `CollectionCounter`: Updates the counter for the next available `collection_id`.
		///
		/// # Events
		///
		/// Emits a `CollectionCreated` event upon successful execution.
		///
		/// # Errors
		///
		/// - Returns `CollectionIdOverflow` if incrementing the `collection_id` counter would result in an overflow.
		#[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn create_collection(origin: OriginFor<T>) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/main-docs/build/origins/
			let who = ensure_signed(origin)?;

			let collection_id = Self::collection_counter();

			CollectionOwner::<T>::insert(collection_id, who.clone());

			// Attempt to increment the collection counter by 1. If this operation
			// would result in an overflow, return early with an error
			let counter =
				collection_id.checked_add(One::one()).ok_or(Error::<T>::CollectionIdOverflow)?;
			CollectionCounter::<T>::put(counter);

			// Emit an event.
			Self::deposit_event(Event::CollectionCreated { collection_id, who });

			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}
	}
}
