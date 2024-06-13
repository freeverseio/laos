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

use precompile_utils::prelude::{Address, EvmResult, PrecompileHandle};
use sp_runtime::traits::PhantomData;

// This is the simplest bytecode to revert without returning any data.
// We will pre-deploy it under all of our precompiles to ensure they can be called from
// within contracts.
// (PUSH1 0x00 PUSH1 0x00 REVERT)
pub const REVERT_BYTECODE: [u8; 5] = [0x60, 0x00, 0x60, 0x00, 0xFD];

pub struct EvolutionCollectionFactoryPrecompile<R>(PhantomData<R>);

impl<R> EvolutionCollectionFactoryPrecompile<R> {
	pub fn new() -> Self {
		Self(PhantomData)
	}
}

#[precompile_utils::precompile]
impl<R> EvolutionCollectionFactoryPrecompile<R>
where
	R: pallet_evm::Config,
{
	#[precompile::public("createCollection(address)")]
	fn create_collection(
		_handle: &mut impl PrecompileHandle,
		owner: Address,
	) -> EvmResult<Address> {
		Ok(Address(owner.into()))
	}
}
