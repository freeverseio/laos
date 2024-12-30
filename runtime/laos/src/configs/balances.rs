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
	currency::UNIT, weights, Balance, Runtime, RuntimeEvent, RuntimeFreezeReason,
	RuntimeHoldReason, System,
};
use frame_support::parameter_types;

parameter_types! {
	/// The minimum amount required to keep an account open, set to zero in this case.
	///
	/// While it's generally advised to have this value greater than zero to avoid potential
	/// DoS vectors, we set it to zero here due to specific concerns about relay attacks.
	/// In such attacks, the reset of the nonce upon account deletion can be exploited.
	/// By setting the ExistentialDeposit to zero, we prevent the scenario where an account's
	/// balance drops to a level that would trigger its deletion and subsequent nonce reset.
	pub const ExistentialDeposit: Balance = 0;
	/// For benchmark purposes, pallet balances and pallet bounties need ED to be greater than 0.
	/// This may be removed in the future (https://github.com/paritytech/polkadot-sdk/issues/7009).
	pub const BenchExistentialDeposit: Balance = UNIT;
	pub const MaxLocks: u32 = 50;
	pub const MaxFreezes: u32 = 50;
	pub const MaxHolds: u32 = 50;
	pub const MaxReserves: u32 = 50;
}

impl pallet_balances::Config for Runtime {
	type MaxLocks = MaxLocks;
	type Balance = Balance;
	type RuntimeEvent = RuntimeEvent;
	type DustRemoval = ();
	#[cfg(feature = "runtime-benchmarks")]
	type ExistentialDeposit = BenchExistentialDeposit;
	#[cfg(not(feature = "runtime-benchmarks"))]
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type MaxReserves = MaxReserves;
	type ReserveIdentifier = [u8; 8];
	type FreezeIdentifier = RuntimeFreezeReason;
	type RuntimeFreezeReason = RuntimeFreezeReason;
	type RuntimeHoldReason = RuntimeHoldReason;
	type MaxFreezes = MaxFreezes;
	type WeightInfo = weights::pallet_balances::WeightInfo<Runtime>;
}
