use crate::{AccountId, Balances, BlockNumber, Permill, Runtime, RuntimeEvent, Vec, Weight, UNIT};
use frame_support::{parameter_types, traits::Get};
use frame_system::EnsureRoot;
use pallet_parachain_staking::{self as staking, Config as StakingConfig};
use pallet_session::{SessionManager, ShouldEndSession};
use sp_consensus_slots::Slot;
use sp_staking::SessionIndex;

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
	pub const MinSelectedCandidates: u32 = 8; // Minimum selected candidates per round.
	pub const MaxTopDelegationsPerCandidate: u32 = 300; // Max top delegations per candidate.
	pub const MaxBottomDelegationsPerCandidate: u32 = 50; // Max bottom delegations per candidate.
	pub const MaxDelegationsPerDelegator: u32 = 100; // Max delegations per delegator.
	pub const MinDelegation: u128 = 500 * UNIT; // Minimum stake to be a delegator.
	pub const MaxCandidates: u32 = 200; // Max candidates allowed.
	pub const SlotsPerYear: u32 = 31_557_600 / 12; // Number of slots per year.
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
	type PayoutCollatorReward = (); // Placeholder for future implementation.
	type OnInactiveCollator = (); // Placeholder for future implementation.
	type OnNewRound = (); // Placeholder for future implementation.
	type SlotProvider = StakingRoundSlotProvider;
	type WeightInfo = staking::weights::SubstrateWeight<Runtime>;
	type MaxCandidates = MaxCandidates;
	type SlotsPerYear = SlotsPerYear;
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
