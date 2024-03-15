use super::MaxTokenUriLength;
use crate::{types::AccountIdToH160, Runtime, RuntimeEvent};
use frame_support::parameter_types;

parameter_types! {
	/// Max length of the `UniversalLocation`
	pub const MaxUniversalLocationLength: u32 = 512;
}

impl pallet_asset_metadata_extender::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type AccountIdToH160 = AccountIdToH160;
	type MaxTokenUriLength = MaxTokenUriLength;
	type MaxUniversalLocationLength = MaxUniversalLocationLength;
}
