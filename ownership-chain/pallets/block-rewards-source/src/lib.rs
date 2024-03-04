#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::pallet_prelude::*;

// Crate's internal imports
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	/// The current storage version.
	const STORAGE_VERSION: StorageVersion = StorageVersion::new(0);

	#[pallet::pallet]
	#[pallet::storage_version(STORAGE_VERSION)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	/// Source of rewards for block producers
	#[pallet::storage]
	#[pallet::getter(fn rewards_account)]
	pub type RewardsAccount<T: Config> = StorageValue<_, T::AccountId, OptionQuery>;

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub rewards_account: Option<T::AccountId>,
	}

	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { rewards_account: Default::default() }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
		fn build(&self) {
			if let Some(rewards_account) = &self.rewards_account {
				RewardsAccount::<T>::put(rewards_account.clone());
			}
		}
	}

	/// Events for this pallet.
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {}

	/// Customs errors for this pallet
	#[pallet::error]
	#[derive(PartialEq)]
	pub enum Error<T> {}
}
