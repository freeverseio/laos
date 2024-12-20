use super::collective::{
	AllOfCouncil, AllOfTechnicalCommittee, CouncilMajority, HalfOfCouncil, TechnicalCommittee,
	TechnicalCommitteeMajority, TwoThirdOfCouncil,
};
use crate::{
	currency::UNIT, weights, AccountId, Balance, Balances, BlockNumber, OriginCaller, Preimage,
	Runtime, RuntimeEvent, Scheduler, Treasury,
};
use frame_support::{parameter_types, traits::EitherOfDiverse};
use frame_system::{EnsureRoot, EnsureSigned};
use pallet_collective::EnsureMember;
use parachains_common::{DAYS, HOURS, MINUTES};
use polkadot_runtime_common::prod_or_fast;

parameter_types! {
	pub  LaunchPeriod: BlockNumber = prod_or_fast!(7 * DAYS, 5 * MINUTES, "LAUNCH_PERIOD");
	pub  VotingPeriod: BlockNumber = prod_or_fast!(7 * DAYS, 5 * MINUTES, "VOTING_PERIOD");
	pub  FastTrackVotingPeriod: BlockNumber = prod_or_fast!(3 * HOURS, 3 * MINUTES, "FAST_TRACK_VOTING_PERIOD");
	pub  EnactmentPeriod: BlockNumber = prod_or_fast!(8 * DAYS, 6 * MINUTES, "ENACTMENT_PERIOD");
	pub  CooloffPeriod: BlockNumber = prod_or_fast!(7 * DAYS, 5 * MINUTES, "COOLOFF_PERIOD");
	pub const MaxProposals: u32 = 100;
	pub const InstantAllowed: bool = true;
	pub const MinimumDeposit: Balance = 1000 * UNIT;
	pub const MaxVotes: u32 = 100;
	pub const MaxDeposits: u32 = 100;
	pub const MaxBlacklisted: u32 = 100;
}

impl pallet_democracy::Config for Runtime {
	type BlacklistOrigin = EnsureRoot<AccountId>;
	// To cancel a proposal before it has been passed, the technical committee must be unanimous or
	// Root must agree.
	type CancelProposalOrigin = EitherOfDiverse<EnsureRoot<AccountId>, AllOfTechnicalCommittee>;
	// To cancel a proposal which has been passed, 2/3 of the council must agree to it.
	type CancellationOrigin = TwoThirdOfCouncil;
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
	type ExternalMajorityOrigin = HalfOfCouncil;
	/// A straight majority of the council can decide what their next motion is.
	type ExternalOrigin = CouncilMajority;
	/// Majority of technical committee can have an ExternalMajority/ExternalDefault vote
	/// be tabled immediately and with a shorter voting/enactment period.
	type FastTrackOrigin = TechnicalCommitteeMajority;
	type FastTrackVotingPeriod = FastTrackVotingPeriod;
	type InstantAllowed = InstantAllowed;
	type InstantOrigin = AllOfTechnicalCommittee;
	// Same as EnactmentPeriod
	/// How often (in blocks) new public referenda are launched.
	type LaunchPeriod = LaunchPeriod;
	type MaxBlacklisted = MaxBlacklisted;
	type MaxDeposits = MaxDeposits;
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
	// Any single technical committee member may veto a coming council proposal, however they can
	// only do it once and it lasts only for the cool-off period.
	type VetoOrigin = EnsureMember<AccountId, TechnicalCommittee>;
	type VoteLockingPeriod = EnactmentPeriod;
	/// How often (in blocks) to check for new votes.
	type VotingPeriod = VotingPeriod;
	type WeightInfo = weights::pallet_democracy::WeightInfo<Runtime>;
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{
		tests::{ExtBuilder, ALICE, BOB},
		AccountId, RuntimeCall, RuntimeOrigin,
	};
	use core::str::FromStr;
	use frame_support::{
		assert_noop, assert_ok,
		traits::{Contains, StorePreimage},
	};

	#[test]
	fn can_veto_proposal() {
		let alice = AccountId::from_str(ALICE).unwrap();
		let bob = AccountId::from_str(BOB).unwrap();

		ExtBuilder::default()
			.with_balances(vec![(alice, 1000 * UNIT), (bob, 1000 * UNIT)])
			.build()
			.execute_with(|| {
				frame_system::Pallet::<Runtime>::set_block_number(1);

				let call_to_execute = frame_system::Call::remark { remark: b"123".to_vec() };
				let call_to_bound = RuntimeCall::System(call_to_execute);
				let preimage = pallet_preimage::Pallet::<Runtime>::bound(call_to_bound).unwrap();
				let preimage_hash = preimage.hash();
				let proposal_length_bound = 60_u32;

				let council_origin =
					OriginCaller::Council(pallet_collective::RawOrigin::Members(1, 1));
				let veto_call =
					Box::new(RuntimeCall::Democracy(pallet_democracy::Call::veto_external {
						proposal_hash: preimage_hash,
					}));

				// adding the external proposal
				assert_ok!(pallet_democracy::Pallet::<Runtime>::external_propose(
					council_origin.clone().into(),
					preimage.clone()
				));

				// checking that alice does not belong to the technical committee
				assert!(
					!pallet_membership::Pallet::<Runtime, pallet_membership::Instance2>::contains(
						&alice
					)
				);

				// alice cannot veto the external proposal as she does not belong to the technical
				// committee
				assert_noop!(
					pallet_collective::Pallet::<Runtime, pallet_collective::Instance2>::execute(
						RuntimeOrigin::signed(alice),
						veto_call.clone(),
						proposal_length_bound
					),
					pallet_collective::Error::<Runtime, pallet_collective::Instance2>::NotMember
				);

				// adding alice to the technical committee
				frame_system::Pallet::<Runtime>::set_block_number(2);
				pallet_membership::Pallet::<Runtime, pallet_membership::Instance2>::add_member(
					RuntimeOrigin::root(),
					alice,
				)
				.unwrap();

				// checking that alice belongs to the technical committee
				assert!(
					pallet_membership::Pallet::<Runtime, pallet_membership::Instance2>::contains(
						&alice
					)
				);

				// alice can veto the proposal
				assert_ok!(
					pallet_collective::Pallet::<Runtime, pallet_collective::Instance2>::execute(
						RuntimeOrigin::signed(alice),
						veto_call,
						proposal_length_bound
					)
				);
				assert!(pallet_democracy::pallet::NextExternal::<Runtime>::get().is_none());

				// the same preimage cannot be proposed again as we're still in the cooloff period
				assert_noop!(
					pallet_democracy::Pallet::<Runtime>::external_propose(
						council_origin.clone().into(),
						preimage.clone()
					),
					pallet_democracy::Error::<Runtime>::ProposalBlacklisted
				);

				// retrieving the block number at which the proposal is no longer blacklisted
				let cooloff_block =
					pallet_democracy::pallet::Blacklist::<Runtime>::get(preimage_hash)
						.map(|item| item.0)
						.unwrap();

				// the same preimage can be proposed again as the cooloff period is over
				frame_system::Pallet::<Runtime>::set_block_number(cooloff_block);
				assert_ok!(pallet_democracy::Pallet::<Runtime>::external_propose(
					council_origin.clone().into(),
					preimage
				));
				assert!(pallet_democracy::pallet::NextExternal::<Runtime>::get().is_some());

				// alice cannot veto the external proposal as she already vetoed it.
				// NOTE: the call executes successfully, but the dispatch of `veto_external`
				// emits `AlreadyVetoed`
				assert_ok!(
					pallet_collective::Pallet::<Runtime, pallet_collective::Instance2>::execute(
						RuntimeOrigin::signed(alice),
						Box::new(RuntimeCall::Democracy(pallet_democracy::Call::veto_external {
							proposal_hash: preimage_hash,
						})),
						proposal_length_bound,
					)
				);
				let events = frame_system::Pallet::<Runtime>::events();
				match events.last().unwrap().event {
					RuntimeEvent::TechnicalCommittee(
						pallet_collective::Event::MemberExecuted { result, .. },
					) => {
						assert!(
							sp_runtime::DispatchError::from(
								pallet_democracy::Error::<Runtime>::AlreadyVetoed
							) == result.unwrap_err()
						);
					},
					_ => panic!("event is not 'MemberExecuted'"),
				};
			});
	}
}
