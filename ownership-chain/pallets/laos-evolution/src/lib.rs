#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::ensure;
pub use pallet::*;
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub mod traits;
pub mod types;

use frame_support::pallet_prelude::*;
use sp_core::H160;
use sp_runtime::{
	traits::{Convert, One},
	ArithmeticError, DispatchError,
};

pub use traits::LaosEvolution;
pub use types::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// Converts `Self::AccountId` to `H160`
		type AccountIdToH160: Convert<Self::AccountId, H160>;
		/// Limit for the length of `token_uri`
		#[pallet::constant]
		type MaxTokenUriLength: Get<u32>;
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
		MintedWithExternalURI {
			collection_id: CollectionId,
			slot: Slot,
			to: AccountIdOf<T>,
			token_uri: TokenUriOf<T>,
			token_id: TokenId,
		},
		/// Asset evolved
		/// [collection_id, token_uri, token_id]
		EvolvedWithExternalURI {
			collection_id: CollectionId,
			token_id: TokenId,
			token_uri: TokenUriOf<T>,
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
		/// This happens when `Slot` is larger than 96 bits
		SlotOverflow,
		/// Asset does not exist
		AssetDoesNotExist,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {}
}

impl<T: Config> LaosEvolution<AccountIdOf<T>, TokenUriOf<T>> for Pallet<T> {
	fn create_collection(owner: AccountIdOf<T>) -> Result<CollectionId, DispatchError> {
		let collection_id = Self::collection_counter();

		CollectionOwner::<T>::insert(collection_id, owner.clone());

		// Attempt to increment the collection counter by 1. If this operation
		// would result in an overflow, return early with an error
		let counter = collection_id.checked_add(One::one()).ok_or(ArithmeticError::Overflow)?;
		CollectionCounter::<T>::put(counter);

		// Emit an event.
		Self::deposit_event(Event::CollectionCreated { collection_id, owner });

		Ok(collection_id)
	}

	fn mint_with_external_uri(
		who: AccountIdOf<T>,
		collection_id: CollectionId,
		slot: Slot,
		to: AccountIdOf<T>,
		token_uri: TokenUriOf<T>,
	) -> Result<TokenId, DispatchError> {
		ensure!(
			CollectionOwner::<T>::contains_key(collection_id),
			Error::<T>::CollectionDoesNotExist
		);
		ensure!(CollectionOwner::<T>::get(collection_id) == Some(who), Error::<T>::NoPermission);

		let to_as_h160 = T::AccountIdToH160::convert(to.clone());
		// compose asset_id	from slot and owner
		let token_id =
			slot_and_owner_to_token_id(slot, to_as_h160).ok_or(Error::<T>::SlotOverflow)?;

		ensure!(TokenURI::<T>::get(collection_id, token_id).is_none(), Error::<T>::AlreadyMinted);

		TokenURI::<T>::insert(collection_id, token_id, token_uri.clone());

		Self::deposit_event(Event::MintedWithExternalURI {
			collection_id,
			slot,
			to,
			token_id,
			token_uri,
		});

		Ok(token_id)
	}

	fn collection_owner(collection_id: CollectionId) -> Option<AccountIdOf<T>> {
		CollectionOwner::<T>::get(collection_id)
	}

	fn token_uri(collection_id: CollectionId, token_id: TokenId) -> Option<TokenUriOf<T>> {
		TokenURI::<T>::get(collection_id, token_id)
	}

	fn evolve_with_external_uri(
		who: AccountIdOf<T>,
		collection_id: CollectionId,
		token_id: TokenId,
		token_uri: TokenUriOf<T>,
	) -> Result<(), DispatchError> {
		ensure!(
			CollectionOwner::<T>::contains_key(collection_id),
			Error::<T>::CollectionDoesNotExist
		);
		ensure!(CollectionOwner::<T>::get(collection_id) == Some(who), Error::<T>::NoPermission);
		ensure!(
			TokenURI::<T>::contains_key(collection_id, token_id),
			Error::<T>::AssetDoesNotExist
		);

		TokenURI::<T>::insert(collection_id, token_id, token_uri.clone());

		Self::deposit_event(Event::EvolvedWithExternalURI { collection_id, token_id, token_uri });

		Ok(())
	}
}

/// Converts `Slot` and `H160` to `TokenId`
///
/// Every slot is identified by a unique `token_id` where `token_id = concat(slot #,
/// owner_address)`
///
/// Returns `None` if `Slot` is larger than 96 bits
fn slot_and_owner_to_token_id(slot: Slot, owner: H160) -> Option<TokenId> {
	// Check if slot is larger than 96 bits
	if slot > MAX_U96 {
		return None
	}

	let mut bytes = [0u8; 32];

	let slot_bytes = slot.to_be_bytes();

	// we also use the last 12 bytes of the slot, since the first 4 bytes are always 0
	bytes[..12].copy_from_slice(&slot_bytes[4..]);
	bytes[12..].copy_from_slice(&owner.0);

	Some(TokenId::from(bytes))
}

/// Enum representing possible errors related to collections.
#[derive(Debug, PartialEq)]
pub enum CollectionError {
	/// Error indicating that the provided address does not have the correct format.
	InvalidFormat,
	/// Error indicating that the provided address does not have a valid version.
	InvalidVersion,
}

/// Converts a `CollectionId` into a custom address type `Address`.
///
/// The function constructs a 20-byte Ethereum-like address with a specific format:
///  - The first 11 bytes are zeros.
///  - The 12th byte is set to `1`, indicating the version.
///  - The last 8 bytes represent the `CollectionId` in big-endian format.
///
/// This function is generic over the return type `Address`, which must be a type
/// that can be constructed from a 20-byte array (`[u8; 20]`). This allows flexibility
/// in the type of address returned, as long as it can be created from the byte array.
///
/// # Type Parameters
///
/// * `Address` - The type of the address to be returned. This type must implement `From<[u8; 20]>`.
///
/// # Arguments
///
/// * `collection_id` - The `CollectionId` (u64 value) to be converted into an address.
///
/// # Returns
///
/// An `Address` type representing the constructed address.
pub fn collection_id_to_address<Address: From<[u8; 20]>>(collection_id: CollectionId) -> Address {
	let mut address = [0u8; 20];
	address[11] = 1; // Set version byte to 1
	address[12..].copy_from_slice(&collection_id.to_be_bytes());
	address.into()
}

/// Converts a given address into a `CollectionId`.
///
/// This function takes an `Address` and attempts to convert it into a `CollectionId`.
/// The `Address` is expected to be a 20-byte array in a specific format:
///  - The first 11 bytes should be zeros.
///  - The 12th byte should be `1`, indicating the version.
///  - The last 8 bytes represent the `CollectionId` in big-endian format.
///
/// # Type Parameters
///
/// * `Address`: A type that can be converted into a 20-byte array.
///
/// # Parameters
///
/// * `address`: The address to convert into a `CollectionId`. It must implement `Into<[u8; 20]>`.
///
/// # Returns
///
/// This function returns a `Result<CollectionId, CollectionError>`:
///  - `Ok(CollectionId)`: If the conversion is successful, returns the `CollectionId`.
///  - `Err(CollectionError::InvalidFormat)`: If the first 11 bytes of the address are not zeros.
///  - `Err(CollectionError::InvalidVersion)`: If the 12th byte of the address is not `1`.
pub fn address_to_collection_id<Address>(address: Address) -> Result<CollectionId, CollectionError>
where
	Address: Into<[u8; 20]>,
{
	let address_bytes: [u8; 20] = address.into();

	// Check if the first 11 bytes are zeros
	for &byte in &address_bytes[..11] {
		ensure!(byte == 0, CollectionError::InvalidFormat);
	}

	// Check if the 12th byte is 1 (version byte)
	ensure!(address_bytes[11] == 1, CollectionError::InvalidVersion);

	let mut collection_id_bytes = [0u8; 8];
	collection_id_bytes.copy_from_slice(&address_bytes[12..20]);
	Ok(u64::from_be_bytes(collection_id_bytes))
}
