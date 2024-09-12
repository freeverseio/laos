use frame_support::traits::EitherOfDiverse;
use frame_system::EnsureRoot;

use crate::{AccountId, Council, Runtime, RuntimeEvent, TechnicalCommittee};

use super::collective::CouncilMajority;

type CouncilMembership = pallet_membership::Instance1;
impl pallet_membership::Config<CouncilMembership> for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type AddOrigin = EnsureRoot<AccountId>;
	type RemoveOrigin = EnsureRoot<AccountId>;
	type SwapOrigin = EnsureRoot<AccountId>;
	type ResetOrigin = EnsureRoot<AccountId>;
	type PrimeOrigin = EnsureRoot<AccountId>;
	type MembershipInitialized = Council;
	type MembershipChanged = Council;
	type MaxMembers = super::collective::MaxMembers;
	type WeightInfo = pallet_membership::weights::SubstrateWeight<Runtime>;
}

type TechnicalCommitteeMembership = pallet_membership::Instance2;
impl pallet_membership::Config<TechnicalCommitteeMembership> for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type AddOrigin = EitherOfDiverse<EnsureRoot<AccountId>, CouncilMajority>;
	type RemoveOrigin = EitherOfDiverse<EnsureRoot<AccountId>, CouncilMajority>;
	type SwapOrigin = EitherOfDiverse<EnsureRoot<AccountId>, CouncilMajority>;
	type ResetOrigin = EitherOfDiverse<EnsureRoot<AccountId>, CouncilMajority>;
	type PrimeOrigin = EitherOfDiverse<EnsureRoot<AccountId>, CouncilMajority>;
	type MembershipInitialized = TechnicalCommittee;
	type MembershipChanged = TechnicalCommittee;
	type MaxMembers = super::collective::MaxMembers;
	type WeightInfo = pallet_membership::weights::SubstrateWeight<Runtime>; // TODO change me
}
