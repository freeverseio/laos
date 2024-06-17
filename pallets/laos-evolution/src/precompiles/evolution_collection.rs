#![cfg_attr(not(feature = "std"), no_std)]

use crate::{
	address_to_collection_id, types::CollectionId, weights::WeightInfo, Pallet as LaosEvolution,
};
use fp_evm::ExitError;
use frame_support::DefaultNoBound;
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
	R: crate::Config,
	// R::AccountId: From<H160> + Into<H160> + Encode,
{
	#[precompile::discriminant]
	fn discriminant(address: H160, gas: u64) -> DiscriminantResult<CollectionId> {
		// maybe here we could avoid the extra_cost calculation cause there's no db read
		match address_to_collection_id(address) {
			Ok(id) => DiscriminantResult::Some(id, gas),
			Err(_) => DiscriminantResult::None(gas),
		}
	}

	#[precompile::public("owner()")]
	#[precompile::view]
	pub fn owner(
		collection_id: CollectionId,
		handle: &mut impl PrecompileHandle,
	) -> EvmResult<Address> {
		if let Some(_owner) = LaosEvolution::<R>::collection_owner(collection_id) {
			let weight = R::WeightInfo::precompile_owner();
			handle.record_external_cost(Some(weight.ref_time()), Some(weight.proof_size()))?;

			Ok(Address::default()) // TODO
		} else {
			Err(revert("collection does not exist"))
		}
	}
}

// #[cfg(test)]
// mod mock;
// #[cfg(test)]
// mod tests;
