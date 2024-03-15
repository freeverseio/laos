use super::MaxTokenUriLength;
use crate::{types::AccountIdToH160, Runtime, RuntimeEvent};

impl pallet_laos_evolution::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type AccountIdToH160 = AccountIdToH160;
	type MaxTokenUriLength = MaxTokenUriLength;
}
