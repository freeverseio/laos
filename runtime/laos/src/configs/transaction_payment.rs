use crate::{
	currency::{TRANSACTION_BYTE_FEE, WEIGHT_FEE},
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

pub struct LengthToFee;
impl WeightToFeePolynomial for LengthToFee {
	type Balance = Balance;

	fn polynomial() -> WeightToFeeCoefficients<Self::Balance> {
		smallvec![WeightToFeeCoefficient {
			degree: 1,
			coeff_frac: Perbill::zero(),
			coeff_integer: TRANSACTION_BYTE_FEE,
			negative: false,
		},]
	}
}

impl pallet_transaction_payment::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type OnChargeTransaction =
		pallet_transaction_payment::CurrencyAdapter<Balances, ToAuthor<Self>>;
	type OperationalFeeMultiplier = ConstU8<5>;
	type WeightToFee = ConstantMultiplier<Balance, ConstU128<{ WEIGHT_FEE }>>;
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
			WEIGHT_FEE
		);
		// in case weight to fee exceeds max_block
		assert_eq!(
			pallet_transaction_payment::Pallet::<Runtime>::weight_to_fee(
				frame_support::weights::Weight::from_parts(u64::MAX, 0)
			),
			<Runtime as frame_system::Config>::BlockWeights::get().max_block.ref_time() as u128 *
				WEIGHT_FEE
		);
	}

	#[test]
	fn test_length_to_fee() {
		use super::*;
		// zero length
		assert_eq!(pallet_transaction_payment::Pallet::<Runtime>::length_to_fee(0), 0);
		assert_eq!(
			pallet_transaction_payment::Pallet::<Runtime>::length_to_fee(1),
			TRANSACTION_BYTE_FEE
		);
		assert_eq!(
			pallet_transaction_payment::Pallet::<Runtime>::length_to_fee(3),
			3 * TRANSACTION_BYTE_FEE
		);
	}
}
