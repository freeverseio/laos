#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

mod functions;
pub mod traits;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{
		pallet_prelude::{OptionQuery, ValueQuery, *},
		sp_runtime::traits::{CheckedAdd, One},
	};
	use frame_system::pallet_prelude::*;
	use sp_arithmetic::traits::Unsigned;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// Collection id type
		type CollectionId: Member
			+ Parameter
			+ MaxEncodedLen
			+ Copy
			+ Default
			+ CheckedAdd
			+ One
			+ Unsigned;
	}

	/// Mapping from collection id to owner
	#[pallet::storage]
	#[pallet::getter(fn owner_of_collection)]
	pub(super) type OwnerOfCollection<T: Config> =
		StorageMap<_, Blake2_128Concat, T::CollectionId, T::AccountId, OptionQuery>;

	/// Collection counter
	#[pallet::storage]
	#[pallet::getter(fn collection_counter)]
	pub(super) type CollectionCounter<T: Config> = StorageValue<_, T::CollectionId, ValueQuery>;

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
		/// Collection id overflow
		CollectionIdOverflow,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())] // TODO set proper weight
		pub fn create_collection(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			match Self::do_create_collection(who) {
				Ok(_) => Ok(()),
				Err(err) => Err(err.into()),
			}
		}
	}

	impl<T: Config> traits::CollectionManager<T::AccountId, T::CollectionId> for Pallet<T> {
		fn owner_of_collection(collection_id: T::CollectionId) -> Option<T::AccountId> {
			OwnerOfCollection::<T>::get(collection_id)
		}

		fn create_collection(owner: T::AccountId) -> Result<T::CollectionId, &'static str> {
			Self::do_create_collection(owner)
		}
	}
}
