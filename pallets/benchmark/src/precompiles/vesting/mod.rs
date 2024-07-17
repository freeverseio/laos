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
pub use crate::{precompiles::register_cost, weights::WeightInfo};
use frame_support::{traits::tokens::currency::Currency, DefaultNoBound};
use frame_system::{pallet_prelude::BlockNumberFor, RawOrigin};
use pallet_vesting::Pallet as PalletVesting;
use precompile_utils::prelude::{revert, solidity, Address, EvmResult, PrecompileHandle};
use scale_info::prelude::{format, string::String};
use sp_core::{H160, U256};
use sp_runtime::{
	traits::{ConvertBack, PhantomData, StaticLookup},
	DispatchError,
};
use sp_std::vec::Vec;

type BalanceOf<Runtime> = <<Runtime as pallet_vesting::Config>::Currency as Currency<
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
	Runtime: crate::Config + pallet_vesting::Config,
	Runtime::AccountIdToH160: ConvertBack<Runtime::AccountId, H160>,
	BalanceOf<Runtime>: Into<U256>,
	BlockNumberFor<Runtime>: Into<U256>,
{
	#[precompile::public("vesting(address)")]
	#[precompile::view]
	pub fn vesting(
		handle: &mut impl PrecompileHandle,
		account: Address,
	) -> EvmResult<Vec<VestingInfo>> {
		match PalletVesting::<Runtime>::vesting(Runtime::AccountIdToH160::convert_back(
			account.into(),
		)) {
			Some(v) => {
				register_cost::<Runtime>(
					handle,
					<Runtime as crate::Config>::WeightInfo::precompile_vesting(v.len() as u32),
				)?;
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
			None => {
				register_cost::<Runtime>(
					handle,
					<Runtime as crate::Config>::WeightInfo::precompile_vesting(0),
				)?;
				Ok(Vec::new())
			},
		}
	}

	#[precompile::public("vest()")]
	pub fn vest(handle: &mut impl PrecompileHandle) -> EvmResult<()> {
		register_cost::<Runtime>(
			handle,
			<Runtime as crate::Config>::WeightInfo::precompile_vest(),
		)?;

		match PalletVesting::<Runtime>::vest(
			<Runtime as frame_system::Config>::RuntimeOrigin::from(RawOrigin::from(Some(
				Runtime::AccountIdToH160::convert_back(handle.context().caller),
			))),
		) {
			Ok(_) => Ok(()),
			Err(err) => Err(revert(convert_dispatch_error_to_string(err))),
		}
	}

	#[precompile::public("vestOther(address)")]
	pub fn vest_other(handle: &mut impl PrecompileHandle, account: Address) -> EvmResult<()> {
		register_cost::<Runtime>(
			handle,
			<Runtime as crate::Config>::WeightInfo::precompile_vest_other(),
		)?;

		let origin = <Runtime as frame_system::Config>::RuntimeOrigin::from(RawOrigin::from(Some(
			Runtime::AccountIdToH160::convert_back(handle.context().caller),
		)));
		let account_id = Runtime::AccountIdToH160::convert_back(account.into());
		let target =
			<<Runtime as frame_system::Config>::Lookup as StaticLookup>::unlookup(account_id);
		match PalletVesting::<Runtime>::vest_other(origin, target) {
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

mod benchmarking;
#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;
