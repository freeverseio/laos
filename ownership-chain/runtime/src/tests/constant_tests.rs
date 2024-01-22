use core::str::FromStr;

use super::*;
use crate::{AccountId, Balances, Runtime, UNIT};
use fp_rpc::runtime_decl_for_ethereum_runtime_rpc_api::EthereumRuntimeRPCApiV5;
use frame_support::{
	assert_ok,
	traits::{
		tokens::{fungible::Balanced, Precision},
		Currency, WithdrawReasons,
	},
};
use sp_core::U256;

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
fn test_multisig_constants() {
	// 1 UNIT
	assert_eq!(<Runtime as pallet_multisig::Config>::DepositBase::get(), UNIT);
	// 0.1 UNIT
	assert_eq!(<Runtime as pallet_multisig::Config>::DepositFactor::get(), UNIT / 10);
	assert_eq!(<Runtime as pallet_multisig::Config>::MaxSignatories::get(), 20);
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
fn account_should_have_a_clief_of_24_blocks_and_vests_over_24x4_blocks_after_cliff() {
	new_test_ext().execute_with(|| {
		let alice = AccountId::from_str(ALICE).unwrap();
		let bob = AccountId::from_str(BOB).unwrap();
		let starting_block = 24_u32;
		let vesting_period = 24 * 4;
		let per_block = UNIT;
		let total_amount = vesting_period * per_block;
		let alice_origin = <Runtime as frame_system::Config>::RuntimeOrigin::signed(alice);
		let bob_origin = <Runtime as frame_system::Config>::RuntimeOrigin::signed(bob);

		assert!(Balances::deposit(&alice, total_amount, Precision::Exact).is_ok());
		assert_eq!(Balances::total_balance(&alice), total_amount);
		assert_eq!(Balances::total_balance(&bob), 0);

		let vesting_info = pallet_vesting::VestingInfo::new(total_amount, per_block, starting_block); 
		assert!(vesting_info.clone().is_valid());

		assert_ok!(pallet_vesting::Pallet::<Runtime>::vested_transfer(alice_origin, bob.clone(), vesting_info));

		for i in 24..=starting_block + vesting_period as u32 {
			frame_system::Pallet::<Runtime>::set_block_number(i);
			assert_ok!(pallet_vesting::Pallet::<Runtime>::vest(bob_origin.clone()));
			assert_eq!(Balances::usable_balance(&bob), (i - starting_block) as u128 * UNIT);
			assert_eq!(Balances::total_balance(&bob), total_amount);
		}
	});
} 
