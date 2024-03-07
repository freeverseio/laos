#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	pallet_prelude::*,
	traits::{Currency, ExistenceRequirement},
};

use sp_runtime::{traits::Zero, ArithmeticError};

mod benchmarking;
pub mod weights;

pub use pallet::*;
pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {

	use super::*;

	/// The current storage version.
	const STORAGE_VERSION: StorageVersion = StorageVersion::new(0);

	pub type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	#[pallet::pallet]
	#[pallet::storage_version(STORAGE_VERSION)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;

		/// The currency type
		type Currency: Currency<Self::AccountId>;
	}

	/// Source of rewards for block producers
	#[pallet::storage]
	#[pallet::getter(fn rewards_account)]
	pub type RewardsAccount<T: Config> = StorageValue<_, T::AccountId, OptionQuery>;

	#[pallet::genesis_config]
	#[derive(frame_support::DefaultNoBound)]
	pub struct GenesisConfig<T: Config> {
		pub rewards_account: Option<T::AccountId>,
	}

	#[pallet::genesis_build]
	impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
		fn build(&self) {
			if let Some(rewards_account) = &self.rewards_account {
				RewardsAccount::<T>::put(rewards_account);
			}
		}
	}

	impl<T: Config> Pallet<T> {
		/// This method sends rewards to the destination account.
		/// On success, simply return the amount transferred.
		/// When rewards account has no funds or it doesn't exist return Ok(0).
		pub fn send_rewards(
			destination: T::AccountId,
			amount: BalanceOf<T>,
		) -> Result<BalanceOf<T>, DispatchError> {
			let source = match RewardsAccount::<T>::get() {
				Some(account) => account,
				None => return Ok(Zero::zero()),
			};

			T::Currency::transfer(&source, &destination, amount, ExistenceRequirement::KeepAlive)
				.map(|_| amount)
				.or_else(|e| match e {
					DispatchError::Arithmetic(ArithmeticError::Underflow) => Ok(Zero::zero()),
					_ => Err(e),
				})
		}
	}
}

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;
