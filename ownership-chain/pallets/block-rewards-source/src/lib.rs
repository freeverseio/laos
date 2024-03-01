#![cfg_attr(not(feature = "std"), no_std)]

// External crate imports
use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
use sp_runtime::DispatchResult;

// Crate's internal imports
pub use pallet::*;

// Internal modules
mod benchmarking;
pub mod weights;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

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
	pub enum Event<T: Config> {
		/// New rewards account has been set.
		/// \[new account\]
		RewardsAccountSet(T::AccountId),
	}

	/// Customs errors for this pallet
	#[pallet::error]
	#[derive(PartialEq)]
	pub enum Error<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Set rewards account.
		///
		/// Only `Root` origin can call this function.
		#[pallet::call_index(0)]
		#[pallet::weight(<T as Config>::WeightInfo::set_rewards_account())]
		pub fn set_rewards_account(origin: OriginFor<T>, account: T::AccountId) -> DispatchResult {
			ensure_root(origin)?;
			<RewardsAccount<T>>::put(account.clone());
			Self::deposit_event(Event::RewardsAccountSet(account));
			Ok(())
		}
	}
}
