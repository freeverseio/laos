#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

pub mod weights;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		dispatch::DispatchResultWithPostInfo,
		pallet_prelude::*,
		traits::{Currency, ExistenceRequirement},
	};
	use frame_system::pallet_prelude::*;
	use sp_runtime::traits::{StaticLookup, Zero};

	#[pallet::config]
	pub trait Config:
		frame_system::Config + pallet_treasury::Config + pallet_vesting::Config
	{
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type WeightInfo: crate::weights::WeightInfo;

		// address of where the funds are
		#[pallet::constant]
		type VaultAccountId: Get<Self::AccountId>;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		SomethingStored { who: T::AccountId },
	}

	#[pallet::error]
	pub enum Error<T> {
		NoneValue,
		StorageOverflow,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
		pub fn fund_treasury(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			let vault_account = T::VaultAccountId::get();

			// Vest all to the Vault AccountId
			pallet_vesting::Pallet::<T>::vest_other(
				frame_system::RawOrigin::Signed(who.clone()).into(),
				T::Lookup::unlookup(vault_account.clone()),
			)?;

			// Retrieve the treasury account
			let treasury_account = pallet_treasury::Pallet::<T>::account_id();

			// Transfer all transferable funds from the vault to the treasury
			let vault_balance =
				<T as pallet_vesting::Config>::Currency::free_balance(&vault_account);
			if vault_balance > Zero::zero() {
				<T as pallet_vesting::Config>::Currency::transfer(
					&vault_account,
					&treasury_account,
					vault_balance,
					ExistenceRequirement::AllowDeath,
				)?;
			}

			Self::deposit_event(Event::SomethingStored { who });
			Ok(().into())
		}
	}
}
