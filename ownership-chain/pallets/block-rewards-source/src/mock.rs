use crate as pallet_block_rewards_source;
use crate::Config;
use frame_support::{
	parameter_types,
	traits::{ConstU16, ConstU64, Get},
	weights::constants::RocksDbWeight,
};
use sp_consensus_slots::Slot;
use sp_core::H256;
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup},
	BuildStorage,
};

type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test
	{
		System: frame_system,
		BlockRewardsSource: pallet_block_rewards_source,
		Balances: pallet_balances,
		BlockAuthor: block_author,
		ParachainStaking: pallet_parachain_staking,
	}
);

pub type AccountId = u64;

impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = RocksDbWeight;
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Nonce = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_types! {
	pub const MaxTokenUriLength: u32 = 512;
}

impl pallet_balances::Config for Test {
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 4];
	type MaxLocks = ();
	type Balance = Balance;
	type RuntimeEvent = RuntimeEvent;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
	type RuntimeHoldReason = ();
	type FreezeIdentifier = ();
	type MaxHolds = ();
	type MaxFreezes = ();
}

pub type Balance = u128;
parameter_types! {
	pub const ExistentialDeposit: u128 = 1;
}

impl Config for Test {
	type WeightInfo = ();
}

impl block_author::Config for Test {}
const GENESIS_NUM_SELECTED_CANDIDATES: u32 = 5;
parameter_types! {
	pub const MinBlocksPerRound: u32 = 3;
	pub const MaxOfflineRounds: u32 = 1;
	pub const LeaveCandidatesDelay: u32 = 2;
	pub const CandidateBondLessDelay: u32 = 2;
	pub const LeaveDelegatorsDelay: u32 = 2;
	pub const RevokeDelegationDelay: u32 = 2;
	pub const DelegationBondLessDelay: u32 = 2;
	pub const RewardPaymentDelay: u32 = 2;
	pub const MinSelectedCandidates: u32 = GENESIS_NUM_SELECTED_CANDIDATES;
	pub const MaxTopDelegationsPerCandidate: u32 = 4;
	pub const MaxBottomDelegationsPerCandidate: u32 = 4;
	pub const MaxDelegationsPerDelegator: u32 = 4;
	pub const MinCandidateStk: u128 = 10;
	pub const MinDelegation: u128 = 3;
	pub const MaxCandidates: u32 = 200;
}
impl pallet_parachain_staking::Config for Test {
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
	type PayoutCollatorReward = ();
	type OnInactiveCollator = ();
	type OnNewRound = ();
	type SlotProvider = StakingRoundSlotProvider;
	type WeightInfo = ();
	type MaxCandidates = MaxCandidates;
	type SlotsPerYear = frame_support::traits::ConstU32<{ 31_557_600 / 6 }>;
}

pub struct StakingRoundSlotProvider;
impl Get<Slot> for StakingRoundSlotProvider {
	fn get() -> Slot {
		let block_number: u64 = System::block_number().into();
		Slot::from(block_number)
	}
}

#[frame_support::pallet]
pub mod block_author {
	use super::*;
	use frame_support::{pallet_prelude::*, traits::Get};

	#[pallet::config]
	pub trait Config: frame_system::Config {}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn block_author)]
	pub(super) type BlockAuthor<T> = StorageValue<_, AccountId, ValueQuery>;

	impl<T: Config> Get<AccountId> for Pallet<T> {
		fn get() -> AccountId {
			<BlockAuthor<T>>::get()
		}
	}
}

pub(crate) struct ExtBuilder {
	rewards_account: Option<AccountId>,
	balances: Vec<(AccountId, Balance)>,
}

impl Default for ExtBuilder {
	fn default() -> Self {
		Self { rewards_account: None, balances: vec![] }
	}
}

impl ExtBuilder {
	pub(crate) fn with_rewards_account(mut self, account: AccountId) -> Self {
		self.rewards_account = Some(account);
		self
	}
	pub(crate) fn with_balances(mut self, balances: Vec<(AccountId, Balance)>) -> Self {
		self.balances = balances;
		self
	}
	// Build genesis storage according to the mock runtime.
	pub(crate) fn build(self) -> sp_io::TestExternalities {
		let mut t = frame_system::GenesisConfig::<Test>::default().build_storage().unwrap().into();

		pallet_balances::GenesisConfig::<Test> { balances: self.balances }
			.assimilate_storage(&mut t)
			.expect("Pallet balances storage can be assimilated");

		pallet_block_rewards_source::GenesisConfig::<Test> {
			rewards_account: self.rewards_account,
		}
		.assimilate_storage(&mut t)
		.unwrap();

		t.into()
	}
}
