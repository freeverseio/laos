use crate::{weights as laos_weights, Runtime, RuntimeCall, RuntimeEvent};

impl pallet_sudo::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type WeightInfo = laos_weights::pallet_sudo::WeightInfo<Runtime>;
}
