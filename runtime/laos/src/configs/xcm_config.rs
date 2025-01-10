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

use crate::{
	types::ToAuthor, AccountId, AllPalletsWithSystem, Balances, ParachainInfo, PolkadotXcm,
	Runtime, RuntimeCall, RuntimeEvent, RuntimeOrigin, Vec,
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
	DenyReserveTransferToRelayChain, DenyThenTry, EnsureXcmOrigin, FixedWeightBounds,
	FrameTransactionalProcessor, FungibleAdapter, IsConcrete, MintLocation, ParentIsPreset,
	RelayChainAsNative, SiblingParachainAsNative, SiblingParachainConvertsVia,
	SignedAccountKey20AsNative, SovereignSignedViaLocation, TakeWeightCredit, TrailingSetTopicAsId,
	UsingComponents, WithComputedOrigin,
};
use xcm_executor::XcmExecutor;

pub const ASSET_HUB_ID: u32 = 1_000;
pub const RELAY_NETWORK: NetworkId = NetworkId::Polkadot;

parameter_types! {
	// Optional network identifier for the Relay Chain; set to `None` for default behavior.
	pub const RelayNetwork: NetworkId = RELAY_NETWORK;
	// Defines the origin for messages coming from the Relay Chain.
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

/// Converts a `MultiLocation` into an `AccountId`.
/// Used for asset ownership and dispatching `Transact` instructions.
pub type LocationToAccountId = (
	// Converts the parent (Relay Chain) location into the parent `AccountId`.
	ParentIsPreset<AccountId>,
	// Converts sibling parachain locations into `AccountId` via `ParaId::into()`.
	SiblingParachainConvertsVia<Sibling, AccountId>,
	// Directly aliases `AccountId20` locations to the local `AccountId`.
	AccountKey20Aliases<RelayNetwork, AccountId>,
);

/// Handles asset transactions on this chain.
pub type LocalAssetTransactor = FungibleAdapter<
	// Uses the Balances pallet as the fungible asset handler.
	Balances,
	// Recognizes assets that are concrete representations of the Relay Chain's location.
	IsConcrete<HereLocation>,
	// Converts `MultiLocation` into local `AccountId` for asset operations.
	LocationToAccountId,
	// Specifies the local `AccountId` type.
	AccountId,
	// Teleportation allowed: making sure the receiver can mint our assets
	Checking,
>;

/// Converts incoming XCM origins into local `Origin` instances for dispatching transactions.
pub type XcmOriginToTransactDispatchOrigin = (
	// Converts a sovereign account from the origin location into a local `Signed` origin.
	SovereignSignedViaLocation<LocationToAccountId, RuntimeOrigin>,
	// Converts the Relay Chain (Parent) location into a native `Relay` origin.
	RelayChainAsNative<RelayChainOrigin, RuntimeOrigin>,
	// Converts sibling parachain locations into a native `SiblingParachain` origin.
	SiblingParachainAsNative<cumulus_pallet_xcm::Origin, RuntimeOrigin>,
	// Converts `AccountId20` from the origin into a local `Signed` origin with the same 20-byte
	// key.
	SignedAccountKey20AsNative<RelayNetwork, RuntimeOrigin>,
	// Passes through XCM origins as native XCM pallet origins.
	XcmPassthrough<RuntimeOrigin>,
);

parameter_types! {
	// One XCM operation is 1_000_000_000 weight - almost certainly a conservative estimate.
	pub UnitWeightCost: Weight = Weight::from_parts(1_000_000_000, 64 * 1024);
	pub const MaxInstructions: u32 = 100;
	pub const MaxAssetsIntoHolding: u32 = 64;
}

// Defines a struct representing either the Parent (Relay Chain) or its Executive plurality.
pub struct ParentOrParentsExecutivePlurality;
impl Contains<Location> for ParentOrParentsExecutivePlurality {
	fn contains(location: &Location) -> bool {
		// Checks if the location matches either:
		// - The Parent location (Relay Chain).
		// - The Executive body of the Parent.
		matches!(
			location.unpack(),
			(1, []) // Matches the Parent location.
            | (1, [Plurality { id: BodyId::Executive, .. }]) /* Matches Parent's
			                                                              * Executive plurality. */
		)
	}
}

pub type Barrier = TrailingSetTopicAsId<
	// Converts trailing topics into IDs for message tracking.
	DenyThenTry<
		// Denies specific operations first, then tries others if denied.
		DenyReserveTransferToRelayChain, /* Denies reserve asset transfers to the Relay Chain
		                                  * for security reasons. */
		(
			TakeWeightCredit, // Consumes the weight credit from the XCM message.
			WithComputedOrigin<
				// Computes the execution origin for authorization.
				(
					// Allows paid execution from any top-level XCM origin.
					AllowTopLevelPaidExecutionFrom<Everything>,
					// Allows free execution (unpaid) from the parent or its executive plurality.
					AllowExplicitUnpaidExecutionFrom<ParentOrParentsExecutivePlurality>,
					// ^^^ Parent and its executive plurality get free execution.
				),
				UniversalLocation, // Universal location used for origin matching.
				ConstU32<8>,       // Maximum depth for recursive origin resolution.
			>,
		),
	>,
>;

parameter_types! {
	pub ParentTokenPerSecondPerByte: (AssetId, u128, u128) = (AssetId(Parent.into()), 1, 1);
}

parameter_types! {
	pub NativeToken: AssetId = AssetId(Location::here());
	pub NativeTokenFilter: AssetFilter = Wild(AllOf { fun: WildFungible, id: NativeToken::get() });
	pub AssetHubLocation: Location = Location::new(1, [Parachain(ASSET_HUB_ID)]);
	pub AssetHubTrustedTeleporter: (AssetFilter, Location) = (NativeTokenFilter::get(), AssetHubLocation::get());
}

// The teleporters we trust:
// - Asset Hub as teleporter of the LAOS token.
pub type TrustedTeleporters = xcm_builder::Case<AssetHubTrustedTeleporter>;

// The reserves we trust:
// - LAOS as a reserve of the LAOS token.
pub struct TrustedReserves;
impl frame_support::traits::ContainsPair<Asset, Location> for TrustedReserves {
	fn contains(asset: &Asset, location: &Location) -> bool {
		matches!(asset, Asset { id: asset_id, fun: Fungible(_) } if asset_id.0 == HereLocation::get() && location == &HereLocation::get())
	}
}

pub struct XcmConfig;
impl xcm_executor::Config for XcmConfig {
	type RuntimeCall = RuntimeCall;
	type XcmSender = XcmRouter;
	// Handles asset operations like deposit and withdrawal.
	type AssetTransactor = LocalAssetTransactor;
	// Converts XCM origins to local dispatch origins.
	type OriginConverter = XcmOriginToTransactDispatchOrigin;
	// This defines which locations we trust as reserve of which assets.
	type IsReserve = TrustedReserves;
	// This defines which locations we trust as teleportes of which assets.
	type IsTeleporter = TrustedTeleporters;
	type UniversalLocation = UniversalLocation;
	// Filters and allows XCM messages based on security policies.
	type Barrier = Barrier;
	// Calculates the weight (execution cost) of XCM messages.
	type Weigher = FixedWeightBounds<UnitWeightCost, RuntimeCall, MaxInstructions>;
	// Converts weight fees into asset payments and handles fee charging.
	type Trader = UsingComponents<
		<Runtime as pallet_transaction_payment::Config>::WeightToFee,
		HereLocation,
		AccountId,
		Balances,
		ToAuthor<Runtime>,
	>;
	// Handles responses from XCM messages.
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
	// Allows all calls to be dispatched via XCM.
	type SafeCallFilter = Everything;
	type Aliasers = Nothing;
	// Ensures transactional integrity during XCM execution.
	type TransactionalProcessor = FrameTransactionalProcessor;
	type HrmpNewChannelOpenRequestHandler = ();
	type HrmpChannelAcceptedHandler = ();
	type HrmpChannelClosingHandler = ();
	type XcmRecorder = PolkadotXcm;
}

/// Disallows local origins from dispatching XCM sends/executions.
pub type LocalOriginToLocation = SignedToAccountId20<RuntimeOrigin, AccountId, RelayNetwork>;

/// Routes XCM messages to the appropriate message queues.
pub type XcmRouter = xcm_builder::WithUniqueTopic<(
	// Uses UMP (Upward Message Passing) to communicate with the Relay Chain (Parent).
	cumulus_primitives_utility::ParentAsUmp<crate::ParachainSystem, (), ()>,
	// Uses XCMP (Cross-Chain Message Passing) to communicate with sibling parachains.
	crate::XcmpQueue,
)>;

//Filter all teleports/reserves that aren't the LAOS token
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
	// Determines which origins are allowed to send XCM messages.
	type SendXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
	type XcmRouter = XcmRouter;
	// Determines which origins are allowed to execute XCM messages.
	type ExecuteXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
	type XcmExecuteFilter = Nothing;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	// This filter wheter an origin may teleport out an asset.
	type XcmTeleportFilter = OnlySendNative;
	// This filter wheter an origin may execute a reserve transfer of an asset.
	type XcmReserveTransferFilter = OnlySendNative;
	// Calculates the weight of XCM messages.
	type Weigher = FixedWeightBounds<UnitWeightCost, RuntimeCall, MaxInstructions>;
	type UniversalLocation = UniversalLocation;
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	const VERSION_DISCOVERY_QUEUE_SIZE: u32 = 100;
	// ^ Overrides the default size for version discovery queue.
	type AdvertisedXcmVersion = pallet_xcm::CurrentXcmVersion;
	type Currency = Balances;
	type CurrencyMatcher = ();
	type TrustedLockers = ();
	// Converts `MultiLocation` to local `AccountId`.
	type SovereignAccountOf = LocationToAccountId;
	type MaxLockers = ConstU32<8>;
	type WeightInfo = pallet_xcm::TestWeightInfo;
	// Only the root account has administrative privileges.
	type AdminOrigin = EnsureRoot<AccountId>;
	type MaxRemoteLockConsumers = ConstU32<0>;
	type RemoteLockConsumerIdentifier = ();
}

impl cumulus_pallet_xcm::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type XcmExecutor = XcmExecutor<XcmConfig>;
}

/// Converts a local `Signed` origin into a `MultiLocation` using `AccountId20`.
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
			// Converts a `Signed` origin into a `MultiLocation::AccountKey20` with the same account
			// key.
			Ok(SystemRawOrigin::Signed(who)) =>
				Ok(Junction::AccountKey20 { network: Network::get(), key: who.into() }.into()),
			Ok(other) => Err(other.into()),
			Err(other) => Err(other),
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	parameter_types! {
		pub SomeLocation: Location = Location::new(1, [Junction::Parachain(123)] );
	}

	#[test]
	fn check_checking_account() {
		assert_eq!(
			CheckingAccount::get().to_string(),
			"0x6d6F646c70792F78636D63680000000000000000"
		);
	}

	#[test]
	fn only_teleport_native_contains_only_native_asset() {
		let assets: Vec<Asset> = vec![(HereLocation::get(), Fungible(1_000)).into()];

		// The first parameter passed to contains may be any location as it's not used by the
		// function. We use HereLocation for simplicity.
		assert!(OnlySendNative::contains(&(HereLocation::get(), assets)));
	}

	#[test]
	fn only_teleport_native_contains_not_all_assets_are_native() {
		let assets: Vec<Asset> = vec![
			(HereLocation::get(), Fungible(1_000)).into(),
			(SomeLocation::get(), Fungible(1_000)).into(),
		];

		// The first parameter passed to contains may be any location as it's not used by the
		// function. We use HereLocation for simplicity.
		assert!(!OnlySendNative::contains(&(HereLocation::get(), assets)));
	}
}
