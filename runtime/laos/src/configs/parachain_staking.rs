// Copyright 2023-2024 Freeverse.io
// This file is part of LAOS.

// LAOS is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// LAOS is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with LAOS.  If not, see <http://www.gnu.org/licenses/>.

use crate::{
	currency::UNIT, weights, AccountId, Balances, BlockNumber, Permill, Runtime, RuntimeEvent, Vec,
	MILLISECS_PER_BLOCK,
};
use frame_support::{parameter_types, traits::Get, weights::Weight};
use frame_system::EnsureRoot;
use pallet_parachain_staking::{self as staking, rewards, Config as StakingConfig};
use pallet_session::{SessionManager, ShouldEndSession};
use sp_consensus_slots::Slot;
use sp_staking::SessionIndex;

const SECONDS_PER_YEAR: u32 = 31_557_600;
const SECONDS_PER_BLOCK: u32 = MILLISECS_PER_BLOCK as u32 / 1000;

// Define runtime constants used across the parachain staking configuration.
parameter_types! {
	pub const MinCandidateStk: u128 = 20_000 * UNIT; // Minimum stake to be a staking candidate.
	pub const MinBlocksPerRound: u32 = 10; // Minimum blocks per staking round.
	pub const MaxOfflineRounds: u32 = 1; // Rounds a collator can miss before being marked inactive.
	pub const LeaveCandidatesDelay: u32 = 4 * 7; // Delay for a collator to leave candidates.
	pub const CandidateBondLessDelay: u32 = 4 * 7; // Delay for candidate bond decrease.
	pub const LeaveDelegatorsDelay: u32 = 4 * 7; // Delay for a delegator to exit.
	pub const RevokeDelegationDelay: u32 = 4 * 7; // Delay for revoking a delegation.
	pub const DelegationBondLessDelay: u32 = 4 * 7; // Delay for delegation bond decrease.
	pub const RewardPaymentDelay: u32 = 2; // Delay for reward payments.
	pub const MinSelectedCandidates: u32 = 5; // Minimum selected candidates per round.
	pub const MaxTopDelegationsPerCandidate: u32 = 300; // Max top delegations per candidate.
	pub const MaxBottomDelegationsPerCandidate: u32 = 50; // Max bottom delegations per candidate.
	pub const MaxDelegationsPerDelegator: u32 = 100; // Max delegations per delegator.
	pub const MinDelegation: u128 = 500 * UNIT; // Minimum stake to be a delegator.
	pub const MaxCandidates: u32 = 200; // Max candidates allowed.
	pub const SlotsPerYear: u32 = SECONDS_PER_YEAR / SECONDS_PER_BLOCK; // Number of slots per year.
}

// Implementing the configuration trait for the parachain staking pallet.
impl StakingConfig for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type MonetaryGovernanceOrigin = EnsureRoot<AccountId>;
	// Direct mapping of the parameter types for easy reference and consistency.
	type MinBlocksPerRound = MinBlocksPerRound;
	type MaxOfflineRounds = MaxOfflineRounds;
	type LeaveCandidatesDelay = LeaveCandidatesDelay;
	type CandidateBondLessDelay = CandidateBondLessDelay;
	type LeaveDelegatorsDelay = LeaveDelegatorsDelay;
	type RevokeDelegationDelay = RevokeDelegationDelay;
	type DelegationBondLessDelay = DelegationBondLessDelay;
	type RewardPaymentDelay = RewardPaymentDelay;
	type MinSelectedCandidates = MinSelectedCandidates;
	type MaxTopDelegationsPerCandidate = MaxTopDelegationsPerCandidate;
	type MaxBottomDelegationsPerCandidate = MaxBottomDelegationsPerCandidate;
	type MaxDelegationsPerDelegator = MaxDelegationsPerDelegator;
	type MinCandidateStk = MinCandidateStk;
	type MinDelegation = MinDelegation;
	type BlockAuthor = BlockAuthor;
	type OnCollatorPayout = ();
	type PayoutReward = rewards::TransferFromRewardsAccount;
	type OnInactiveCollator = (); // Placeholder for future implementation.
	type OnNewRound = (); // Placeholder for future implementation.
	type SlotProvider = StakingRoundSlotProvider;
	type MaxCandidates = MaxCandidates;
	type SlotsPerYear = SlotsPerYear;
	type WeightInfo = weights::pallet_parachain_staking::WeightInfo<Runtime>;
}

// Custom struct for identifying the block author.
pub struct BlockAuthor;

// Implementation to fetch the current block author.
impl Get<AccountId> for BlockAuthor {
	// Returns the AccountId of the current block author.
	fn get() -> AccountId {
		pallet_authorship::Pallet::<Runtime>::author().unwrap_or_default()
	}
}

// Provider for the current staking round's slot.
pub struct StakingRoundSlotProvider;

// Implementation to fetch the current slot based on the block number.
impl Get<Slot> for StakingRoundSlotProvider {
	// Converts the current block number to a slot.
	fn get() -> Slot {
		let block_number: u64 = frame_system::Pallet::<Runtime>::block_number().into();
		Slot::from(block_number)
	}
}

// Adapter to integrate the parachain staking logic with the session management.
pub struct ParachainStakingAdapter;

// SessionManager implementation for handling session transitions in staking.
impl SessionManager<AccountId> for ParachainStakingAdapter {
	// Determines the set of collators for the upcoming session.
	fn new_session(new_index: SessionIndex) -> Option<Vec<AccountId>> {
		log::warn!(
			"assembling new collators for new session {} at #{:?}",
			new_index,
			frame_system::Pallet::<Runtime>::block_number(),
		);

		let collators = staking::Pallet::<Runtime>::selected_candidates().to_vec();
		if collators.is_empty() {
			log::error!("💥 keeping old session because of empty collator set!");
			None
		} else {
			Some(collators)
		}
	}

	// Placeholder for session ending logic, if any.
	fn end_session(_end_index: SessionIndex) {}

	// Placeholder for session starting logic, if any.
	fn start_session(_start_index: SessionIndex) {}
}

// Implementation to determine if a session should end based on staking rounds.
impl ShouldEndSession<BlockNumber> for ParachainStakingAdapter {
	// Logic to determine if the current session should end, based on the staking round's status.
	fn should_end_session(now: BlockNumber) -> bool {
		let round = staking::Pallet::<Runtime>::round();
		round.should_update(now.into())
	}
}

// Implementation for estimating session rotations.
impl frame_support::traits::EstimateNextSessionRotation<BlockNumber> for ParachainStakingAdapter {
	// Provides the average length of a session.
	fn average_session_length() -> BlockNumber {
		staking::Pallet::<Runtime>::round().length
	}

	// Estimates the progress of the current session.
	fn estimate_current_session_progress(now: BlockNumber) -> (Option<Permill>, Weight) {
		let round = staking::Pallet::<Runtime>::round();
		let passed_blocks = now.saturating_sub(round.first.try_into().unwrap());

		(
			Some(Permill::from_rational(passed_blocks, round.length)),
			<Runtime as frame_system::Config>::DbWeight::get().reads(1),
		)
	}

	// Estimates when the next session rotation will occur.
	fn estimate_next_session_rotation(_now: BlockNumber) -> (Option<BlockNumber>, Weight) {
		let round = staking::Pallet::<Runtime>::round();
		let first_round: BlockNumber = round.first.try_into().unwrap();
		(
			Some(first_round + round.length),
			<Runtime as frame_system::Config>::DbWeight::get().reads(1),
		)
	}
}

#[cfg(test)]
mod tests {
	use super::{MinCandidateStk, MinDelegation, MinSelectedCandidates};
	use crate::{
		configs::parachain_staking::{MinBlocksPerRound, ParachainStakingAdapter, SlotsPerYear},
		tests::ExtBuilder,
		Balances, ParachainStaking, RuntimeOrigin, System,
	};
	use frame_support::traits::{EstimateNextSessionRotation, Hooks};
	use pallet_session::{SessionManager, ShouldEndSession};
	use sp_runtime::{Percent, Permill};
	use test_utils::roll_one_block;

	const ALICE: [u8; 20] = [1; 20];
	const BOB: [u8; 20] = [2; 20];
	const CHARLIE: [u8; 20] = [3; 20];

	#[test]
	fn new_session_works() {
		// Create a set of candidates with increasing stakes.
		// top `MinSelectedCandidates` candidates are selected.
		let candidates = (0..MinSelectedCandidates::get() * 2)
			.map(|i| ([(i + 4) as u8; 20].into(), MinCandidateStk::get() + i as u128))
			.collect::<Vec<_>>();

		let min_delegation = MinDelegation::get();

		ExtBuilder::default()
			.with_balances(vec![
				(ALICE.into(), min_delegation * 4),
				(BOB.into(), min_delegation * 4),
				(CHARLIE.into(), min_delegation * 4),
			])
			.with_candidates(candidates.clone())
			.build()
			.execute_with(|| {
				// by default, last `MinSelectedCandidates` candidates are selected, because they
				// have the highest stake
				let session = ParachainStakingAdapter::new_session(0).unwrap();
				assert_eq!(
					session,
					candidates
						.iter()
						.skip(candidates.len() - MinSelectedCandidates::get() as usize)
						.map(|(a, _)| *a)
						.collect::<Vec<_>>()
				);

				// Roll to the first block of the next session.
				for _ in 0..MinBlocksPerRound::get() {
					roll_one_block!(true);
				}

				// New session has the same candidates as the previous session.
				let new_session = ParachainStakingAdapter::new_session(1).unwrap();

				assert_eq!(
					new_session,
					// take last `MinSelectedCandidates` candidates
					candidates
						.iter()
						.skip(candidates.len() - MinSelectedCandidates::get() as usize)
						.map(|(a, _)| *a)
						.collect::<Vec<_>>()
				);

				// do some delegations for Alice, Bob and Charlie to 1, 2, 3 candidates
				// they have the lowest stake, they should be top candidates with this delegation
				for (i, acc) in [ALICE, BOB, CHARLIE].iter().enumerate() {
					// check that the candidate is not in the last session's candidates
					assert!(!new_session.contains(&candidates[i].0.clone()));

					ParachainStaking::delegate_with_auto_compound(
						RuntimeOrigin::signed((*acc).into()),
						candidates[i].0,
						min_delegation * 3,
						Percent::from_percent(100),
						0,
						0,
						0,
					)
					.unwrap();
				}

				// Roll to the first block of the next session.
				for _ in 0..MinBlocksPerRound::get() {
					roll_one_block!(true);
				}

				// Check that the new session has the top `MinSelectedCandidates` candidates.
				let new_session = ParachainStakingAdapter::new_session(2).unwrap();
				for (i, _) in [ALICE, BOB, CHARLIE].iter().enumerate() {
					assert!(new_session.contains(&candidates[i].0.clone()));
				}
			});
	}

	#[test]
	fn new_session_empty_candidates() {
		ExtBuilder::default().build().execute_with(|| {
			// Check that the new session has no candidates, this should never be the case,
			// i.e we should always provide candidates in genesis
			let new_session = ParachainStakingAdapter::new_session(1);
			assert!(new_session.is_none());
		});
	}

	#[test]
	fn test_should_end_session() {
		ExtBuilder::default().build().execute_with(|| {
			// Roll to the last block of the current session.
			for _ in 0..MinBlocksPerRound::get() - 1 {
				roll_one_block!(true);
			}

			// Check that the session should not end
			assert!(!ParachainStakingAdapter::should_end_session(System::block_number()));

			// Roll to the first block of the next session.
			// don't run `on_initialize` for staking, otherwise it will update the round.
			roll_one_block!(false);

			// Check that the session should end
			assert!(ParachainStakingAdapter::should_end_session(System::block_number()));
		});
	}

	#[test]
	fn average_session_length_works() {
		ExtBuilder::default().build().execute_with(|| {
			// Check that the average session length is equal to the configured value.
			assert_eq!(ParachainStakingAdapter::average_session_length(), MinBlocksPerRound::get());
		});
	}

	#[test]
	fn estimate_current_session_progress() {
		ExtBuilder::default().build().execute_with(|| {
			// Roll to the half of the current session.
			for _ in 0..ParachainStakingAdapter::average_session_length() / 2 {
				roll_one_block!(true);
			}

			// Estimate the current session progress.
			let (progress, _) =
				ParachainStakingAdapter::estimate_current_session_progress(System::block_number());
			assert_eq!(progress, Some(Permill::from_percent(50)));

			// Roll to the last block of the current session.
			for _ in 0..ParachainStakingAdapter::average_session_length() / 2 {
				roll_one_block!(true);
			}

			// Estimate the current session progress.
			let (progress, _) =
				ParachainStakingAdapter::estimate_current_session_progress(System::block_number());
			// 0% progress because new session has started
			assert_eq!(progress, Some(Permill::from_percent(0)));
		});
	}

	#[test]
	fn estimate_next_session_rotation_works() {
		ExtBuilder::default().build().execute_with(|| {
			roll_one_block!(true);

			// Estimate the next session rotation.
			let (next_session, _) = ParachainStakingAdapter::estimate_next_session_rotation(1);
			assert_eq!(next_session, Some(MinBlocksPerRound::get()));
		});
	}

	#[test]
	fn test_slot_per_year() {
		assert_eq!(SlotsPerYear::get(), 31_557_600 / 12);
	}
}
