use frame_support::traits::EitherOfDiverse;
use frame_system::EnsureRoot;

use crate::{weights, AccountId, Runtime, RuntimeEvent, TechnicalCommittee};

use super::collective::CouncilMajority;

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
	type MaxMembers = super::collective::MaxMembersTechnicalCommittee;
	type WeightInfo = weights::pallet_membership::WeightInfo<Runtime>;
}
