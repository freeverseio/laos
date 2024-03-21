use laos_primitives::Balance;

// Unit = the base number of indivisible units for balances
// 18 decimals
pub(crate) const UNIT: Balance = 1_000_000_000_000_000_000;
pub(crate) const MILLIUNIT: Balance = UNIT / 1000;

// Constants in ETH terms
const MEGAWEI: Balance = 1_000_000;
const GIGAWEI: Balance = 1_000_000_000;

const STORAGE_ITEM_FEE: Balance = 10 * UNIT;
const STORAGE_BYTE_FEE: Balance = 10 * MILLIUNIT;
pub(crate) const TRANSACTION_BYTE_FEE: Balance = 100 * GIGAWEI;
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
