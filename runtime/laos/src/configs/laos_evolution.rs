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

impl pallet_laos_evolution::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type AccountIdToH160 = AccountIdToH160;
	type MaxTokenUriLength = MaxTokenUriLength;
	type WeightInfo = weights::pallet_laos_evolution::WeightInfo<Runtime>;
	type GasWeightMapping = <Runtime as pallet_evm::Config>::GasWeightMapping;
	type OnCreateCollection = CollectionManager;
}

// This is the simplest bytecode to revert without returning any data.
// We will pre-deploy it under all of our precompiles to ensure they can be called from
// within contracts.
// (PUSH1 0x00 PUSH1 0x00 REVERT)
pub const REVERT_BYTECODE: [u8; 5] = [0x60, 0x00, 0x60, 0x00, 0xFD];

// Currently, we insert [`REVERT_BYTECODE`] as an
// `AccountCode` for the collection address.
//
// This is done to ensure internal calls to the collection address do not
// fail.
pub struct CollectionManager;
impl pallet_laos_evolution::traits::OnCreateCollection for CollectionManager {
	fn on_create_collection(address: sp_core::H160) {
		pallet_evm::Pallet::<Runtime>::create_account(address, REVERT_BYTECODE.into());
	}
}
