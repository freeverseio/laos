//! Mock runtime for tests/benchmarks

use frame_support::{
	ord_parameter_types, parameter_types,
	traits::{ConstU16, ConstU64, FindAuthor, ValidatorRegistration},
	weights::constants::RocksDbWeight,
	PalletId,
};
use frame_system::EnsureSignedBy;
use pallet_balances::AccountData;
use sp_core::{ConstBool, ConstU128, ConstU32, H256};
use sp_runtime::{
	testing::UintAuthorityId,
	traits::{AccountIdConversion, BlakeTwo256, IdentityLookup, OpaqueKeys},
	BuildStorage, RuntimeAppPublic,
};

pub const INITIAL_BALANCE: Balance = 100;

type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test
	{
		System: frame_system,
		Timestamp: pallet_timestamp,
		Session: pallet_session,
		Aura: pallet_aura,
		Balances: pallet_balances,
		CollatorSelection: pallet_collator_selection,
		Authorship: pallet_authorship,

		CollatorRewards: crate
	}
);

pub type AccountId = u64;
pub type Balance = u128;

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
	type AccountData = AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

// Pallet Balances
impl pallet_balances::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	type Balance = Balance;
	type DustRemoval = ();
	type ExistentialDeposit = ConstU128<0>;
	type AccountStore = System;
	type ReserveIdentifier = ();
	type RuntimeHoldReason = ();
	type FreezeIdentifier = ();
	type MaxLocks = ();
	type MaxReserves = ();
	type MaxHolds = ();
	type MaxFreezes = ();
}

impl pallet_aura::Config for Test {
	type AuthorityId = sp_consensus_aura::sr25519::AuthorityId;
	type MaxAuthorities = ConstU32<100_000>;
	type DisabledValidators = ();
	type AllowMultipleBlocksPerSlot = ConstBool<false>;
	#[cfg(feature = "experimental")]
	type SlotDuration = pallet_aura::MinimumPeriodTimesTwo<Self>;
}

/// Always return 4 as the author.
pub struct Author4;
impl FindAuthor<u64> for Author4 {
	fn find_author<'a, I>(_digests: I) -> Option<u64>
	where
		I: 'a + IntoIterator<Item = (frame_support::ConsensusEngineId, &'a [u8])>,
	{
		Some(4)
	}
}

impl pallet_timestamp::Config for Test {
	type Moment = u64;
	type OnTimestampSet = Aura;
	type MinimumPeriod = ConstU64<1>;
	type WeightInfo = ();
}

ord_parameter_types! {
	pub const RootAccount: u64 = 777;
}

parameter_types! {
	pub const PotId: PalletId = PalletId(*b"PotStake");
}

pub struct IsRegistered;
impl ValidatorRegistration<u64> for IsRegistered {
	fn is_registered(id: &u64) -> bool {
		*id != 42u64
	}
}

impl pallet_collator_selection::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type UpdateOrigin = EnsureSignedBy<RootAccount, u64>;
	type PotId = PotId;
	type MaxCandidates = ConstU32<20>;
	type MinEligibleCollators = ConstU32<1>;
	type MaxInvulnerables = ConstU32<20>;
	type KickThreshold = Period;
	type ValidatorId = <Self as frame_system::Config>::AccountId;
	type ValidatorIdOf = IdentityCollator;
	type ValidatorRegistration = IsRegistered;
	type WeightInfo = ();
}

sp_runtime::impl_opaque_keys! {
	pub struct MockSessionKeys {
		// a key for aura authoring
		pub aura: UintAuthorityId,
	}
}

impl From<UintAuthorityId> for MockSessionKeys {
	fn from(aura: sp_runtime::testing::UintAuthorityId) -> Self {
		Self { aura }
	}
}

parameter_types! {
	pub static SessionHandlerCollators: Vec<u64> = Vec::new();
	pub static SessionChangeBlock: u64 = 0;
}

pub struct TestSessionHandler;
impl pallet_session::SessionHandler<u64> for TestSessionHandler {
	const KEY_TYPE_IDS: &'static [sp_runtime::KeyTypeId] = &[UintAuthorityId::ID];
	fn on_genesis_session<Ks: OpaqueKeys>(keys: &[(u64, Ks)]) {
		SessionHandlerCollators::set(keys.iter().map(|(a, _)| *a).collect::<Vec<_>>())
	}
	fn on_new_session<Ks: OpaqueKeys>(_: bool, keys: &[(u64, Ks)], _: &[(u64, Ks)]) {
		SessionChangeBlock::set(System::block_number());
		dbg!(keys.len());
		SessionHandlerCollators::set(keys.iter().map(|(a, _)| *a).collect::<Vec<_>>())
	}
	fn on_before_session_ending() {}
	fn on_disabled(_: u32) {}
}

parameter_types! {
	pub const Offset: u64 = 0;
	pub const Period: u64 = 10;
}

/// A convertor from collators id. Since this pallet does not have stash/controller, this is
/// just identity.
pub struct IdentityCollator;
impl<T> sp_runtime::traits::Convert<T, Option<T>> for IdentityCollator {
	fn convert(t: T) -> Option<T> {
		Some(t)
	}
}

impl pallet_session::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type ValidatorId = <Self as frame_system::Config>::AccountId;
	// we don't have stash and controller, thus we don't need the convert as well.
	type ValidatorIdOf = IdentityCollator;
	type ShouldEndSession = pallet_session::PeriodicSessions<Period, Offset>;
	type NextSessionRotation = pallet_session::PeriodicSessions<Period, Offset>;
	type SessionManager = CollatorSelection;
	type SessionHandler = TestSessionHandler;
	type Keys = MockSessionKeys;
	type WeightInfo = ();
}

// Pallet authorship
impl pallet_authorship::Config for Test {
	type FindAuthor = Author4;
	type EventHandler = CollatorRewards;
}

parameter_types! {
	pub CommunityIncentivesAccountId: AccountId = PalletId(*b"mock/cin").into_account_truncating();
	pub const RewardPerBlock: Balance = 2;
}

impl crate::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type CommunityIncentivesAccountId = CommunityIncentivesAccountId;
	type RewardPerBlock = RewardPerBlock;
	type Currency = Balances;
	type WeightInfo = ();
}

pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::<Test>::default().build_storage().unwrap();
	let invulnerables = vec![2, 1]; // unsorted

	let community_incentives_account = CommunityIncentivesAccountId::get();

	let balances = vec![
		(1, INITIAL_BALANCE),
		(2, INITIAL_BALANCE),
		(3, INITIAL_BALANCE),
		(4, INITIAL_BALANCE),
		(5, INITIAL_BALANCE),
		(community_incentives_account, INITIAL_BALANCE),
	];
	let keys = balances
		.iter()
		.map(|&(i, _)| (i, i, MockSessionKeys { aura: UintAuthorityId(i) }))
		.collect::<Vec<_>>();
	let collator_selection = pallet_collator_selection::GenesisConfig::<Test> {
		desired_candidates: 2,
		candidacy_bond: 10,
		invulnerables,
	};
	let session = pallet_session::GenesisConfig::<Test> { keys };
	pallet_balances::GenesisConfig::<Test> { balances }
		.assimilate_storage(&mut t)
		.unwrap();
	// collator selection must be initialized before session.
	collator_selection.assimilate_storage(&mut t).unwrap();
	session.assimilate_storage(&mut t).unwrap();

	t.into()
}

pub fn initialize_to_block(n: u64) {
	for i in System::block_number() + 1..=n {
		System::set_block_number(i);
		<AllPalletsWithSystem as frame_support::traits::OnInitialize<u64>>::on_initialize(i);
	}
}
