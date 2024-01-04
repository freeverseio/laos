#![cfg_attr(not(feature = "std"), no_std)] // duda para que esto
use frame_support::pallet_prelude::*;
pub use pallet::*;
use sp_runtime::{traits::One, ArithmeticError, DispatchError};
use traits::AssetMetadataExtender; // duda para que esto
pub mod traits;

/// Wrapper around `BoundedVec` for `tokenUri`
#[frame_support::pallet]
pub mod pallet {
	use super::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_); // duda para que esto

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		// /// Because this pallet emits events, it depends on the runtime's definition of an event.
		// type RuntimeEvent: From<Event<Self>> + IsType<<Self as
		// frame_system::Config>::RuntimeEvent>; // duda entender
	}

	/// Universal location extensions counter
	#[pallet::storage]
	#[pallet::getter(fn universal_location_extensions_counter)]
	pub(super) type UniversalLocationExtensionsCounter<T: Config> =
		StorageMap<_, Blake2_128Concat, u32, u32, OptionQuery>;
	// TODO u32

	/// TODO
	#[pallet::storage]
	#[pallet::getter(fn universal_location_extensions)]
	pub(super) type UniversalLocationExtensions<T: Config> =
		StorageDoubleMap<_, Blake2_128Concat, u32, Blake2_128Concat, u32, u32, OptionQuery>; // TODO u32 and

	/// TODO
	#[pallet::storage]
	#[pallet::getter(fn claimer_extensions)]
	pub(super) type ClaimerExtensions<T: Config> =
		StorageDoubleMap<_, Blake2_128Concat, u32, Blake2_128Concat, u32, u32, OptionQuery>; // TODO u32
}

impl<T: Config> AssetMetadataExtender for Pallet<T> {
	fn create_extension(
		claimer: u32,
		universal_location: u32,
		token_uri: u32,
	) -> Result<(), DispatchError> {
		// TODO ensure ul + claimer doesnt exist

		// insert extension
		let index = Self::universal_location_extensions_counter(universal_location).unwrap();
		UniversalLocationExtensions::<T>::insert(universal_location, index, token_uri);
		// insert claimer extension
		ClaimerExtensions::<T>::insert(claimer, universal_location, token_uri);
		// increase index
		let next_index = index.checked_add(One::one()).ok_or(ArithmeticError::Overflow)?;
		UniversalLocationExtensionsCounter::<T>::insert(universal_location, next_index);

		Ok(())
	}
}

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;
