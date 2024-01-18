//! Pallet that handles collator block rewards.
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod weights;

use frame_support::{
	pallet_prelude::*,
	traits::{Currency, ExistenceRequirement},
};
use frame_system::pallet_prelude::*;
use pallet_authorship::EventHandler;

use self::weights::WeightInfo;
pub use pallet::*;
use sp_runtime::traits::Zero;

/// Explicit `Balance` type.
type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

/// Explicit `AccountId` type.
type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

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

		/// Currency type for this pallet.
		type Currency: Currency<Self::AccountId>;

		/// Community incentives account
		#[pallet::constant]
		type CommunityIncentivesAccountId: Get<Self::AccountId>;

		/// Reward per block
		#[pallet::constant]
		type RewardPerBlock: Get<BalanceOf<Self>>;

		/// Weight information for this pallet.
		type WeightInfo: WeightInfo;
	}

	/// Events for the pallet.
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Collator block reward has been claimed.
		CollatorRewarded { collator: T::AccountId, amount: BalanceOf<T> },
	}
}

impl<T: Config> EventHandler<AccountIdOf<T>, BlockNumberFor<T>> for Pallet<T> {
	fn note_author(author: AccountIdOf<T>) {
		let community_incentives_account = T::CommunityIncentivesAccountId::get();

		// to make sure we still send out rewards even if the balance is less than reward per block
		let reward =
			T::Currency::free_balance(&community_incentives_account).min(T::RewardPerBlock::get());

		// early return if community incentives account run out of funds
		if reward.is_zero() {
			return;
		}

		let _result = T::Currency::transfer(
			&community_incentives_account,
			&author,
			reward,
			ExistenceRequirement::KeepAlive,
		);

		debug_assert!(_result.is_ok(), "Reward transfer failed. Should never happen.");

		Self::deposit_event(Event::CollatorRewarded { collator: author, amount: reward });

		// need to let the system pallet know that we are using extra weight here
		frame_system::Pallet::<T>::register_extra_weight_unchecked(
			T::WeightInfo::note_author(),
			DispatchClass::Mandatory,
		);
	}
}
