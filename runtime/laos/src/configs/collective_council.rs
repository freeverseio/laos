use crate::{weights, AccountId, BlockNumber, Runtime, RuntimeCall, RuntimeEvent, RuntimeOrigin};
use frame_support::{pallet_prelude::Weight, parameter_types};
use frame_system::EnsureRoot;
use laos_primitives::RuntimeBlockWeights;
#[cfg(not(feature = "fast-mode"))]
use parachains_common::DAYS;
#[cfg(feature = "fast-mode")]
use parachains_common::MINUTES;
use sp_runtime::Perbill;

#[cfg(feature = "fast-mode")]
pub const COUNCIL_MOTION_DURATION: BlockNumber = 5 * MINUTES;
#[cfg(not(feature = "fast-mode"))]
pub const COUNCIL_MOTION_DURATION: BlockNumber = 7 * DAYS;

pub type CouncilMajority =
	pallet_collective::EnsureProportionMoreThan<AccountId, CouncilCollective, 1, 2>;
pub type AllOfCouncil =
	pallet_collective::EnsureProportionAtLeast<AccountId, CouncilCollective, 1, 1>;
pub type TwoThirdOfCouncil =
	pallet_collective::EnsureProportionAtLeast<AccountId, CouncilCollective, 2, 3>;
pub type HalfOfCouncil =
	pallet_collective::EnsureProportionAtLeast<AccountId, CouncilCollective, 1, 2>;

parameter_types! {
	pub const CouncilMotionDuration: BlockNumber = COUNCIL_MOTION_DURATION;
	pub const CouncilMaxProposals: u32 = 7;
	pub const CouncilMaxMembers: u32 = 20;
	pub MaxProposalWeight: Weight = Perbill::from_percent(50) * RuntimeBlockWeights::get().max_block;
	pub MaxCollectivesProposalWeight: Weight = Perbill::from_percent(50) * RuntimeBlockWeights::get().max_block;
}

pub type CouncilCollective = pallet_collective::Instance1;
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
