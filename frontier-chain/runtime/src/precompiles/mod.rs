#![allow(clippy::new_without_default)]

use pallet_evm::{
	IsPrecompileResult, Precompile, PrecompileHandle, PrecompileResult, PrecompileSet,
};
use sp_core::H160;
use sp_std::marker::PhantomData;

use pallet_evm_living_assets_evolution::LivingAssetsEvolutionPrecompile;
use pallet_evm_precompile_modexp::Modexp;
use pallet_evm_precompile_simple::{ECRecover, ECRecoverPublicKey, Identity, Ripemd160, Sha256};
use pallet_living_assets_evolution::TokenUriOf;

use crate::{AccountId, Runtime};

/// Set of precompiles of evochain.
pub struct LaosEvolutionPrecompiles<Runtime>(PhantomData<Runtime>);

impl<Runtime> LaosEvolutionPrecompiles<Runtime>
where
	Runtime: pallet_evm::Config,
{
	pub fn new() -> Self {
		Self(Default::default())
	}
	pub fn used_addresses() -> [H160; 7] {
		[hash(1), hash(2), hash(3), hash(4), hash(5), hash(1025), hash(1026)]
	}
}

type LivingAssetsEvolution = LivingAssetsEvolutionPrecompile<
	pallet_evm::IdentityAddressMapping,
	AccountId,
	TokenUriOf<Runtime>,
	pallet_living_assets_evolution::Pallet<Runtime>,
>;

impl<Runtime> PrecompileSet for LaosEvolutionPrecompiles<Runtime>
where
	Runtime: pallet_evm::Config + pallet_living_assets_evolution::Config,
{
	fn execute(&self, handle: &mut impl PrecompileHandle) -> Option<PrecompileResult> {
		match handle.code_address() {
			// Ethereum precompiles :
			a if a == hash(1) => Some(ECRecover::execute(handle)),
			a if a == hash(2) => Some(Sha256::execute(handle)),
			a if a == hash(3) => Some(Ripemd160::execute(handle)),
			a if a == hash(4) => Some(Identity::execute(handle)),
			a if a == hash(5) => Some(Modexp::execute(handle)),
			// Non-Frontier specific nor Ethereum precompiles :
			a if a == hash(1025) => Some(ECRecoverPublicKey::execute(handle)),
			a if a == hash(1026) => Some(LivingAssetsEvolution::execute(handle)),
			_ => None,
		}
	}

	fn is_precompile(&self, address: H160, _gas: u64) -> IsPrecompileResult {
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

#[cfg(test)]
mod test;
