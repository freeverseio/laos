//! Runtime tests

#![cfg(test)]
mod precompile_tests;
mod version_tests;
mod xcm_mock;
mod xcm_tests;

pub use xcm_mock::ParachainXcmRouter;

use sp_runtime::{traits::SignedExtension, BuildStorage};

use core::str::FromStr;

use super::*;
use crate::{AccountId, Balances, Runtime, UNIT};
use fp_rpc::runtime_decl_for_ethereum_runtime_rpc_api::EthereumRuntimeRPCApiV5;
use frame_support::{
	assert_ok,
	dispatch::GetDispatchInfo,
	traits::{
		tokens::{fungible::Balanced, Precision},
		Currency, UnfilteredDispatchable,
	},
};
use pallet_transaction_payment::ChargeTransactionPayment;
use sp_core::U256;

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::<crate::Runtime>::default()
		.build_storage()
		.unwrap()
		.into();

	pallet_balances::GenesisConfig::<crate::Runtime> {
		balances: vec![
			([0u8; 20].into(), 1_000_000_000_000_000_000_000u128),
			([1u8; 20].into(), 1_000_000_000_000_000_000_000u128),
		],
	}
	.assimilate_storage(&mut t)
	.unwrap();

	pallet_sudo::GenesisConfig::<crate::Runtime> { key: Some(AccountId::from_str(BOB).unwrap()) }
		.assimilate_storage(&mut t)
		.unwrap();

	t.into()
}

const ALICE: &str = "0xf24FF3a9CF04c71Dbc94D0b566f7A27B94566cac";
const BOB: &str = "0x6c2b9c9b5007740e52d80dddb8e197b0c844f239";

#[test]
fn minimum_balance_should_be_0() {
	assert_eq!(Balances::minimum_balance(), 0);
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
fn send_1_minimum_unit_to_wallet_with_0_wei_balance_should_increase_balance_by_1_wei() {
	new_test_ext().execute_with(|| {
		let alice = AccountId::from_str(ALICE).unwrap();
		assert_eq!(Runtime::account_basic(alice.into()).balance, 0.into());

		let minimum_amount = 1;
		assert!(Balances::deposit(&alice, minimum_amount, Precision::Exact).is_ok());
		assert_eq!(Balances::total_balance(&alice), minimum_amount);

		assert_eq!(Runtime::account_basic(alice.into()).balance, 1.into());
	})
}

#[test]
fn check_pallet_vesting_configuration() {
	assert_eq!(<Runtime as pallet_vesting::Config>::MinVestedTransfer::get(), UNIT);
	assert_eq!(
		<Runtime as pallet_vesting::Config>::UnvestedFundsAllowedWithdrawReasons::get(),
		WithdrawReasons::except(WithdrawReasons::TRANSFER | WithdrawReasons::RESERVE)
	);
	assert_eq!(<Runtime as pallet_vesting::Config>::MAX_VESTING_SCHEDULES, 28);
}

#[test]
fn account_vests_correctly_over_time() {
	new_test_ext().execute_with(|| {
		let alice = AccountId::from_str(ALICE).unwrap();
		let bob = AccountId::from_str(BOB).unwrap();
		let cliff_duration = 24_u32;
		let vesting_duration = (cliff_duration * 4) as u128;
		let amount_vested_per_block = UNIT;
		let total_vested_amount = vesting_duration * amount_vested_per_block;

		// Deposit the total vested amount to Alice's account and validate balances
		assert!(Balances::deposit(&alice, total_vested_amount, Precision::Exact).is_ok());
		assert_eq!(Balances::total_balance(&alice), total_vested_amount);
		assert_eq!(Balances::total_balance(&bob), 0);

		// Create a vesting schedule for Bob
		let vesting_info = pallet_vesting::VestingInfo::new(
			total_vested_amount,
			amount_vested_per_block,
			cliff_duration,
		);
		assert!(vesting_info.is_valid());

		// Transfer vested funds from Alice to Bob
		assert_ok!(Vesting::vested_transfer(
			RuntimeOrigin::signed(alice),
			bob.clone(),
			vesting_info
		));

		assert_eq!(Balances::total_balance(&alice), 0);
		assert_eq!(Balances::total_balance(&bob), total_vested_amount);
		assert_eq!(Balances::usable_balance(&bob), 0);

		// Simulate block progression and check Bob's balance each block
		for block_num in cliff_duration..=cliff_duration + vesting_duration as u32 {
			frame_system::Pallet::<Runtime>::set_block_number(block_num);
			assert_ok!(Vesting::vest(RuntimeOrigin::signed(bob.clone())));
			let vested_amount = (block_num - cliff_duration) as u128 * amount_vested_per_block;
			assert_eq!(Balances::usable_balance(&bob), vested_amount);
		}

		// Check that Bob's balance is now the total vested amount
		assert_eq!(Balances::usable_balance(&bob), total_vested_amount);
	});
}

