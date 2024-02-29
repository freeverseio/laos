use crate::*;

use frame_support::{
	parameter_types,
	traits::{ConstU128, ConstU32},
};

parameter_types! {
	pub const MinCandidateStk: u128 = 20_000 * UNIT;
}

impl pallet_parachain_staking::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type MonetaryGovernanceOrigin = frame_system::EnsureRoot<AccountId>;
	/// Minimum round length is 2 minutes (10 * 12 second block times)
	type MinBlocksPerRound = ConstU32<10>;
	/// If a collator doesn't produce any block on this number of rounds, it is notified as inactive
	type MaxOfflineRounds = ConstU32<1>;
	/// Rounds before the collator leaving the candidates request can be executed
	type LeaveCandidatesDelay = ConstU32<{ 4 * 7 }>;
	/// Rounds before the candidate bond increase/decrease can be executed
	type CandidateBondLessDelay = ConstU32<{ 4 * 7 }>;
	/// Rounds before the delegator exit can be executed
	type LeaveDelegatorsDelay = ConstU32<{ 4 * 7 }>;
	/// Rounds before the delegator revocation can be executed
	type RevokeDelegationDelay = ConstU32<{ 4 * 7 }>;
	/// Rounds before the delegator bond increase/decrease can be executed
	type DelegationBondLessDelay = ConstU32<{ 4 * 7 }>;
	/// Rounds before the reward is paid
	type RewardPaymentDelay = ConstU32<2>;
	/// Minimum collators selected per round, default at genesis and minimum forever after
	type MinSelectedCandidates = ConstU32<8>;
	/// Maximum top delegations per candidate
	type MaxTopDelegationsPerCandidate = ConstU32<300>;
	/// Maximum bottom delegations per candidate
	type MaxBottomDelegationsPerCandidate = ConstU32<50>;
	/// Maximum delegations per delegator
	type MaxDelegationsPerDelegator = ConstU32<100>;
	/// Minimum stake required to be reserved to be a candidate
	type MinCandidateStk = MinCandidateStk;
	/// Minimum stake required to be reserved to be a delegator
	type MinDelegation = ConstU128<{ 500 * UNIT }>;
	type BlockAuthor = (); // TODO
	type OnCollatorPayout = ();
	type PayoutCollatorReward = (); // TODO
	type OnInactiveCollator = (); // TODO
	type OnNewRound = (); // TODO
	type SlotProvider = (); // TODO
	type WeightInfo = pallet_parachain_staking::weights::SubstrateWeight<Runtime>;
	type MaxCandidates = ConstU32<200>;
	type SlotsPerYear = ConstU32<{ 31_557_600 / 12 }>;
}
