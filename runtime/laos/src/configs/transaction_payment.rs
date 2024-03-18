use crate::{
	currency::{GIGAWEI, KILOWEI},
	types::ToAuthor,
	Balance, Balances, Runtime, RuntimeEvent,
};
use frame_support::weights::{
	ConstantMultiplier, WeightToFeeCoefficient, WeightToFeeCoefficients, WeightToFeePolynomial,
};
use polkadot_runtime_common::SlowAdjustingFeeUpdate;
use smallvec::smallvec;
use sp_core::{ConstU128, ConstU8};
use sp_runtime::Perbill;

// Provide a common factor between runtimes based on a supply of 10_000_000 tokens.
pub const SUPPLY_FACTOR: Balance = 100;
/// One byte of transaction data has a fee of 1/1000 of a micro unit.
pub const TRANSACTION_BYTE_FEE: Balance = 1 * GIGAWEI * SUPPLY_FACTOR;
/// Weight to fee conversion factor.
pub const WEIGHT_TO_FEE: u128 = 50 * KILOWEI * SUPPLY_FACTOR;

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
				coeff_integer: SUPPLY_FACTOR,
				negative: false,
			},
		]
	}
}

// TODO: following has to be checked with this: https://github.com/moonbeam-foundation/moonbeam/blob/31dd0f3703d844b139a2f0fafde91025a8b97771/runtime/moonbeam/src/lib.rs#L361
impl pallet_transaction_payment::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type OnChargeTransaction =
		pallet_transaction_payment::CurrencyAdapter<Balances, ToAuthor<Self>>;
	type OperationalFeeMultiplier = ConstU8<5>;
	type WeightToFee = ConstantMultiplier<Balance, ConstU128<{ WEIGHT_TO_FEE }>>;
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
		assert_eq!(
			pallet_transaction_payment::Pallet::<Runtime>::length_to_fee(1),
			TRANSACTION_BYTE_FEE + SUPPLY_FACTOR
		);
		assert_eq!(
			pallet_transaction_payment::Pallet::<Runtime>::length_to_fee(3),
			3 * TRANSACTION_BYTE_FEE + 27 * SUPPLY_FACTOR // 27 because degree 3
		);
	}
}
