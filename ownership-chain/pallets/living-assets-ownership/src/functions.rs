//! Contains helper and utility functions of the pallet
use super::*;
use frame_support::sp_runtime::traits::One;

impl<T: Config> Pallet<T> {
	/// See [Self::create_collection]
	pub fn do_create_collection(
		who: T::AccountId,
		base_uri: BaseURIOf<T>,
	) -> Result<CollectionId, Error<T>> {
		// Retrieve the current collection count to use as the new collection's ID
		let collection_id = Self::collection_counter();

		CollectionBaseURI::<T>::insert(collection_id, base_uri);

		// Attempt to increment the collection counter by 1. If this operation
		// would result in an overflow, return early with an error
		let counter =
			collection_id.checked_add(One::one()).ok_or(Error::<T>::CollectionIdOverflow)?;
		CollectionCounter::<T>::put(counter);

		Self::deposit_event(Event::CollectionCreated { collection_id, who });

		Ok(collection_id)
	}
}
