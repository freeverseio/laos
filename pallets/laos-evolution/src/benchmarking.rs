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

//! Benchmarking setup for pallet-living-assets-evolution
#![cfg(feature = "runtime-benchmarks")]
use super::*;

use crate::precompiles::{
	evolution_collection::EvolutionCollectionPrecompileSet,
	evolution_collection_factory::EvolutionCollectionFactoryPrecompile,
};
#[allow(unused)]
use crate::Pallet as LaosEvolution;
use fp_evm::Transfer;
use frame_benchmarking::v2::*;
use pallet_evm::{Context, ExitError, ExitReason, Log, PrecompileHandle};
use precompile_utils::{prelude::Address, solidity::codec::UnboundedString};
use sp_core::{H160, H256, U256};
use sp_std::{vec, vec::Vec};

pub struct MockHandle {
	pub input: Vec<u8>,
	pub gas_limit: Option<u64>,
	pub context: Context,
	pub is_static: bool,
	pub gas_used: u64,
	pub logs: Vec<Log>,
	pub code_address: H160,
}

impl MockHandle {
	pub fn new(caller: H160) -> Self {
		Self {
			input: vec![],
			gas_limit: None,
			context: Context { address: H160::zero(), caller, apparent_value: U256::zero() },
			is_static: false,
			gas_used: 0,
			logs: vec![],
			code_address: H160::zero(),
		}
	}
}

impl PrecompileHandle for MockHandle {
	/// Perform subcall in provided context.
	/// Precompile specifies in which context the subcall is executed.
	fn call(
		&mut self,
		_: H160,
		_: Option<Transfer>,
		_: Vec<u8>,
		_: Option<u64>,
		_: bool,
		_: &Context,
	) -> (ExitReason, Vec<u8>) {
		unimplemented!()
	}

	fn record_cost(&mut self, cost: u64) -> Result<(), ExitError> {
		self.gas_used += cost;
		Ok(())
	}

	fn record_external_cost(
		&mut self,
		_: Option<u64>,
		_: Option<u64>,
		_: Option<u64>,
	) -> Result<(), ExitError> {
		Ok(())
	}

	fn refund_external_cost(&mut self, _: Option<u64>, _: Option<u64>) {}

	fn log(&mut self, address: H160, topics: Vec<H256>, data: Vec<u8>) -> Result<(), ExitError> {
		let log = Log { address, topics, data };
		self.logs.push(log);
		Ok(())
	}

	fn remaining_gas(&self) -> u64 {
		1000000000000
	}

	fn code_address(&self) -> H160 {
		self.code_address
	}

	fn input(&self) -> &[u8] {
		&self.input
	}

	fn context(&self) -> &Context {
		&self.context
	}

	fn is_static(&self) -> bool {
		self.is_static
	}

	fn gas_limit(&self) -> Option<u64> {
		self.gas_limit
	}
}

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn precompile_discriminant() {
		let address = H160::zero();
		let gas = 100000000;
		#[block]
		{
			let _ = EvolutionCollectionPrecompileSet::<T>::discriminant(address, gas);
		}
	}

	#[benchmark]
	fn precompile_create_collection() {
		let owner = Address::from(H160::zero());
		let mut handle = MockHandle::new(owner.into());

		#[block]
		{
			let res =
				EvolutionCollectionFactoryPrecompile::<T>::create_collection(&mut handle, owner);
			assert!(res.is_ok());
		}
	}

	#[benchmark]
	fn precompile_mint(s: Linear<0, { <T as Config>::MaxTokenUriLength::get() }>) {
		let caller: T::AccountId = whitelisted_caller();
		let owner = caller.clone();
		let to = Address::from(H160::from_low_u64_be(1));
		let slot = Slot::try_from(2).unwrap();
		let token_uri = vec![1u8; s.try_into().unwrap()].try_into().unwrap();
		let collection_id = LaosEvolution::<T>::create_collection(owner).unwrap();
		let mut handle = MockHandle::new(T::AccountIdToH160::convert(caller));

		#[block]
		{
			let res = EvolutionCollectionPrecompileSet::<T>::mint(
				collection_id,
				&mut handle,
				to,
				slot,
				token_uri,
			);
			assert!(res.is_ok());
		}
	}

	#[benchmark]
	fn precompile_evolve(s: Linear<0, { <T as Config>::MaxTokenUriLength::get() }>) {
		let caller: T::AccountId = whitelisted_caller();
		let owner = caller.clone();
		let token_uri: UnboundedString = vec![1u8; s.try_into().unwrap()].try_into().unwrap();
		let collection_id = LaosEvolution::<T>::create_collection(owner).unwrap();
		let mut handle = MockHandle::new(T::AccountIdToH160::convert(caller));
		let to = Address::from(H160::from_low_u64_be(1));
		let slot = Slot::try_from(2).unwrap();
		let token_id = EvolutionCollectionPrecompileSet::<T>::mint(
			collection_id,
			&mut handle,
			to,
			slot,
			token_uri.clone(),
		)
		.unwrap();

		#[block]
		{
			let res = EvolutionCollectionPrecompileSet::<T>::evolve(
				collection_id,
				&mut handle,
				token_id,
				token_uri,
			);
			assert!(res.is_ok());
		}
	}

	#[benchmark]
	fn precompile_transfer_ownership() {
		let caller: T::AccountId = whitelisted_caller();
		let owner = caller.clone();
		let collection_id = LaosEvolution::<T>::create_collection(owner).unwrap();
		let mut handle = MockHandle::new(T::AccountIdToH160::convert(caller));
		let to = Address::from(H160::from_low_u64_be(1));

		#[block]
		{
			let res = EvolutionCollectionPrecompileSet::<T>::transfer_ownership(
				collection_id,
				&mut handle,
				to,
			);
			assert!(res.is_ok());
		}
	}

	#[benchmark]
	fn precompile_owner() {
		let caller: T::AccountId = whitelisted_caller();
		let owner = caller.clone();
		let collection_id = LaosEvolution::<T>::create_collection(owner).unwrap();
		let mut handle = MockHandle::new(T::AccountIdToH160::convert(caller));

		#[block]
		{
			let res = EvolutionCollectionPrecompileSet::<T>::owner(collection_id, &mut handle);
			assert!(res.is_ok());
		}
	}

	#[benchmark]
	fn precompile_token_uri() {
		let caller: T::AccountId = whitelisted_caller();
		let owner = caller.clone();
		let token_uri: UnboundedString = vec![1u8; 100].try_into().unwrap();
		let collection_id = LaosEvolution::<T>::create_collection(owner).unwrap();
		let mut handle = MockHandle::new(T::AccountIdToH160::convert(caller));
		let to = Address::from(H160::from_low_u64_be(1));
		let slot = Slot::try_from(2).unwrap();
		let token_id = EvolutionCollectionPrecompileSet::<T>::mint(
			collection_id,
			&mut handle,
			to,
			slot,
			token_uri.clone(),
		)
		.unwrap();

		#[block]
		{
			let res = EvolutionCollectionPrecompileSet::<T>::token_uri(
				collection_id,
				&mut handle,
				token_id,
			);
			assert!(res.is_ok());
		}
	}

	#[benchmark]
	fn create_collection() {
		let caller: T::AccountId = whitelisted_caller();
		let owner = caller.clone();

		#[block]
		{
			LaosEvolution::<T>::create_collection(owner.clone()).unwrap();
		}
		assert_eq!(CollectionOwner::<T>::get(0), Some(caller));
	}

	impl_benchmark_test_suite!(LaosEvolution, crate::mock::new_test_ext(), crate::mock::Test);

	#[benchmark]
	fn mint_with_external_uri(s: Linear<0, { <T as Config>::MaxTokenUriLength::get() }>) {
		let caller: T::AccountId = whitelisted_caller();
		let owner = caller.clone();
		let collection_id = LaosEvolution::<T>::create_collection(owner.clone()).unwrap();

		#[block]
		{
			let token_id = LaosEvolution::<T>::mint_with_external_uri(
				owner.clone(),
				collection_id,
				(s as u128).try_into().unwrap(),
				owner.clone(),
				vec![1u8; s as usize].try_into().unwrap(),
			)
			.unwrap();

			assert_eq!(
				LaosEvolution::<T>::token_uri(collection_id, token_id),
				Some(vec![1u8; s as usize].try_into().unwrap())
			);
		};
	}

	#[benchmark]
	fn evolve_with_external_uri(s: Linear<0, { <T as Config>::MaxTokenUriLength::get() }>) {
		let caller: T::AccountId = whitelisted_caller();
		let owner = caller.clone();
		let collection_id = LaosEvolution::<T>::create_collection(owner.clone()).unwrap();
		let token_id = LaosEvolution::<T>::mint_with_external_uri(
			owner.clone(),
			0,
			(s as u128).try_into().unwrap(),
			owner.clone(),
			vec![0u8; s as usize].try_into().unwrap(),
		)
		.unwrap();

		#[block]
		{
			LaosEvolution::<T>::evolve_with_external_uri(
				owner.clone(),
				collection_id,
				token_id,
				vec![1u8; s as usize].try_into().unwrap(),
			)
			.unwrap();
		}

		assert_eq!(
			LaosEvolution::<T>::token_uri(collection_id, token_id),
			Some(vec![1u8; s as usize].try_into().unwrap())
		);
	}

	#[benchmark]
	fn transfer_ownership() {
		let caller: T::AccountId = whitelisted_caller();
		let owner = caller.clone();
		let new_owner: T::AccountId = account("new_owner", 0, 0);
		let collection_id = LaosEvolution::<T>::create_collection(owner.clone()).unwrap();

		#[block]
		{
			LaosEvolution::<T>::transfer_ownership(owner.clone(), new_owner.clone(), collection_id)
				.unwrap();
		}

		assert_eq!(CollectionOwner::<T>::get(collection_id), Some(new_owner));
	}
}
