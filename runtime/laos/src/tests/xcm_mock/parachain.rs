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

//! Mock parachain runtime, as close to the real runtime as possible.
//! NOTE: uses the same XCM configuration as the real runtime, i.e `xcm_config.rs`.

use cumulus_primitives_core::{
	AssetId::{self as XcmAssetId, Concrete},
	Fungibility, InteriorMultiLocation,
	Junction::{self, GlobalConsensus, Parachain},
	Junctions::{Here, X1, X2},
	MultiAsset, MultiLocation, NetworkId, Plurality, XcmContext, XcmError,
};
use frame_support::{
	construct_runtime, derive_impl, match_types, parameter_types,
	traits::{AsEnsureOriginWithArg, Everything, FindAuthor, Nothing, OriginTrait},
	weights::{constants::WEIGHT_REF_TIME_PER_SECOND, Weight},
	PalletId,
};
use frame_system::{EnsureRoot, RawOrigin as SystemRawOrigin};
use laos_primitives::MAXIMUM_BLOCK_WEIGHT;
use pallet_assets::AssetsCallback;
use pallet_evm::{
	runner, EnsureAddressNever, EnsureAddressRoot, FeeCalculator, FixedGasWeightMapping,
	IdentityAddressMapping, SubstrateBlockHashMapping,
};

use orml_traits::{
	location::{RelativeReserveProvider, Reserve},
	parameter_type_with_key,
};
use pallet_xcm::XcmPassthrough;
use parity_scale_codec::Encode;
use polkadot_parachain_primitives::primitives::Sibling;
use sp_core::{ConstU128, ConstU32, H160, U256};
use sp_io::storage;
use sp_runtime::{
	traits::{AccountIdConversion, Convert, IdentityLookup, MaybeEquivalence, TryConvert},
	ConsensusEngineId,
};
use sp_std::{boxed::Box, prelude::*, str::FromStr};
use staging_xcm::v3::BodyId;
use staging_xcm_builder::{
	AccountKey20Aliases, AllowExplicitUnpaidExecutionFrom, AllowTopLevelPaidExecutionFrom,
	ConvertedConcreteId, CurrencyAdapter, DenyReserveTransferToRelayChain, DenyThenTry,
	EnsureXcmOrigin, FixedRateOfFungible, FixedWeightBounds, FungiblesAdapter, IsConcrete,
	NativeAsset, NoChecking, ParentIsPreset, RelayChainAsNative, SiblingParachainAsNative,
	SiblingParachainConvertsVia, SignedAccountKey20AsNative, SovereignSignedViaLocation,
	TakeWeightCredit, TrailingSetTopicAsId, WithComputedOrigin,
};
use staging_xcm_executor::{traits::WeightTrader, XcmExecutor};
use xcm_simulator::PhantomData;

use crate::precompiles::LaosPrecompiles;

pub type Block = frame_system::mocking::MockBlock<Runtime>;
pub type AccountId = H160;
pub type Balance = u128;

// Configure a mock runtime to test the pallet.
construct_runtime!(
	pub struct Runtime {
		System: frame_system,
		Balances: pallet_balances,
		Timestamp: pallet_timestamp,
		ParachainInfo: parachain_info = 3,
		EVM: pallet_evm,
		ParachainSystem: cumulus_pallet_parachain_system,

		XcmpQueue: cumulus_pallet_xcmp_queue,
		PolkadotXcm: pallet_xcm,
		CumulusXcm: cumulus_pallet_xcm,
		DmpQueue: cumulus_pallet_dmp_queue,

		Xtokens: orml_xtokens,
		Assets: pallet_assets = 123,
		LaosEvolution: pallet_laos_evolution,
	}
);

#[derive_impl(frame_system::config_preludes::TestDefaultConfig as frame_system::DefaultConfig)]
impl frame_system::Config for Runtime {
	type Block = Block;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnSetCode = cumulus_pallet_parachain_system::ParachainSetCode<Runtime>;
}

parameter_types! {
	pub const ExistentialDeposit: u64 = 0;
}

#[derive_impl(pallet_balances::config_preludes::TestDefaultConfig as pallet_balances::DefaultConfig)]
impl pallet_balances::Config for Runtime {
	type Balance = Balance;
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type RuntimeHoldReason = ();
	type DustRemoval = ();
}

parameter_types! {
	pub const MinimumPeriod: u64 = 1000;
}

impl pallet_timestamp::Config for Runtime {
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = MinimumPeriod;
	type WeightInfo = ();
}

parameter_types! {
	pub NullAddress: AccountId = AccountId::zero();
}

pub struct FixedGasPrice;
impl FeeCalculator for FixedGasPrice {
	fn min_gas_price() -> (U256, Weight) {
		// Return some meaningful gas price and weight
		(1_000_000_000u128.into(), Weight::from_parts(7u64, 0))
	}
}
pub struct FindAuthorTruncated;
impl FindAuthor<H160> for FindAuthorTruncated {
	fn find_author<'a, I>(_digests: I) -> Option<H160>
	where
		I: 'a + IntoIterator<Item = (ConsensusEngineId, &'a [u8])>,
	{
		Some(H160::from_str("1234500000000000000000000000000000000000").unwrap())
	}
}

const BLOCK_GAS_LIMIT: u64 = 150_000_000;
const MAX_POV_SIZE: u64 = 5 * 1024 * 1024;

parameter_types! {
	pub BlockGasLimit: U256 = U256::from(BLOCK_GAS_LIMIT);
	pub const GasLimitPovSizeRatio: u64 = BLOCK_GAS_LIMIT.saturating_div(MAX_POV_SIZE);
	pub WeightPerGas: Weight = Weight::from_parts(20_000, 0);
	pub PrecompilesValue: LaosPrecompiles<Runtime> = LaosPrecompiles::<_>::new();
}

impl pallet_evm::Config for Runtime {
	type FeeCalculator = FixedGasPrice;
	type GasWeightMapping = FixedGasWeightMapping<Self>;
	type WeightPerGas = WeightPerGas;
	type BlockHashMapping = SubstrateBlockHashMapping<Self>;
	type CallOrigin = EnsureAddressRoot<Self::AccountId>;
	type WithdrawOrigin = EnsureAddressNever<Self::AccountId>;
	type AddressMapping = IdentityAddressMapping;
	type Currency = Balances;
	type RuntimeEvent = RuntimeEvent;
	type PrecompilesType = LaosPrecompiles<Self>;
	type PrecompilesValue = PrecompilesValue;
	type ChainId = ();
	type BlockGasLimit = BlockGasLimit;
	type Runner = runner::stack::Runner<Self>;
	type OnChargeTransaction = ();
	type OnCreate = ();
	type FindAuthor = FindAuthorTruncated;
	type GasLimitPovSizeRatio = GasLimitPovSizeRatio;
	type Timestamp = Timestamp;
	type WeightInfo = ();
}

pub type AssetId = u32;

pub struct AssetsCallbackHandle;
impl AssetsCallback<AssetId, AccountId> for AssetsCallbackHandle {
	fn created(_id: &AssetId, _owner: &AccountId) -> Result<(), ()> {
		if Self::should_err() {
			Err(())
		} else {
			storage::set(Self::CREATED.as_bytes(), &().encode());
			Ok(())
		}
	}

	fn destroyed(_id: &AssetId) -> Result<(), ()> {
		if Self::should_err() {
			Err(())
		} else {
			storage::set(Self::DESTROYED.as_bytes(), &().encode());
			Ok(())
		}
	}
}

impl AssetsCallbackHandle {
	pub const CREATED: &'static str = "asset_created";
	pub const DESTROYED: &'static str = "asset_destroyed";

	const RETURN_ERROR: &'static str = "return_error";

	// Configures `Self` to return `Ok` when callbacks are invoked
	pub fn set_return_ok() {
		storage::clear(Self::RETURN_ERROR.as_bytes());
	}

	// Configures `Self` to return `Err` when callbacks are invoked
	pub fn set_return_error() {
		storage::set(Self::RETURN_ERROR.as_bytes(), &().encode());
	}

	// If `true`, callback should return `Err`, `Ok` otherwise.
	fn should_err() -> bool {
		storage::exists(Self::RETURN_ERROR.as_bytes())
	}
}

impl pallet_assets::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Balance = Balance;
	type AssetId = AssetId;
	type AssetIdParameter = u32;
	type Currency = Balances;
	type CreateOrigin = AsEnsureOriginWithArg<frame_system::EnsureSigned<AccountId>>;
	type ForceOrigin = frame_system::EnsureRoot<AccountId>;
	type AssetDeposit = ConstU128<1>;
	type AssetAccountDeposit = ConstU128<10>;
	type MetadataDepositBase = ConstU128<1>;
	type MetadataDepositPerByte = ConstU128<1>;
	type ApprovalDeposit = ConstU128<1>;
	type StringLimit = ConstU32<50>;
	type Freezer = ();
	type WeightInfo = ();
	type CallbackHandle = AssetsCallbackHandle;
	type Extra = ();
	type RemoveItemsLimit = ConstU32<5>;
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = ();
}

impl parachain_info::Config for Runtime {}

parameter_types! {
	pub const ReservedXcmpWeight: Weight = MAXIMUM_BLOCK_WEIGHT.saturating_div(4);
	pub const ReservedDmpWeight: Weight = MAXIMUM_BLOCK_WEIGHT.saturating_div(4);
}

impl cumulus_pallet_parachain_system::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type OnSystemEvent = ();
	type SelfParaId = parachain_info::Pallet<Runtime>;
	type OutboundXcmpMessageSource = XcmpQueue;
	type DmpMessageHandler = DmpQueue;
	type ReservedDmpWeight = ReservedDmpWeight;
	type XcmpMessageHandler = XcmpQueue;
	type ReservedXcmpWeight = ReservedXcmpWeight;
	type CheckAssociatedRelayNumber = cumulus_pallet_parachain_system::RelayNumberStrictlyIncreases;
}

impl cumulus_pallet_xcmp_queue::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type ChannelInfo = ParachainSystem;
	type VersionWrapper = ();
	type ExecuteOverweightOrigin = EnsureRoot<AccountId>;
	type ControllerOrigin = EnsureRoot<AccountId>;
	type ControllerOriginConverter = XcmOriginToTransactDispatchOrigin;
	type WeightInfo = ();
	type PriceForSiblingDelivery = ();
}

impl cumulus_pallet_dmp_queue::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type ExecuteOverweightOrigin = EnsureRoot<AccountId>;
}

// Below is XCM related configuration

parameter_types! {
	pub const ParentLocation: MultiLocation = MultiLocation::parent();
	pub const OurLocation: MultiLocation = MultiLocation::here();
	pub const RelayNetwork: Option<NetworkId> = Some(NetworkId::Kusama);
	pub RelayChainOrigin: RuntimeOrigin = cumulus_pallet_xcm::Origin::Relay.into();
	pub UniversalLocation: InteriorMultiLocation = X2(GlobalConsensus(RelayNetwork::get().unwrap()), Parachain(ParachainInfo::parachain_id().into()));
	pub DummyCheckingAccount: AccountId = PalletId(*b"laos_xcm").into_account_truncating();
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

/// Means for transacting native currency on this chain.
pub type LocalAssetTransactor = CurrencyAdapter<
	// Use this currency:
	Balances,
	// Use this currency when it is a fungible asset matching the given location or name:
	IsConcrete<OurLocation>, /* Do a simple pun to convert an AccountId20 MultiLocation into a
	                          * native chain account ID: */
	LocationToAccountId,
	// Our chain's account ID type (we can't get away without mentioning it explicitly):
	AccountId,
	// We don't track any teleports.
	(),
>;

/// Used to convert asset id to MultiLocation.
/// For mock use case only, since it is hard-coded to return same location for all assets.
pub struct AssetLocationIdConverter;

impl MaybeEquivalence<MultiLocation, AssetId> for AssetLocationIdConverter {
	fn convert(a: &MultiLocation) -> Option<AssetId> {
		match a {
			// native currency of Parachain 1 is asset id 2 in Parachain 2
			MultiLocation { parents: 1, interior: X1(Parachain(1)) } => Some(2),
			// native currency of Parachain 2 is asset id 1 in Parachain 1
			MultiLocation { parents: 1, interior: X1(Parachain(2)) } => Some(1),
			_ => None,
		}
	}

	fn convert_back(b: &AssetId) -> Option<MultiLocation> {
		match b {
			1 => Some(MultiLocation { parents: 1, interior: X1(Parachain(2)) }),
			2 => Some(MultiLocation { parents: 1, interior: X1(Parachain(1)) }),
			_ => None,
		}
	}
}

/// Simple, one-to-one `u128` to `Balance` converter.
pub struct ConvertBalance;

impl MaybeEquivalence<u128, Balance> for ConvertBalance {
	fn convert(a: &u128) -> Option<Balance> {
		Some(*a)
	}

	fn convert_back(b: &Balance) -> Option<u128> {
		Some(*b)
	}
}

/// Means for transacting assets on this chain.
pub type FungiblesTransactor = FungiblesAdapter<
	Assets,
	ConvertedConcreteId<AssetId, Balance, AssetLocationIdConverter, ConvertBalance>,
	LocationToAccountId,
	AccountId,
	NoChecking,
	DummyCheckingAccount,
>;

pub type AssetTransactor = (LocalAssetTransactor, FungiblesTransactor);

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

match_types! {
	pub type ParentOrParentsExecutivePlurality: impl Contains<MultiLocation> = {
		MultiLocation { parents: 1, interior: Here } |
		MultiLocation { parents: 1, interior: X1(Plurality { id: BodyId::Executive, .. }) }
	};
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

/// Used as weight trader for foreign assets.
///
/// In case foreigin asset is supported as payment asset, XCM execution time
/// on-chain can be paid by the foreign asset, using the configured rate.
///
/// Currently it's mocked to support three assets.
pub struct FixedRateOfForeignAsset {
	/// Total used weight
	weight: Weight,
	/// Total consumed assets
	consumed: u128,
	/// Asset Id (as MultiLocation) and units per second for payment
	asset_location_and_units_per_second: Option<(MultiLocation, u128)>,
}

impl WeightTrader for FixedRateOfForeignAsset {
	fn new() -> Self {
		Self { weight: Weight::zero(), consumed: 0, asset_location_and_units_per_second: None }
	}

	fn buy_weight(
		&mut self,
		weight: Weight,
		payment: staging_xcm_executor::Assets,
		_context: &XcmContext,
	) -> Result<staging_xcm_executor::Assets, XcmError> {
		// Atm in pallet, we only support one asset so this should work
		let payment_asset = payment.fungible_assets_iter().next().ok_or(XcmError::TooExpensive)?;

		match payment_asset {
			MultiAsset {
				id: staging_xcm::latest::AssetId::Concrete(asset_location),
				fun: Fungibility::Fungible(_),
			} => {
				let units_per_second = match AssetLocationIdConverter::convert(&asset_location) {
					Some(1) => 1_000_000_000_u128,
					Some(2) => 2_000_000_000_u128,
					_ => return Err(XcmError::TooExpensive),
				};

				let amount = units_per_second.saturating_mul(weight.ref_time() as u128) // TODO: change this to u64?
								/ (WEIGHT_REF_TIME_PER_SECOND as u128);
				if amount == 0 {
					return Ok(payment);
				}

				let unused = payment
					.checked_sub((asset_location, amount).into())
					.map_err(|_| XcmError::TooExpensive)?;

				self.weight = self.weight.saturating_add(weight);

				// If there are multiple calls to `BuyExecution` but with different assets, we need
				// to be able to handle that. Current primitive implementation will just keep total
				// track of consumed asset for the FIRST consumed asset. Others will just be ignored
				// when refund is concerned.
				if let Some((old_asset_location, _)) = self.asset_location_and_units_per_second {
					if old_asset_location == asset_location {
						self.consumed = self.consumed.saturating_add(amount);
					}
				} else {
					self.consumed = self.consumed.saturating_add(amount);
					self.asset_location_and_units_per_second =
						Some((asset_location, units_per_second));
				}

				Ok(unused)
			},
			_ => Err(XcmError::TooExpensive),
		}
	}

	fn refund_weight(&mut self, weight: Weight, _context: &XcmContext) -> Option<MultiAsset> {
		if let Some((asset_location, units_per_second)) = self.asset_location_and_units_per_second {
			let weight = weight.min(self.weight);
			let amount = units_per_second.saturating_mul(weight.ref_time() as u128) /
				(WEIGHT_REF_TIME_PER_SECOND as u128);

			self.weight = self.weight.saturating_sub(weight);
			self.consumed = self.consumed.saturating_sub(amount);

			if amount > 0 {
				Some((asset_location, amount).into())
			} else {
				None
			}
		} else {
			None
		}
	}
}

impl Drop for FixedRateOfForeignAsset {
	fn drop(&mut self) {}
}

pub struct XcmConfig;
impl staging_xcm_executor::Config for XcmConfig {
	type RuntimeCall = RuntimeCall;
	type XcmSender = XcmRouter;
	// How to withdraw and deposit an asset.
	type AssetTransactor = AssetTransactor;
	type OriginConverter = XcmOriginToTransactDispatchOrigin;
	type IsReserve = NativeAsset;
	type IsTeleporter = (); // Teleporting is disabled.
	type UniversalLocation = UniversalLocation;
	type Barrier = Barrier;
	type Weigher = FixedWeightBounds<UnitWeightCost, RuntimeCall, MaxInstructions>;
	type Trader = (FixedRateOfFungible<NativePerSecond, ()>, FixedRateOfForeignAsset);
	type ResponseHandler = PolkadotXcm;
	type AssetTrap = PolkadotXcm;
	type AssetClaims = PolkadotXcm;
	type SubscriptionService = PolkadotXcm;
	type PalletInstancesInfo = AllPalletsWithSystem;
	type MaxAssetsIntoHolding = ConstU32<32>;
	type AssetLocker = ();
	type AssetExchanger = ();
	type FeeManager = ();
	type MessageExporter = ();
	type UniversalAliases = Nothing;
	type CallDispatcher = RuntimeCall;
	type SafeCallFilter = Everything;
	type Aliasers = Nothing;
}

/// No local origins on this chain are allowed to dispatch XCM sends/executions.
pub type LocalOriginToLocation = SignedToAccountId20<RuntimeOrigin, AccountId, RelayNetwork>;

/// The means for routing XCM messages which are not for local execution into the right message
/// queues.
pub type XcmRouter = super::ParachainXcmRouter<ParachainInfo>;

#[cfg(feature = "runtime-benchmarks")]
parameter_types! {
	pub ReachableDest: Option<MultiLocation> = Some(cumulus_primitives_core::Parent.into());
}

impl pallet_xcm::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type SendXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
	type XcmRouter = XcmRouter;
	type ExecuteXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
	type XcmExecuteFilter = Everything;
	// ^ Disable dispatchable execute on the XCM pallet.
	// Needs to be `Everything` for local testing.
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type XcmTeleportFilter = Everything;
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
	type MaxLockers = ConstU32<8>;
	type WeightInfo = pallet_xcm::TestWeightInfo;
	#[cfg(feature = "runtime-benchmarks")]
	type ReachableDest = ReachableDest;
	type AdminOrigin = EnsureRoot<AccountId>;
	type MaxRemoteLockConsumers = ConstU32<0>;
	type RemoteLockConsumerIdentifier = ();
}

impl cumulus_pallet_xcm::Config for Runtime {
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

parameter_type_with_key! {
	pub ParachainMinFee: |location: MultiLocation| -> Option<u128> {
		match (location.parents, location.first_interior()) {
			(1, Some(Parachain(4u32))) => Some(50u128),
			_ => None,
		}
	};
}

/// Convert `AccountId` to `MultiLocation`.
pub struct AccountIdToMultiLocation;
impl Convert<AccountId, MultiLocation> for AccountIdToMultiLocation {
	fn convert(account: AccountId) -> MultiLocation {
		X1(Junction::AccountKey20 { network: None, key: account.into() }).into()
	}
}

parameter_types! {
	pub const UnitWeightCost: Weight = Weight::from_parts(1_000_000_000, 0);
	pub const MaxInstructions: u32 = 100;
	pub NativePerSecond: (XcmAssetId, u128, u128) = (Concrete(OurLocation::get()), 1_000_000, 1024 * 1024);
	pub const MaxAssetsForTransfer: usize = 2;
}

/// `MultiAsset` reserve location provider. It's based on `RelativeReserveProvider` and in
/// addition will convert self absolute location to relative location.
pub struct AbsoluteAndRelativeReserveProvider<AbsoluteLocation>(PhantomData<AbsoluteLocation>);
impl<AbsoluteLocation: sp_core::Get<MultiLocation>> Reserve
	for AbsoluteAndRelativeReserveProvider<AbsoluteLocation>
{
	fn reserve(asset: &MultiAsset) -> Option<MultiLocation> {
		RelativeReserveProvider::reserve(asset).map(|reserve_location| {
			if reserve_location == AbsoluteLocation::get() {
				MultiLocation::here()
			} else {
				reserve_location
			}
		})
	}
}

/// Our asset ID converter
pub struct AssetIdConvert;
impl Convert<AssetId, Option<MultiLocation>> for AssetIdConvert {
	fn convert(asset_id: AssetId) -> Option<MultiLocation> {
		if asset_id == 0 {
			Some(MultiLocation::here())
		} else {
			AssetLocationIdConverter::convert_back(&asset_id)
		}
	}
}

// The XCM message wrapper wrapper
impl orml_xtokens::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Balance = Balance;
	type CurrencyId = AssetId;
	type AccountIdToMultiLocation = AccountIdToMultiLocation;
	type CurrencyIdConvert = AssetIdConvert;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type SelfLocation = OurLocation;
	type Weigher =
		staging_xcm_builder::FixedWeightBounds<UnitWeightCost, RuntimeCall, MaxInstructions>;
	type BaseXcmWeight = UnitWeightCost;
	type UniversalLocation = UniversalLocation;
	type MaxAssetsForTransfer = MaxAssetsForTransfer;
	type MinXcmFee = ParachainMinFee;
	type MultiLocationsFilter = Everything;
	type ReserveProvider = AbsoluteAndRelativeReserveProvider<UniversalLocation>;
}

parameter_types! {
	pub const MaxTokenUriLength: u32 = 512;
}

pub struct AccountIdToH160;

impl sp_runtime::traits::Convert<AccountId, H160> for AccountIdToH160 {
	fn convert(account_id: AccountId) -> H160 {
		account_id
	}
}
pub struct H160ToAccountId;

impl Convert<H160, AccountId> for H160ToAccountId {
	fn convert(h160: H160) -> AccountId {
		AccountId::from(h160)
	}
}

impl pallet_laos_evolution::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type AccountIdToH160 = AccountIdToH160;
	type H160ToAccountId = H160ToAccountId;
	type MaxTokenUriLength = MaxTokenUriLength;
	type WeightInfo = ();
	type GasWeightMapping = pallet_evm::FixedGasWeightMapping<Self>;
	type OnCreateCollection = ();
}
