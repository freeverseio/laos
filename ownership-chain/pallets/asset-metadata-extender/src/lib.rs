#![cfg_attr(not(feature = "std"), no_std)]

mod benchmarking;
pub mod traits;
pub mod types;
pub mod weights;

use frame_support::pallet_prelude::*;
pub use pallet::*;
use sp_core::H160;
use sp_runtime::{
	traits::{Convert, One},
	ArithmeticError, DispatchResult,
};
pub use traits::AssetMetadataExtender as AssetMetadataExtenderT;
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

		/// Limit for the length of `token_uri`
		#[pallet::constant]
		type MaxTokenUriLength: Get<u32>;

		/// Limit for the length of `universal_location`
		#[pallet::constant]
		type MaxUniversalLocationLength: Get<u32>;

		/// Converts `Self::AccountId` to `H160`
		type AccountIdToH160: Convert<Self::AccountId, H160>;
	}

	/// Extensions counter for a given location
	#[pallet::storage]
	#[pallet::getter(fn extensions_counter)]
	pub(super) type ExtensionsCounter<T: Config> =
		StorageMap<_, Blake2_128Concat, UniversalLocationOf<T>, Index, ValueQuery>;

	/// Records all claimers with index that performed an extension for a given asset location
	#[pallet::storage]
	#[pallet::getter(fn claimers_by_location_and_index)]
	pub(super) type ClaimersByLocationAndIndex<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		UniversalLocationOf<T>,
		Blake2_128Concat,
		Index,
		AccountIdOf<T>,
		OptionQuery,
	>;

	/// Records all the token uris associated with a universal location performed by a claimer.
	#[pallet::storage]
	#[pallet::getter(fn token_uris_by_claimer_and_location)]
	pub(super) type TokenUrisByClaimerAndLocation<T: Config> = StorageDoubleMap<
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
		/// Extension created
		/// parameters. [universal_location, claimer, token_uri]
		ExtensionCreated {
			universal_location: UniversalLocationOf<T>,
			claimer: AccountIdOf<T>,
			token_uri: TokenUriOf<T>,
		},

		/// Extension updated
		/// parameters. [universal_location, claimer, token_uri]
		ExtensionUpdated {
			universal_location: UniversalLocationOf<T>,
			claimer: AccountIdOf<T>,
			token_uri: TokenUriOf<T>,
		},
	}

	/// Customs errors for this pallet
	#[pallet::error]
	#[derive(PartialEq)]
	pub enum Error<T> {
		/// A claimer can perform one extension for a given universal location
		ExtensionAlreadyExists,
		/// A claimer can update an extension only if it exists
		ExtensionDoesNotExist,
	}
}

impl<T: Config> AssetMetadataExtenderT<T> for Pallet<T> {
	fn create_token_uri_extension(
		claimer: AccountIdOf<T>,
		universal_location: UniversalLocationOf<T>,
		token_uri: TokenUriOf<T>,
	) -> DispatchResult {
		ensure!(
			!TokenUrisByClaimerAndLocation::<T>::contains_key(
				claimer.clone(),
				universal_location.clone()
			),
			Error::<T>::ExtensionAlreadyExists
		);

		let index = Self::extensions_counter(universal_location.clone());
		ClaimersByLocationAndIndex::<T>::insert(universal_location.clone(), index, claimer.clone());
		TokenUrisByClaimerAndLocation::<T>::insert(
			claimer.clone(),
			universal_location.clone(),
			token_uri.clone(),
		);
		let next_index = index.checked_add(One::one()).ok_or(ArithmeticError::Overflow)?;
		ExtensionsCounter::<T>::insert(universal_location.clone(), next_index);

		Self::deposit_event(Event::ExtensionCreated { universal_location, claimer, token_uri });

		Ok(())
	}

	fn update_token_uri_extension(
		claimer: AccountIdOf<T>,
		universal_location: UniversalLocationOf<T>,
		token_uri: TokenUriOf<T>,
	) -> DispatchResult {
		ensure!(
			TokenUrisByClaimerAndLocation::<T>::contains_key(
				claimer.clone(),
				universal_location.clone()
			),
			Error::<T>::ExtensionDoesNotExist
		);

		TokenUrisByClaimerAndLocation::<T>::insert(
			claimer.clone(),
			universal_location.clone(),
			token_uri.clone(),
		);

		Self::deposit_event(Event::ExtensionUpdated { claimer, universal_location, token_uri });

		Ok(())
	}

	fn balance_of(universal_location: UniversalLocationOf<T>) -> u32 {
		ExtensionsCounter::<T>::get(universal_location)
	}

	fn claimer_by_index(
		universal_location: UniversalLocationOf<T>,
		index: u32,
	) -> Option<AccountIdOf<T>> {
		ClaimersByLocationAndIndex::<T>::get(universal_location, index)
	}

	fn token_uri_extension_by_index(
		universal_location: UniversalLocationOf<T>,
		index: u32,
	) -> Option<TokenUriOf<T>> {
		let claimer = Self::claimer_by_index(universal_location.clone(), index)?;
		TokenUrisByClaimerAndLocation::<T>::get(claimer, universal_location)
	}
}

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;
