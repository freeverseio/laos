use crate::{AccountId, EnsureRoot, Runtime, RuntimeEvent, XcmExecutor};

use super::xcm_config::XcmConfig;

impl cumulus_pallet_dmp_queue::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type ExecuteOverweightOrigin = EnsureRoot<AccountId>;
}
