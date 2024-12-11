use crate::{currency::UNIT, Balance, Runtime, RuntimeEvent};
use frame_support::{parameter_types, PalletId};

parameter_types! {
	pub Step: u32 = 10;
	pub MinAmountForFees: Balance = UNIT;
	pub const TreasuryFundingPalletId: PalletId = PalletId(*b"py/trsfn");
}

impl pallet_treasury_funding::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type PalletId = TreasuryFundingPalletId;
	type WeightInfo = ();
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn check_trasury_funding_address() {
		assert_eq!(
			pallet_treasury_funding::Pallet::<Runtime>::account_id().to_string(),
			"0x6D6F646C70792f747273666E0000000000000000"
		);
	}
}

