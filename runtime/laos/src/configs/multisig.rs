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
	currency::calculate_deposit, weights, Balance, Balances, Runtime, RuntimeCall, RuntimeEvent,
};
use frame_support::parameter_types;

parameter_types! {
	// One storage item; key size is 32; value is size 4+4+16+32 bytes = 56 bytes
	pub const DepositBase: Balance = calculate_deposit(1, 56);
	// Additional storage item size of 32 bytes.
	pub const DepositFactor: Balance = calculate_deposit(0, 32);
	pub const MaxSignatories: u32 = 20;
}

impl pallet_multisig::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type Currency = Balances;
	type DepositBase = DepositBase;
	type DepositFactor = DepositFactor;
	type MaxSignatories = MaxSignatories;
	type WeightInfo = weights::pallet_multisig::WeightInfo<Runtime>;
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::currency::MILLIUNIT;

	#[test]
	fn check_deposits() {
		assert_eq!(<Runtime as pallet_multisig::Config>::DepositBase::get(), 10_560 * MILLIUNIT);
		assert_eq!(<Runtime as pallet_multisig::Config>::DepositFactor::get(), 320 * MILLIUNIT);
	}
}
