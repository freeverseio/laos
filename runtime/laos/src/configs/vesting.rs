use crate::{currency::UNIT, Balance, Balances, Runtime, RuntimeEvent};
use frame_support::{parameter_types, traits::WithdrawReasons};
use sp_runtime::traits::ConvertInto;

parameter_types! {
	pub const MinVestedTransfer: Balance = UNIT;
	pub UnvestedFundsAllowedWithdrawReasons: WithdrawReasons =
		WithdrawReasons::except(WithdrawReasons::TRANSFER | WithdrawReasons::RESERVE);
}

impl pallet_vesting::Config for Runtime {
	const MAX_VESTING_SCHEDULES: u32 = 28;

	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type BlockNumberToBalance = ConvertInto;
	type MinVestedTransfer = MinVestedTransfer;
	type UnvestedFundsAllowedWithdrawReasons = UnvestedFundsAllowedWithdrawReasons;
	type WeightInfo = pallet_vesting::weights::SubstrateWeight<Runtime>;
}
