use crate::{Balance, Runtime, RuntimeEvent, RuntimeFreezeReason, System};
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
	pub const MaxLocks: u32 = 50;
	pub const MaxReserves: u32 = 50;
}

impl pallet_balances::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeHoldReason = ();
	type FreezeIdentifier = RuntimeFreezeReason;
	type Balance = Balance;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type ReserveIdentifier = [u8; 8];
	type MaxHolds = MaxLocks;
	type MaxFreezes = MaxReserves;
	type MaxLocks = MaxLocks;
	type MaxReserves = MaxReserves;
	type WeightInfo = pallet_balances::weights::SubstrateWeight<Runtime>;
}
