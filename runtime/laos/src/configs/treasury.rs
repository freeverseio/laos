use crate::{ Runtime, AccountId, RuntimeEvent, Balances, Balance, EnsureRoot, BlockNumber, Permill, currency::UNIT };
use frame_support::{ parameter_types, PalletId };
 use parachains_common::DAYS;

pub type SpendOrigin = frame_support::traits::NeverEnsureOrigin<Balance>;
pub const SPEND_PERIOD: BlockNumber = 7 * DAYS;

parameter_types! {
    pub const ProposalBond: Permill = Permill::from_percent(5);
    pub const ProposalBondMinimum: Balance = 50 * UNIT;
    pub const Burn: Permill = Permill::zero();
    pub const SpendPeriod: BlockNumber = SPEND_PERIOD;
    pub const MaxApprovals: u32 = 100;
    pub const TreasuryId: PalletId = PalletId(*b"plmc/tsy");
}

impl pallet_treasury::Config for Runtime {
	type ApproveOrigin = EnsureRoot<AccountId>; //EitherOfDiverse<
	// 	EnsureRoot<AccountId>,
	// 	pallet_collective::EnsureProportionAtLeast<AccountId, CouncilCollective, 1, 1>,
	// >;
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = TreasuryBenchmarkHelper;
	type Burn = Burn;
	type BurnDestination = ();
	type Currency = Balances;
	type MaxApprovals = MaxApprovals;
	type OnSlash = (); // Treasury;
	type PalletId = TreasuryId;
	type ProposalBond = ProposalBond;
	type ProposalBondMaximum = ();
	type ProposalBondMinimum = ProposalBondMinimum;
	type RejectOrigin = EnsureRoot<AccountId>; // EitherOfDiverse<
	// 	EnsureRoot<AccountId>,
	// 	pallet_collective::EnsureProportionAtLeast<AccountId, CouncilCollective, 1, 2>,
	// >;
	type RuntimeEvent = RuntimeEvent;
	type SpendFunds = ();
	type SpendOrigin = SpendOrigin;
	type SpendPeriod = SpendPeriod;
	type WeightInfo = (); //weights::pallet_treasury::WeightInfo<Runtime>;
}