use crate::Runtime;

impl pallet_block_rewards_source::Config for Runtime {
	type WeightInfo = pallet_block_rewards_source::weights::SubstrateWeight<Runtime>;
}
