use crate::asset_hub::RuntimeCall;
use frame_support::parameter_types;
use xcm::latest::prelude::*;
use xcm_builder::FixedWeightBounds;

parameter_types! {
	pub const UnitWeightCost: Weight = Weight::from_parts(1, 1);
	pub const MaxInstructions: u32 = 100;
}

pub type Weigher = FixedWeightBounds<UnitWeightCost, RuntimeCall, MaxInstructions>;
