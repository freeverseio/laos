use super::MaxCollectivesProposalWeight;
use crate::{
	weights, AccountId, BlockNumber, EnsureRoot, Runtime, RuntimeCall, RuntimeEvent, RuntimeOrigin,
};
use frame_support::parameter_types;
use parachains_common::DAYS;

const COUNCIL_MOTION_DURATION: BlockNumber = 7 * DAYS;

parameter_types! {
	pub const CouncilMotionDuration: BlockNumber = COUNCIL_MOTION_DURATION;
	pub const CouncilMaxProposals: u32 = 7;
	pub const CouncilMaxMembers: u32 = 20;
}

type CouncilCollective = pallet_collective::Instance1;
impl pallet_collective::Config<CouncilCollective> for Runtime {
	type DefaultVote = pallet_collective::MoreThanMajorityThenPrimeDefaultVote;
	type MaxMembers = CouncilMaxMembers;
	type MaxProposalWeight = MaxCollectivesProposalWeight;
	type MaxProposals = CouncilMaxProposals;
	type MotionDuration = CouncilMotionDuration;
	type Proposal = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type RuntimeOrigin = RuntimeOrigin;
	type SetMembersOrigin = EnsureRoot<AccountId>;
	type WeightInfo = weights::pallet_collective::WeightInfo<Runtime>;
}
