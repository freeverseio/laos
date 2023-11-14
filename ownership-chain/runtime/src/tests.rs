use super::*;
use frame_support::assert_ok;
use sp_core::U256;

#[test]
fn test_block_and_gas_limit_constants() {
	use crate::Runtime;

	let system_block_weights = <Runtime as frame_system::Config>::BlockWeights::get();

	assert_ok!(system_block_weights.clone().validate());
	// 0.5s of block time
	assert_eq!(system_block_weights.max_block.ref_time(), 500_000_000_000);

	// EVM constants
	let block_gas_limit = <Runtime as pallet_evm::Config>::BlockGasLimit::get();

	// 15M gas
	assert_eq!(block_gas_limit, U256::from(15_000_000));
}

#[test]
fn test_multisig_constants() {
	// 1 UNIT
	assert_eq!(<Runtime as pallet_multisig::Config>::DepositBase::get(), UNIT);
	// 0.1 UNIT
	assert_eq!(<Runtime as pallet_multisig::Config>::DepositFactor::get(), UNIT / 10);
	assert_eq!(<Runtime as pallet_multisig::Config>::MaxSignatories::get(), 20);
}
