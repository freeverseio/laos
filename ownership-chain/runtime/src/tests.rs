use super::*;
use fp_rpc::runtime_decl_for_ethereum_runtime_rpc_api::EthereumRuntimeRPCApiV5;
use frame_support::{
	assert_ok,
	traits::tokens::{fungible::Balanced, Precision},
};
use sp_core::U256;
use std::str::FromStr;

const ALICE: &str = "0xf24FF3a9CF04c71Dbc94D0b566f7A27B94566cac";

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	frame_system::GenesisConfig::<Runtime>::default()
		.build_storage()
		.unwrap()
		.into()
}

#[test]
fn minimum_balance_should_be_0() {
	assert_eq!(Balances::minimum_balance(), 0);
}

#[test]
fn deposit_smallest_unit_should_succeed() {
	new_test_ext().execute_with(|| {
		let alice = AccountId::from_str(ALICE).unwrap();
		assert_eq!(Balances::total_balance(&alice), 0);

		let minimum_amount = 1;
		assert!(Balances::deposit(&alice, minimum_amount, Precision::Exact).is_ok());
		assert_eq!(Balances::total_balance(&alice), minimum_amount);
	})
}

#[test]
fn test_block_and_gas_limit_constants() {
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

#[test]
fn send_1_wei_to_wallet_with_0_balance_should_increase_balance_by_1_wei() {
	new_test_ext().execute_with(|| {
		let alice = AccountId::from_str(ALICE).unwrap();
		assert_eq!(Runtime::account_basic(alice.into()).balance, 0.into());

		let minimum_amount = 1;
		assert!(Balances::deposit(&alice, minimum_amount, Precision::Exact).is_ok());
		assert_eq!(Balances::total_balance(&alice), minimum_amount);

		assert_eq!(Runtime::account_basic(alice.into()).balance, 1.into());
	})
}
