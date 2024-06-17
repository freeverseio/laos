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

use crate::precompiles::EvolutionCollectionPrecompileSet;
#[allow(unused)]
use crate::Pallet as LaosEvolution;
use fp_evm::Transfer;
use frame_benchmarking::v2::*;
use sp_std::{vec, vec::Vec};

struct MockPrecompileHandle;
impl pallet_evm::PrecompileHandle for MockPrecompileHandle {
	fn call(
		&mut self,
		_: sp_core::H160,
		_: Option<Transfer>,
		_: Vec<u8>,
		_: Option<u64>,
		_: bool,
		_: &pallet_evm::Context,
	) -> (pallet_evm::ExitReason, Vec<u8>) {
		unimplemented!()
	}

	fn record_cost(&mut self, _: u64) -> Result<(), pallet_evm::ExitError> {
		unimplemented!()
	}

	fn remaining_gas(&self) -> u64 {
		unimplemented!()
	}

	fn log(
		&mut self,
		_: sp_core::H160,
		_: Vec<sp_core::H256>,
		_: Vec<u8>,
	) -> Result<(), pallet_evm::ExitError> {
		unimplemented!()
	}

	fn code_address(&self) -> sp_core::H160 {
		unimplemented!()
	}

	fn input(&self) -> &[u8] {
		unimplemented!()
	}

	fn context(&self) -> &pallet_evm::Context {
		unimplemented!()
	}

	fn is_static(&self) -> bool {
		true
	}

	fn gas_limit(&self) -> Option<u64> {
		unimplemented!()
	}

	fn record_external_cost(
		&mut self,
		_ref_time: Option<u64>,
		_proof_size: Option<u64>,
	) -> Result<(), fp_evm::ExitError> {
		Ok(())
	}

	fn refund_external_cost(&mut self, _ref_time: Option<u64>, _proof_size: Option<u64>) {}
}

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn precompile_owner() {
		let caller: T::AccountId = whitelisted_caller();
		let owner = caller.clone();
		let collection_id = LaosEvolution::<T>::create_collection(owner).unwrap();
		let mut handle = MockPrecompileHandle;

		#[block]
		{
			EvolutionCollectionPrecompileSet::<T>::owner(collection_id, &mut handle);
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
				s as Slot,
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
			s as Slot,
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
	fn enable_public_minting() {
		let caller: T::AccountId = whitelisted_caller();
		let owner = caller.clone();
		let collection_id = LaosEvolution::<T>::create_collection(owner.clone()).unwrap();

		#[block]
		{
			LaosEvolution::<T>::enable_public_minting(owner, collection_id).unwrap();
		}
		assert!(CollectionPublicMintingEnabled::<T>::contains_key(collection_id));
	}

	#[benchmark]
	fn disable_public_minting() {
		let caller: T::AccountId = whitelisted_caller();
		let owner = caller.clone();
		let collection_id = LaosEvolution::<T>::create_collection(owner.clone()).unwrap();
		LaosEvolution::<T>::enable_public_minting(owner.clone(), collection_id).unwrap();

		#[block]
		{
			LaosEvolution::<T>::disable_public_minting(owner, collection_id).unwrap();
		}
		assert!(!CollectionPublicMintingEnabled::<T>::contains_key(collection_id));
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
