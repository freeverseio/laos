use crate::{Balance, Balances, Runtime, RuntimeEvent, UNIT};
use frame_support::{parameter_types, traits::WithdrawReasons};
use sp_runtime::traits::ConvertInto;

// Configuration of the Vesting Pallet
parameter_types! {
	pub const MinVestedTransfer: Balance = UNIT;
	pub UnvestedFundsAllowedWithdrawReasons: WithdrawReasons =
		WithdrawReasons::except(WithdrawReasons::TRANSFER | WithdrawReasons::RESERVE);
}

impl pallet_vesting::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type BlockNumberToBalance = ConvertInto;
	type WeightInfo = pallet_vesting::weights::SubstrateWeight<Runtime>;
	type MinVestedTransfer = MinVestedTransfer;
	type UnvestedFundsAllowedWithdrawReasons = UnvestedFundsAllowedWithdrawReasons;
	const MAX_VESTING_SCHEDULES: u32 = 28;
}
