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

use crate::{
	configs::collective::CouncilMajority, currency::calculate_deposit, weights, AccountId, Balance,
	Balances, Runtime, RuntimeEvent, RuntimeHoldReason,
};
use frame_support::{
	parameter_types,
	traits::{fungible::HoldConsideration, EitherOfDiverse, LinearStoragePrice},
};
use frame_system::EnsureRoot;

parameter_types! {
	pub const PreimageBaseDeposit: Balance = calculate_deposit(2, 64);
	pub const PreimageByteDeposit: Balance = calculate_deposit(0, 1);
	pub const PreimageHoldReason: RuntimeHoldReason =
		RuntimeHoldReason::Preimage(pallet_preimage::HoldReason::Preimage);
}

impl pallet_preimage::Config for Runtime {
	type Currency = Balances;
	type ManagerOrigin = EitherOfDiverse<EnsureRoot<AccountId>, CouncilMajority>;
	type RuntimeEvent = RuntimeEvent;
	type Consideration = HoldConsideration<
		AccountId,
		Balances,
		PreimageHoldReason,
		LinearStoragePrice<PreimageBaseDeposit, PreimageByteDeposit, Balance>,
	>;
	type WeightInfo = weights::pallet_preimage::WeightInfo<Runtime>;
}
