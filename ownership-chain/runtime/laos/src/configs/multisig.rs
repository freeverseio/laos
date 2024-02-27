use crate::{Balances, Runtime, RuntimeCall, RuntimeEvent, UNIT};
use frame_support::parameter_types;
use ownership_parachain_primitives::Balance;

parameter_types! {
	// One storage item; key size is 32; value is size 4+4+16+32 bytes = 56 bytes
	// Fixed to 1 UNIT
	pub const DepositBase: Balance = UNIT;
	// Additional storage item size of 32 bytes.
	// Fixed to 0.1 UNIT
	pub const DepositFactor: Balance = UNIT / 10;
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
