use crate::{
	AccountId, BlockNumber, EnsureRoot, Runtime, RuntimeCall, RuntimeEvent, RuntimeOrigin,
};
use frame_support::{pallet_prelude::Weight, parameter_types};
use laos_primitives::RuntimeBlockWeights;
use parachains_common::DAYS;
use sp_runtime::Perbill;

const COUNCIL_MOTION_DURATION: BlockNumber = 7 * DAYS;

parameter_types! {
	pub const CouncilMotionDuration: BlockNumber = COUNCIL_MOTION_DURATION;
	pub const CouncilMaxProposals: u32 = 7;
	pub const CouncilMaxMembers: u32 = 20;
	pub MaxProposalWeight: Weight = Perbill::from_percent(50) * RuntimeBlockWeights::get().max_block;
	pub MaxCollectivesProposalWeight: Weight = Perbill::from_percent(50) * RuntimeBlockWeights::get().max_block;
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
	type WeightInfo = (); //  weights::pallet_collective::WeightInfo<Runtime>;
}
