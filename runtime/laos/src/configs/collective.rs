use crate::{weights, AccountId, BlockNumber, Runtime, RuntimeCall, RuntimeEvent, RuntimeOrigin};
use frame_support::{pallet_prelude::Weight, parameter_types, traits::EitherOfDiverse};
use frame_system::EnsureRoot;
use laos_primitives::RuntimeBlockWeights;
use parachains_common::{DAYS, MINUTES};
use polkadot_runtime_common::prod_or_fast;
use sp_runtime::Perbill;

parameter_types! {
	pub const MotionDuration: BlockNumber = prod_or_fast!(7 * DAYS, 5 * MINUTES);
	pub const MaxProposals: u32 = 7;
	pub const MaxMembers: u32 = 20;
	pub MaxProposalWeight: Weight = Perbill::from_percent(50) * RuntimeBlockWeights::get().max_block;
}

pub type CouncilMajority =
	pallet_collective::EnsureProportionMoreThan<AccountId, CouncilCollective, 1, 2>;
pub type AllOfCouncil =
	pallet_collective::EnsureProportionAtLeast<AccountId, CouncilCollective, 1, 1>;
pub type TwoThirdOfCouncil =
	pallet_collective::EnsureProportionAtLeast<AccountId, CouncilCollective, 2, 3>;
pub type TechnicalCommitteeMajority =
	pallet_collective::EnsureProportionMoreThan<AccountId, TechnicalCommittee, 1, 2>;
pub type AllOfTechnicalCommittee =
	pallet_collective::EnsureProportionAtLeast<AccountId, TechnicalCommittee, 1, 1>;

pub type CouncilCollective = pallet_collective::Instance1;
impl pallet_collective::Config<CouncilCollective> for Runtime {
	type DefaultVote = pallet_collective::MoreThanMajorityThenPrimeDefaultVote;
	type MaxMembers = MaxMembers;
	type MaxProposalWeight = MaxProposalWeight;
	type MaxProposals = MaxProposals;
	type MotionDuration = MotionDuration;
	type Proposal = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type RuntimeOrigin = RuntimeOrigin;
	type SetMembersOrigin = EnsureRoot<AccountId>;
	type WeightInfo = weights::pallet_collective::WeightInfo<Runtime>;
}

pub type TechnicalCommittee = pallet_collective::Instance2;
impl pallet_collective::Config<TechnicalCommittee> for Runtime {
	type DefaultVote = pallet_collective::MoreThanMajorityThenPrimeDefaultVote;
	type MaxMembers = MaxMembers;
	type MaxProposalWeight = MaxProposalWeight;
	type MaxProposals = MaxProposals;
	type MotionDuration = MotionDuration;
	type Proposal = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type RuntimeOrigin = RuntimeOrigin;
	// the root or a turnout of 50%+1 of the council can select the technical committee
	type SetMembersOrigin = EitherOfDiverse<EnsureRoot<AccountId>, CouncilMajority>;
	type WeightInfo = weights::pallet_collective::WeightInfo<Runtime>;
}
