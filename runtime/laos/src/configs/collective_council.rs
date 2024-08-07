use crate::{ AccountId, BlockNumber, Runtime, EnsureRoot, RuntimeCall, RuntimeOrigin, RuntimeEvent };
use frame_support::parameter_types;
use parachains_common::DAYS;

const COUNCIL_MOTION_DURATION: BlockNumber = 7 * DAYS;

parameter_types! {
    pub const CouncilMotionDuration: BlockNumber = COUNCIL_MOTION_DURATION;
	pub const CouncilMaxProposals: u32 = 7;
	pub const CouncilMaxMembers: u32 = 20;
    pub const MaxProposalWeight: Weight = Perbill::from_percent(50) * RuntimeBlockWeights::get().max_block;
}

impl pallet_collective::Config for Runtime {
	type DefaultVote = pallet_collective::MoreThanMajorityThenPrimeDefaultVote;
	type MaxMembers = CouncilMaxMembers;
	type MaxProposalWeight = MaxCollectivesProposalWeight;
	type MaxProposals = CouncilMaxProposals;
	type MotionDuration = CouncilMotionDuration;
	type Proposal = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type RuntimeOrigin = RuntimeOrigin;
	type SetMembersOrigin = EnsureRoot<AccountId>;
	type WeightInfo = (); //  weights::pallet_collective::WeightInfo<Runtime>;
}
