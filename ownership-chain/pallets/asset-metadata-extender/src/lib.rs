#![cfg_attr(not(feature = "std"), no_std)] // duda para que esto

pub mod traits;
pub mod types;

use frame_support::pallet_prelude::*;
pub use pallet::*;
use sp_runtime::{traits::One, ArithmeticError, DispatchError};
use traits::AssetMetadataExtender;
use types::*;

/// Wrapper around `BoundedVec` for `tokenUri`
#[frame_support::pallet]
pub mod pallet {
	use super::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_); // duda para que esto

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Limit for the length of `token_uri`
		#[pallet::constant]
		type MaxTokenUriLength: Get<u32>;

		/// Limit for the length of `universal_location`
		#[pallet::constant]
		type MaxUniversalLocationLength: Get<u32>;
	}

	/// Universal location extensions counter TODO docs
	#[pallet::storage]
	#[pallet::getter(fn universal_location_extensions_counter)]
	pub(super) type UniversalLocationExtensionsCounter<T: Config> =
		StorageMap<_, Blake2_128Concat, UniversalLocationOf<T>, Index, ValueQuery>;

	// TODO docs
	#[pallet::storage]
	#[pallet::getter(fn universal_location_extensions)]
	pub(super) type UniversalLocationExtensions<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		UniversalLocationOf<T>,
		Blake2_128Concat,
		Index,
		Extension<T>,
		OptionQuery,
	>;

	// TODO docs
	#[pallet::storage]
	#[pallet::getter(fn claimer_extensions)]
	pub(super) type ClaimerExtensions<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		AccountIdOf<T>,
		Blake2_128Concat,
		UniversalLocationOf<T>,
		TokenUriOf<T>,
		OptionQuery,
	>;

	/// Events for this pallet.
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Metadata extension created
		/// parameters. [universal_location, claimer, token_uri]
		ExtensionCreated {
			universal_location: UniversalLocationOf<T>, /* TODO change universal_location ->
			                                             * location */
			claimer: AccountIdOf<T>,
			token_uri: TokenUriOf<T>,
		},
	}

	// Customs errors for this pallet
	#[pallet::error]
	#[derive(PartialEq)]
	pub enum Error<T> {
		// One claimer one can perform one extension for a given universal location
		ExtensionAlreadyExists,
	}
}

impl<T: Config> AssetMetadataExtender<AccountIdOf<T>, TokenUriOf<T>, UniversalLocationOf<T>>
	for Pallet<T>
{
	fn create_extension(
		claimer: AccountIdOf<T>,
		universal_location: UniversalLocationOf<T>,
		token_uri: TokenUriOf<T>,
	) -> Result<(), DispatchError> {
		ensure!(
			ClaimerExtensions::<T>::contains_key(claimer.clone(), universal_location.clone()) ==
				false,
			Error::<T>::ExtensionAlreadyExists
		);

		let index = Self::universal_location_extensions_counter(universal_location.clone());
		UniversalLocationExtensions::<T>::insert(
			universal_location.clone(),
			index,
			Extension { claimer: claimer.clone(), token_uri: token_uri.clone() },
		);
		ClaimerExtensions::<T>::insert(
			claimer.clone(),
			universal_location.clone(),
			token_uri.clone(),
		);
		let next_index = index.checked_add(One::one()).ok_or(ArithmeticError::Overflow)?;
		UniversalLocationExtensionsCounter::<T>::insert(universal_location.clone(), next_index);

		Self::deposit_event(Event::ExtensionCreated { universal_location, claimer, token_uri });

		Ok(())
	}
}

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;
