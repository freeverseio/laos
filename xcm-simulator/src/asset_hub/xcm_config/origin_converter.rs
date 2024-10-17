use crate::asset_hub::{
	constants::RelayNetwork, location_converter::LocationConverter, RuntimeOrigin,
};
use pallet_xcm::XcmPassthrough;
use xcm_builder::{SignedAccountId32AsNative, SovereignSignedViaLocation};

type XcmOriginToCallOrigin = (
	SovereignSignedViaLocation<LocationConverter, RuntimeOrigin>,
	SignedAccountId32AsNative<RelayNetwork, RuntimeOrigin>,
	XcmPassthrough<RuntimeOrigin>,
);

pub type OriginConverter = XcmOriginToCallOrigin;
