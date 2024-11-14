use crate::{
	currency::UNIT, weights, Balance, Balances, BlockNumber, Council, Runtime, RuntimeEvent,
	Treasury, DAYS, MINUTES,
};
use frame_support::{parameter_types, traits::LockIdentifier};
use polkadot_runtime_common::{prod_or_fast, CurrencyToVote};

parameter_types! {
	pub const CandidacyBond: Balance = 1000 * UNIT;
	pub TermDuration: BlockNumber = prod_or_fast!(28 * DAYS, 10 * MINUTES);
	pub const DesiredRunnersUp: u32 = 20;
	pub const MaxCandidates: u32 = 30;
	pub const MaxVoters: u32 = 200;
	pub const MaxVotesPerVoter: u32 = 8;
	pub const VotingBondBase: Balance = 1000 * UNIT;
	pub const VotingBondFactor: Balance = 100 * UNIT;
	pub const ElectionsPhragmenPalletId: LockIdentifier = *b"phrelect";
}

impl pallet_elections_phragmen::Config for Runtime {
	/// How much should be locked up in order to submit one's candidacy.
	type CandidacyBond = CandidacyBond;
	type ChangeMembers = Council;
	type Currency = Balances;
	type CurrencyToVote = CurrencyToVote;
	/// Number of members to elect.
	type DesiredMembers = super::collective::MaxMembersCouncil;
	/// Number of runners_up to keep.
	type DesiredRunnersUp = DesiredRunnersUp;
	type InitializeMembers = Council;
	type LoserCandidate = Treasury;
	type MaxCandidates = MaxCandidates;
	type MaxVoters = MaxVoters;
	type MaxVotesPerVoter = MaxVotesPerVoter;
	type PalletId = ElectionsPhragmenPalletId;
	type RuntimeEvent = RuntimeEvent;
	/// How long each seat is kept. This defines the next block number at which
	/// an election round will happen. If set to zero, no elections are ever
	/// triggered and the module will be in passive mode.
	type TermDuration = TermDuration;
	type VotingBondBase = VotingBondBase;
	type VotingBondFactor = VotingBondFactor;
	type KickedMember = Treasury;
	type WeightInfo = weights::pallet_elections_phragmen::WeightInfo<Runtime>;
}
