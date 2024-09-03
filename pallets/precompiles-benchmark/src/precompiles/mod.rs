pub mod vesting;

use fp_evm::ExitError;
use frame_support::pallet_prelude::Weight;
use pallet_evm::GasWeightMapping;
use precompile_utils::prelude::PrecompileHandle;

// TODO this function is duplicated in all the precompiles, we should refactor it
pub fn register_cost<Runtime: crate::Config>(
	handle: &mut impl PrecompileHandle,
	weight: Weight,
) -> Result<(), ExitError> {
	let required_gas = Runtime::GasWeightMapping::weight_to_gas(weight);
	let remaining_gas = handle.remaining_gas();
	if required_gas > remaining_gas {
		return Err(ExitError::OutOfGas);
	}
	handle.record_cost(required_gas)?;
	handle.record_external_cost(Some(weight.ref_time()), Some(weight.proof_size()), Some(0))?;
	Ok(())
}
