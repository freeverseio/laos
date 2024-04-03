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

// This file is part of Astar.

// Copyright 2019-2022 PureStake Inc.
// Copyright (C) 2022-2023 Stake Technologies Pte.Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later
//
// This file is part of Utils package, originally developed by Purestake Inc.
// Utils package used in Astar Network in terms of GPLv3.
//
// Utils is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Utils is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Utils.  If not, see <http://www.gnu.org/licenses/>.

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::all)]

extern crate alloc;

use crate::alloc::borrow::ToOwned;
use fp_evm::{
	Context, ExitError, ExitSucceed, PrecompileFailure, PrecompileHandle, PrecompileOutput,
};
use frame_support::weights::Weight;
use precompile_utils::solidity::revert::revert;

use pallet_evm::{GasWeightMapping, Log};
use sp_core::{Get, H160, H256, U256};
use sp_runtime::DispatchError;
use sp_std::{vec, vec::Vec};

mod data;

pub use data::{Address, Bytes, EvmData, EvmDataReader, EvmDataWriter};
pub use laos_precompile_utils_macro::{generate_function_selector, keccak256};

#[cfg(feature = "testing")]
pub mod testing;
#[cfg(test)]
mod tests;

/// Alias for Result returning an EVM precompile error.
pub type EvmResult<T = ()> = Result<T, PrecompileFailure>;

/// Return an error with provided (static) text.
/// Using the `revert` function of `Gasometer` is preferred as erroring
/// consumed all the gas limit and the error message is not easily
/// retrievable.
pub fn error<T: Into<alloc::borrow::Cow<'static, str>>>(text: T) -> PrecompileFailure {
	PrecompileFailure::Error { exit_status: ExitError::Other(text.into()) }
}

/// Builder for PrecompileOutput.
#[derive(Clone, Debug)]
pub struct LogsBuilder {
	address: H160,
}

impl LogsBuilder {
	/// Create a new builder with no logs.
	/// Takes the address of the precompile (usually `context.address`).
	pub fn new(address: H160) -> Self {
		Self { address }
	}

	/// Create a 0-topic log.
	#[must_use]
	pub fn log0(&self, data: impl Into<Vec<u8>>) -> Log {
		Log { address: self.address, topics: vec![], data: data.into() }
	}

	/// Create a 1-topic log.
	#[must_use]
	pub fn log1(&self, topic0: impl Into<H256>, data: impl Into<Vec<u8>>) -> Log {
		Log { address: self.address, topics: vec![topic0.into()], data: data.into() }
	}

	/// Create a 2-topics log.
	#[must_use]
	pub fn log2(
		&self,
		topic0: impl Into<H256>,
		topic1: impl Into<H256>,
		data: impl Into<Vec<u8>>,
	) -> Log {
		Log { address: self.address, topics: vec![topic0.into(), topic1.into()], data: data.into() }
	}

	/// Create a 3-topics log.
	#[must_use]
	pub fn log3(
		&self,
		topic0: impl Into<H256>,
		topic1: impl Into<H256>,
		topic2: impl Into<H256>,
		data: impl Into<Vec<u8>>,
	) -> Log {
		Log {
			address: self.address,
			topics: vec![topic0.into(), topic1.into(), topic2.into()],
			data: data.into(),
		}
	}

	/// Create a 4-topics log.
	#[must_use]
	pub fn log4(
		&self,
		topic0: impl Into<H256>,
		topic1: impl Into<H256>,
		topic2: impl Into<H256>,
		topic3: impl Into<H256>,
		data: impl Into<Vec<u8>>,
	) -> Log {
		Log {
			address: self.address,
			topics: vec![topic0.into(), topic1.into(), topic2.into(), topic3.into()],
			data: data.into(),
		}
	}
}

/// Extension trait allowing to record logs into a PrecompileHandle.
pub trait LogExt {
	fn record(self, handle: &mut impl PrecompileHandle) -> EvmResult;

	fn compute_cost(&self) -> EvmResult<u64>;
}

impl LogExt for Log {
	fn record(self, handle: &mut impl PrecompileHandle) -> EvmResult {
		handle.record_log_costs(&[&self])?;
		handle.log(self.address, self.topics, self.data)?;
		Ok(())
	}

	fn compute_cost(&self) -> EvmResult<u64> {
		log_costs(self.topics.len(), self.data.len())
	}
}

/// Helper struct that requires `Runtime` to calculate `read` and `write` costs.
///
/// This struct is used to calculate the cost of a DB read or write in gas.
#[derive(Clone, Copy, Debug)]
pub struct GasCalculator<Runtime>(sp_std::marker::PhantomData<Runtime>);

impl<Runtime> GasCalculator<Runtime>
where
	Runtime: pallet_evm::Config + frame_system::Config,
{
	/// Cost of a Substrate DB write in gas.
	pub fn db_write_gas_cost(writes: u64) -> u64 {
		<Runtime as pallet_evm::Config>::GasWeightMapping::weight_to_gas(
			<Runtime as frame_system::Config>::DbWeight::get().writes(writes),
		)
	}

	/// Cost of a Substrate DB read in gas.
	pub fn db_read_gas_cost(reads: u64) -> u64 {
		<Runtime as pallet_evm::Config>::GasWeightMapping::weight_to_gas(
			<Runtime as frame_system::Config>::DbWeight::get().reads(reads),
		)
	}

	/// Convert weight to gas.
	pub fn weight_to_gas(weight: Weight) -> u64 {
		<Runtime as pallet_evm::Config>::GasWeightMapping::weight_to_gas(weight)
	}
}

/// Represents modifiers a Solidity function can be annotated with.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum FunctionModifier {
	/// Function that doesn't modify the state.
	View,
	/// Function that modifies the state but refuse receiving funds.
	/// Correspond to a Solidity function with no modifiers.
	NonPayable,
	/// Function that modifies the state and accept funds.
	Payable,
}

pub trait PrecompileHandleExt: PrecompileHandle {
	#[must_use]
	/// Record cost of a log manually.
	/// This can be useful to record log costs early when their content have static size.
	fn record_log_costs_manual(&mut self, topics: usize, data_len: usize) -> EvmResult;

	#[must_use]
	/// Record cost of logs.
	fn record_log_costs(&mut self, logs: &[&Log]) -> EvmResult;

	#[must_use]
	/// Check that a function call is compatible with the context it is
	/// called into.
	fn check_function_modifier(&self, modifier: FunctionModifier) -> EvmResult;

	#[must_use]
	/// Read the selector from the input data.
	fn read_selector<T>(&self) -> EvmResult<T>
	where
		T: num_enum::TryFromPrimitive<Primitive = u32>;

	#[must_use]
	/// Returns a reader of the input, skipping the selector.
	fn read_input(&self) -> EvmResult<EvmDataReader>;
}

pub fn log_costs(topics: usize, data_len: usize) -> EvmResult<u64> {
	// Cost calculation is copied from EVM code that is not publicly exposed by the crates.
	// https://github.com/rust-blockchain/evm/blob/master/gasometer/src/costs.rs#L148

	const G_LOG: u64 = 375;
	const G_LOGDATA: u64 = 8;
	const G_LOGTOPIC: u64 = 375;

	let topic_cost = G_LOGTOPIC
		.checked_mul(topics as u64)
		.ok_or(PrecompileFailure::Error { exit_status: ExitError::OutOfGas })?;

	let data_cost = G_LOGDATA
		.checked_mul(data_len as u64)
		.ok_or(PrecompileFailure::Error { exit_status: ExitError::OutOfGas })?;

	G_LOG
		.checked_add(topic_cost)
		.ok_or(PrecompileFailure::Error { exit_status: ExitError::OutOfGas })?
		.checked_add(data_cost)
		.ok_or(PrecompileFailure::Error { exit_status: ExitError::OutOfGas })
}

impl<T: PrecompileHandle> PrecompileHandleExt for T {
	#[must_use]
	/// Record cost of a log manualy.
	/// This can be useful to record log costs early when their content have static size.
	fn record_log_costs_manual(&mut self, topics: usize, data_len: usize) -> EvmResult {
		self.record_cost(log_costs(topics, data_len)?)?;

		Ok(())
	}

	#[must_use]
	/// Record cost of logs.
	fn record_log_costs(&mut self, logs: &[&Log]) -> EvmResult {
		for log in logs {
			self.record_log_costs_manual(log.topics.len(), log.data.len())?;
		}

		Ok(())
	}

	#[must_use]
	/// Check that a function call is compatible with the context it is
	/// called into.
	fn check_function_modifier(&self, modifier: FunctionModifier) -> EvmResult {
		check_function_modifier(self.context(), self.is_static(), modifier)
	}

	#[must_use]
	/// Read the selector from the input data.
	fn read_selector<S>(&self) -> EvmResult<S>
	where
		S: num_enum::TryFromPrimitive<Primitive = u32>,
	{
		EvmDataReader::read_selector(self.input())
	}

	#[must_use]
	/// Returns a reader of the input, skipping the selector.
	fn read_input(&self) -> EvmResult<EvmDataReader> {
		EvmDataReader::new_skip_selector(self.input())
	}
}

/// Reverts [DispatchError](sp_runtime::DispatchError) by converting it to a readable message.
#[must_use]
pub fn revert_dispatch_error(error: DispatchError) -> PrecompileFailure {
	match error {
		DispatchError::Arithmetic(_) => revert("arithmetic overflow/underflow"),
		DispatchError::BadOrigin => revert("bad origin"),
		DispatchError::CannotLookup => revert("cannot lookup"),
		DispatchError::Module(m) => revert(m.message.unwrap_or("unknown module error")),
		DispatchError::Other(msg) => revert(msg),
		_ => revert("unknown error"),
	}
}

#[must_use]
pub fn succeed(output: impl AsRef<[u8]>) -> PrecompileOutput {
	PrecompileOutput { exit_status: ExitSucceed::Returned, output: output.as_ref().to_owned() }
}

#[must_use]
/// Check that a function call is compatible with the context it is
/// called into.
fn check_function_modifier(
	context: &Context,
	is_static: bool,
	modifier: FunctionModifier,
) -> EvmResult {
	if is_static && modifier != FunctionModifier::View {
		return Err(revert("can't call non-static function in static context"))
	}

	if modifier != FunctionModifier::Payable && context.apparent_value > U256::zero() {
		return Err(revert("function is not payable"))
	}

	Ok(())
}
