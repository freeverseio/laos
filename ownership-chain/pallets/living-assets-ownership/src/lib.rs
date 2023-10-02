#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
use parity_scale_codec::alloc::string::ToString;
use sp_std::vec::Vec;

pub use pallet::*;
pub use types::*;

mod functions;
pub mod traits;
mod types;
#[frame_support::pallet]
pub mod pallet {

	use super::*;
	use frame_support::pallet_prelude::{OptionQuery, ValueQuery, *};
	use sp_runtime::traits::Convert;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Specifies the advised maximum length for a Base URI.
		///
		/// The URI standard (RFC 3986) doesn't dictates a limit for the length of URIs.
		/// However it seems the max supported length in browsers is 2,048 characters.
		///
		/// The base should be capped at 2,015 characters in length. This ensures room for
		/// the token URI formation, where it combines `BaseURILimit`, a `'/'`, and a `AssetId`
		/// (which takes up 33 characters).
		#[pallet::constant]
		type BaseURILimit: Get<u32>;

		/// Simply a null address in EVM.
		#[pallet::constant]
		type NullAddress: Get<AccountIdOf<Self>>;

		/// Type alias for implementing the `AssetIdToInitialOwner` trait for a given account ID
		/// type. This allows you to specify which account should initially own each new asset.
		type AssetIdToInitialOwner: Convert<AssetId, Self::AccountId>;
	}

	/// Collection counter
	#[pallet::storage]
	#[pallet::getter(fn collection_counter)]
	pub(super) type CollectionCounter<T: Config> = StorageValue<_, CollectionId, ValueQuery>;

	/// Collection base URI
	#[pallet::storage]
	#[pallet::getter(fn collection_base_uri)]
	pub(super) type CollectionBaseURI<T: Config> =
		StorageMap<_, Blake2_128Concat, CollectionId, BaseURIOf<T>, OptionQuery>;

	/// Asset owner
	#[pallet::storage]
	pub(super) type AssetOwner<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		CollectionId,
		Blake2_128Concat,
		AssetId,
		AccountIdOf<T>,
		OptionQuery,
	>;

	fn asset_owner<T: Config>(collection_id: CollectionId, asset_id: AssetId) -> AccountIdOf<T> {
		AssetOwner::<T>::get(collection_id, asset_id)
			.unwrap_or_else(|| T::AssetIdToInitialOwner::convert(asset_id))
	}

	/// Pallet events
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Collection created
		/// parameters. [collection_id, who]
		CollectionCreated { collection_id: CollectionId, who: AccountIdOf<T> },
		/// Asset transferred to `who`
		/// parameters. [collection_id, asset_id, who]
		AssetTransferred { collection_id: CollectionId, asset_id: AssetId, to: AccountIdOf<T> },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	#[derive(PartialEq)]
	pub enum Error<T> {
		/// Collection id overflow
		CollectionIdOverflow,
		/// Collection does not exist
		CollectionDoesNotExist,
		// NoPermission,
		NoPermission,
		// AssetDoesNotExist,
		AssetDoesNotExist,
		// CannotTransferSelf,
		CannotTransferSelf,
		// TransferToNullAddress,
		TransferToNullAddress,
	}

	impl<T: Config> AsRef<[u8]> for Error<T> {
		fn as_ref(&self) -> &[u8] {
			match self {
				Error::__Ignore(_, _) => b"__Ignore",
				Error::CollectionIdOverflow => b"CollectionIdOverflow",
				Error::CollectionDoesNotExist => b"CollectionDoesNotExist",
				Error::NoPermission => b"NoPermission",
				Error::AssetDoesNotExist => b"AssetDoesNotExist",
				Error::CannotTransferSelf => b"CannotTransferSelf",
				Error::TransferToNullAddress => b"TransferToNullAddress",
			}
		}
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {}

	impl<T: Config> traits::CollectionManager<AccountIdOf<T>, BaseURIOf<T>> for Pallet<T> {
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

	impl<T: Config> traits::Erc721<AccountIdOf<T>> for Pallet<T> {
		type Error = Error<T>;

		fn owner_of(
			collection_id: CollectionId,
			asset_id: AssetId,
		) -> Result<AccountIdOf<T>, Self::Error> {
			ensure!(
				Pallet::<T>::collection_base_uri(collection_id).is_some(),
				Error::CollectionDoesNotExist
			);
			Ok(asset_owner::<T>(collection_id, asset_id))
		}

		fn transfer_from(
			origin: AccountIdOf<T>,
			collection_id: CollectionId,
			from: AccountIdOf<T>,
			to: AccountIdOf<T>,
			asset_id: AssetId,
		) -> Result<(), Self::Error> {
			ensure!(
				Pallet::<T>::collection_base_uri(collection_id).is_some(),
				Error::CollectionDoesNotExist
			);
			ensure!(origin == from, Error::NoPermission);
			ensure!(asset_owner::<T>(collection_id, asset_id) == from, Error::NoPermission);
			ensure!(from != to, Error::CannotTransferSelf);
			ensure!(to != T::NullAddress::get(), Error::TransferToNullAddress);

			AssetOwner::<T>::set(collection_id, asset_id, Some(to.clone()));
			Self::deposit_event(Event::AssetTransferred { collection_id, asset_id, to });

			Ok(())
		}

		fn token_uri(
			collection_id: CollectionId,
			asset_id: AssetId,
		) -> Result<Vec<u8>, Self::Error> {
			let base_uri = Pallet::<T>::collection_base_uri(collection_id)
				.ok_or(Error::CollectionDoesNotExist)?;

			// concatenate base_uri with asset_id
			let mut token_uri = base_uri.to_vec();
			token_uri.push(b'/');
			token_uri.extend_from_slice(asset_id.to_string().as_bytes());
			Ok(token_uri)
		}
	}
}

/// `ASSET_PRECOMPILE_ADDRESS_PREFIX` is a predefined prefix used to identify collection addresses.
///
/// All addresses that start with this prefix are considered as collection addresses.
/// Since `CollectionId` is represented as a `u64`, it leaves these bits free to be
/// utilized for such a prefix.
///
/// Usage of this prefix provides a consistent and recognizable pattern for distinguishing
/// collection addresses from other types of addresses in the system.
pub const ASSET_PRECOMPILE_ADDRESS_PREFIX: &[u8] = &[0xff; 12];

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
pub fn address_to_collection_id<Address: Into<[u8; 20]>>(
	address: Address,
) -> Result<CollectionId, CollectionError> {
	let address_bytes: [u8; 20] = address.into();
	if &address_bytes[0..12] != ASSET_PRECOMPILE_ADDRESS_PREFIX {
		return Err(CollectionError::InvalidPrefix)
	}
	let id_bytes: [u8; 8] = address_bytes[12..].try_into().unwrap();
	Ok(CollectionId::from_be_bytes(id_bytes))
}

/// Checks if a given `Address` address is a collection address.
///
/// This function examines the prefix of the given `Address` address to determine if it is a
/// collection address, based on the `ASSET_PRECOMPILE_ADDRESS_PREFIX`.
///
/// # Arguments
///
/// * `address`: The `Address` address to be checked.
///
/// # Returns
///
/// * A boolean indicating if the address is a collection address.
pub fn is_collection_address<Address: Into<[u8; 20]>>(address: Address) -> bool {
	&address.into()[0..12] == ASSET_PRECOMPILE_ADDRESS_PREFIX
}

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;
