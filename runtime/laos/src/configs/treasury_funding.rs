use crate::{currency::UNIT, weights, Balance, Runtime, RuntimeEvent};
use frame_support::{parameter_types, PalletId};

parameter_types! {
	pub Step: u32 = 10;
	pub MinAmountForFees: Balance = UNIT;
	pub const TreasuryFundingPalletId: PalletId = PalletId(*b"ls/trsfn");
}

impl pallet_treasury_funding::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type PalletId = TreasuryFundingPalletId;
	type WeightInfo = weights::pallet_treasury_funding::WeightInfo<Runtime>;
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn check_trasury_funding_address() {
		assert_eq!(
			pallet_treasury_funding::Pallet::<Runtime>::account_id().to_string(),
			"0x6d6f646C6c732F747273666e0000000000000000"
		);
	}
}
