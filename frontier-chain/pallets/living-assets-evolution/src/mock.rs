use crate as pallet_living_assets_evolution;
use frame_support::{
	parameter_types,
	traits::{ConstU16, ConstU64},
};
use sp_core::{H160, H256};
use sp_runtime::{
	traits::{BlakeTwo256, Convert, IdentityLookup},
	BuildStorage,
};

type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test
	{
		System: frame_system,
		LivingAssets: pallet_living_assets_evolution,
	}
);

pub type AccountId = u64;
impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
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
}

/// A struct responsible for converting an `AccountId` to an `H160` address.
///
/// The `AccountIdToH160` struct provides a conversion from `AccountId`, typically used
/// as a native identity in a blockchain, to an `H160` address, commonly used in Ethereum-like
/// networks.
pub struct MockAccountIdToH160;
impl Convert<AccountId, H160> for MockAccountIdToH160 {
	fn convert(account_id: AccountId) -> H160 {
		let mut bytes = [0u8; 20];
		let account_id_bytes = account_id.to_be_bytes();

		bytes[0..8].copy_from_slice(&account_id_bytes);
		H160::from(bytes)
	}
}

impl pallet_living_assets_evolution::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	type MaxTokenUriLength = MaxTokenUriLength;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	frame_system::GenesisConfig::<Test>::default().build_storage().unwrap().into()
}
