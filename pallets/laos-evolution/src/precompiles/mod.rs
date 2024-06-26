pub mod evolution_collection;
pub mod evolution_collection_factory;

use fp_evm::ExitError;
use frame_support::pallet_prelude::Weight;
use pallet_evm::GasWeightMapping;
use precompile_utils::prelude::PrecompileHandle;

pub fn register_cost<Runtime: crate::Config>(
	handle: &mut impl PrecompileHandle,
	weight: Weight,
) -> Result<(), ExitError> {
	let required_gas = Runtime::GasWeightMapping::weight_to_gas(weight);
	let remaining_gas = handle.remaining_gas();
	if required_gas > remaining_gas {
		return Err(ExitError::OutOfGas);
	}
	handle.record_cost(required_gas)
}
