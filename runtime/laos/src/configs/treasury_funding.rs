use crate::{currency::UNIT, AccountId, Balance, Runtime, RuntimeEvent};
use frame_support::parameter_types;
use hex_literal::hex;

parameter_types! {
	pub TreasuryFundingVault: AccountId = hex!("9d531e3e6b0415cd79839f1fafced4822b14c23d").into();
	pub Step: u32 = 10;
	pub MinAmountForFees: Balance = UNIT;
}

impl pallet_treasury_funding::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	type VaultAccountId = TreasuryFundingVault;
	type OperationStep = Step;
	type MinAmountForFees = MinAmountForFees;
}
