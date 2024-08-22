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
	currency::{TRANSACTION_BYTE_FEE, WEIGHT_TO_FEE},
	types::ToAuthor,
	Balance, Balances, Runtime, RuntimeEvent,
};
use frame_support::{
	parameter_types,
	weights::{
		ConstantMultiplier, WeightToFeeCoefficient, WeightToFeeCoefficients, WeightToFeePolynomial,
	},
};
use polkadot_runtime_common::SlowAdjustingFeeUpdate;
use smallvec::smallvec;
use sp_runtime::Perbill;

pub struct LengthToFee;
impl WeightToFeePolynomial for LengthToFee {
	type Balance = Balance;

	fn polynomial() -> WeightToFeeCoefficients<Self::Balance> {
		smallvec![
			WeightToFeeCoefficient {
				degree: 1,
				coeff_frac: Perbill::zero(),
				coeff_integer: TRANSACTION_BYTE_FEE,
				negative: false,
			},
			WeightToFeeCoefficient {
				degree: 3,
				coeff_frac: Perbill::zero(),
				coeff_integer: 100,
				negative: false,
			},
		]
	}
}

parameter_types! {
	pub const OperationalFeeMultiplier: u8 = 5;
	pub const WeightToFee: u128 = WEIGHT_TO_FEE;
}

impl pallet_transaction_payment::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type OnChargeTransaction =
		pallet_transaction_payment::FungibleAdapter<Balances, ToAuthor<Self>>;
	type OperationalFeeMultiplier = OperationalFeeMultiplier;
	type WeightToFee = ConstantMultiplier<Balance, WeightToFee>;
	type LengthToFee = LengthToFee;
	type FeeMultiplierUpdate = SlowAdjustingFeeUpdate<Self>;
}

mod tests {
	#[test]
	fn test_weight_to_fee() {
		use super::*;
		// zero weight
		assert_eq!(
			pallet_transaction_payment::Pallet::<Runtime>::weight_to_fee(
				frame_support::weights::Weight::from_parts(0, 0)
			),
			0
		);
		assert_eq!(
			pallet_transaction_payment::Pallet::<Runtime>::weight_to_fee(
				frame_support::weights::Weight::from_parts(1, 0)
			),
			WEIGHT_TO_FEE
		);
		// in case weight to fee exceeds max_block
		assert_eq!(
			pallet_transaction_payment::Pallet::<Runtime>::weight_to_fee(
				frame_support::weights::Weight::from_parts(u64::MAX, 0)
			),
			<Runtime as frame_system::Config>::BlockWeights::get().max_block.ref_time() as u128 *
				WEIGHT_TO_FEE
		);
	}

	#[test]
	fn test_length_to_fee() {
		use super::*;
		// zero length
		assert_eq!(pallet_transaction_payment::Pallet::<Runtime>::length_to_fee(0), 0);
		assert_eq!(pallet_transaction_payment::Pallet::<Runtime>::length_to_fee(1), 100000000100);
		assert_eq!(pallet_transaction_payment::Pallet::<Runtime>::length_to_fee(3), 300000002700);
	}
}
