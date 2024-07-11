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
use frame_support::{pallet_prelude::Weight, traits::tokens::currency::Currency, DefaultNoBound};
use frame_system::{pallet_prelude::BlockNumberFor, RawOrigin};
use pallet_evm::GasWeightMapping;
use pallet_vesting::{Config, Pallet};
use precompile_utils::{
	precompile,
	prelude::{revert, solidity, Address, EvmResult, PrecompileHandle},
};
use scale_info::prelude::{format, string::String};
use sp_core::{H160, U256};
use sp_runtime::{
	traits::{ConvertBack, PhantomData, StaticLookup},
	DispatchError,
};
use sp_std::vec::Vec;

type BalanceOf<Runtime> = <<Runtime as Config>::Currency as Currency<
	<Runtime as frame_system::Config>::AccountId,
>>::Balance;

#[derive(Default, solidity::Codec)]
pub struct VestingInfo {
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
	Runtime: Config + ConvertBack<Runtime::AccountId, H160>,
	BalanceOf<Runtime>: Into<U256>,
	BlockNumberFor<Runtime>: Into<U256>,
{
	#[precompile::public("vesting(address)")]
	#[precompile::view]
	pub fn vesting(
		_handle: &mut impl PrecompileHandle,
		account: Address,
	) -> EvmResult<Vec<VestingInfo>> {
		// TODO super::register_cost::<Runtime>(handle, Runtime::WeightInfo::precompile_vesting())?;

		match Pallet::<Runtime>::vesting(Runtime::convert_back(account.into())) {
			Some(v) => {
				let mut output: Vec<VestingInfo> = Vec::with_capacity(v.len());

				for i in v {
					output.push(VestingInfo {
						locked: i.locked().into(),
						per_block: i.per_block().into(),
						starting_block: i.starting_block().into(),
					})
				}

				Ok(output)
			},
			None => Ok(Vec::new()),
		}
	}

	#[precompile::public("vest()")]
	pub fn vest(handle: &mut impl PrecompileHandle) -> EvmResult<()> {
		// TODO super::register_cost::<Runtime>(handle, Runtime::WeightInfo::precompile_vest())?;

		match Pallet::<Runtime>::vest(<Runtime as frame_system::Config>::RuntimeOrigin::from(
			RawOrigin::from(Some(Runtime::convert_back(handle.context().caller))),
		)) {
			Ok(_) => Ok(()),
			Err(err) => Err(revert(convert_dispatch_error_to_string(err))),
		}
	}

	#[precompile::public("vestOther(address)")]
	pub fn vest_other(handle: &mut impl PrecompileHandle, account: Address) -> EvmResult<()> {
		// TODO super::register_cost::<Runtime>(handle, Runtime::WeightInfo::precompile_vest_other())?;

		let origin = <Runtime as frame_system::Config>::RuntimeOrigin::from(RawOrigin::from(Some(
			Runtime::convert_back(handle.context().caller),
		)));
		let account_id = Runtime::convert_back(account.into());
		let target =
			<<Runtime as frame_system::Config>::Lookup as StaticLookup>::unlookup(account_id);
		match Pallet::<Runtime>::vest_other(origin, target) {
			Ok(_) => Ok(()),
			Err(err) => Err(revert(convert_dispatch_error_to_string(err))),
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
// 	let required_gas = Runtime::GasWeightMapping::weight_to_gas(weight);
// 	let remaining_gas = handle.remaining_gas();
// 	if required_gas > remaining_gas {
// 		return Err(ExitError::OutOfGas);
// 	}
// 	handle.record_cost(required_gas)?;
// 	handle.record_external_cost(Some(weight.ref_time()), Some(weight.proof_size()))?;
// 	Ok(())
// }

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;
