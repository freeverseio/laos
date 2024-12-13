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

use crate::{weights, Runtime, RuntimeEvent};
use frame_support::{parameter_types, PalletId};

parameter_types! {
	pub const TreasuryFundingPalletId: PalletId = PalletId(*b"ls/trsfn");
}

impl pallet_treasury_funding::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type PalletId = TreasuryFundingPalletId;
	type WeightInfo = weights::pallet_treasury_funding::WeightInfo<Runtime>;
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn check_trasury_funding_address() {
		assert_eq!(
			pallet_treasury_funding::Pallet::<Runtime>::account_id().to_string(),
			"0x6d6f646C6c732F747273666e0000000000000000"
		);
	}
}
