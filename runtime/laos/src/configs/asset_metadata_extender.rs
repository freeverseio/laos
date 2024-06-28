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

use super::MaxTokenUriLength;
use crate::{types::AccountIdToH160, weights, Runtime, RuntimeEvent};
use frame_support::parameter_types;

parameter_types! {
	/// Max length of the `UniversalLocation`
	pub const MaxUniversalLocationLength: u32 = 512;
}

impl pallet_asset_metadata_extender::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type AccountIdToH160 = AccountIdToH160;
	type MaxTokenUriLength = MaxTokenUriLength;
	type MaxUniversalLocationLength = MaxUniversalLocationLength;
	type GasWeightMapping = <Runtime as pallet_evm::Config>::GasWeightMapping;
	type WeightInfo = weights::pallet_asset_metadata_extender::WeightInfo<Runtime>;
}
