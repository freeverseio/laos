//! Runtime tests

#![cfg(test)]
mod precompile_tests;
mod version_tests;
mod xcm_mock;
mod xcm_tests;

use pallet_parachain_staking::{InflationInfo, Range};
pub use xcm_mock::ParachainXcmRouter;

use sp_runtime::BuildStorage;

use core::str::FromStr;

use super::*;
use crate::{
	configs::parachain_staking::{MinCandidateStk, MinDelegation},
	AccountId, Balances, Runtime, UNIT,
};
use fp_rpc::runtime_decl_for_ethereum_runtime_rpc_api::EthereumRuntimeRPCApiV5;
use frame_support::{
	assert_ok,
	traits::{
		tokens::{fungible::Balanced, Precision},
		Currency,
	},
};
use sp_core::U256;
use test_utils::{assert_events_eq, roll_one_block};

#[derive(Default)]
pub(crate) struct ExtBuilder {
	rewards_account: Option<AccountId>,
	balances: Vec<(AccountId, u128)>,
	candidates: Vec<(AccountId, Balance)>,
}

impl ExtBuilder {
	pub(crate) fn with_rewards_account(mut self, account: AccountId) -> Self {
		self.rewards_account = Some(account);
		self
	}

	pub(crate) fn with_balances(mut self, balances: Vec<(AccountId, u128)>) -> Self {
		self.balances = balances;
		self
	}

	pub fn with_candidates(mut self, candidates: Vec<(AccountId, Balance)>) -> Self {
		self.candidates = candidates;
		self
	}

	// Build genesis storage according to the mock runtime.
	pub(crate) fn build(self) -> sp_io::TestExternalities {
		let mut t = frame_system::GenesisConfig::<crate::Runtime>::default()
			.build_storage()
			.unwrap()
			.into();

		// get deduplicated list of all accounts and balances
		let all_accounts = self
			.balances
			.iter()
			.map(|a| a.clone())
			.chain(self.candidates.iter().map(|(a, b)| (a.clone(), b * 2)))
			.collect::<Vec<_>>();

		pallet_balances::GenesisConfig::<crate::Runtime> { balances: all_accounts }
			.assimilate_storage(&mut t)
			.unwrap();

		pallet_sudo::GenesisConfig::<crate::Runtime> {
			key: Some(AccountId::from_str(BOB).unwrap()),
		}
		.assimilate_storage(&mut t)
		.unwrap();

		pallet_parachain_staking::GenesisConfig::<crate::Runtime> {
			candidates: self.candidates,
			blocks_per_round: 10,
			inflation_config: InflationInfo {
				expect: Range { min: 100_000, ideal: 200_000, max: 500_000 },
				annual: Range {
					min: Perbill::from_percent(50),
					ideal: Perbill::from_percent(50),
					max: Perbill::from_percent(50),
				},
				round: Range {
					min: Perbill::from_percent(5),
					ideal: Perbill::from_percent(5),
					max: Perbill::from_percent(5),
				},
			},
			..Default::default()
		}
		.assimilate_storage(&mut t)
		.unwrap();

		pallet_block_rewards_handler::GenesisConfig::<crate::Runtime> {
			rewards_account: self.rewards_account,
		}
		.assimilate_storage(&mut t)
		.unwrap();

		t.into()
	}
}

pub(crate) fn new_test_ext() -> sp_io::TestExternalities {
	ExtBuilder::default().build()
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
	ExtBuilder::default().build().execute_with(|| {
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
	ExtBuilder::default().build().execute_with(|| {
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

const A: [u8; 20] = [33; 20];
const B: [u8; 20] = [34; 20];
const C: [u8; 20] = [35; 20];
const D: [u8; 20] = [36; 20];
const E: [u8; 20] = [37; 20];

#[test]
// set rewards account with funds
fn payout_distribution_to_solo_collators() {
	let min_staked = MinCandidateStk::get();
	ExtBuilder::default()
		.with_candidates(vec![
			(B.into(), 90 + min_staked),
			(A.into(), 100 + min_staked),
			(C.into(), 80 + min_staked),
			(D.into(), 70 + min_staked),
		])
		.with_rewards_account(E.into())
		.with_balances(vec![(E.into(), 80_000_000_000_000_000_000_000)])
		.build()
		.execute_with(|| {
			roll_to_round_begin(2);

			let b_initial_balance = (90 + min_staked) * 2;
			assert_eq!(Balances::free_balance(&B.into()), b_initial_balance);

			// ~ set block author as 1 for all blocks this round
			// same as set_author(2, 1, 100);
			pallet_parachain_staking::Points::<Runtime>::mutate(2, |p| *p += 100);
			pallet_parachain_staking::AwardedPts::<Runtime>::mutate(2, AccountId::from(B), |p| {
				*p += 100
			});
			roll_to_round_begin(4);

			// pay total issuance to 1 at 2nd block
			// same as roll_blocks(3);
			for _ in 0..3 {
				roll_one_block!(true);
			}
			System::block_number();

			assert!(Balances::free_balance(&B.into()) > b_initial_balance);
		});
}

const GENESIS_BLOCKS_PER_ROUND: BlockNumber = 10;
/// Rolls block-by-block to the beginning of the specified round.
/// This will complete the block in which the round change occurs.
/// Returns the number of blocks played.
pub(crate) fn roll_to_round_begin(round: BlockNumber) -> BlockNumber {
	let block = (round - 1) * GENESIS_BLOCKS_PER_ROUND;
	roll_to(block)
}

/// Rolls to the desired block. Returns the number of blocks played.
pub(crate) fn roll_to(n: BlockNumber) -> BlockNumber {
	let mut num_blocks = 0;
	let mut block = System::block_number();
	while block < n {
		roll_one_block!(true);
		block = System::block_number();
		num_blocks += 1;
	}
	num_blocks
}
