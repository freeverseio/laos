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

#[allow(unused)]
use crate::Pallet as AssetMetadataExtender;
use crate::{
	precompiles::asset_metadata_extender::AssetMetadataExtenderPrecompile,
	traits::AssetMetadataExtender as AssetMetadataExtenderT,
};
use fp_evm::Transfer;
use frame_benchmarking::v2::*;
use pallet_evm::{Context, ExitError, ExitReason, Log, PrecompileHandle};
use precompile_utils::prelude::Address;
use sp_core::{H160, H256, U256};
use sp_runtime::traits::Convert;
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
	pub fn new() -> Self {
		let caller = whitelisted_caller();
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

	impl_benchmark_test_suite!(
		AssetMetadataExtender,
		crate::mock::new_test_ext(),
		crate::mock::Test
	);

	#[benchmark]
	fn precompile_extend(
		t: Linear<0, { <T as Config>::MaxTokenUriLength::get() }>,
		u: Linear<0, { <T as Config>::MaxUniversalLocationLength::get() }>,
	) {
		let mut handle = MockHandle::new();

		let ul: UniversalLocationOf<T> = vec![1u8; u.try_into().unwrap()].try_into().unwrap();
		let token_uri: TokenUriOf<T> = vec![1u8; t.try_into().unwrap()].try_into().unwrap();
		let caller: T::AccountId = whitelisted_caller();
		let claimer = caller.clone();

		#[block]
		{
			AssetMetadataExtenderPrecompile::<T>::extend(
				&mut handle,
				ul.clone().to_vec().into(),
				token_uri.clone().to_vec().into(),
			)
			.unwrap();
		};

		assert_eq!(
			AssetMetadataExtender::<T>::token_uris_by_claimer_and_location(claimer, ul),
			Some(token_uri)
		);
	}

	#[benchmark]
	fn precompile_update(
		t: Linear<0, { <T as Config>::MaxTokenUriLength::get() }>,
		u: Linear<0, { <T as Config>::MaxUniversalLocationLength::get() }>,
	) {
		let mut handle = MockHandle::new();

		let caller: T::AccountId = whitelisted_caller();
		let claimer = caller.clone();
		let universal_location: UniversalLocationOf<T> = vec![1u8; u as usize].try_into().unwrap();
		let token_uri: TokenUriOf<T> = vec![1u8; t as usize].try_into().unwrap();

		AssetMetadataExtender::<T>::create_token_uri_extension(
			claimer.clone(),
			universal_location.clone(),
			token_uri,
		)
		.unwrap();

		let new_token_uri: TokenUriOf<T> = vec![2u8; t as usize].try_into().unwrap();

		#[block]
		{
			AssetMetadataExtenderPrecompile::<T>::update(
				&mut handle,
				universal_location.clone().to_vec().into(),
				new_token_uri.clone().to_vec().into(),
			)
			.unwrap();
		};

		assert_eq!(
			AssetMetadataExtender::<T>::token_uris_by_claimer_and_location(
				claimer,
				universal_location
			),
			Some(new_token_uri)
		);
	}

	#[benchmark]
	fn precompile_balance_of(u: Linear<0, { <T as Config>::MaxUniversalLocationLength::get() }>) {
		let mut handle = MockHandle::new();
		let universal_location: UniversalLocationOf<T> = vec![1u8; u as usize].try_into().unwrap();

		#[block]
		{
			AssetMetadataExtenderPrecompile::<T>::balance_of(
				&mut handle,
				universal_location.clone().to_vec().into(),
			)
			.unwrap();
		};
	}

	#[benchmark]
	fn precompile_claimer_by_index(
		u: Linear<0, { <T as Config>::MaxUniversalLocationLength::get() }>,
	) {
		let mut handle = MockHandle::new();

		let caller: T::AccountId = whitelisted_caller();
		let claimer = caller.clone();
		let universal_location: UniversalLocationOf<T> = vec![1u8; u as usize].try_into().unwrap();
		let token_uri: TokenUriOf<T> = vec![1u8; 100usize].try_into().unwrap();

		AssetMetadataExtender::<T>::create_token_uri_extension(
			claimer.clone(),
			universal_location.clone(),
			token_uri,
		)
		.unwrap();

		#[block]
		{
			AssetMetadataExtenderPrecompile::<T>::claimer_by_index(
				&mut handle,
				universal_location.clone().to_vec().into(),
				0u32,
			)
			.unwrap();
		};
	}

	#[benchmark]
	fn precompile_extension_by_index(
		u: Linear<0, { <T as Config>::MaxUniversalLocationLength::get() }>,
	) {
		let mut handle = MockHandle::new();

		let caller: T::AccountId = whitelisted_caller();
		let claimer = caller.clone();
		let universal_location: UniversalLocationOf<T> = vec![1u8; u as usize].try_into().unwrap();
		let token_uri: TokenUriOf<T> = vec![1u8; 100usize].try_into().unwrap();

		{
			AssetMetadataExtender::<T>::create_token_uri_extension(
				claimer.clone(),
				universal_location.clone(),
				token_uri,
			)
			.unwrap();
		};

		#[block]
		{
			AssetMetadataExtenderPrecompile::<T>::extension_by_index(
				&mut handle,
				universal_location.clone().to_vec().into(),
				0u32,
			)
			.unwrap();
		};
	}

	#[benchmark]
	fn precompile_extension_by_location_and_claimer(
		u: Linear<0, { <T as Config>::MaxUniversalLocationLength::get() }>,
	) {
		let mut handle = MockHandle::new();

		let caller: T::AccountId = whitelisted_caller();
		let claimer = caller.clone();
		let universal_location: UniversalLocationOf<T> = vec![1u8; u as usize].try_into().unwrap();
		let token_uri: TokenUriOf<T> = vec![1u8; 100usize].try_into().unwrap();

		AssetMetadataExtender::<T>::create_token_uri_extension(
			claimer.clone(),
			universal_location.clone(),
			token_uri,
		)
		.unwrap();

		#[block]
		{
			AssetMetadataExtenderPrecompile::<T>::extension_by_location_and_claimer(
				&mut handle,
				universal_location.clone().to_vec().into(),
				Address(<T as Config>::AccountIdToH160::convert(claimer)),
			)
			.unwrap();
		};
	}

	#[benchmark]
	fn precompile_has_extension_by_claimer(
		u: Linear<0, { <T as Config>::MaxUniversalLocationLength::get() }>,
	) {
		let mut handle = MockHandle::new();
		let universal_location: UniversalLocationOf<T> = vec![1u8; u as usize].try_into().unwrap();
		let claimer = Address::from(H160::zero());

		#[block]
		{
			AssetMetadataExtenderPrecompile::<T>::has_extension_by_claimer(
				&mut handle,
				universal_location.clone().to_vec().into(),
				claimer,
			)
			.unwrap();
		};
	}

	#[benchmark]
	fn create_token_uri_extension(
		t: Linear<0, { <T as Config>::MaxTokenUriLength::get() }>,
		u: Linear<0, { <T as Config>::MaxUniversalLocationLength::get() }>,
	) {
		let claimer: T::AccountId = whitelisted_caller();

		let ul: UniversalLocationOf<T> = vec![1u8; u as usize].try_into().unwrap();
		let token_uri: TokenUriOf<T> = vec![1u8; t as usize].try_into().unwrap();

		#[block]
		{
			AssetMetadataExtender::<T>::create_token_uri_extension(
				claimer.clone(),
				ul.clone(),
				token_uri.clone(),
			)
			.unwrap();
		};

		assert_eq!(
			AssetMetadataExtender::<T>::token_uris_by_claimer_and_location(claimer, ul,),
			Some(token_uri)
		);
	}

	#[benchmark]
	fn update_token_uri_extension(
		t: Linear<0, { <T as Config>::MaxTokenUriLength::get() }>,
		u: Linear<0, { <T as Config>::MaxUniversalLocationLength::get() }>,
	) {
		let claimer: T::AccountId = whitelisted_caller();
		let universal_location: UniversalLocationOf<T> = vec![1u8; u as usize].try_into().unwrap();
		let token_uri: TokenUriOf<T> = vec![1u8; t as usize].try_into().unwrap();

		{
			AssetMetadataExtender::<T>::create_token_uri_extension(
				claimer.clone(),
				universal_location.clone(),
				token_uri,
			)
			.unwrap();
		};

		let new_token_uri: TokenUriOf<T> = vec![2u8; t as usize].try_into().unwrap();

		#[block]
		{
			AssetMetadataExtender::<T>::update_token_uri_extension(
				claimer.clone(),
				universal_location.clone(),
				new_token_uri.clone(),
			)
			.unwrap();
		};

		assert_eq!(
			AssetMetadataExtender::<T>::token_uris_by_claimer_and_location(
				claimer,
				universal_location
			),
			Some(new_token_uri)
		);
	}
}
