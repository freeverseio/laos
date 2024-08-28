use crate::{currency::UNIT, Balance, Balances, BlockNumber, Council, Runtime, RuntimeEvent, weights, Treasury};
use frame_support::parameter_types;
#[cfg(not(feature = "fast-mode"))]
use parachains_common::DAYS;
#[cfg(feature = "fast-mode")]
use parachains_common::HOURS;
use polkadot_runtime_common::CurrencyToVote;

#[cfg(feature = "fast-mode")]
pub const TERM_DURATION: BlockNumber = 4 * HOURS;
#[cfg(not(feature = "fast-mode"))]
pub const TERM_DURATION: BlockNumber = 28 * DAYS;

#[cfg(feature = "fast-mode")]
pub const ELECTION_VOTING_LOCK_DURATION: BlockNumber = 4 * HOURS;
#[cfg(not(feature = "fast-mode"))]
pub const ELECTION_VOTING_LOCK_DURATION: BlockNumber = 28 * DAYS;

parameter_types! {
	pub const CandidacyBond: Balance = 1000 * UNIT;
	pub TermDuration: BlockNumber = TERM_DURATION;
	pub VotingLockPeriod: BlockNumber = ELECTION_VOTING_LOCK_DURATION;
	pub const DesiredMembers: u32 = 9;
	pub const DesiredRunnersUp: u32 = 20;
	pub const MaxCandidates: u32 = 30;
	pub const MaxVoters: u32 = 200;
	pub const MaxVotesPerVoter: u32 = 8;
	pub const VotingBondBase: Balance = 1000 * UNIT;
	pub const VotingBondFactor: Balance = 100 * UNIT;
}

impl pallet_elections_phragmen::Config for Runtime {
	/// How much should be locked up in order to submit one's candidacy.
	type CandidacyBond = CandidacyBond;
	type ChangeMembers = Council;
	type Currency = Balances;
	type CurrencyToVote = CurrencyToVote;
	/// Number of members to elect.
	type DesiredMembers = DesiredMembers;
	/// Number of runners_up to keep.
	type DesiredRunnersUp = DesiredRunnersUp;
	type InitializeMembers = Council;
	type LoserCandidate = Treasury;
	type MaxCandidates = MaxCandidates;
	type MaxVoters = MaxVoters;
	type MaxVotesPerVoter = MaxVotesPerVoter;
	type PalletId = (); // TODO
	type RuntimeEvent = RuntimeEvent;
	/// How long each seat is kept. This defines the next block number at which
	/// an election round will happen. If set to zero, no elections are ever
	/// triggered and the module will be in passive mode.
	type TermDuration = TermDuration;
	type VotingBondBase = VotingBondBase;
	type VotingBondFactor = VotingBondFactor;
	type KickedMember = (); 
	type WeightInfo = weights::pallet_elections_phragmen::WeightInfo<Runtime>;
}
