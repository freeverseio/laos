use super::collective::CouncilMajority;
use crate::{
	currency::UNIT, weights, AccountId, Balance, Balances, BlockNumber, Permill, Runtime,
	RuntimeEvent, Treasury,
};
use frame_support::{
	parameter_types,
	traits::{
		tokens::{PayFromAccount, UnityAssetBalanceConversion},
		EitherOfDiverse,
	},
	PalletId,
};
use frame_system::EnsureRoot;
use parachains_common::{DAYS, MINUTES};
use polkadot_runtime_common::prod_or_fast;
use sp_runtime::traits::IdentityLookup;

parameter_types! {
	pub const Burn: Permill = Permill::zero();
	pub const ProposalBond: Permill = Permill::from_percent(5);
	pub const ProposalBondMinimum: Balance = 100 * UNIT;
	pub const SpendPeriod: BlockNumber = prod_or_fast!(7 * DAYS, 5 * MINUTES);
	pub const MaxApprovals: u32 = 100;
	pub const TreasuryId: PalletId = PalletId(*b"py/trsry");
	pub const PayoutPeriod: BlockNumber = 5;
	pub TreasuryAccount: AccountId = Treasury::account_id();
}

type ApproveOrigin = EitherOfDiverse<EnsureRoot<AccountId>, CouncilMajority>;
type RejectOrigin = EitherOfDiverse<EnsureRoot<AccountId>, CouncilMajority>;

impl pallet_treasury::Config for Runtime {
	type AssetKind = ();
	type ApproveOrigin = ApproveOrigin;
	type BalanceConverter = UnityAssetBalanceConversion;
	type Beneficiary = AccountId;
	type BeneficiaryLookup = IdentityLookup<Self::Beneficiary>;
	type Burn = Burn;
	type BurnDestination = ();
	type Currency = Balances;
	type MaxApprovals = MaxApprovals;
	type OnSlash = Treasury;
	type PalletId = TreasuryId;
	type Paymaster = PayFromAccount<Balances, TreasuryAccount>;
	type PayoutPeriod = PayoutPeriod;
	type ProposalBond = ProposalBond;
	type ProposalBondMaximum = ();
	type ProposalBondMinimum = ProposalBondMinimum;
	type RejectOrigin = RejectOrigin;
	type RuntimeEvent = RuntimeEvent;
	type SpendFunds = ();
	type SpendOrigin = frame_support::traits::NeverEnsureOrigin<Balance>;
	type SpendPeriod = SpendPeriod;
	type WeightInfo = weights::pallet_treasury::WeightInfo<Runtime>;
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = TreasuryBenchmarkHelper;
}

#[cfg(feature = "runtime-benchmarks")]
pub struct TreasuryBenchmarkHelper;

#[cfg(feature = "runtime-benchmarks")]
impl pallet_treasury::ArgumentsFactory<(), AccountId> for TreasuryBenchmarkHelper {
	fn create_asset_kind(_seed: u32) {}

	fn create_beneficiary(seed: [u8; 32]) -> AccountId {
		AccountId::from(seed)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn check_trasury_address() {
		assert_eq!(
			pallet_treasury::Pallet::<Runtime>::account_id().to_string(),
			"0x6d6f646C70792f74727372790000000000000000"
		);
	}
}
