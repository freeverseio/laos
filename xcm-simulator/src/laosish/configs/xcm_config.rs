// Copyright 2023-2024 Freeverse.io
// This file is part of LAOS.

// LAOS is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// LAOS is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with LAOS.  If not, see <http://www.gnu.org/licenses/>.

use crate::laosish::{
	mock_msg_queue, AccountId, AllPalletsWithSystem, Balances, MsgQueue, ParachainInfo,
	PolkadotXcm, Runtime, RuntimeCall, RuntimeEvent, RuntimeOrigin,
};
use core::marker::PhantomData;
use frame_support::{
	parameter_types,
	traits::{ConstU32, Contains, Everything, Nothing, OriginTrait},
	weights::Weight,
};
use frame_system::{EnsureRoot, RawOrigin as SystemRawOrigin};
use pallet_xcm::XcmPassthrough;
use polkadot_parachain_primitives::primitives::Sibling;
use sp_runtime::traits::TryConvert;
use xcm::latest::prelude::*;
use xcm_builder::{
	AccountKey20Aliases, AllowExplicitUnpaidExecutionFrom, AllowTopLevelPaidExecutionFrom,
	DenyReserveTransferToRelayChain, DenyThenTry, EnsureDecodableXcm, EnsureXcmOrigin,
	FixedRateOfFungible, FixedWeightBounds, FrameTransactionalProcessor, FungibleAdapter,
	IsConcrete, MintLocation, ParentIsPreset, RelayChainAsNative, SiblingParachainAsNative,
	SiblingParachainConvertsVia, SignedAccountKey20AsNative, SovereignSignedViaLocation,
	TakeWeightCredit, TrailingSetTopicAsId, WithComputedOrigin,
};
use xcm_executor::XcmExecutor;
use xcm_simulator::AssetFilter;

parameter_types! {
	pub const RelayNetwork: NetworkId = NetworkId::Kusama;
	pub RelayChainOrigin: RuntimeOrigin = cumulus_pallet_xcm::Origin::Relay.into();
	// For the real deployment, it is recommended to set `RelayNetwork` according to the relay chain
	// and prepend `UniversalLocation` with `GlobalConsensus(RelayNetwork::get())`.
	pub UniversalLocation: InteriorLocation = (
		GlobalConsensus(RelayNetwork::get()),
		Parachain(ParachainInfo::parachain_id().into()),
	).into();
	pub HereLocation: Location = Location::here();
	pub CheckingAccount: AccountId = PolkadotXcm::check_account();
	pub Checking: (AccountId, MintLocation) = (CheckingAccount::get(), MintLocation::Local);
}

/// Type for specifying how a `MultiLocation` can be converted into an `AccountId`. This is used
/// when determining ownership of accounts for asset transacting and when attempting to use XCM
/// `Transact` in order to determine the dispatch Origin.
pub type LocationToAccountId = (
	// The parent (Relay-chain) origin converts to the parent `AccountId`.
	ParentIsPreset<AccountId>,
	// Sibling parachain origins convert to AccountId via the `ParaId::into`.
	SiblingParachainConvertsVia<Sibling, AccountId>,
	// Straight up local `AccountId20` origins just alias directly to `AccountId`.
	AccountKey20Aliases<RelayNetwork, AccountId>,
);

/// Means for transacting assets on this chain.
pub type LocalAssetTransactor = FungibleAdapter<
	// Use this currency:
	Balances,
	// Use this currency when it is a fungible asset matching the given location or name:
	IsConcrete<HereLocation>,
	// Do a simple pun to convert an AccountId20 Location into a native chain account ID:
	LocationToAccountId,
	// Our chain's account ID type (we can't get away without mentioning it explicitly):
	AccountId,
	// We track any teleports.
	Checking,
>;

/// This is the type we use to convert an (incoming) XCM origin into a local `Origin` instance,
/// ready for dispatching a transaction with Xcm's `Transact`. There is an `OriginKind` which can
/// biases the kind of local `Origin` it will become.
pub type XcmOriginToTransactDispatchOrigin = (
	// Sovereign account converter; this attempts to derive an `AccountId` from the origin location
	// using `LocationToAccountId` and then turn that into the usual `Signed` origin. Useful for
	// foreign chains who want to have a local sovereign account on this chain which they control.
	SovereignSignedViaLocation<LocationToAccountId, RuntimeOrigin>,
	// Native converter for Relay-chain (Parent) location; will convert to a `Relay` origin when
	// recognized.
	RelayChainAsNative<RelayChainOrigin, RuntimeOrigin>,
	// Native converter for sibling Parachains; will convert to a `SiblingPara` origin when
	// recognized.
	SiblingParachainAsNative<cumulus_pallet_xcm::Origin, RuntimeOrigin>,
	// Native signed account converter; this just converts an `AccountId20` origin into a normal
	// `RuntimeOrigin::Signed` origin of the same 20-byte value.
	SignedAccountKey20AsNative<RelayNetwork, RuntimeOrigin>,
	// Xcm origins can be represented natively under the Xcm pallet's Xcm origin.
	XcmPassthrough<RuntimeOrigin>,
);

parameter_types! {
	// One XCM operation is 1_000_000_000 weight - almost certainly a conservative estimate.
	pub UnitWeightCost: Weight = Weight::from_parts(1_000_000_000, 64 * 1024);
	pub const MaxInstructions: u32 = 100;
	pub const MaxAssetsIntoHolding: u32 = 64;
}

pub struct ParentOrParentsExecutivePlurality;
impl Contains<Location> for ParentOrParentsExecutivePlurality {
	fn contains(location: &Location) -> bool {
		matches!(location.unpack(), (1, []) | (1, [Plurality { id: BodyId::Executive, .. }]))
	}
}

pub type Barrier = TrailingSetTopicAsId<
	DenyThenTry<
		DenyReserveTransferToRelayChain,
		(
			TakeWeightCredit,
			WithComputedOrigin<
				(
					AllowTopLevelPaidExecutionFrom<Everything>,
					AllowExplicitUnpaidExecutionFrom<ParentOrParentsExecutivePlurality>,
					// ^^^ Parent and its exec plurality get free execution
				),
				UniversalLocation,
				ConstU32<8>,
			>,
		),
	>,
>;

parameter_types! {
	pub ParentTokenPerSecondPerByte: (AssetId, u128, u128) = (Parent.into(), 1, 1);
}

parameter_types! {
	pub NativeToken: AssetId = AssetId(Location::here());
	pub NativeTokenFilter: AssetFilter = Wild(AllOf { fun: WildFungible, id: NativeToken::get() });
	pub AssetHubLocation: Location = Location::new(1, [Parachain(crate::PARA_ASSETHUB_ID)]);
	pub AssetHubTrustedTeleporter: (AssetFilter, Location) = (NativeTokenFilter::get(), AssetHubLocation::get());
}

pub type TrustedTeleporters = xcm_builder::Case<AssetHubTrustedTeleporter>;

pub struct Reserves;
impl frame_support::traits::ContainsPair<Asset, Location> for Reserves {
	fn contains(asset: &Asset, location: &Location) -> bool {
		matches!(asset, Asset { id: asset_id, fun: Fungible(_) } if asset_id.0 == HereLocation::get() && location == &HereLocation::get())
	}
}

pub struct XcmConfig;
impl xcm_executor::Config for XcmConfig {
	type RuntimeCall = RuntimeCall;
	type XcmSender = XcmRouter;
	// How to withdraw and deposit an asset.
	type AssetTransactor = LocalAssetTransactor;
	type OriginConverter = XcmOriginToTransactDispatchOrigin;
	type IsReserve = Reserves;
	type IsTeleporter = TrustedTeleporters;
	type UniversalLocation = UniversalLocation;
	type Barrier = Barrier;
	type Weigher = FixedWeightBounds<UnitWeightCost, RuntimeCall, MaxInstructions>;
	type Trader = FixedRateOfFungible<ParentTokenPerSecondPerByte, ()>;
	type ResponseHandler = PolkadotXcm;
	type AssetTrap = PolkadotXcm;
	type AssetClaims = PolkadotXcm;
	type SubscriptionService = PolkadotXcm;
	type PalletInstancesInfo = AllPalletsWithSystem;
	type MaxAssetsIntoHolding = MaxAssetsIntoHolding;
	type AssetLocker = ();
	type AssetExchanger = ();
	type FeeManager = ();
	type MessageExporter = ();
	type UniversalAliases = Nothing;
	type CallDispatcher = RuntimeCall;
	type SafeCallFilter = Everything;
	type Aliasers = Nothing;
	type TransactionalProcessor = FrameTransactionalProcessor;
	type HrmpNewChannelOpenRequestHandler = ();
	type HrmpChannelAcceptedHandler = ();
	type HrmpChannelClosingHandler = ();
	type XcmRecorder = PolkadotXcm;
}

/// No local origins on this chain are allowed to dispatch XCM sends/executions.
pub type LocalOriginToLocation = SignedToAccountId20<RuntimeOrigin, AccountId, RelayNetwork>;
pub type XcmRouter = EnsureDecodableXcm<crate::ParachainXcmRouter<MsgQueue>>;

//Filter all teleports/reserves that aren't the native asset
pub struct OnlySendNative;
impl Contains<(Location, Vec<Asset>)> for OnlySendNative {
	fn contains(t: &(Location, Vec<Asset>)) -> bool {
		t.1.iter().all(|asset| {
			log::trace!(target: "xcm::OnlySendNative", "Asset to be sent out: {:?}", asset);
			if let Asset { id: asset_id, fun: Fungible(_) } = asset {
				asset_id.0 == HereLocation::get()
			} else {
				false
			}
		})
	}
}

impl pallet_xcm::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type SendXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
	type XcmRouter = XcmRouter;
	type ExecuteXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
	type XcmExecuteFilter = Nothing;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type XcmTeleportFilter = OnlySendNative;
	type XcmReserveTransferFilter = OnlySendNative;
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
	type MaxLockers = ConstU32<8>;
	type WeightInfo = pallet_xcm::TestWeightInfo;
	type AdminOrigin = EnsureRoot<AccountId>;
	type MaxRemoteLockConsumers = ConstU32<0>;
	type RemoteLockConsumerIdentifier = ();
}

impl cumulus_pallet_xcm::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type XcmExecutor = XcmExecutor<XcmConfig>;
}

impl mock_msg_queue::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type XcmExecutor = XcmExecutor<XcmConfig>;
}

pub struct SignedToAccountId20<RuntimeOrigin, AccountId, Network>(
	PhantomData<(RuntimeOrigin, AccountId, Network)>,
);
impl<
		RuntimeOrigin: OriginTrait + Clone,
		AccountId: Into<[u8; 20]>,
		Network: frame_support::traits::Get<Option<NetworkId>>,
	> TryConvert<RuntimeOrigin, Location> for SignedToAccountId20<RuntimeOrigin, AccountId, Network>
where
	RuntimeOrigin::PalletsOrigin: From<SystemRawOrigin<AccountId>>
		+ TryInto<SystemRawOrigin<AccountId>, Error = RuntimeOrigin::PalletsOrigin>,
{
	fn try_convert(o: RuntimeOrigin) -> Result<Location, RuntimeOrigin> {
		o.try_with_caller(|caller| match caller.try_into() {
			Ok(SystemRawOrigin::Signed(who)) =>
				Ok(Junction::AccountKey20 { network: Network::get(), key: who.into() }.into()),
			Ok(other) => Err(other.into()),
			Err(other) => Err(other),
		})
	}
}
