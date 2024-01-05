use crate as pallet_asset_metadata_extender;
use frame_support::{
	parameter_types,
	traits::{ConstU16, ConstU64},
	weights::constants::RocksDbWeight,
};
use sp_core::{H160, H256};
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup},
	BoundedVec, BuildStorage,
};

type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test
	{
		System: frame_system,
		AssetMetadataExtender: pallet_asset_metadata_extender,
	}
);

pub type AccountId = H160;
pub type TokenUri =
	BoundedVec<u8, <Test as pallet_asset_metadata_extender::Config>::MaxTokenUriLength>;
pub type UniversalLocation =
	BoundedVec<u8, <Test as pallet_asset_metadata_extender::Config>::MaxUniversalLocationLength>;

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
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_types! {
	pub const MaxTokenUriLength: u32 = 512;
	pub const MaxUniversalLocationLength: u32 = 512;
}

impl pallet_asset_metadata_extender::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type MaxUniversalLocationLength = MaxUniversalLocationLength;
	type MaxTokenUriLength = MaxTokenUriLength;
}
// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	frame_system::GenesisConfig::<Test>::default().build_storage().unwrap().into()
}
