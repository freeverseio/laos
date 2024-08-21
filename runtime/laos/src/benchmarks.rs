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

frame_benchmarking::define_benchmarks!(
	[pallet_timestamp, Timestamp]
	[pallet_sudo, Sudo]
	[pallet_utility, Utility]
	[pallet_multisig, Multisig]
	[pallet_proxy, Proxy]
	[pallet_session, SessionBench::<Runtime>] // TODO check why SessionBench::<Runtime>
	[pallet_parachain_staking, ParachainStaking]
	[cumulus_pallet_xcmp_queue, XcmpQueue]
	[pallet_evm, EVM]
	[pallet_laos_evolution, LaosEvolution]
	[pallet_asset_metadata_extender, AssetMetadataExtender]
	[pallet_precompiles_benchmark, PrecompilesBenchmark]
	[pallet_vesting, Vesting]
	[pallet_message_queue, MessageQueue]
	[cumulus_pallet_parachain_system, ParachainSystem]
	// TODO pallet_xcm?
);
