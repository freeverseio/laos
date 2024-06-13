#![cfg_attr(not(feature = "std"), no_std)]

use fp_evm::ExitError;
use frame_support::DefaultNoBound;
use pallet_laos_evolution::{
	address_to_collection_id, types::CollectionId, Pallet as LaosEvolution,
};
use parity_scale_codec::Encode;
use precompile_utils::prelude::{
	revert, Address, DiscriminantResult, EvmResult, PrecompileHandle, RuntimeHelper,
};
use sp_core::H160;
use sp_runtime::traits::PhantomData;

#[derive(Clone, DefaultNoBound)]
pub struct EvolutionCollectionPrecompileSet<R>(PhantomData<R>);

impl<R> EvolutionCollectionPrecompileSet<R> {
	pub fn new() -> Self {
		Self(PhantomData)
	}
}

#[precompile_utils::precompile]
#[precompile::precompile_set]
impl<R> EvolutionCollectionPrecompileSet<R>
where
	R: pallet_evm::Config + pallet_laos_evolution::Config,
	R::AccountId: From<H160> + Into<H160> + Encode,
{
	#[precompile::discriminant]
	fn discriminant(address: H160, gas: u64) -> DiscriminantResult<CollectionId> {
		let extra_cost = RuntimeHelper::<R>::db_read_gas_cost();
		if gas < extra_cost {
			return DiscriminantResult::OutOfGas;
		}

		// maybe here we could avoid the extra_cost calculation cause there's no db read
		match address_to_collection_id(address) {
			Ok(id) => DiscriminantResult::Some(id, extra_cost),
			Err(_) => DiscriminantResult::None(extra_cost),
		}
	}

	#[precompile::public("owner()")]
	#[precompile::view]
	fn owner(
		collection_id: CollectionId,
		_handle: &mut impl PrecompileHandle,
	) -> EvmResult<Address> {
		if let Some(owner) = LaosEvolution::<R>::collection_owner(collection_id) {
			// TODO: handle the cost
			// handle.record_cost(GasCalculator::<Runtime>::db_read_gas_cost(1))?;

			Ok(Address(owner.into()))
		} else {
			Err(revert("collection does not exist"))
		}
	}
}

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;
