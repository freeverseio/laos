use crate::{AccountId, Balances, Runtime, RuntimeEvent, UNIT};
use frame_support::parameter_types;

parameter_types! {
	// Minimum stake required to be reserved to be a candidate
	pub const MinCandidateStk: u128 = 20_000 * UNIT;
	// Minimum round length is 2 minutes (10 * 12 second block times)
	pub const MinBlocksPerRound: u32 = 10;
	// If a collator doesn't produce any block on this number of rounds, it is notified as inactive
	pub const MaxOfflineRounds: u32 = 1;
	// Rounds before the collator leaving the candidates request can be executed
	pub const LeaveCandidatesDelay: u32 = 4 * 7;
	// Rounds before the candidate bond increase/decrease can be executed
	pub const CandidateBondLessDelay: u32 = 4 * 7;
	// Rounds before the delegator exit can be executed
	pub const LeaveDelegatorsDelay: u32 = 4 * 7;
	// Rounds before the delegator revocation can be executed
	pub const RevokeDelegationDelay: u32 = 4 * 7;
	// Rounds before the delegator bond increase/decrease can be executed
	pub const DelegationBondLessDelay: u32 = 4 * 7;
	// Rounds before the reward is paid
	pub const RewardPaymentDelay: u32 = 2;
	// Minimum collators selected per round, default at genesis and minimum forever after
	pub const MinSelectedCandidates: u32 = 8;
	// Maximum top delegations per candidate
	pub const MaxTopDelegationsPerCandidate: u32 = 300;
	// Maximum bottom delegations per candidate
	pub const MaxBottomDelegationsPerCandidate: u32 = 50;
	// Maximum delegations per delegator
	pub const MaxDelegationsPerDelegator: u32 = 100;
	// Minimum stake required to be reserved to be a delegator
	pub const MinDelegation: u128 = 500 * UNIT;
	pub const MaxCandidates: u32 = 200;
	/// Number of blocks per year: num of seconds in a year / num of seconds per block
	pub const SlotsPerYear: u32 = 31_557_600 / 12;
}

impl pallet_parachain_staking::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type MonetaryGovernanceOrigin = frame_system::EnsureRoot<AccountId>;
	type MinBlocksPerRound = MinBlocksPerRound;
	type MaxOfflineRounds = MaxOfflineRounds;
	type LeaveCandidatesDelay = LeaveCandidatesDelay;
	type CandidateBondLessDelay = CandidateBondLessDelay;
	type LeaveDelegatorsDelay = LeaveDelegatorsDelay;
	type RevokeDelegationDelay = RevokeDelegationDelay;
	type DelegationBondLessDelay = DelegationBondLessDelay;
	type RewardPaymentDelay = RewardPaymentDelay;
	type MinSelectedCandidates = MinSelectedCandidates;
	type MaxTopDelegationsPerCandidate = MaxTopDelegationsPerCandidate;
	type MaxBottomDelegationsPerCandidate = MaxBottomDelegationsPerCandidate;
	type MaxDelegationsPerDelegator = MaxDelegationsPerDelegator;
	type MinCandidateStk = MinCandidateStk;
	type MinDelegation = MinDelegation;
	type BlockAuthor = (); // TODO
	type OnCollatorPayout = ();
	type PayoutCollatorReward = (); // TODO
	type OnInactiveCollator = (); // TODO
	type OnNewRound = (); // TODO
	type WeightInfo = pallet_parachain_staking::weights::SubstrateWeight<Runtime>;
	type MaxCandidates = MaxCandidates;
	type SlotsPerYear = SlotsPerYear;
}
