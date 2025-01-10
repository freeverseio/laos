// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! Parachain runtime mock.

use core::marker::PhantomData;
use frame_support::{
	construct_runtime, derive_impl, parameter_types,
	traits::{ContainsPair, EnsureOrigin, EnsureOriginWithArg, Everything, EverythingBut, Nothing},
	weights::{constants::WEIGHT_REF_TIME_PER_SECOND, Weight},
};

use frame_system::EnsureRoot;
use sp_core::ConstU32;
use sp_runtime::{
	traits::{Get, IdentityLookup},
	AccountId32,
};
use sp_std::prelude::*;
use xcm_simulator::{Asset, AssetFilter};

use crate::mock_msg_queue;
use assets_common::{foreign_creators::ForeignCreators, matching::FromSiblingParachain};
use frame_support::traits::AsEnsureOriginWithArg;
use frame_system::EnsureSigned;
use pallet_xcm::XcmPassthrough;
use parachains_common::AssetIdForTrustBackedAssets;
use polkadot_parachain_primitives::primitives::Sibling;
use sp_runtime::codec;
use xcm::latest::prelude::*;
use xcm_builder::{
	Account32Hash, AccountId32Aliases, AllowUnpaidExecutionFrom, ConvertedConcreteId,
	EnsureDecodableXcm, EnsureXcmOrigin, FixedRateOfFungible, FixedWeightBounds,
	FrameTransactionalProcessor, FungibleAdapter, FungiblesAdapter,
	GlobalConsensusParachainConvertsFor, IsConcrete, MatchedConvertedConcreteId, NativeAsset,
	NoChecking, NonFungiblesAdapter, ParentIsPreset, SiblingParachainConvertsVia,
	SignedAccountId32AsNative, SignedToAccountId32, SovereignSignedViaLocation, StartsWith,
};
use xcm_executor::{
	traits::{ConvertLocation, JustTry},
	Config, XcmExecutor,
};

pub type SovereignAccountOf = (
	SiblingParachainConvertsVia<Sibling, AccountId>,
	AccountId32Aliases<RelayNetwork, AccountId>,
	ParentIsPreset<AccountId>,
);

pub type AccountId = AccountId32;
pub type Balance = u128;

parameter_types! {
	pub const BlockHashCount: u64 = 250;
}

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Runtime {
	type Nonce = u64;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
	type BlockHashCount = BlockHashCount;
	type AccountData = pallet_balances::AccountData<Balance>;
}

parameter_types! {
	pub ExistentialDeposit: Balance = 1;
	pub const MaxLocks: u32 = 50;
	pub const MaxReserves: u32 = 50;
}

impl pallet_balances::Config for Runtime {
	type MaxLocks = MaxLocks;
	type Balance = Balance;
	type RuntimeEvent = RuntimeEvent;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
	type MaxReserves = MaxReserves;
	type ReserveIdentifier = [u8; 8];
	type RuntimeHoldReason = RuntimeHoldReason;
	type RuntimeFreezeReason = RuntimeFreezeReason;
	type FreezeIdentifier = ();
	type MaxFreezes = ConstU32<0>;
}

#[cfg(feature = "runtime-benchmarks")]
pub struct UniquesHelper;
#[cfg(feature = "runtime-benchmarks")]
impl pallet_uniques::BenchmarkHelper<Location, AssetInstance> for UniquesHelper {
	fn collection(i: u16) -> Location {
		GeneralIndex(i as u128).into()
	}
	fn item(i: u16) -> AssetInstance {
		AssetInstance::Index(i as u128)
	}
}

impl pallet_uniques::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type CollectionId = Location;
	type ItemId = AssetInstance;
	type Currency = Balances;
	type CreateOrigin = ForeignCreatorsUnique;
	type ForceOrigin = frame_system::EnsureRoot<AccountId>;
	type CollectionDeposit = frame_support::traits::ConstU128<1_000>;
	type ItemDeposit = frame_support::traits::ConstU128<1_000>;
	type MetadataDepositBase = frame_support::traits::ConstU128<1_000>;
	type AttributeDepositBase = frame_support::traits::ConstU128<1_000>;
	type DepositPerByte = frame_support::traits::ConstU128<1>;
	type StringLimit = ConstU32<64>;
	type KeyLimit = ConstU32<64>;
	type ValueLimit = ConstU32<128>;
	type Locker = ();
	type WeightInfo = ();
	#[cfg(feature = "runtime-benchmarks")]
	type Helper = UniquesHelper;
}

// `EnsureOriginWithArg` impl for `CreateOrigin` which allows only XCM origins
// which are locations containing the class location.
pub struct ForeignCreatorsUnique;
impl EnsureOriginWithArg<RuntimeOrigin, Location> for ForeignCreatorsUnique {
	type Success = AccountId;

	fn try_origin(
		o: RuntimeOrigin,
		a: &Location,
	) -> sp_std::result::Result<Self::Success, RuntimeOrigin> {
		let origin_location = pallet_xcm::EnsureXcm::<Everything>::try_origin(o.clone())?;
		if !a.starts_with(&origin_location) {
			return Err(o)
		}
		SovereignAccountOf::convert_location(&origin_location).ok_or(o)
	}

	#[cfg(feature = "runtime-benchmarks")]
	fn try_successful_origin(a: &Location) -> Result<RuntimeOrigin, ()> {
		Ok(pallet_xcm::Origin::Xcm(a.clone()).into())
	}
}

parameter_types! {
	pub const ReservedXcmpWeight: Weight = Weight::from_parts(WEIGHT_REF_TIME_PER_SECOND.saturating_div(4), 0);
	pub const ReservedDmpWeight: Weight = Weight::from_parts(WEIGHT_REF_TIME_PER_SECOND.saturating_div(4), 0);
}

parameter_types! {
	pub const KsmLocation: Location = Location::parent();
	pub const HereLocation: Location = Location::here();
	pub const RelayNetwork: NetworkId = NetworkId::Kusama;
	pub UniversalLocation: InteriorLocation = [GlobalConsensus(RelayNetwork::get()), Parachain(MsgQueue::parachain_id().into())].into();
	pub CheckingAccount: AccountId = PolkadotXcm::check_account();
}

pub type LocationToAccountId = (
	ParentIsPreset<AccountId>,
	SiblingParachainConvertsVia<Sibling, AccountId>,
	AccountId32Aliases<RelayNetwork, AccountId>,
	Account32Hash<(), AccountId>,
);

pub type XcmOriginToCallOrigin = (
	SovereignSignedViaLocation<LocationToAccountId, RuntimeOrigin>,
	SignedAccountId32AsNative<RelayNetwork, RuntimeOrigin>,
	XcmPassthrough<RuntimeOrigin>,
);

parameter_types! {
	pub const UnitWeightCost: Weight = Weight::from_parts(1, 1);
	pub KsmPerSecondPerByte: (AssetId, u128, u128) = (Parent.into(), 1, 1);
	pub const MaxInstructions: u32 = 100;
	pub const MaxAssetsIntoHolding: u32 = 64;
	pub ForeignPrefix: Location = (Parent,).into();
}

/// Type that matches foreign assets.
/// We do this by matching on all possible Locations and excluding the ones
/// inside our local chain.
pub type ForeignAssetsMatcher = MatchedConvertedConcreteId<
	Location,                                // Asset id.
	Balance,                                 // Balance type.
	EverythingBut<StartsWith<HereLocation>>, // Location matcher.
	JustTry,                                 // How to convert from Location to AssetId.
	JustTry,                                 // How to convert from u128 to Balance.
>;

/// AssetTransactor for handling other parachains' native tokens.
pub type ForeignFungiblesTransactor = FungiblesAdapter<
	// Use this implementation of the `fungibles::*` traits.
	// `Balances` is the name given to the balances pallet in this particular example.
	ForeignAssets,
	// This transactor deals with the native token of sibling parachains.
	ForeignAssetsMatcher,
	// How we convert from a Location to an account id.
	LocationToAccountId,
	// The `AccountId` type.
	AccountId,
	// Not tracking teleports since we only use reserve asset transfers.
	NoChecking,
	// The account for checking.
	CheckingAccount,
>;

pub type LocalAssetTransactor = (
	FungibleAdapter<Balances, IsConcrete<KsmLocation>, LocationToAccountId, AccountId, ()>,
	NonFungiblesAdapter<
		ForeignUniques,
		ConvertedConcreteId<Location, AssetInstance, JustTry, JustTry>,
		SovereignAccountOf,
		AccountId,
		NoChecking,
		(),
	>,
);

pub type XcmRouter = EnsureDecodableXcm<super::ParachainXcmRouter<MsgQueue>>;
pub type Barrier = AllowUnpaidExecutionFrom<Everything>;

parameter_types! {
	pub NftCollectionOne: AssetFilter
		= Wild(AllOf { fun: WildNonFungible, id: (Parent, GeneralIndex(1)).into() });
	pub NftCollectionOneForRelay: (AssetFilter, Location)
		= (NftCollectionOne::get(), (Parent,).into());
}
pub type TrustedTeleporters = xcm_builder::Case<NftCollectionOneForRelay>;
pub type TrustedReserves = EverythingBut<xcm_builder::Case<NftCollectionOneForRelay>>;

pub struct XcmConfig;
impl Config for XcmConfig {
	type RuntimeCall = RuntimeCall;
	type XcmSender = XcmRouter;
	type AssetTransactor = (LocalAssetTransactor, ForeignFungiblesTransactor);
	type OriginConverter = XcmOriginToCallOrigin;
	type IsReserve = (NativeAsset, TrustedReserves);
	type IsTeleporter = TrustedTeleporters;
	type UniversalLocation = UniversalLocation;
	type Barrier = Barrier;
	type Weigher = FixedWeightBounds<UnitWeightCost, RuntimeCall, MaxInstructions>;
	type Trader = FixedRateOfFungible<KsmPerSecondPerByte, ()>;
	type ResponseHandler = ();
	type AssetTrap = ();
	type AssetLocker = PolkadotXcm;
	type AssetExchanger = ();
	type AssetClaims = ();
	type SubscriptionService = ();
	type PalletInstancesInfo = ();
	type FeeManager = ();
	type MaxAssetsIntoHolding = MaxAssetsIntoHolding;
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

impl mock_msg_queue::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type XcmExecutor = XcmExecutor<XcmConfig>;
}

pub type LocalOriginToLocation = SignedToAccountId32<RuntimeOrigin, AccountId, RelayNetwork>;

pub struct TrustedLockerCase<T>(PhantomData<T>);
impl<T: Get<(Location, AssetFilter)>> ContainsPair<Location, Asset> for TrustedLockerCase<T> {
	fn contains(origin: &Location, asset: &Asset) -> bool {
		let (o, a) = T::get();
		a.matches(asset) && &o == origin
	}
}

parameter_types! {
	pub RelayTokenForRelay: (Location, AssetFilter) = (Parent.into(), Wild(AllOf { id: Parent.into(), fun: WildFungible }));
}

pub type TrustedLockers = TrustedLockerCase<RelayTokenForRelay>;

impl pallet_xcm::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type SendXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
	type XcmRouter = XcmRouter;
	type ExecuteXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
	type XcmExecuteFilter = Everything;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type XcmTeleportFilter = Nothing;
	type XcmReserveTransferFilter = Everything;
	type Weigher = FixedWeightBounds<UnitWeightCost, RuntimeCall, MaxInstructions>;
	type UniversalLocation = UniversalLocation;
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	const VERSION_DISCOVERY_QUEUE_SIZE: u32 = 100;
	type AdvertisedXcmVersion = pallet_xcm::CurrentXcmVersion;
	type Currency = Balances;
	type CurrencyMatcher = ();
	type TrustedLockers = TrustedLockers;
	type SovereignAccountOf = LocationToAccountId;
	type MaxLockers = ConstU32<8>;
	type MaxRemoteLockConsumers = ConstU32<0>;
	type RemoteLockConsumerIdentifier = ();
	type WeightInfo = pallet_xcm::TestWeightInfo;
	type AdminOrigin = EnsureRoot<AccountId>;
}

pub type ForeignCreatorsSovereignAccountOf = (
	SiblingParachainConvertsVia<Sibling, AccountId>,
	AccountId32Aliases<RelayNetwork, AccountId>,
	ParentIsPreset<AccountId>,
	GlobalConsensusParachainConvertsFor<UniversalLocation, AccountId>,
);

// Called "Trust Backed" assets because these are generally registered by some account, and users of
// the asset assume it has some claimed backing. The pallet is called `Assets` in
// `construct_runtime` to avoid breaking changes on storage reads.
pub type TrustBackedAssetsInstance = pallet_assets::Instance1;
impl pallet_assets::Config<TrustBackedAssetsInstance> for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Balance = Balance;
	type AssetId = AssetIdForTrustBackedAssets;
	type AssetIdParameter = codec::Compact<AssetIdForTrustBackedAssets>;
	type Currency = Balances;
	type CreateOrigin = AsEnsureOriginWithArg<EnsureSigned<AccountId>>;
	type ForceOrigin = EnsureRoot<AccountId>;
	type AssetDeposit = frame_support::traits::ConstU128<1_000>;
	type MetadataDepositBase = frame_support::traits::ConstU128<1_000>;
	type MetadataDepositPerByte = frame_support::traits::ConstU128<1_000>;
	type ApprovalDeposit = ExistentialDeposit;
	type StringLimit = frame_support::traits::ConstU32<50>;
	type Freezer = ();
	type Extra = ();
	type WeightInfo = ();
	type CallbackHandle = ();
	type AssetAccountDeposit = frame_support::traits::ConstU128<1_000>;
	type RemoveItemsLimit = frame_support::traits::ConstU32<1000>;
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = ();
}

/// Simple conversion of `u32` into an `AssetId` for use in benchmarking.
#[cfg(feature = "runtime-benchmarks")]
pub struct XcmBenchmarkHelper;
#[cfg(feature = "runtime-benchmarks")]
impl pallet_assets::BenchmarkHelper<xcm::v4::Location> for XcmBenchmarkHelper {
	fn create_asset_id_parameter(id: u32) -> xcm::v4::Location {
		xcm::v4::Location::new(1, xcm::v4::Junction::Parachain(id))
	}
}

/// Assets managed by some foreign location. Note: we do not declare a `ForeignAssetsCall` type, as
/// this type is used in proxy definitions. We assume that a foreign location would not want to set
/// an individual, local account as a proxy for the issuance of their assets. This issuance should
/// be managed by the foreign location's governance.
pub type ForeignAssetsInstance = pallet_assets::Instance2;
impl pallet_assets::Config<ForeignAssetsInstance> for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Balance = Balance;
	type AssetId = Location;
	type AssetIdParameter = Location;
	type Currency = Balances;
	type CreateOrigin = ForeignCreators<
		FromSiblingParachain<parachain_info::Pallet<Runtime>, Location>,
		ForeignCreatorsSovereignAccountOf,
		AccountId,
		Location,
	>;
	type ForceOrigin = EnsureRoot<AccountId>;
	type AssetDeposit = frame_support::traits::ConstU128<1_000>;
	type MetadataDepositBase = frame_support::traits::ConstU128<1_000>;
	type MetadataDepositPerByte = frame_support::traits::ConstU128<1_000>;
	type ApprovalDeposit = ExistentialDeposit;
	type StringLimit = ConstU32<64>;
	type Freezer = ();
	type Extra = ();
	type WeightInfo = ();
	type CallbackHandle = ();
	type AssetAccountDeposit = frame_support::traits::ConstU128<1_000>;
	type RemoveItemsLimit = frame_support::traits::ConstU32<1000>;
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = XcmBenchmarkHelper;
}

impl parachain_info::Config for Runtime {}

type Block = frame_system::mocking::MockBlock<Runtime>;

construct_runtime!(
	pub enum Runtime
	{
		System: frame_system,
		ParachainInfo: parachain_info,
		Balances: pallet_balances,
		MsgQueue: mock_msg_queue,
		PolkadotXcm: pallet_xcm,
		ForeignUniques: pallet_uniques,
		Assets: pallet_assets::<Instance1> = 50,
		ForeignAssets: pallet_assets::<Instance2> = 53,
	}
);
