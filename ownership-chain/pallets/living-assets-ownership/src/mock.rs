use crate::{self as pallet_livingassets_ownership};
use frame_support::traits::{ConstU16, ConstU64};
use sp_core::{ConstU32, H256, U256};
use sp_runtime::{
	traits::{BlakeTwo256, Convert, IdentifyAccount, IdentityLookup, Verify},
	BuildStorage,
};
use sp_std::{boxed::Box, prelude::*};

type Block = frame_system::mocking::MockBlock<Test>;
type Nonce = u32;
type Signature = fp_account::EthereumSignature;
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

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

frame_support::parameter_types! {
	pub NullAddress: AccountId = [0u8; 20].into();
}

impl pallet_livingassets_ownership::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type BaseURILimit = ConstU32<256>;
	type NullAddress = NullAddress;
	type AssetIdToInitialOwner = MockAssetIdToInitialOwner;
}

pub struct MockAssetIdToInitialOwner;
impl Convert<U256, AccountId> for MockAssetIdToInitialOwner {
	fn convert(asset_id: U256) -> AccountId {
		// initial owner is the last 20 bytes of the asset id
		let mut initial_owner = [0u8; 20];
		let asset_id_bytes: [u8; 32] = asset_id.into();
		initial_owner.copy_from_slice(&asset_id_bytes[12..]);
		AccountId::from(initial_owner)
	}
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	RuntimeGenesisConfig::default().build_storage().unwrap().into()
}
