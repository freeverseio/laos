use crate::{Runtime, RuntimeEvent};

impl pallet_block_rewards_source::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = pallet_block_rewards_source::weights::SubstrateWeight<Runtime>;
}
