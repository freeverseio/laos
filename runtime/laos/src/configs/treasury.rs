use crate::{
	currency::UNIT, AccountId, Balance, Balances, BlockNumber, EnsureRoot, Permill, Runtime,
	RuntimeEvent, Treasury,
};
use frame_support::{parameter_types, PalletId};
use parachains_common::DAYS;

pub type SpendOrigin = frame_support::traits::NeverEnsureOrigin<Balance>;

parameter_types! {
	pub const ProposalBond: Permill = Permill::from_percent(5);
	pub const ProposalBondMinimum: Balance = 100 * UNIT;
	pub const SpendPeriod: BlockNumber = 7 * DAYS;
	pub const MaxApprovals: u32 = 100;
	pub const TreasuryId: PalletId = PalletId(*b"py/trsry");
}

impl pallet_treasury::Config for Runtime {
	type ApproveOrigin = EnsureRoot<AccountId>; //EitherOfDiverse<
											 // 	EnsureRoot<AccountId>,
											 // 	pallet_collective::EnsureProportionAtLeast<AccountId, CouncilCollective, 1, 1>,
											 // >;
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = TreasuryBenchmarkHelper;
	type Burn = ();
	type BurnDestination = ();
	type Currency = Balances;
	type MaxApprovals = MaxApprovals;
	type OnSlash = Treasury;
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
