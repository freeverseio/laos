//! Trait implementations
use crate::*;

impl<T: Config> traits::CollectionManager<CollectionId, AccountIdOf<T>, BaseURIOf<T>>
	for Pallet<T>
{
	type Error = Error<T>;

	fn base_uri(collection_id: CollectionId) -> Option<BaseURIOf<T>> {
		CollectionBaseURI::<T>::get(collection_id)
	}

	fn create_collection(
		owner: AccountIdOf<T>,
		base_uri: BaseURIOf<T>,
	) -> Result<CollectionId, Self::Error> {
		Self::do_create_collection(owner, base_uri)
	}
}

impl<T: Config> traits::Erc721 for Pallet<T> {
	type Error = Error<T>;

	fn owner_of(collection_id: CollectionId, asset_id: AssetId) -> Result<H160, Self::Error> {
		ensure!(
			Pallet::<T>::collection_base_uri(collection_id).is_some(),
			Error::CollectionDoesNotExist
		);
		Ok(asset_owner::<T>(collection_id, asset_id))
	}

	fn transfer_from(
		origin: H160,
		collection_id: CollectionId,
		from: H160,
		to: H160,
		asset_id: AssetId,
	) -> Result<(), Self::Error> {
		ensure!(
			Pallet::<T>::collection_base_uri(collection_id).is_some(),
			Error::CollectionDoesNotExist
		);
		ensure!(origin == from, Error::NoPermission);
		ensure!(asset_owner::<T>(collection_id, asset_id) == from, Error::NoPermission);
		ensure!(from != to, Error::CannotTransferSelf);
		ensure!(to != H160::zero(), Error::TransferToNullAddress);

		AssetOwner::<T>::set(collection_id, asset_id, Some(to.clone()));
		Self::deposit_event(Event::AssetTransferred { collection_id, asset_id, to });

		Ok(())
	}

	fn token_uri(collection_id: CollectionId, asset_id: AssetId) -> Result<Vec<u8>, Self::Error> {
		let base_uri =
			Pallet::<T>::collection_base_uri(collection_id).ok_or(Error::CollectionDoesNotExist)?;

		// concatenate base_uri with asset_id
		let mut token_uri = base_uri.to_vec();
		token_uri.push(b'/');
		token_uri.extend_from_slice(asset_id.to_string().as_bytes());
		Ok(token_uri)
	}
}
