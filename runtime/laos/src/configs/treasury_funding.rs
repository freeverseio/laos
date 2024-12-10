use crate::{Runtime, RuntimeEvent};

impl pallet_treasury_funding::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
}
