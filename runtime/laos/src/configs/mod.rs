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

mod asset_metadata_extender;
mod aura;
mod authorship;
mod balances;
mod base_fee;
mod benchmark;
mod collective;
mod cumulus_parachain_system;
mod cumulus_xcmp_queue;
mod democracy;
mod election_phragmen;
mod ethereum;
pub(crate) mod evm;
mod identity;
pub(crate) mod laos_evolution;
mod membership;
mod multisig;
pub(crate) mod parachain_staking;
mod preimage;
mod proxy;
mod scheduler;
mod session;
mod sudo;
pub mod system;
mod timestamp;
mod transaction_payment;
mod treasury;
mod utility;
mod vesting;
pub(crate) mod xcm_config;
mod xcm_message_queue;

use frame_support::parameter_types;

use crate::Runtime;

parameter_types! {
	/// Max length of the `TokenUri`
	pub const MaxTokenUriLength: u32 = 512;
}

impl cumulus_pallet_aura_ext::Config for Runtime {}
impl pallet_evm_chain_id::Config for Runtime {}
impl staging_parachain_info::Config for Runtime {}
