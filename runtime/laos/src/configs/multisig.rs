use super::calculate_deposit;
use crate::{Balance, Balances, Runtime, RuntimeCall, RuntimeEvent};
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
	type WeightInfo = pallet_multisig::weights::SubstrateWeight<Runtime>;
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::MILLIUNIT;

	#[test]
	fn check_deposits() {
		assert_eq!(<Runtime as pallet_multisig::Config>::DepositBase::get(), 10_560 * MILLIUNIT);
		assert_eq!(<Runtime as pallet_multisig::Config>::DepositFactor::get(), 320 * MILLIUNIT);
	}
}
