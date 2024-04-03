frame_benchmarking::define_benchmarks!(
	[frame_system, SystemBench::<Runtime>]
	[pallet_timestamp, Timestamp]
	[pallet_sudo, Sudo]
	[pallet_utility, Utility]
	[pallet_multisig, Multisig]
	[pallet_proxy, Proxy]
	[pallet_balances, Balances]
	[pallet_vesting, Vesting]
	[pallet_session, SessionBench::<Runtime>] // TODO check why SessionBench::<Runtime>
	[pallet_parachain_staking, ParachainStaking]
	[cumulus_pallet_xcmp_queue, XcmpQueue]
	[pallet_evm, EVM]
	[pallet_laos_evolution, LaosEvolution]
	[pallet_asset_metadata_extender, AssetMetadataExtender]
	// TODO pallet_xcm?
);
