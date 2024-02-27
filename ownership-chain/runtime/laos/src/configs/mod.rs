use crate::{MILLIUNIT, UNIT};
use ownership_parachain_primitives::Balance;

mod multisig;
mod proxy;

// Define storage fees as constants for clarity and reuse
const STORAGE_ITEM_FEE: Balance = 10 * UNIT;
const STORAGE_BYTE_FEE: Balance = 10 * MILLIUNIT;

/// Calculates the deposit required based on the number of items and bytes.
const fn calculate_deposit(items: u32, bytes: u32) -> Balance {
	(items as Balance) * STORAGE_ITEM_FEE + (bytes as Balance) * STORAGE_BYTE_FEE
}
