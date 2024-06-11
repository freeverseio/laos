#![cfg_attr(not(feature = "std"), no_std)]

use fp_evm::ExitError;
use precompile_utils::prelude::*;
use sp_core::H160;
use sp_runtime::traits::PhantomData;

pub struct EvolutionCollectionPrecompile<R>(PhantomData<R>);

#[precompile_utils::precompile]
#[precompile::precompile_set]
impl<R> EvolutionCollectionPrecompile<R>
where
	R: pallet_evm::Config,
{
	#[precompile::discriminant]
	fn discriminant(address: H160, gas: u64) -> DiscriminantResult<R> {
		DiscriminantResult::None(gas)
		// Replace with your discriminant logic.
		// Some(match address {
		//     a if a == H160::from(42) => 1
		//     a if a == H160::from(43) => 2,
		//     _ => return None,
		// })
	}

	// #[precompile::public("example(uint32)")]
	// fn example(discriminant: u8, handle: &mut impl PrecompileHandle, arg: u32) -> EvmResult {
	//     // Discriminant can be used here.
	//     Ok(arg * discriminant)
	// }
}
