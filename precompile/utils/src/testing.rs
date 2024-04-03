// Copyright 2023-2024 LAOS Chain Foundation
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

use super::*;
use fp_evm::{ExitReason, Transfer};

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
	pub fn new(input: Vec<u8>, gas_limit: Option<u64>, context: Context) -> Self {
		Self {
			input,
			gas_limit,
			context,
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

	fn record_external_cost(&mut self, _: Option<u64>, _: Option<u64>) -> Result<(), ExitError> {
		Ok(())
	}

	fn refund_external_cost(&mut self, _: Option<u64>, _: Option<u64>) {}

	fn log(&mut self, address: H160, topics: Vec<H256>, data: Vec<u8>) -> Result<(), ExitError> {
		let log = Log { address, topics, data };
		self.logs.push(log);
		Ok(())
	}

	fn remaining_gas(&self) -> u64 {
		unimplemented!()
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

/// Create a mock handle for testing precompiled contracts.
///
/// This function takes an input string representing the data to be sent to the precompiled contract
/// and a cost value, returning a `MockHandle` that can be used for testing.
///
/// # Arguments
///
/// * `input` - The input data as a hexadecimal string.
/// * `cost` - A cost value as u64.
/// * `value` - The amount of coins transferred as u64.
pub fn create_mock_handle(input: Vec<u8>, cost: u64, value: u64, caller: H160) -> MockHandle {
	let context: Context =
		Context { address: Default::default(), caller, apparent_value: From::from(value) };

	MockHandle::new(input, Some(cost), context)
}

/// Create a mock handle for testing precompiled contracts without a specific cost or value.
///
/// This function takes an input string representing the data to be sent to the precompiled contract
/// and returns a `MockHandle` that can be used for testing.
///
/// # Arguments
///
/// * `input` - The input data as a hexadecimal string.
pub fn create_mock_handle_from_input(input: Vec<u8>) -> MockHandle {
	create_mock_handle(input, 0, 0, H160::zero())
}
