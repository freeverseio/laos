use crate::{MILLIUNIT, UNIT};
use ownership_parachain_primitives::Balance;

mod block_rewards_source;
mod evm;
pub mod fee;
mod multisig;
pub mod parachain_staking;
mod proxy;

// Define storage fees as constants for clarity and reuse
const STORAGE_ITEM_FEE: Balance = 10 * UNIT;
const STORAGE_BYTE_FEE: Balance = 10 * MILLIUNIT;

/// Calculates the deposit required based on the number of items and bytes.
const fn calculate_deposit(items: u32, bytes: u32) -> Balance {
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
