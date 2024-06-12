#![cfg_attr(not(feature = "std"), no_std)]

use fp_evm::ExitError;
use frame_support::DefaultNoBound;
use pallet_laos_evolution::{address_to_collection_id, types::CollectionId};
use precompile_utils::prelude::*; // TODO remove *
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
		match address_to_collection_id(address) {
			Ok(id) => DiscriminantResult::Some(id, gas),
			Err(_) => DiscriminantResult::None(gas),
		}

		// Replace with your discriminant logic.
		// Some(match address {
		//     a if a == H160::from(42) => 1
		//     a if a == H160::from(43) => 2,
		//     _ => return None,
		// })
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
