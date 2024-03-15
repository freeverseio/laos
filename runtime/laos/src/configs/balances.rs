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
	pub const MaxHolds: u32 = 0;
	pub const MaxFreezes: u32 = 1;
}

impl pallet_balances::Config for Runtime {
	type MaxLocks = MaxLocks;
	/// The type for recording an account's balance.
	type Balance = Balance;
	/// The ubiquitous event type.
	type RuntimeEvent = RuntimeEvent;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = pallet_balances::weights::SubstrateWeight<Runtime>;
	type MaxReserves = MaxReserves;
	type ReserveIdentifier = [u8; 8];
	type FreezeIdentifier = RuntimeFreezeReason;
	type MaxHolds = MaxHolds;
	type RuntimeHoldReason = ();
	type MaxFreezes = MaxFreezes;
}
