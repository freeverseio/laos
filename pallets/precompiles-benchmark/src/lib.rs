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
use sp_core::H160;
use sp_runtime::traits::{Convert, ConvertBack};

pub use pallet::*;
pub mod precompiles;
pub mod weights;
pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {
	use crate::*;

	#[pallet::config(with_default)]
	pub trait Config: frame_system::Config {
		#[pallet::no_default_bounds]
		/// Converts `Self::AccountId` to `H160`
		type AccountIdToH160: ConvertBack<Self::AccountId, H160>;

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
		type AccountId = H160;

		#[derive_impl(frame_system::config_preludes::TestDefaultConfig as frame_system::pallet::DefaultConfig, no_aggregated_types)]
		impl frame_system::DefaultConfig for TestDefaultConfig {}

		#[register_default_impl(TestDefaultConfig)]
		impl DefaultConfig for TestDefaultConfig {
			type AccountIdToH160 = AccountIdToH160;
			type WeightInfo = ();
		}

		pub struct AccountIdToH160;
		impl Convert<AccountId, H160> for AccountIdToH160 {
			fn convert(account_id: AccountId) -> H160 {
				H160(account_id.0)
			}
		}

		impl ConvertBack<AccountId, H160> for AccountIdToH160 {
			fn convert_back(account_id: H160) -> AccountId {
				AccountId::from(account_id)
			}
		}
	}
}
