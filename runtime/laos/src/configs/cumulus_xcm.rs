use crate::{types::xcm_config::XcmConfig, Runtime, RuntimeEvent};
use staging_xcm_executor::XcmExecutor;

impl cumulus_pallet_xcm::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type XcmExecutor = XcmExecutor<XcmConfig>;
}
