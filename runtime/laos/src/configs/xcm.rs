use crate::{
	configs::xcm_config::{
		LocalOriginToLocation, LocationToAccountId, MaxInstructions, UnitWeightCost,
		UniversalLocation, XcmConfig, XcmRouter,
	},
	AccountId, Balances, Runtime, RuntimeCall, RuntimeEvent, RuntimeOrigin, XcmExecutor,
};
use frame_support::{
	parameter_types,
	traits::{Everything, Nothing},
};
use frame_system::EnsureRoot;
use staging_xcm_builder::{EnsureXcmOrigin, FixedWeightBounds};

parameter_types! {
	pub const MaxLockers: u32 = 8;
	pub const MaxRemoteLockConsumers: u32 = 0;
}

#[cfg(feature = "runtime-benchmarks")]
parameter_types! {
	pub ReachableDest: Option<MultiLocation> = Some(Parent.into());

}

impl pallet_xcm::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type SendXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
	type XcmRouter = XcmRouter;
	type ExecuteXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
	type XcmExecuteFilter = Nothing;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type XcmTeleportFilter = Nothing;
	type XcmReserveTransferFilter = Everything;
	type Weigher = FixedWeightBounds<UnitWeightCost, RuntimeCall, MaxInstructions>;
	type UniversalLocation = UniversalLocation;
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	const VERSION_DISCOVERY_QUEUE_SIZE: u32 = 100;
	// ^ Override for AdvertisedXcmVersion default
	type AdvertisedXcmVersion = pallet_xcm::CurrentXcmVersion;
	type Currency = Balances;
	type CurrencyMatcher = ();
	type TrustedLockers = ();
	type SovereignAccountOf = LocationToAccountId;
	type MaxLockers = MaxLockers;
	type WeightInfo = pallet_xcm::TestWeightInfo;
	#[cfg(feature = "runtime-benchmarks")]
	type ReachableDest = ReachableDest;
	type AdminOrigin = EnsureRoot<AccountId>;
	type MaxRemoteLockConsumers = MaxRemoteLockConsumers;
	type RemoteLockConsumerIdentifier = ();
}
