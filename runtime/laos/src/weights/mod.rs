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

// This file is part of Substrate.

//! Expose the auto generated weight files.

pub mod block_weights;
pub mod cumulus_pallet_parachain_system;
pub mod cumulus_pallet_xcmp_queue;
pub mod extrinsic_weights;
pub mod pallet_asset_metadata_extender;
pub mod pallet_evm;
pub mod pallet_laos_evolution;
pub mod pallet_message_queue;
pub mod pallet_multisig;
pub mod pallet_parachain_staking;
pub mod pallet_precompiles_benchmark;
pub mod pallet_proxy;
pub mod pallet_session;
pub mod pallet_sudo;
pub mod pallet_timestamp;
pub mod pallet_utility;
pub mod pallet_vesting;
pub mod paritydb_weights;
pub mod rocksdb_weights;

pub use rocksdb_weights::constants::RocksDbWeight;
