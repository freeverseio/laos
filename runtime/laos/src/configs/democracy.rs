use super::collective_council::{
	AllOfCouncil, CouncilCollective, HalfOfCouncil, TwoThirdOfCouncil,
};
use crate::{
	currency::UNIT, weights, AccountId, Balance, Balances, BlockNumber, OriginCaller, Preimage,
	Runtime, RuntimeEvent, Scheduler, Treasury,
};
use frame_support::{parameter_types, traits::EitherOfDiverse};
use frame_system::{EnsureRoot, EnsureSigned};
use parachains_common::{DAYS, HOURS, MINUTES};
use polkadot_runtime_common::prod_or_fast;
use sp_core::ConstU32;

parameter_types! {
	pub  LaunchPeriod: BlockNumber = prod_or_fast!(7 * DAYS, 1 * MINUTES, "LAUNCH_PERIOD");
	pub  VotingPeriod: BlockNumber = prod_or_fast!(7 * DAYS, 1 * MINUTES, "VOTING_PERIOD");
	pub  FastTrackVotingPeriod: BlockNumber = prod_or_fast!(3 * HOURS, 1 * MINUTES, "FAST_TRACK_VOTING_PERIOD");
	pub const InstantAllowed: bool = false;
	pub const MinimumDeposit: Balance = 1000 * UNIT;
	pub  EnactmentPeriod: BlockNumber = prod_or_fast!(8 * DAYS, 1 * MINUTES, "ENACTMENT_PERIOD");
	pub  CooloffPeriod: BlockNumber = prod_or_fast!(7 * DAYS, 1 * MINUTES, "COOLOFF_PERIOD");
	pub const MaxProposals: u32 = 100;
	pub const MaxVotes: u32 = 100;
}

impl pallet_democracy::Config for Runtime {
	type BlacklistOrigin = EnsureRoot<AccountId>;
	// To cancel a proposal before it has been passed, must be root.
	type CancelProposalOrigin = EnsureRoot<AccountId>;
	// To cancel a proposal which has been passed, 2/3 of the council must agree to
	// it.
	type CancellationOrigin = EitherOfDiverse<EnsureRoot<AccountId>, TwoThirdOfCouncil>;
	/// Period in blocks where an external proposal may not be re-submitted
	/// after being vetoed.
	type CooloffPeriod = CooloffPeriod;
	type Currency = Balances;
	/// The minimum period of locking and the period between a proposal being
	/// approved and enacted.
	///
	/// It should generally be a little more than the unstake period to ensure
	/// that voting stakers have an opportunity to remove themselves from the
	/// system in the case where they are on the losing side of a vote.
	type EnactmentPeriod = EnactmentPeriod;
	/// A unanimous council can have the next scheduled referendum be a straight
	/// default-carries (NTB) vote.
	type ExternalDefaultOrigin = AllOfCouncil;
	/// A simple-majority can have the next scheduled referendum be a straight
	/// majority-carries vote.
	type ExternalMajorityOrigin =
		pallet_collective::EnsureProportionAtLeast<AccountId, CouncilCollective, 1, 2>;
	/// A straight majority of the council can decide what their next motion is.
	type ExternalOrigin = HalfOfCouncil;
	/// Half of the council can have an ExternalMajority/ExternalDefault vote
	/// be tabled immediately and with a shorter voting/enactment period.
	type FastTrackOrigin = EitherOfDiverse<EnsureRoot<AccountId>, HalfOfCouncil>;
	type FastTrackVotingPeriod = FastTrackVotingPeriod;
	type InstantAllowed = InstantAllowed;
	type InstantOrigin = EitherOfDiverse<EnsureRoot<AccountId>, AllOfCouncil>;
	// Same as EnactmentPeriod
	/// How often (in blocks) new public referenda are launched.
	type LaunchPeriod = LaunchPeriod;
	type MaxBlacklisted = ConstU32<100>;
	type MaxDeposits = ConstU32<100>;
	type MaxProposals = MaxProposals;
	type MaxVotes = MaxVotes;
	/// The minimum amount to be used as a deposit for a public referendum
	/// proposal.
	type MinimumDeposit = MinimumDeposit;
	type PalletsOrigin = OriginCaller;
	type Preimages = Preimage;
	type RuntimeEvent = RuntimeEvent;
	type Scheduler = Scheduler;
	/// Handler for the unbalanced reduction when slashing a preimage deposit.
	type Slash = Treasury;
	type SubmitOrigin = EnsureSigned<AccountId>;
	// Any single council member may veto a coming council proposal, however they
	// can only do it once and it lasts only for the cooloff period.
	type VetoOrigin = pallet_collective::EnsureMember<AccountId, CouncilCollective>;
	type VoteLockingPeriod = EnactmentPeriod;
	/// How often (in blocks) to check for new votes.
	type VotingPeriod = VotingPeriod;
	type WeightInfo = weights::pallet_democracy::WeightInfo<Runtime>;
}