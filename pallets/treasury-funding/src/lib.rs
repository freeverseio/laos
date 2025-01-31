// Copyright 2023-2024 Freeverse.io
// This file is part of LAOS.

// LAOS is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// LAOS is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with LAOS.  If not, see <http://www.gnu.org/licenses/>.

//! # Funding Treasury Pallet
//!
//! - [`Config`]
//! - [`Call`]
//!
//! ## Overview
//!
//! The Funding Treasury pallet provides a way to fund the treasury by transferring available funds
//! from a (maybe) pre-vested vault account. Everyone may use this pallet to transfer all the
//! available funds from the vault account. This way, a vest schedule can be defined over some
//! funds that have to the sent to the treasury.
//!
//! ### Dispatchable Functions
//!
//! The  Funding Treasury pallet provides a single dispatchable function
//!
//! - `fund_treasury`: Vest all funds of the vault account and transfer them to the treasury.

#![cfg_attr(not(feature = "std"), no_std)]
// TODO: This line is needed cause from frontier2409 and rust 1.81 onwards, manual inspect clippy
// flag is giving a false positive in all our pallets. From this issue: https://github.com/rust-lang/rust-clippy/issues/13185
// it seems that the flag --manual-inspect may not be 100% ready yet, as it's giving problems to
// different codes. Maybe we can remove this attribute in the future
#![allow(clippy::manual_inspect)]

pub use pallet::*;

pub mod weights;

pub use weights::WeightInfo;

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
			if pallet_vesting::Pallet::<T>::vesting(vault_account.clone()).is_some() {
				// Vest all funds in the vault account.
				pallet_vesting::Pallet::<T>::vest_other(
					origin,
					T::Lookup::unlookup(vault_account.clone()),
				)?;
			}

			// Retrieve the sovereign account of pallet treasury.
			let treasury_account = pallet_treasury::Pallet::<T>::account_id();

			// Transfer all free balance from the vault to the treasury without keeping the vault
			// alive.
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

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
