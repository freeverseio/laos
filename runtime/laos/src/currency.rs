use laos_primitives::Balance;

// Unit = the base number of indivisible units for balances
// 18 decimals
pub(crate) const UNIT: Balance = 1_000_000_000_000_000_000;
pub(crate) const MILLIUNIT: Balance = UNIT / 1000;

// Constants in ETH terms
pub(crate) const WEI: Balance = 1;
pub(crate) const KILOWEI: Balance = 1_000 * WEI;
pub(crate) const GIGAWEI: Balance = 1_000_000_000 * WEI;

// Define storage fees as constants for clarity and reuse
const STORAGE_ITEM_FEE: Balance = 10 * UNIT;
const STORAGE_BYTE_FEE: Balance = 10 * MILLIUNIT;

/// One byte of transaction data has a fee of 1/1000 of a micro unit.
pub(crate) const TRANSACTION_BYTE_FEE: Balance = 1 * GIGAWEI;
/// Weight to fee conversion factor.
pub(crate) const WEIGHT_TO_FEE: u128 = 50 * KILOWEI;

/// Calculates the deposit required based on the number of items and bytes.
pub(crate) const fn calculate_deposit(items: u32, bytes: u32) -> Balance {
	(items as Balance) * STORAGE_ITEM_FEE + (bytes as Balance) * STORAGE_BYTE_FEE
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
