use crate as pallet_asset_metadata_extender;
use frame_support::{derive_impl, parameter_types};
use sp_core::H160;
use sp_runtime::{traits::IdentityLookup, BuildStorage};

type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test
	{
		System: frame_system,
		AssetMetadataExtender: pallet_asset_metadata_extender,
	}
);

type AccountId = H160;

#[derive_impl(frame_system::config_preludes::TestDefaultConfig as frame_system::DefaultConfig)]
impl frame_system::Config for Test {
	type Block = Block;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
}

parameter_types! {
	pub const MaxTokenUriLength: u32 = 512;
	pub const MaxUniversalLocationLength: u32 = 512;
}

impl pallet_asset_metadata_extender::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type MaxUniversalLocationLength = MaxUniversalLocationLength;
	type MaxTokenUriLength = MaxTokenUriLength;
	type AccountIdToH160 = AccountIdToH160;
}

pub struct AccountIdToH160;

impl sp_runtime::traits::Convert<AccountId, H160> for AccountIdToH160 {
	fn convert(account_id: AccountId) -> H160 {
		account_id
	}
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	frame_system::GenesisConfig::<Test>::default().build_storage().unwrap().into()
}
