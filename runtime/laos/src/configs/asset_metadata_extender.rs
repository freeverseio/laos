use crate::{
	AccountIdToH160, MaxTokenUriLength, MaxUniversalLocationLength, Runtime, RuntimeEvent,
};

impl pallet_asset_metadata_extender::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type AccountIdToH160 = AccountIdToH160;
	type MaxTokenUriLength = MaxTokenUriLength;
	type MaxUniversalLocationLength = MaxUniversalLocationLength;
}
