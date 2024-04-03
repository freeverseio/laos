use crate::{weights as laos_weights, OriginCaller, Runtime, RuntimeCall, RuntimeEvent};

impl pallet_utility::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type PalletsOrigin = OriginCaller;
	type WeightInfo = laos_weights::pallet_utility::WeightInfo<Runtime>;
}
