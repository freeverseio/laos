use crate::{
	AccountId, BlockNumber, EnsureRoot, Runtime, RuntimeCall, RuntimeEvent, RuntimeOrigin,
};
use frame_support::{pallet_prelude::Weight, parameter_types};
use laos_primitives::RuntimeBlockWeights;
use parachains_common::DAYS;
use sp_runtime::Perbill;

const TECHNICAL_MOTION_DURATION: BlockNumber = 7 * DAYS;

parameter_types! {
	pub const TechnicalMotionDuration: BlockNumber = TECHNICAL_MOTION_DURATION;
	pub const TechnicalMaxProposals: u32 = 7;
	pub const TechnicalMaxMembers: u32 = 5;
	pub MaxProposalWeight: Weight = Perbill::from_percent(50) * RuntimeBlockWeights::get().max_block;
	pub MaxCollectivesProposalWeight: Weight = Perbill::from_percent(50) * RuntimeBlockWeights::get().max_block;
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
