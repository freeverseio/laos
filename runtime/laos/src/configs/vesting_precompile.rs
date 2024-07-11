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

use crate::{weights, Runtime};
use frame_support::derive_impl;
use pallet_vesting_precompile::{config_preludes, pallet, pallet::Config};
use sp_core::U256;

pub struct BlockNumberForToU256;

impl sp_runtime::traits::Convert<frame_system::pallet_prelude::BlockNumberFor<Runtime>, U256>
	for BlockNumberForToU256
{
	fn convert(b: frame_system::pallet_prelude::BlockNumberFor<Runtime>) -> U256 {
		U256::from(b)
	}
}

#[derive_impl(config_preludes::TestDefaultConfig as pallet::DefaultConfig)]
impl Config for Runtime {
	type BlockNumberForToU256 = BlockNumberForToU256;
	type GasWeightMapping = pallet_evm::FixedGasWeightMapping<Self>;
	type WeightInfo = weights::pallet_vesting_precompile::WeightInfo<Runtime>;
}
