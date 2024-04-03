use crate::{
	weights as laos_weights, AccountId, ParachainSystem, Runtime, RuntimeEvent, XcmExecutor,
};

use super::xcm_config::{XcmConfig, XcmOriginToTransactDispatchOrigin};

use frame_system::EnsureRoot;

impl cumulus_pallet_xcmp_queue::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type ChannelInfo = ParachainSystem;
	type VersionWrapper = ();
	type ExecuteOverweightOrigin = EnsureRoot<AccountId>;
	type ControllerOrigin = EnsureRoot<AccountId>;
	type ControllerOriginConverter = XcmOriginToTransactDispatchOrigin;
	type PriceForSiblingDelivery = ();
	type WeightInfo = laos_weights::cumulus_pallet_xcmp_queue::WeightInfo<Runtime>;
}
