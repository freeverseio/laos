use crate::{
	configs::xcm_config::{
		LocationToAccountId, MaxInstructions, RelayNetwork, UnitWeightCost, UniversalLocation,
		XcmConfig, XcmRouter,
	},
	AccountId, Balances, Runtime, RuntimeCall, RuntimeEvent, RuntimeOrigin, XcmExecutor,
};
use frame_support::{
	parameter_types,
	traits::{Everything, Nothing, OriginTrait},
};
use frame_system::{EnsureRoot, RawOrigin as SystemRawOrigin};
use sp_runtime::traits::TryConvert;
use staging_xcm::latest::prelude::*;
use staging_xcm_builder::{EnsureXcmOrigin, FixedWeightBounds};

use core::marker::PhantomData;

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

/// No local origins on this chain are allowed to dispatch XCM sends/executions.
pub type LocalOriginToLocation = SignedToAccountId20<RuntimeOrigin, AccountId, RelayNetwork>;

pub struct SignedToAccountId20<RuntimeOrigin, AccountId, Network>(
	PhantomData<(RuntimeOrigin, AccountId, Network)>,
);
impl<
		RuntimeOrigin: OriginTrait + Clone,
		AccountId: Into<[u8; 20]>,
		Network: frame_support::traits::Get<Option<NetworkId>>,
	> TryConvert<RuntimeOrigin, MultiLocation>
	for SignedToAccountId20<RuntimeOrigin, AccountId, Network>
where
	RuntimeOrigin::PalletsOrigin: From<SystemRawOrigin<AccountId>>
		+ TryInto<SystemRawOrigin<AccountId>, Error = RuntimeOrigin::PalletsOrigin>,
{
	fn try_convert(o: RuntimeOrigin) -> Result<MultiLocation, RuntimeOrigin> {
		o.try_with_caller(|caller| match caller.try_into() {
			Ok(SystemRawOrigin::Signed(who)) =>
				Ok(Junction::AccountKey20 { network: Network::get(), key: who.into() }.into()),
			Ok(other) => Err(other.into()),
			Err(other) => Err(other),
		})
	}
}
