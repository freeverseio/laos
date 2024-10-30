pub mod asset_transactor;
pub mod barrier;
pub mod constants;
pub mod location_converter;
pub mod origin_converter;
pub mod reserve;
pub mod teleporter;
pub mod weigher;

use super::{MsgQueue, PolkadotXcm, RuntimeCall};
use frame_support::traits::{Everything, Nothing};
use xcm_builder::{FixedRateOfFungible, FrameTransactionalProcessor};

// Generated from `decl_test_network!`
pub type XcmRouter = crate::ParachainXcmRouter<MsgQueue>;

pub struct XcmConfig;
impl xcm_executor::Config for XcmConfig {
	type RuntimeCall = RuntimeCall;
	type XcmSender = XcmRouter;
	type AssetTransactor = asset_transactor::AssetTransactor;
	type OriginConverter = origin_converter::OriginConverter;
	type IsReserve = reserve::TrustedReserves;
	type IsTeleporter = teleporter::TrustedTeleporters;
	type UniversalLocation = constants::UniversalLocation;
	type Barrier = barrier::Barrier;
	type Weigher = weigher::Weigher;
	type Trader = FixedRateOfFungible<constants::KsmPerSecondPerByte, ()>;
	type ResponseHandler = PolkadotXcm;
	type AssetTrap = PolkadotXcm;
	type AssetLocker = PolkadotXcm;
	type AssetExchanger = ();
	type AssetClaims = PolkadotXcm;
	type SubscriptionService = PolkadotXcm;
	type PalletInstancesInfo = ();
	type FeeManager = ();
	type MaxAssetsIntoHolding = constants::MaxAssetsIntoHolding;
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
