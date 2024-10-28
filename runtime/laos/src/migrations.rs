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

use crate::{Runtime, Vesting};
use frame_support::traits::GetStorageVersion;
use sp_runtime::DispatchError;
use sp_std::vec::Vec;

pub type Migrations = (cumulus_pallet_xcmp_queue::migration::v4::MigrationToV4<Runtime>, MigrationPalletVestingTo6SecBlockProduction);

pub struct MigrationPalletVestingTo6SecBlockProduction;
impl frame_support::traits::OnRuntimeUpgrade for MigrationPalletVestingTo6SecBlockProduction {
    #[cfg(feature = "try-runtime")]
	fn pre_upgrade() -> Result<Vec<u8>, DispatchError> {
		log::info!("Pallet Vesting migrating from {:#?}", Vesting::on_chain_storage_version());
		Ok(Vec::new())
	}

	#[cfg(feature = "try-runtime")]
	fn post_upgrade(_state: Vec<u8>) -> Result<(), DispatchError> {
		// log::info!("{} migrated to {:#?}", Pallet::name(), Pallet::on_chain_storage_version());
		Ok(())
	}

	fn on_runtime_upgrade() -> frame_support::weights::Weight {log::info!("Pallet Vesting migrating from {:#?}", Vesting::on_chain_storage_version());
log::info!("ciaooooooooo");
log::info!("Pallet Vesting migrating from {:#?}", Vesting::on_chain_storage_version());
		// if Pallet::on_chain_storage_version() == StorageVersion::new(0) {
		// 	Pallet::current_storage_version().put::<Pallet>();
		// }
		<Runtime as frame_system::Config>::DbWeight::get().reads_writes(1, 1)
	}
}
