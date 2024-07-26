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

#![allow(clippy::new_without_default)]

use frame_support::parameter_types;

use pallet_asset_metadata_extender::precompiles::asset_metadata_extender::AssetMetadataExtenderPrecompile;
use pallet_evm_precompile_blake2::Blake2F;
use pallet_evm_precompile_bn128::{Bn128Add, Bn128Mul, Bn128Pairing};
use pallet_evm_precompile_modexp::Modexp;
use pallet_evm_precompile_parachain_staking::ParachainStakingPrecompile;
use pallet_evm_precompile_simple::{ECRecover, Identity, Ripemd160, Sha256};
use pallet_laos_evolution::{
	precompiles::{
		evolution_collection::EvolutionCollectionPrecompileSet,
		evolution_collection_factory::EvolutionCollectionFactoryPrecompile,
	},
	ASSET_PRECOMPILE_ADDRESS_PREFIX,
};
use pallet_precompiles_benchmark::precompiles::vesting::VestingPrecompile;
use precompile_utils::precompile_set::{
	AcceptDelegateCall, AddressU64, CallableByContract, CallableByPrecompile, PrecompileAt,
	PrecompileSetBuilder, PrecompileSetStartingWith, PrecompilesInRangeInclusive,
};

use crate::Runtime;

/// Precompile checks for ethereum spec precompiles
/// We allow DELEGATECALL to stay compliant with Ethereum behavior.
type EthereumPrecompilesChecks = (AcceptDelegateCall, CallableByContract, CallableByPrecompile);

#[precompile_utils::precompile_name_from_address]
pub type LaosPrecompilesSetAt = (
	// Ethereum precompiles:
	// We allow DELEGATECALL to stay compliant with Ethereum behavior.
	PrecompileAt<AddressU64<1>, ECRecover, EthereumPrecompilesChecks>,
	PrecompileAt<AddressU64<2>, Sha256, EthereumPrecompilesChecks>,
	PrecompileAt<AddressU64<3>, Ripemd160, EthereumPrecompilesChecks>,
	PrecompileAt<AddressU64<4>, Identity, EthereumPrecompilesChecks>,
	PrecompileAt<AddressU64<5>, Modexp, EthereumPrecompilesChecks>,
	PrecompileAt<AddressU64<6>, Bn128Add, EthereumPrecompilesChecks>,
	PrecompileAt<AddressU64<7>, Bn128Mul, EthereumPrecompilesChecks>,
	PrecompileAt<AddressU64<8>, Bn128Pairing, EthereumPrecompilesChecks>,
	PrecompileAt<AddressU64<9>, Blake2F, EthereumPrecompilesChecks>,
	// LAOS custom precompiles
	PrecompileAt<
		AddressU64<1027>,
		EvolutionCollectionFactoryPrecompile<Runtime>,
		(CallableByContract, CallableByPrecompile),
	>,
	PrecompileAt<
		AddressU64<1029>,
		AssetMetadataExtenderPrecompile<Runtime>,
		(CallableByContract, CallableByPrecompile),
	>,
	PrecompileAt<
		AddressU64<1030>,
		VestingPrecompile<Runtime>,
		(CallableByContract, CallableByPrecompile),
	>,
	PrecompileAt<
		AddressU64<2048>,
		ParachainStakingPrecompile<Runtime>,
		(CallableByContract, CallableByPrecompile),
	>,
);

parameter_types! {
	pub AssetPrefix: &'static [u8] = ASSET_PRECOMPILE_ADDRESS_PREFIX;
}

pub type LaosPrecompiles<R> = PrecompileSetBuilder<
	R,
	(
		// Skip precompiles if out of range.
		PrecompilesInRangeInclusive<
			// range of precompiles reserved addresses
			(AddressU64<1>, AddressU64<4096>),
			LaosPrecompilesSetAt,
		>,
		PrecompileSetStartingWith<
			AssetPrefix,
			EvolutionCollectionPrecompileSet<R>,
			CallableByContract,
		>,
	),
>;
