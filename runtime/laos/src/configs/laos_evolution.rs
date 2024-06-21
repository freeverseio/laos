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
use crate::{
	types::{AccountIdToH160, H160ToAccountId},
	weights, Runtime, RuntimeEvent,
};

impl pallet_laos_evolution::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type AccountIdToH160 = AccountIdToH160;
	type H160ToAccountId = H160ToAccountId;
	type MaxTokenUriLength = MaxTokenUriLength;
	type WeightInfo = (); // TODO weights::pallet_laos_evolution::WeightInfo<Runtime>;
	type GasWeightMapping = <Runtime as pallet_evm::Config>::GasWeightMapping;
}
