#![allow(clippy::new_without_default)]

use pallet_evm::{
	ExitRevert, IsPrecompileResult, Precompile, PrecompileFailure, PrecompileHandle,
	PrecompileResult, PrecompileSet,
};
use sp_core::H160;
use sp_std::marker::PhantomData;

use pallet_evm_asset_metadata_extender::AssetMetadataExtenderPrecompile;
use pallet_evm_evolution_collection::EvolutionCollectionPrecompile;
use pallet_evm_evolution_collection_factory::EvolutionCollectionFactoryPrecompile;
use pallet_evm_precompile_blake2::Blake2F;
use pallet_evm_precompile_bn128::{Bn128Add, Bn128Mul, Bn128Pairing};
use pallet_evm_precompile_modexp::Modexp;
use pallet_evm_precompile_simple::{ECRecover, Identity, Ripemd160, Sha256};
use pallet_laos_evolution::address_to_collection_id;

use crate::Runtime;

pub struct FrontierPrecompiles<Runtime>(PhantomData<Runtime>);

impl<Runtime> FrontierPrecompiles<Runtime>
where
	Runtime: pallet_evm::Config,
{
	pub fn new() -> Self {
		Self(Default::default())
	}
	pub fn used_addresses() -> [H160; 12] {
		[
			hash(1),
			hash(2),
			hash(3),
			hash(4),
			hash(5),
			hash(6),
			hash(7),
			hash(8),
			hash(9),
			hash(1027),
			hash(1028), // This will be deprecated in the future
			hash(1029),
		]
	}

	pub(crate) fn is_delegatecall_to_custom_precompile(
		&self,
		code_address: H160,
		context_address: H160,
	) -> bool {
		// Check if the code address is a precompile
		if let IsPrecompileResult::Answer { is_precompile, .. } =
			self.is_precompile(code_address, u64::MAX)
		{
			// Return true if:
			// 1. It is a precompile.
			// 2. The code address is beyond the first nine standard Ethereum precompiles.
			// 3. The context address is different from the code address.
			// This indicates a delegate call to a custom precompile.
			return is_precompile && code_address > hash(9) && context_address != code_address;
		}

		// If none of the above conditions are met, return false
		false
	}
}

type AssetMetadataExtender = AssetMetadataExtenderPrecompile<Runtime>;

type EvolutionCollectionFactory = EvolutionCollectionFactoryPrecompile<Runtime>;

type EvolutionCollection = EvolutionCollectionPrecompile<Runtime>;

impl<Runtime> PrecompileSet for FrontierPrecompiles<Runtime>
where
	Runtime: pallet_evm::Config,
{
	fn execute(&self, handle: &mut impl PrecompileHandle) -> Option<PrecompileResult> {
		if self
			.is_delegatecall_to_custom_precompile(handle.code_address(), handle.context().address)
		{
			return Some(Err(PrecompileFailure::Revert {
				exit_status: ExitRevert::Reverted,
				output: b"cannot be called with DELEGATECALL or CALLCODE".to_vec(),
			}));
		}

		match handle.code_address() {
			// Ethereum precompiles :
			a if a == hash(1) => Some(ECRecover::execute(handle)),
			a if a == hash(2) => Some(Sha256::execute(handle)),
			a if a == hash(3) => Some(Ripemd160::execute(handle)),
			a if a == hash(4) => Some(Identity::execute(handle)),
			a if a == hash(5) => Some(Modexp::execute(handle)),
			a if a == hash(6) => Some(Bn128Add::execute(handle)),
			a if a == hash(7) => Some(Bn128Mul::execute(handle)),
			a if a == hash(8) => Some(Bn128Pairing::execute(handle)),
			a if a == hash(9) => Some(Blake2F::execute(handle)),
			a if a == hash(1027) => Some(EvolutionCollectionFactory::execute(handle)),
			a if a == hash(1028) => Some(AssetMetadataExtender::execute(handle)),
			a if address_to_collection_id(a).is_ok() => Some(EvolutionCollection::execute(handle)),
			_ => None,
		}
	}

	fn is_precompile(&self, address: H160, _gas: u64) -> IsPrecompileResult {
		if address_to_collection_id(address).is_ok() {
			return IsPrecompileResult::Answer { is_precompile: true, extra_cost: 0 }
		}

		IsPrecompileResult::Answer {
			is_precompile: Self::used_addresses().contains(&address),
			extra_cost: 0,
		}
	}
}

fn hash(a: u64) -> H160 {
	H160::from_low_u64_be(a)
}

#[cfg(test)]
mod mock;
