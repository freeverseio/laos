use frame_support::{
	parameter_types,
    ConsensusEngineId,
	traits::{ConstU16, ConstU64, FindAuthor},
	weights::constants::RocksDbWeight,
};
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
		Authorship: pallet_authorship,
		Balances: pallet_balances,
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
	pub const ExistentialDeposit: u128 = 1;
}

type Balance = u128;

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

impl pallet_authorship::Config for Test {
	type FindAuthor = AuthorAccount<AccountId>;
	type EventHandler = ();
}

pub struct AuthorAccount<T>(sp_std::marker::PhantomData<T>);

impl<T> FindAuthor<T>
	for AuthorAccount<T>
{
	fn find_author<'a, I>(digests: I) -> Option<T>
    where I: 'a + IntoIterator<Item = (ConsensusEngineId, &'a [u8])>
	{
        None
	}
}

pub(crate) struct ExtBuilder {}

impl Default for ExtBuilder {
	fn default() -> Self {
		Self {}
	}
}

impl ExtBuilder {
	// Build genesis storage according to the mock runtime.
	pub(crate) fn build(self) -> sp_io::TestExternalities {
		frame_system::GenesisConfig::<Test>::default().build_storage().unwrap().into()
	}
}
