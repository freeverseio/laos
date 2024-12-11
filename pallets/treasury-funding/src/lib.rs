#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

pub mod weights;

#[frame_support::pallet]
pub mod pallet {
	use crate::weights::WeightInfo;
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
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::event]
	pub enum Event<T: Config> {}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
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
		fn fund_treasury_internal() -> DispatchResult {
			// Retrieve the vault account.
			let vault_account = T::VaultAccountId::get();

			// Vest all funds to the vault account.
			pallet_vesting::Pallet::<T>::vest_other(
				frame_system::RawOrigin::Root.into(),
				T::Lookup::unlookup(vault_account.clone()),
			)?;

			// Get the treasury account.
			let treasury_account = pallet_treasury::Pallet::<T>::account_id();

			// Get the vault's current free balance.
			let vault_balance =
				<T as pallet_vesting::Config>::Currency::free_balance(&vault_account);

			// Transfer all funds to the treasury if balance is positive.
			if vault_balance > Zero::zero() {
				<T as pallet_vesting::Config>::Currency::transfer(
					&vault_account,
					&treasury_account,
					vault_balance,
					ExistenceRequirement::AllowDeath,
				)?;
			}

			Ok(())
		}
	}
}
