frame_benchmarking::define_benchmarks!(
	[frame_system, SystemBench::<Runtime>]
	[pallet_balances, Balances]
	[pallet_session, SessionBench::<Runtime>]
	[pallet_timestamp, Timestamp]
	[cumulus_pallet_xcmp_queue, XcmpQueue]
	[pallet_laos_evolution, LaosEvolution]
	[pallet_asset_metadata_extender, AssetMetadataExtender]
	[pallet_parachain_staking, ParachainStaking]
	[pallet_block_rewards_handler, BlockRewardsHandler]
);
