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

//! LAOS precompile module.

#![cfg_attr(not(feature = "std"), no_std)]
use fp_evm::ExitError;
use frame_support::{pallet_prelude::Weight, DefaultNoBound};
use frame_system::RawOrigin;
use pallet_evm::GasWeightMapping;
use pallet_vesting::{Config, Pallet};
use precompile_utils::{precompile, prelude::{
	revert, solidity, Address, EvmResult, PrecompileHandle,
}};
use scale_info::prelude::{format, string::String};
use sp_core::{H160, U256};
use sp_runtime::{
	traits::{ConvertBack, PhantomData},
	DispatchError,
};

#[derive(Default, solidity::Codec)]
struct VestingInfo {
	locked: U256,
	per_block: U256,
	starting_block: U256,
}

#[derive(Clone, DefaultNoBound)]
pub struct VestingPrecompile<Runtime>(PhantomData<Runtime>);

impl<Runtime> VestingPrecompile<Runtime> {
	pub fn new() -> Self {
		Self(PhantomData)
	}
}

#[precompile_utils::precompile]
impl<Runtime> VestingPrecompile<Runtime>
where
	Runtime: Config + ConvertBack<Runtime::AccountId, H160>
{
	// #[precompile::public("vesting(address)")]
	// #[precompile::view]
	// pub fn vesting(handle: &mut impl PrecompileHandle, account: H160) -> EvmResult<Vec<VestingInfo>> {
	// 	// super::register_cost::<Runtime>(handle, Runtime::WeightInfo::precompile_owner())?;

	// 	let res = Pallet::vesting(account).unwrap();
	// 	// Ok(Pallet::vesting(account))
	// 	// if let Some(owner) = LaosEvolution::<Runtime>::collection_owner(collection_id) {
	// 	// 	Ok(Address(R::AccountIdToH160::convert(owner)))
	// 	// } else {
	// 	// 	Err(revert("collection does not exist"))
	// 	// }
	// }

	#[precompile::public("vest()")]
	pub fn vest(handle: &mut impl PrecompileHandle) -> EvmResult<()> {
		match Pallet::<Runtime>::vest(<Runtime as frame_system::Config>::RuntimeOrigin::from(RawOrigin::from(Some(Runtime::convert_back(handle.context().caller))))) {
			Ok(_) => Ok(()),
			Err(err) => Err(revert(convert_dispatch_error_to_string(err)))
		}
	}
}

fn convert_dispatch_error_to_string(err: DispatchError) -> String {
	match err {
		DispatchError::Module(mod_err) => mod_err.message.unwrap_or("Unknown module error").into(),
		_ => format!("{:?}", err),
	}
}

// pub fn register_cost<Runtime: Config>(
// 	handle: &mut impl PrecompileHandle,
// 	weight: Weight,
// ) -> Result<(), ExitError> {
// 	let required_gas = <pallet_evm::FixedGasWeightMapping<Runtime> as GasWeightMapping>::weight_to_gas(weight);
// 	let remaining_gas = handle.remaining_gas();
// 	if required_gas > remaining_gas {
// 		return Err(ExitError::OutOfGas);
// 	}
// 	handle.record_cost(required_gas)?;
// 	handle.record_external_cost(Some(weight.ref_time()), Some(weight.proof_size()))?;
// 	Ok(())
// }

// #[cfg(test)]
// mod mock;
// #[cfg(test)]
// mod tests;
