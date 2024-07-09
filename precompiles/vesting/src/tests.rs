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

//! Living assets precompile tests.

use super::*;
use mock::*;
use precompile_utils::testing::{Alice, Precompile1, PrecompileTesterExt};

/// Get precompiles from the mock.
fn precompiles() -> LaosPrecompiles<Test> {
	PrecompilesInstance::get()
}

#[test]
fn selectors() {
	assert!(PrecompileCall::vest_selectors().contains(&0x458EFDE3));
	assert!(PrecompileCall::vest_other_selectors().contains(&0x55E60C8));
	assert!(PrecompileCall::vesting_selectors().contains(&0xE388C423));
}

#[test]
fn vest_reverts_no_vested_funds() {
	new_test_ext().execute_with(|| {
		precompiles()
			.prepare_test(Alice, Precompile1, PrecompileCall::vest {})
			.execute_reverts(|r| r == b"NotVesting");
	});
}
