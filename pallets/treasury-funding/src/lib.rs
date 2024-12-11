#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

pub mod weights;

#[frame_support::pallet]
pub mod pallet {
	use crate::weights::WeightInfo;
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use sp_runtime::{
		traits::{StaticLookup, Zero},
		Saturating,
	};

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

		/// Vault account where initial funds are held.
		#[pallet::constant]
		type VaultAccountId: Get<Self::AccountId>;

		/// Configuration attribute for the operation step.
		#[pallet::constant]
		type OperationStep: Get<u32>;

		/// minimum amount to pay fees
		#[pallet::constant]
		type MinAmountForFees: Get<Self::Balance>;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event emitted when the treasury is funded.
		TreasuryFundingExecuted,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		/// Periodically checks and funds the treasury if conditions are met.
		fn on_initialize(block_number: BlockNumberFor<T>) -> Weight {
			let step = T::OperationStep::get();
			if block_number % step.into() == Zero::zero() {
				if let Err(e) = Self::fund_treasury_internal() {
					log::warn!("Failed to execute fund_treasury_internal: {:?}", e);
				}
			}
			<T as Config>::WeightInfo::fund_treasury()
		}
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
			let _who = ensure_signed(origin)?;

			Self::fund_treasury_internal()?;

			Ok(().into())
		}
	}

	impl<T: Config> Pallet<T> {
		/// Internal function to handle the logic of funding the treasury.
		fn fund_treasury_internal() -> DispatchResult {
			// Retrieve the vault account.
			let vault_account = T::VaultAccountId::get();

			// Vest all funds of the vault account.
			pallet_vesting::Pallet::<T>::vest(
				frame_system::RawOrigin::Signed(vault_account.clone()).into(),
			)?;

			// Get the treasury account.
			let treasury_account = pallet_treasury::Pallet::<T>::account_id();

			// Transfer all free balance from the vault to the treasury.
			let transferable_balance =
				pallet_balances::Pallet::<T>::usable_balance_for_fees(&vault_account);

			pallet_balances::Pallet::<T>::transfer_keep_alive(
				frame_system::RawOrigin::Signed(vault_account.clone()).into(),
				T::Lookup::unlookup(treasury_account.clone()),
				transferable_balance.saturating_sub(T::MinAmountForFees::get()),
			)?;

			Self::deposit_event(Event::TreasuryFundingExecuted);

			Ok(())
		}
	}
}
