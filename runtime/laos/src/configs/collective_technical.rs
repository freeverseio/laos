use super::MaxCollectivesProposalWeight;
use crate::{
	AccountId, BlockNumber, EnsureRoot, Runtime, RuntimeCall, RuntimeEvent, RuntimeOrigin,
};
use frame_support::parameter_types;
use parachains_common::DAYS;

const TECHNICAL_MOTION_DURATION: BlockNumber = 7 * DAYS;

parameter_types! {
	pub const TechnicalMotionDuration: BlockNumber = TECHNICAL_MOTION_DURATION;
	pub const TechnicalMaxProposals: u32 = 7;
	pub const TechnicalMaxMembers: u32 = 5;
}

type TechnicalCollective = pallet_collective::Instance2;
impl pallet_collective::Config<TechnicalCollective> for Runtime {
	type DefaultVote = pallet_collective::MoreThanMajorityThenPrimeDefaultVote;
	type MaxMembers = TechnicalMaxMembers;
	type MaxProposalWeight = MaxCollectivesProposalWeight;
	type MaxProposals = TechnicalMaxProposals;
	type MotionDuration = TechnicalMotionDuration;
	type Proposal = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type RuntimeOrigin = RuntimeOrigin;
	type SetMembersOrigin = EnsureRoot<AccountId>;
	type WeightInfo = pallet_collective::weights::SubstrateWeight<Runtime>;
}
