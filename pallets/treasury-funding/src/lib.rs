#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

pub mod weights;

#[frame_support::pallet]
pub mod pallet {
	use crate::weights::WeightInfo;
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*, PalletId};
	use frame_system::pallet_prelude::*;
	use sp_runtime::traits::{AccountIdConversion, StaticLookup};

	#[pallet::config]
	pub trait Config:
		frame_system::Config
		+ pallet_treasury::Config
		+ pallet_vesting::Config
		+ pallet_balances::Config
	{
		/// Specifies the type for runtime events.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Provides weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;

		/// A unique identifier used to generate the internal Pot account.
		#[pallet::constant]
		type PalletId: Get<PalletId>;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Emitted when the treasury is successfully funded.
		TreasuryFundingExecuted,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Transfers all free balance from the vault to the treasury after vesting funds.
		///
		/// **Requirements:**
		/// - The origin must be signed.
		/// - Performs the following actions:
		///   1. Vest all funds of the vault account.
		///   2. Transfer the vault's free balance to the treasury account.
		///
		/// **Weight:** Based on `T::WeightInfo::fund_treasury()`.
		#[pallet::call_index(0)]
		#[pallet::weight(<T as Config>::WeightInfo::fund_treasury())]
		pub fn fund_treasury(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			// Ensure the caller is a signed origin.
			let _who = ensure_signed(origin.clone())?;

			// Retrieve the vault account associated with this pallet.
			let vault_account = Self::account_id();

			// Check if any vesting schedule exists for the vault account.
			if let Some(_) = pallet_vesting::Pallet::<T>::vesting(vault_account.clone()) {
				// Vest all funds in the vault account.
				pallet_vesting::Pallet::<T>::vest_other(
					origin,
					T::Lookup::unlookup(vault_account.clone()),
				)?;
			}

			// Retrieve the treasury account associated with this pallet.
			let treasury_account = pallet_treasury::Pallet::<T>::account_id();

			// Transfer all free balance from the vault to the treasury without keeping the vault alive.
			let keep_alive = false;
			pallet_balances::Pallet::<T>::transfer_all(
				frame_system::RawOrigin::Signed(vault_account.clone()).into(),
				T::Lookup::unlookup(treasury_account.clone()),
				keep_alive,
			)?;

			// Emit an event indicating the treasury funding was successful.
			Self::deposit_event(Event::TreasuryFundingExecuted);

			Ok(().into())
		}
	}

	impl<T: Config> Pallet<T> {
		/// Returns the account ID associated with this pallet.
		pub fn account_id() -> T::AccountId {
			<T as Config>::PalletId::get().into_account_truncating()
		}
	}
}
