#![cfg_attr(not(feature = "std"), no_std)]

use fp_evm::ExitError;
use frame_support::DefaultNoBound;
use pallet_laos_evolution::{address_to_collection_id, types::CollectionId};
use precompile_utils::prelude::{DiscriminantResult, EvmResult, PrecompileHandle, RuntimeHelper};
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
	R: pallet_evm::Config,
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

	#[precompile::public("example(uint32)")]
	fn example(
		_discriminant: CollectionId,
		_handle: &mut impl PrecompileHandle,
		_arg: u64,
	) -> EvmResult {
		// Discriminant can be used here.
		Ok(())
	}
}
