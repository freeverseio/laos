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

//! Runtime tests

mod metadata;
mod precompile_tests;
mod version_tests;

use core::{marker::PhantomData, str::FromStr};
use sp_runtime::BuildStorage;

use super::*;
use crate::{
	configs::collective::MaxMembersTechnicalCommittee, currency::UNIT, AccountId, Balances, Runtime,
};
use fp_rpc::runtime_decl_for_ethereum_runtime_rpc_api::EthereumRuntimeRPCApiV5;
use frame_support::{
	assert_ok,
	storage::bounded_vec::BoundedVec,
	traits::{
		tokens::{fungible::Balanced, Precision},
		Currency, WithdrawReasons,
	},
};
use sp_core::U256;

#[derive(Default)]
pub(crate) struct ExtBuilder {
	balances: Vec<(AccountId, u128)>,
	candidates: Vec<(AccountId, Balance)>,
}

impl ExtBuilder {
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
			.unwrap();

		let bob = AccountId::from_str(BOB).expect("This shouldn't fail");
		let charlie = AccountId::from_str(CHARLIE).expect("This shouldn't fail");

		let mut technical_committee_and_council_members =
			BoundedVec::with_bounded_capacity(MaxMembersTechnicalCommittee::get() as usize);

		technical_committee_and_council_members
			.try_push(bob)
			.expect("The technical committee bound is greater than 2 members;qed");

		technical_committee_and_council_members
			.try_push(charlie)
			.expect("The technical committee bound is greater than 2 members;qed");

		// get deduplicated list of all accounts and balances
		let all_accounts = self
			.balances
			.iter()
			.copied()
			.chain(self.candidates.iter().map(|(a, b)| (*a, b * 2)))
			.collect::<Vec<_>>();
		pallet_balances::GenesisConfig::<crate::Runtime> { balances: all_accounts }
			.assimilate_storage(&mut t)
			.unwrap();

		pallet_parachain_staking::GenesisConfig::<crate::Runtime> {
			candidates: self.candidates,
			blocks_per_round: 10,
			..Default::default()
		}
		.assimilate_storage(&mut t)
		.unwrap();

		pallet_membership::GenesisConfig::<crate::Runtime, pallet_membership::Instance2> {
			members: technical_committee_and_council_members.clone(),
			phantom: PhantomData,
		}
		.assimilate_storage(&mut t)
		.unwrap();

		pallet_collective::GenesisConfig::<crate::Runtime, pallet_membership::Instance1> {
			phantom: PhantomData,
			members: technical_committee_and_council_members.into_inner(),
		}
		.assimilate_storage(&mut t)
		.unwrap();

		t.into()
	}
}

pub(crate) fn new_test_ext() -> sp_io::TestExternalities {
	ExtBuilder::default().build()
}

pub(crate) const ALICE: &str = "0xf24FF3a9CF04c71Dbc94D0b566f7A27B94566cac";
pub(crate) const BOB: &str = "0x6c2b9c9b5007740e52d80dddb8e197b0c844f239";
pub(crate) const CHARLIE: &str = "0x798d4Ba9baf0064Ec19eB4F0a1a45785ae9D6DFc";

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
		assert_ok!(Vesting::vested_transfer(RuntimeOrigin::signed(alice), bob, vesting_info));

		assert_eq!(Balances::total_balance(&alice), 0);
		assert_eq!(Balances::total_balance(&bob), total_vested_amount);
		assert_eq!(Balances::usable_balance(bob), 0);

		// Simulate block progression and check Bob's balance each block
		for block_num in cliff_duration..=cliff_duration + vesting_duration as u32 {
			frame_system::Pallet::<Runtime>::set_block_number(block_num);
			assert_ok!(Vesting::vest(RuntimeOrigin::signed(bob)));
			let vested_amount = (block_num - cliff_duration) as u128 * amount_vested_per_block;
			assert_eq!(Balances::usable_balance(bob), vested_amount);
		}

		// Check that Bob's balance is now the total vested amount
		assert_eq!(Balances::usable_balance(bob), total_vested_amount);
	});
}
