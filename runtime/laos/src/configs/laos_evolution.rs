use crate::{AccountIdToH160, MaxTokenUriLength, Runtime, RuntimeEvent};

impl pallet_laos_evolution::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type AccountIdToH160 = AccountIdToH160;
	type MaxTokenUriLength = MaxTokenUriLength;
}
