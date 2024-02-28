use crate::*;

parameter_types! {
	/// Minimum round length is 1 hour
	pub const MinBlocksPerRound: BlockNumber = prod_or_fast!(HOURS, 10);
	/// Default length of a round/session is 2 hours
	pub const DefaultBlocksPerRound: BlockNumber = prod_or_fast!(2 * HOURS, 10);
	/// Unstaked balance can be unlocked after 7 days
	pub const StakeDuration: BlockNumber = prod_or_fast!(7 * DAYS, 30);
	/// Collator exit requests are delayed by 4 hours (2 rounds/sessions)
	pub const ExitQueueDelay: u32 = 2;
	/// Minimum 16 collators selected per round, default at genesis and minimum forever after
	pub const MinCollators: u32 = prod_or_fast!(16, 1);
	/// At least 4 candidates which cannot leave the network if there are no other candidates.
	pub const MinRequiredCollators: u32 = 4;
	/// We only allow one delegation per round.
	pub const MaxDelegationsPerRound: u32 = 1;
	/// Maximum 25 delegators per collator at launch, might be increased later
	#[derive(Debug, Eq, PartialEq)]
	pub const MaxDelegatorsPerCollator: u32 = 35;
	/// Minimum stake required to be reserved to be a collator is 10_000
	pub const MinCollatorStake: Balance = 10 * UNIT;
	/// Minimum stake required to be reserved to be a delegator is 1000
	pub const MinDelegatorStake: Balance = 2 * UNIT;
	/// Maximum number of collator candidates
	#[derive(Debug, Eq, PartialEq)]
	pub const MaxCollatorCandidates: u32 = prod_or_fast!(75, 16);
	/// Maximum number of concurrent requests to unlock unstaked balance
	pub const MaxUnstakeRequests: u32 = 10;
	/// The starting block number for the network rewards
	pub const NetworkRewardStart: BlockNumber = BLOCKS_PER_YEAR.saturating_mul(1);
	/// The rate in percent for the network rewards
	pub const NetworkRewardRate: Perquintill = Perquintill::from_percent(10);	 // pub CollatorRewardsAccount: AccountId =
}

impl pallet_parachain_staking::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type CurrencyBalance = Balance;
	type FreezeIdentifier = RuntimeFreezeReason;
	type MinBlocksPerRound = MinBlocksPerRound;
	type DefaultBlocksPerRound = DefaultBlocksPerRound;
	type StakeDuration = StakeDuration;
	type ExitQueueDelay = ExitQueueDelay;
	type MinCollators = MinCollators;
	type MinRequiredCollators = MinRequiredCollators;
	type MaxDelegationsPerRound = MaxDelegationsPerRound;
	type MaxDelegatorsPerCollator = MaxDelegatorsPerCollator;
	type MinCollatorStake = MinCollatorStake;
	type MinCollatorCandidateStake = MinCollatorStake;
	type MaxTopCandidates = MaxCollatorCandidates;
	type MinDelegatorStake = MinDelegatorStake;
	type MaxUnstakeRequests = MaxUnstakeRequests;
	type NetworkRewardRate = NetworkRewardRate;
	type NetworkRewardStart = NetworkRewardStart;
	type NetworkRewardBeneficiary = ToCollatorRewards<Runtime>;
	type WeightInfo = pallet_parachain_staking::default_weights::SubstrateWeight<Runtime>;

	const BLOCKS_PER_YEAR: BlockNumberFor<Self> = BLOCKS_PER_YEAR;
}

