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

use laos_primitives::Balance;

// Unit = the base number of indivisible units for balances
// 18 decimals
pub const DECIMALS: u32 = 18;
pub const UNIT: Balance = (10 as Balance).pow(DECIMALS);
pub(crate) const MILLIUNIT: Balance = UNIT / 1000;

// Constants in ETH terms
const MEGAWEI: Balance = 1_000_000;
const GIGAWEI: Balance = 1_000_000_000;

const STORAGE_ITEM_FEE: Balance = 10 * UNIT;
const STORAGE_BYTE_FEE: Balance = 10 * MILLIUNIT;

/// One byte of transaction data has a fee of 100 GigaWei.
pub(crate) const TRANSACTION_BYTE_FEE: Balance = 100 * GIGAWEI;
/// Weight to fee conversion factor.
pub(crate) const WEIGHT_TO_FEE: Balance = 5 * MEGAWEI;

pub(crate) const fn calculate_deposit(items: u32, bytes: u32) -> Balance {
	items as Balance * STORAGE_ITEM_FEE + (bytes as Balance) * STORAGE_BYTE_FEE
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn check_calculate_deposits() {
		assert_eq!(calculate_deposit(0, 0), 0);
		assert_eq!(calculate_deposit(0, 1), 10 * MILLIUNIT);
		assert_eq!(calculate_deposit(1, 0), 10 * UNIT);
		assert_eq!(calculate_deposit(1, 1), 10 * UNIT + 10 * MILLIUNIT);
		assert_eq!(calculate_deposit(1, 2), 10 * UNIT + 20 * MILLIUNIT);
		assert_eq!(calculate_deposit(2, 2), 20 * UNIT + 20 * MILLIUNIT);
	}
}
