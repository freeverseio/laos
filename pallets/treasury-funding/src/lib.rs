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
		/// Event type for the runtime.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Weight information for extrinsics.
		type WeightInfo: WeightInfo;

		/// Identifier from which the internal Pot is generated.
		#[pallet::constant]
		type PalletId: Get<PalletId>;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event emitted when the treasury is funded.
		TreasuryFundingExecuted,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Transfers all free balance from the vault to the treasury after vesting funds.
		///
		/// - **Signed origin** required.
		/// - **Actions performed**:
		///   1. Vests all funds in the vault account.
		///   2. Transfers the vault's free balance to the treasury.
		///
		/// Weight: Determined by `T::WeightInfo::fund_treasury()`.
		#[pallet::call_index(0)]
		#[pallet::weight(<T as Config>::WeightInfo::fund_treasury())]
		pub fn fund_treasury(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let _who = ensure_signed(origin.clone())?;

			// Retrieve the vault account.
			let vault_account = Self::account_id();

			// Vest all funds of the vault account.
			pallet_vesting::Pallet::<T>::vest_other(
				origin,
				T::Lookup::unlookup(vault_account.clone()),
			)?;

			// Get the treasury account.
			let treasury_account = pallet_treasury::Pallet::<T>::account_id();

			let keep_alive = false;
			pallet_balances::Pallet::<T>::transfer_all(
				frame_system::RawOrigin::Signed(vault_account.clone()).into(),
				T::Lookup::unlookup(treasury_account.clone()),
				keep_alive,
			)?;

			Self::deposit_event(Event::TreasuryFundingExecuted);

			Ok(().into())
		}
	}

	impl<T: Config> Pallet<T> {
		pub fn account_id() -> T::AccountId {
			<T as Config>::PalletId::get().into_account_truncating()
		}
	}
}
