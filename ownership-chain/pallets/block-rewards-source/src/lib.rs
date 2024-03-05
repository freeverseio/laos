#![cfg_attr(not(feature = "std"), no_std)]

// External crates
use frame_support::{
	pallet_prelude::*,
	traits::{Currency, ExistenceRequirement},
};

// Standard library
use sp_runtime::ArithmeticError;

// Your own modules
mod benchmarking;
pub mod weights;

// Re-exports
pub use pallet::*;
pub use weights::WeightInfo;

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
		/// Weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;
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

	/// Customs errors for this pallet
	#[pallet::error]
	#[derive(PartialEq)]
	pub enum Error<T> {}

	impl<T: Config + pallet_parachain_staking::Config>
		pallet_parachain_staking::PayoutCollatorReward<T> for Pallet<T>
	{
		/// This method is part of the `PayoutCollatorReward` trait.
		/// Attempt to retrieve the rewards account and execute the transfer if available.
		/// Returns the weight associated with the transfer or the fallback action.
		fn payout_collator_reward(
			_round_index: pallet_parachain_staking::RoundIndex,
			collator_id: T::AccountId,
			amount: pallet_parachain_staking::BalanceOf<T>,
		) -> Weight {
			if let Some(rewards_account) = RewardsAccount::<T>::get() {
				return <T as pallet_parachain_staking::Config>::Currency::transfer(
					&rewards_account,
					&collator_id,
					amount,
					ExistenceRequirement::KeepAlive,
				)
				.map_or_else(
					|_| Weight::zero(),
					|_| <T as Config>::WeightInfo::payout_collator_reward(),
				);
			}

			// Fallback weight if no rewards account is set or transfer fails. Adjust as needed.
			Weight::zero()
		}

		/// This method is part of the `PayoutCollatorReward` trait.
		/// On success, simply return the amount transferred.
		/// When rewards account has no funds or it doesn't exist return Ok(0).
		fn deposit_into_existing(
			delegator_id: &T::AccountId,
			amount: pallet_parachain_staking::BalanceOf<T>,
		) -> Result<pallet_parachain_staking::BalanceOf<T>, DispatchError> {
			let rewards_account = match RewardsAccount::<T>::get() {
				Some(account) => account,
				None => return Ok(0u32.into()),
			};

			<T as pallet_parachain_staking::Config>::Currency::transfer(
				&rewards_account,
				delegator_id,
				amount,
				ExistenceRequirement::KeepAlive,
			)
			.map(|_| amount)
			.or_else(|e| match e {
				DispatchError::Arithmetic(ArithmeticError::Underflow) => Ok(0u32.into()),
				_ => Err(e),
			})
		}
	}
}

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;
