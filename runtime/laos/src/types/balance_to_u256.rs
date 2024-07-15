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

use crate::Balance;
use sp_core::U256;
use sp_runtime::traits::Convert;

/// Converts [`Balance`] to [`U256`]
pub struct BalanceToU256;

impl Convert<Balance, U256> for BalanceToU256 {
	fn convert(b: Balance) -> U256 {
		U256::from(b)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use sp_core::U256;

	#[test]
	fn convert_balance_to_u256() {
		let balance: Balance = 1;
		let u = BalanceToU256::convert(balance);
		assert_eq!(u, U256::from(1));
	}
}
