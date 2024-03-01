use crate::{AccountId, Balances, Permill, Runtime, RuntimeEvent, Session, Vec, Weight, UNIT};
use frame_support::{parameter_types, traits::Get};
use sp_consensus_slots::Slot;
use sp_staking::SessionIndex;

parameter_types! {
	// Minimum stake required to be reserved to be a candidate
	pub const MinCandidateStk: u128 = 20_000 * UNIT;
	// Minimum round length is 2 minutes (10 * 12 second block times)
	pub const MinBlocksPerRound: u32 = 10;
	// If a collator doesn't produce any block on this number of rounds, it is notified as inactive
	pub const MaxOfflineRounds: u32 = 1;
	// Rounds before the collator leaving the candidates request can be executed
	pub const LeaveCandidatesDelay: u32 = 4 * 7;
	// Rounds before the candidate bond increase/decrease can be executed
	pub const CandidateBondLessDelay: u32 = 4 * 7;
	// Rounds before the delegator exit can be executed
	pub const LeaveDelegatorsDelay: u32 = 4 * 7;
	// Rounds before the delegator revocation can be executed
	pub const RevokeDelegationDelay: u32 = 4 * 7;
	// Rounds before the delegator bond increase/decrease can be executed
	pub const DelegationBondLessDelay: u32 = 4 * 7;
	// Rounds before the reward is paid
	pub const RewardPaymentDelay: u32 = 2;
	// Minimum collators selected per round, default at genesis and minimum forever after
	pub const MinSelectedCandidates: u32 = 8;
	// Maximum top delegations per candidate
	pub const MaxTopDelegationsPerCandidate: u32 = 300;
	// Maximum bottom delegations per candidate
	pub const MaxBottomDelegationsPerCandidate: u32 = 50;
	// Maximum delegations per delegator
	pub const MaxDelegationsPerDelegator: u32 = 100;
	// Minimum stake required to be reserved to be a delegator
	pub const MinDelegation: u128 = 500 * UNIT;
	pub const MaxCandidates: u32 = 200;
	/// Number of blocks per year: num of seconds in a year / num of seconds per block
	pub const SlotsPerYear: u32 = 31_557_600 / 12;
}

impl pallet_parachain_staking::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type MonetaryGovernanceOrigin = frame_system::EnsureRoot<AccountId>;
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
	type PayoutCollatorReward = (); // TODO
	type OnInactiveCollator = (); // TODO
	type OnNewRound = (); // TODO
	type SlotProvider = StakingRoundSlotProvider;
	type WeightInfo = pallet_parachain_staking::weights::SubstrateWeight<Runtime>;
	type MaxCandidates = MaxCandidates;
	type SlotsPerYear = SlotsPerYear;
}

pub struct BlockAuthor;
impl Get<AccountId> for BlockAuthor {
	fn get() -> AccountId {
		let author = pallet_authorship::Pallet::<Runtime>::author();
		author.unwrap_or_default() // TODO check if it's correct
	}
}
/// TODO:
/// Temporary type that we should replace by RelayChainSlotProvider once async backing is enabled.
pub struct StakingRoundSlotProvider;
impl Get<Slot> for StakingRoundSlotProvider {
	fn get() -> Slot {
		let block_number: u64 = frame_system::pallet::Pallet::<Runtime>::block_number().into();
		Slot::from(block_number)
	}
}

pub struct SessionManager;
impl pallet_session::SessionManager<AccountId> for SessionManager {
	/// 1. A new session starts.
	/// 2. In hook new_session: Read the current top n candidates from the TopCandidates and assign
	///    this set to author blocks for the next session.
	/// 3. AuRa queries the authorities from the session pallet for this session and picks authors
	///    on round-robin-basis from list of authorities.
	fn new_session(new_index: SessionIndex) -> Option<Vec<AccountId>> {
		log::warn!(
			"assembling new collators for new session {} at #{:?}",
			new_index,
			frame_system::pallet::Pallet::<Runtime>::block_number(),
		);

		let collators =
			pallet_parachain_staking::pallet::Pallet::<Runtime>::selected_candidates().to_vec();
		if collators.is_empty() {
			// we never want to pass an empty set of collators. This would brick the chain.
			log::error!("💥 keeping old session because of empty collator set!");
			None
		} else {
			Some(collators)
		}
	}

	fn end_session(_end_index: SessionIndex) {
		// we too are not caring.
	}

	fn start_session(_start_index: SessionIndex) {
		// we too are not caring.
	}
}

pub struct ShouldEndSession;
impl pallet_session::ShouldEndSession<u32> for ShouldEndSession {
	fn should_end_session(now: u32) -> bool {
		let round = pallet_parachain_staking::pallet::Pallet::<Runtime>::round();
		// always update when a new round should start
		round.should_update(now as u64)
	}
}

pub struct NextSessionRotation;
impl frame_support::traits::EstimateNextSessionRotation<u32> for NextSessionRotation {
	fn average_session_length() -> u32 {
		pallet_parachain_staking::pallet::Pallet::<Runtime>::round().length
	}

	fn estimate_current_session_progress(now: u32) -> (Option<Permill>, Weight) {
		let round = pallet_parachain_staking::pallet::Pallet::<Runtime>::round();
		// TODO check the try_into
		let passed_blocks = now.saturating_sub(round.first.try_into().unwrap());

		(
			Some(Permill::from_rational(passed_blocks, round.length)),
			// One read for the round info, blocknumber is read free
			<Runtime as frame_system::Config>::DbWeight::get().reads(1),
		)
	}

	fn estimate_next_session_rotation(_now: u32) -> (Option<u32>, Weight) {
		let round = pallet_parachain_staking::pallet::Pallet::<Runtime>::round();

		// TODO check this try_into
		let first_round: u32 = round.first.try_into().unwrap();
		(
			Some(first_round + round.length),
			// One read for the round info, blocknumber is read free
			<Runtime as frame_system::Config>::DbWeight::get().reads(1),
		)
	}
}
