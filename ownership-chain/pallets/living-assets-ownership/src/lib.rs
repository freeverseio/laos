#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;
use parity_scale_codec::alloc::string::ToString;
use sp_core::H160;
use sp_std::vec::Vec;
mod functions;
pub mod traits;

#[frame_support::pallet]
pub mod pallet {

	use super::*;
	use frame_support::{
		pallet_prelude::{OptionQuery, ValueQuery, *},
		traits::tokens::nonfungibles_v2::*,
		BoundedVec,
	};
	use sp_core::{H160, U256};
	use sp_runtime::traits::Convert;

	/// Collection id type
	pub type CollectionId = u64;

	/// Base URI type
	pub type BaseURI<T> = BoundedVec<u8, <T as Config>::BaseURILimit>;

	/// Account id type
	pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

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
		/// the token URI formation, where it combines `BaseURILimit`, a `'/'`, and a `tokenID`
		/// (which takes up 33 characters).
		#[pallet::constant]
		type BaseURILimit: Get<u32>;

		/// Collection config
		type CollectionConfig: CollectionConfig;

		/// NonFungibles provider
		type NonFungibles: Create<AccountIdOf<Self>, Self::CollectionConfig>
			+ Inspect<AccountIdOf<Self>, CollectionId = CollectionId>;

		/// This associated type defines a conversion from the `AccountId` type, which is internal
		/// to the implementing type (represented by `Self`), to an `H160` type. The `H160` type
		/// is commonly used to represent Ethereum addresses.
		type AccountIdToH160: Convert<Self::AccountId, H160>;

		/// This associated type defines a conversion from an `H160` type back to the `AccountId`
		/// type, which is internal to the implementing type (represented by `Self`). This
		/// conversion is often necessary for mapping Ethereum addresses back to native account IDs.
		type H160ToAccountId: Convert<H160, Self::AccountId>;

		/// Type alias for implementing the `AssetIdToInitialOwner` trait for a given account ID
		/// type. This allows you to specify which account should initially own each new asset.
		type AssetIdToInitialOwner: Convert<U256, Self::AccountId>;
	}

	/// Collection counter
	#[pallet::storage]
	#[pallet::getter(fn collection_counter)]
	pub(super) type CollectionCounter<T: Config> = StorageValue<_, CollectionId, ValueQuery>;

	/// Collection base URI
	#[pallet::storage]
	#[pallet::getter(fn collection_base_uri)]
	pub(super) type CollectionBaseURI<T: Config> =
		StorageMap<_, Blake2_128Concat, CollectionId, BaseURI<T>, OptionQuery>;

	/// Asset owner
	#[pallet::storage]
	pub(super) type AssetOwner<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		CollectionId,
		Blake2_128Concat,
		U256,
		T::AccountId,
		OptionQuery,
	>;

	fn asset_owner<T: Config>(collection_id: CollectionId, asset_id: U256) -> T::AccountId {
		AssetOwner::<T>::get(collection_id, asset_id)
			.unwrap_or_else(|| T::AssetIdToInitialOwner::convert(asset_id))
	}

	/// Pallet events
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Collection created
		/// parameters. [collection_id, who]
		CollectionCreated { collection_id: CollectionId, who: T::AccountId },
		/// Asset transferred to `who`
		/// parameters. [collection_id, asset_id, who]
		AssetTransferred { collection_id: CollectionId, asset_id: U256, to: T::AccountId },
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

	impl<T: Config> traits::CollectionManager for Pallet<T> {
		type Error = Error<T>;
		type AccountId = T::AccountId;
		type BaseURI = BaseURI<T>;

		fn base_uri(collection_id: CollectionId) -> Option<Self::BaseURI> {
			CollectionBaseURI::<T>::get(collection_id)
		}

		fn create_collection(
			owner: T::AccountId,
			base_uri: Self::BaseURI,
		) -> Result<CollectionId, Self::Error> {
			Self::do_create_collection(owner, base_uri)
		}
	}

	impl<T: Config> traits::Erc721 for Pallet<T> {
		type Error = Error<T>;

		fn owner_of(collection_id: CollectionId, asset_id: U256) -> Result<H160, Self::Error> {
			Pallet::<T>::collection_base_uri(collection_id).ok_or(Error::CollectionDoesNotExist)?;
			Ok(T::AccountIdToH160::convert(asset_owner::<T>(collection_id, asset_id)))
		}

		fn transfer_from(
			origin: H160,
			collection_id: CollectionId,
			from: H160,
			to: H160,
			asset_id: U256,
		) -> Result<(), Self::Error> {
			Pallet::<T>::collection_base_uri(collection_id).ok_or(Error::CollectionDoesNotExist)?;
			ensure!(origin == from, Error::NoPermission);
			ensure!(
				T::AccountIdToH160::convert(asset_owner::<T>(collection_id, asset_id)) == from,
				Error::NoPermission
			);
			ensure!(from != to, Error::CannotTransferSelf);
			ensure!(to != H160::zero(), Error::TransferToNullAddress);

			let to = T::H160ToAccountId::convert(to.clone());
			AssetOwner::<T>::set(collection_id, asset_id, Some(to.clone()));
			Self::deposit_event(Event::AssetTransferred { collection_id, asset_id, to });

			Ok(())
		}

		fn token_uri(collection_id: CollectionId, asset_id: U256) -> Result<Vec<u8>, Self::Error> {
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

/// Converts a `CollectionId` into an `H160` address format.
///
/// This function takes the given `CollectionId`, which is assumed to be a `u64`,
/// and maps it into an `H160` address, prepending it with the `ASSET_PRECOMPILE_ADDRESS_PREFIX`.
///
/// # Arguments
///
/// * `collection_id`: The ID of the collection to be converted.
///
/// # Returns
///
/// * An `H160` representation of the collection ID.
pub fn collection_id_to_address(collection_id: CollectionId) -> H160 {
	let mut bytes = [0u8; 20];
	bytes[12..20].copy_from_slice(&collection_id.to_be_bytes());
	for (i, byte) in ASSET_PRECOMPILE_ADDRESS_PREFIX.iter().enumerate() {
		bytes[i] = *byte;
	}
	H160(bytes)
}

/// Converts an `H160` address into a `CollectionId` format.
///
/// This function takes the given `H160` address, checks for the correct prefix, and extracts
/// the `CollectionId` from it. If the prefix is incorrect, it returns a
/// `CollectionError::InvalidPrefix` error.
///
/// # Arguments
///
/// * `address`: The `H160` address to be converted.
///
/// # Returns
///
/// * A `Result` which is either the `CollectionId` or an error indicating the address is invalid.
pub fn address_to_collection_id(address: H160) -> Result<CollectionId, CollectionError> {
	if &address.0[0..12] != ASSET_PRECOMPILE_ADDRESS_PREFIX {
		return Err(CollectionError::InvalidPrefix)
	}
	let id_bytes: [u8; 8] = address.0[12..].try_into().unwrap();
	Ok(CollectionId::from_be_bytes(id_bytes))
}

/// Checks if a given `H160` address is a collection address.
///
/// This function examines the prefix of the given `H160` address to determine if it is a
/// collection address, based on the `ASSET_PRECOMPILE_ADDRESS_PREFIX`.
///
/// # Arguments
///
/// * `address`: The `H160` address to be checked.
///
/// # Returns
///
/// * A boolean indicating if the address is a collection address.
pub fn is_collection_address(address: H160) -> bool {
	&address.to_fixed_bytes()[0..12] == ASSET_PRECOMPILE_ADDRESS_PREFIX
}

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;
