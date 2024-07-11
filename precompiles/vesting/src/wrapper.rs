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

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{pallet, traits::tokens::currency::Currency};
pub use pallet::*;
use sp_core::{H160, U256};
use sp_runtime::traits::{Convert, ConvertBack};

#[pallet]
pub mod pallet {

	use frame_system::pallet_prelude::BlockNumberFor;

	use super::*;
	use crate::*;

	pub type BalanceOf<Runtime> = <<Runtime as pallet_vesting::Config>::Currency as Currency<
		<Runtime as frame_system::Config>::AccountId,
	>>::Balance;

	#[pallet::config(with_default)]
	pub trait Config: frame_system::Config + pallet_vesting::Config {
		#[pallet::no_default_bounds]
		/// Converts `Self::AccountId` to `H160`
		type AccountIdToH160: ConvertBack<Self::AccountId, H160>;

		#[pallet::no_default_bounds]
		/// Converts `BalanceOf<Self>` to `U256`
		type BalanceOfToU256: Convert<BalanceOf<Self>, U256>;

		#[pallet::no_default]
		/// Converts `BlockNumberFor<Self>` to `U256`
		type BlockNumberForToU256: Convert<BlockNumberFor<Self>, U256>;

		#[pallet::no_default]
		/// Gas weight mapping
		type GasWeightMapping: pallet_evm::GasWeightMapping;

		/// WeightInfo of the calls
		type WeightInfo: crate::weights::WeightInfo;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	pub mod config_preludes {
		use super::*;
		use frame_support::{
			derive_impl, pallet_prelude::inject_runtime_type, register_default_impl,
		};

		pub struct TestDefaultConfig;

		#[derive_impl(frame_system::config_preludes::TestDefaultConfig as frame_system::pallet::DefaultConfig, no_aggregated_types)]
		impl frame_system::DefaultConfig for TestDefaultConfig {}

		#[register_default_impl(TestDefaultConfig)]
		impl DefaultConfig for TestDefaultConfig {
			type AccountIdToH160 = AccountIdToH160;
			type BalanceOfToU256 = BalanceOfToU256;
			type WeightInfo = ();
		}

		pub struct AccountIdToH160;
		impl Convert<laos_primitives::AccountId, H160> for AccountIdToH160 {
			fn convert(account_id: laos_primitives::AccountId) -> H160 {
				H160(account_id.0)
			}
		}

		impl ConvertBack<laos_primitives::AccountId, H160> for AccountIdToH160 {
			fn convert_back(account_id: H160) -> laos_primitives::AccountId {
				laos_primitives::AccountId::from(account_id)
			}
		}

		pub struct BalanceOfToU256;

		impl Convert<laos_primitives::Balance, U256> for BalanceOfToU256 {
			fn convert(b: laos_primitives::Balance) -> U256 {
				U256::from(b)
			}
		}
	}
}
