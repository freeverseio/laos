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

use crate::{
	weights::RocksDbWeight, AccountId, Balance, Block, PalletInfo, Runtime, RuntimeCall,
	RuntimeEvent, RuntimeOrigin, RuntimeTask, RuntimeVersion, VERSION,
};
use frame_support::{parameter_types, traits::Everything};
use sp_runtime::traits::{BlakeTwo256, IdentityLookup};

parameter_types! {
	pub const Version: RuntimeVersion = VERSION;
	pub const SS58Prefix: u16 = 42;
	pub const BlockHashCount: u32 = 256;
}

impl frame_system::Config for Runtime {
	/// The identifier used to distinguish between accounts.
	type AccountId = AccountId;
	/// The aggregated dispatch type that is available for extrinsics.
	type RuntimeCall = RuntimeCall;
	/// The lookup mechanism to get account ID from whatever is passed in dispatchers.
	type Lookup = IdentityLookup<AccountId>;
	/// The block type
	type Block = Block;
	/// The type for hashing blocks and tries.
	type Hash = laos_primitives::Hash;
	/// The type for storing how many extrinsics an account has signed.
	type Nonce = laos_primitives::Nonce;
	/// The hashing algorithm used.
	type Hashing = BlakeTwo256;
	/// The ubiquitous event type.
	type RuntimeEvent = RuntimeEvent;
	/// The ubiquitous origin type.
	type RuntimeOrigin = RuntimeOrigin;
	/// The aggregated RuntimeTask type.
	type RuntimeTask = RuntimeTask;
	/// Maximum number of block number to block hash mappings to keep (oldest pruned first).
	type BlockHashCount = BlockHashCount;
	/// Runtime version.
	type Version = Version;
	/// Converts a module to an index of this module in the runtime.
	type PalletInfo = PalletInfo;
	/// The data to be stored in an account.
	type AccountData = pallet_balances::AccountData<Balance>;
	/// What to do if a new account is created.
	type OnNewAccount = ();
	/// What to do if an account is fully reaped from the system.
	type OnKilledAccount = ();
	/// The weight of database operations that the runtime can invoke.
	type DbWeight = RocksDbWeight;
	/// The basic call filter to use in dispatchable.
	type BaseCallFilter = Everything;
	/// Weight information for the extrinsics of this pallet.
	type SystemWeightInfo = ();
	/// Block & extrinsics weights: base values and limits.
	type BlockWeights = laos_primitives::BlockWeights;
	/// The maximum length of a block (in bytes).
	type BlockLength = laos_primitives::BlockLength;
	/// This is used as an identifier of the chain. 42 is the generic substrate prefix.
	type SS58Prefix = SS58Prefix;
	/// The action to take on a Runtime Upgrade
	type OnSetCode = cumulus_pallet_parachain_system::ParachainSetCode<Self>;
	type MaxConsumers = frame_support::traits::ConstU32<16>;
	type SingleBlockMigrations = ();
	type MultiBlockMigrator = ();
	type PreInherents = ();
	type PostInherents = ();
	type PostTransactions = ();
}

// tests
#[cfg(test)]
mod tests {
	use super::*;
	use crate::{
		currency::UNIT,
		tests::{new_test_ext, ExtBuilder, ALICE, BOB},
	};
	use core::str::FromStr;
	use frame_support::{assert_err, assert_ok, dispatch::PostDispatchInfo, pallet_prelude::Pays};
	use pallet_ethereum::Transaction;
	use sp_core::{H160, H256, U256};
	use sp_runtime::{
		traits::Dispatchable,
		DispatchError::{BadOrigin, Other},
	};

	#[test]
	fn transfer_should_be_allowed() {
		let alice = AccountId::from_str(ALICE).unwrap();

		ExtBuilder::default()
			.with_balances(vec![(alice, 1000 * UNIT)])
			.build()
			.execute_with(|| {
				let to_account = AccountId::from_str(BOB).unwrap();
				let transfer_amount = 100;
				let call = RuntimeCall::Balances(pallet_balances::Call::transfer_keep_alive {
					dest: to_account,
					value: transfer_amount,
				});
				assert_ok!(call.dispatch(RuntimeOrigin::signed(alice)));
			});
	}

	#[test]
	fn transfer_all_should_be_allowed() {
		let alice = AccountId::from_str(ALICE).unwrap();

		ExtBuilder::default()
			.with_balances(vec![(alice, 1000 * UNIT)])
			.build()
			.execute_with(|| {
				let to_account = AccountId::from_str(BOB).unwrap();
				let call = RuntimeCall::Balances(pallet_balances::Call::transfer_all {
					dest: to_account,
					keep_alive: false,
				});
				assert_ok!(call.dispatch(RuntimeOrigin::signed(alice)));
			});
	}

	#[test]
	fn transfer_keep_alive_should_be_allowed() {
		let alice = AccountId::from_str(ALICE).unwrap();

		ExtBuilder::default()
			.with_balances(vec![(alice, 1000 * UNIT)])
			.build()
			.execute_with(|| {
				let to_account = AccountId::from_str(BOB).unwrap();
				let transfer_amount = 1000000000000000000;

				let call = RuntimeCall::Balances(pallet_balances::Call::transfer_keep_alive {
					dest: to_account,
					value: transfer_amount,
				});
				assert_ok!(call.dispatch(RuntimeOrigin::signed(alice)));
			});
	}

	#[test]
	fn transfer_allow_death_should_be_allowed() {
		let alice = AccountId::from_str(ALICE).unwrap();

		ExtBuilder::default()
			.with_balances(vec![(alice, 1000 * UNIT)])
			.build()
			.execute_with(|| {
				let to_account = AccountId::from_str(BOB).unwrap();
				let transfer_amount = 100;

				let call = RuntimeCall::Balances(pallet_balances::Call::transfer_allow_death {
					dest: to_account,
					value: transfer_amount,
				});
				assert_ok!(call.dispatch(RuntimeOrigin::signed(alice)));
			});
	}

	#[test]
	fn vested_transfer_should_be_allowed() {
		let alice = AccountId::from_str(ALICE).unwrap();

		ExtBuilder::default()
			.with_balances(vec![(alice, 100000 * UNIT)])
			.build()
			.execute_with(|| {
				let to_account = AccountId::from_str(BOB).unwrap();
				let transfer_amount = 1000000000000000000;
				let per_block = 10;
				let starting_block = 100;

				let vesting_schedule =
					pallet_vesting::VestingInfo::new(transfer_amount, per_block, starting_block);

				let call = RuntimeCall::Vesting(pallet_vesting::Call::vested_transfer {
					target: to_account,
					schedule: vesting_schedule,
				});
				assert_ok!(call.dispatch(RuntimeOrigin::signed(alice)));
			});
	}

	#[test]
	fn join_candidates_should_be_allowed() {
		new_test_ext().execute_with(|| {
			let account = AccountId::from_str(ALICE).unwrap();
			let stake = 20_000 * UNIT;

			assert_ok!(pallet_balances::Pallet::<Runtime>::force_set_balance(
				RuntimeOrigin::root(),
				account,
				stake
			));

			let call =
				RuntimeCall::ParachainStaking(pallet_parachain_staking::Call::join_candidates {
					bond: stake,
					candidate_count: 32,
				});

			assert_ok!(call.dispatch(RuntimeOrigin::signed(account)));
		});
	}

	#[test]
	fn evm_create_should_be_allowed() {
		new_test_ext().execute_with(|| {
			let account = AccountId::from_str(ALICE).unwrap();

			let call = RuntimeCall::EVM(pallet_evm::Call::create {
				source: H160::from(account.0),
				init: vec![],
				gas_limit: 100_000,
				max_fee_per_gas: U256::from(100_000),
				max_priority_fee_per_gas: None,
				nonce: None,
				access_list: vec![],
				value: U256::zero(),
			});

			assert_err!(
				call.dispatch(RuntimeOrigin::signed(account)),
				sp_runtime::DispatchErrorWithPostInfo {
					post_info: PostDispatchInfo { actual_weight: None, pays_fee: Pays::Yes },
					error: BadOrigin,
				}
			);

			let call_2 = RuntimeCall::EVM(pallet_evm::Call::create2 {
				source: H160::from(account.0),
				init: vec![],
				salt: H256::zero(),
				gas_limit: 100_000,
				max_fee_per_gas: U256::from(100_000),
				max_priority_fee_per_gas: None,
				nonce: None,
				access_list: vec![],
				value: U256::zero(),
			});

			assert_err!(
				call_2.dispatch(RuntimeOrigin::signed(account)),
				sp_runtime::DispatchErrorWithPostInfo {
					post_info: PostDispatchInfo { actual_weight: None, pays_fee: Pays::Yes },
					error: BadOrigin,
				}
			);
		});
	}

	#[test]
	fn evm_call_should_be_allowed() {
		new_test_ext().execute_with(|| {
			let account = AccountId::from_str(ALICE).unwrap();

			let call = RuntimeCall::EVM(pallet_evm::Call::call {
				source: H160::from(account.0),
				target: H160([0x2; 20]),
				value: U256::zero(),
				input: vec![],
				gas_limit: 100_000,
				max_fee_per_gas: U256::from(100_000),
				max_priority_fee_per_gas: None,
				nonce: None,
				access_list: vec![],
			});

			assert_err!(
				call.dispatch(RuntimeOrigin::signed(account)),
				sp_runtime::DispatchErrorWithPostInfo {
					post_info: PostDispatchInfo { actual_weight: None, pays_fee: Pays::Yes },
					error: BadOrigin,
				}
			);
		});
	}

	#[test]
	fn evm_withdraw_should_be_allowed() {
		new_test_ext().execute_with(|| {
			let call =
				RuntimeCall::EVM(pallet_evm::Call::withdraw { address: H160([0x2; 20]), value: 0 });
			let account = AccountId::from_str(ALICE).unwrap();

			assert_err!(
				call.dispatch(RuntimeOrigin::signed(account)),
				sp_runtime::DispatchErrorWithPostInfo {
					post_info: PostDispatchInfo { actual_weight: None, pays_fee: Pays::Yes },
					error: BadOrigin,
				}
			);
		});
	}

	#[test]
	fn ethereum_transact_should_be_allowed() {
		new_test_ext().execute_with(|| {
			let call = RuntimeCall::Ethereum(pallet_ethereum::Call::transact {
				transaction: Transaction::Legacy(ethereum::LegacyTransaction {
					nonce: U256::zero(),
					gas_price: U256::zero(),
					gas_limit: U256::from(100_000),
					action: ethereum::TransactionAction::Call(H160::zero()),
					value: U256::zero(),
					input: vec![],
					signature: ethereum::TransactionSignature::new(
						123,
						H256::from_low_u64_be(1),
						H256::from_low_u64_be(2),
					)
					.unwrap(),
				}),
			});

			let account = AccountId::from_str(ALICE).unwrap();

			assert_err!(
				call.dispatch(RuntimeOrigin::signed(account)),
				sp_runtime::DispatchErrorWithPostInfo {
					post_info: PostDispatchInfo { actual_weight: None, pays_fee: Pays::Yes },
					error: Other("bad origin: expected to be an Ethereum transaction"),
				}
			);
		});
	}
}
