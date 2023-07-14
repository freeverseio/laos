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
    }

    /// Mapping from collection id to owner
    #[pallet::storage]
    #[pallet::getter(fn owner_of_collection)]
    pub(super) type OwnerOfCollection<T: Config> =
        StorageMap<_, Blake2_128Concat, u64, T::AccountId, OptionQuery>;

    /// Pallet events
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Collection created
        /// parameters. [collection_id, who]
        CollectionCreated {
            collection_id: u64,
            who: T::AccountId,
        },
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
        pub fn create_collection(origin: OriginFor<T>, collection_id: u64) -> DispatchResult {
            let who = ensure_signed(origin)?;

            ensure!(
                !OwnerOfCollection::<T>::contains_key(collection_id),
                Error::<T>::CollectionAlreadyExists
            );

            OwnerOfCollection::<T>::insert(collection_id, &who);

            Self::deposit_event(Event::CollectionCreated { collection_id, who });

            Ok(())
        }
    }
}
