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

use crate::{currency::UNIT, weights, Balance, Balances, Runtime, RuntimeEvent, System};
use frame_support::{parameter_types, traits::WithdrawReasons};
use sp_runtime::traits::ConvertInto;

parameter_types! {
	pub const MinVestedTransfer: Balance = UNIT;
	pub UnvestedFundsAllowedWithdrawReasons: WithdrawReasons =
		WithdrawReasons::except(WithdrawReasons::TRANSFER | WithdrawReasons::RESERVE);
}

impl pallet_vesting::Config for Runtime {
	const MAX_VESTING_SCHEDULES: u32 = 28;

	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type BlockNumberToBalance = ConvertInto;
	type BlockNumberProvider = System;
	type MinVestedTransfer = MinVestedTransfer;
	type UnvestedFundsAllowedWithdrawReasons = UnvestedFundsAllowedWithdrawReasons;
	type WeightInfo = weights::pallet_vesting::WeightInfo<Runtime>;
}
