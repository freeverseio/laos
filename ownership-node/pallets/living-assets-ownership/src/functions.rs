//! Contains helper and utility functions of the pallet
use super::*;
use frame_support::{
	ensure,
    sp_runtime::{DispatchResult}
};

impl<T: Config> Pallet<T> {
    /// See [Self::create_collection]
    pub fn do_create_collection(collection_id: T::CollectionId, who: T::AccountId) -> DispatchResult {
        ensure!(
            !OwnerOfCollection::<T>::contains_key(collection_id),
            Error::<T>::CollectionAlreadyExists
        );

        OwnerOfCollection::<T>::insert(collection_id, &who);

        Self::deposit_event(Event::CollectionCreated { collection_id, who });

        Ok(())
    }
}
