use super::fee::DealWithFees;
use crate::{Balance, Balances, Runtime, RuntimeEvent, WeightToFee, MICROUNIT};
use frame_support::{parameter_types, weights::ConstantMultiplier};
use polkadot_runtime_common::SlowAdjustingFeeUpdate;

parameter_types! {
	/// Relay Chain `TransactionByteFee` / 10
	pub const TransactionByteFee: Balance = 10 * MICROUNIT; // TODO check this value with the one used in deposit function
	pub const OperationalFeeMultiplier: u8 = 5;
}

impl pallet_transaction_payment::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type OnChargeTransaction =
		pallet_transaction_payment::CurrencyAdapter<Balances, DealWithFees<Runtime>>;
	type WeightToFee = WeightToFee;
	type LengthToFee = ConstantMultiplier<Balance, TransactionByteFee>;
	type FeeMultiplierUpdate = SlowAdjustingFeeUpdate<Self>;
	type OperationalFeeMultiplier = OperationalFeeMultiplier;
}
