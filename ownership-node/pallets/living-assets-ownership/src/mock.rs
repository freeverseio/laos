use crate::{self as pallet_livingassets_ownership};
use frame_support::traits::{ConstU16, ConstU64};
use sp_core::{ConstU32, H160, H256, U256};
use sp_runtime::{
	traits::{BlakeTwo256, Convert, IdentityLookup},
	BuildStorage,
};
use sp_std::{boxed::Box, prelude::*};

type Block = frame_system::mocking::MockBlock<Test>;
type Nonce = u32;
type AccountId = u64;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test
	{
		System: frame_system,
		LivingAssetsModule: pallet_livingassets_ownership,
	}
);

impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Block = Block;
	type Hash = H256;
	type Nonce = Nonce;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
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

impl pallet_livingassets_ownership::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type BaseURILimit = ConstU32<256>;
	type AccountIdToH160 = MockAccountIdToH160;
	type H160ToAccountId = MockH160ToAccountId;
	type AssetIdToInitialOwner = MockAssetIdToInitialOwner;
}

pub struct MockAccountIdToH160;
impl Convert<AccountId, H160> for MockAccountIdToH160 {
	fn convert(account_id: AccountId) -> H160 {
		H160::from_low_u64_be(account_id)
	}
}
pub struct MockH160ToAccountId;
impl Convert<H160, AccountId> for MockH160ToAccountId {
	fn convert(account_id: H160) -> AccountId {
		H160::to_low_u64_be(&account_id)
	}
}

pub struct MockAssetIdToInitialOwner;
impl Convert<U256, AccountId> for MockAssetIdToInitialOwner {
	fn convert(asset_id: U256) -> AccountId {
		let mut first_eight_bytes = [0u8; 8];
		let asset_id_bytes: [u8; 32] = asset_id.into();
		first_eight_bytes.copy_from_slice(&asset_id_bytes[asset_id_bytes.len() - 8..]);
		u64::from_be_bytes(first_eight_bytes).into()
	}
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	RuntimeGenesisConfig::default().build_storage().unwrap().into()
}
