use crate::{
	currency::calculate_deposit, weights, AccountId, Balance, Balances, EnsureRoot, Runtime,
	RuntimeEvent,
};
use frame_support::parameter_types;

parameter_types! {
	pub const PreimageBaseDeposit: Balance = calculate_deposit(2, 64);
	pub const PreimageByteDeposit: Balance = calculate_deposit(0, 1);
}

impl pallet_preimage::Config for Runtime {
	type BaseDeposit = PreimageBaseDeposit;
	type ByteDeposit = PreimageByteDeposit;
	type Currency = Balances;
	type ManagerOrigin = EnsureRoot<AccountId>;
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = weights::pallet_preimage::WeightInfo<Runtime>;
}
