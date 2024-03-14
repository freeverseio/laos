use crate::{
	weights::RocksDbWeight, AccountId, Balance, Block, BlockHashCount, PalletInfo, Runtime,
	RuntimeCall, RuntimeEvent, RuntimeOrigin, SS58Prefix, Version,
};
use frame_support::traits::Contains;
use sp_runtime::traits::{BlakeTwo256, IdentityLookup};

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
	type Hash = ownership_parachain_primitives::Hash;
	/// The type for storing how many extrinsics an account has signed.
	type Nonce = ownership_parachain_primitives::Nonce;
	/// The hashing algorithm used.
	type Hashing = BlakeTwo256;
	/// The ubiquitous event type.
	type RuntimeEvent = RuntimeEvent;
	/// The ubiquitous origin type.
	type RuntimeOrigin = RuntimeOrigin;
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
	type BaseCallFilter = BaseCallFilter;
	/// Weight information for the extrinsics of this pallet.
	type SystemWeightInfo = ();
	/// Block & extrinsics weights: base values and limits.
	type BlockWeights = ownership_parachain_primitives::BlockWeights;
	/// The maximum length of a block (in bytes).
	type BlockLength = ownership_parachain_primitives::BlockLength;
	/// This is used as an identifier of the chain. 42 is the generic substrate prefix.
	type SS58Prefix = SS58Prefix;
	/// The action to take on a Runtime Upgrade
	type OnSetCode = cumulus_pallet_parachain_system::ParachainSetCode<Self>;
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

pub struct BaseCallFilter;
impl Contains<RuntimeCall> for BaseCallFilter {
	fn contains(c: &RuntimeCall) -> bool {
		use pallet_balances::Call::*;
		use pallet_ethereum::Call::*;
		use pallet_evm::Call::*;
		use pallet_parachain_staking::Call::*;
		use pallet_vesting::Call::*;

		match c {
			// Transferability lock.
			RuntimeCall::Balances(inner_call) => match inner_call {
				transfer { .. } => false,
				transfer_all { .. } => false,
				transfer_keep_alive { .. } => false,
				transfer_allow_death { .. } => false,
				_ => true,
			},
			RuntimeCall::Vesting(inner_call) => match inner_call {
				// Vested transfes are not allowed.
				vested_transfer { .. } => false,
				_ => true,
			},
			RuntimeCall::ParachainStaking(inner_call) => match inner_call {
				// New candidates are not allowed.
				join_candidates { .. } => false,
				_ => true,
			},
			// Ethereum pallet calls are not allowed.
			RuntimeCall::Ethereum(inner_call) => match inner_call {
				_ => false,
			},
			// EVM pallet calls are not allowed.
			RuntimeCall::EVM(inner_call) => match inner_call {
				_ => false,
			},
			_ => true,
		}
	}
}

// tests
#[cfg(test)]
mod tests {
	use super::*;
	use crate::{
		tests::{new_test_ext, ALICE, BOB},
		Runtime,
	};
	use core::str::FromStr;
	use frame_support::assert_err;
	use precompile_utils::testing::Zero;
	use sp_core::{H160, U256};
	use sp_runtime::traits::Dispatchable;

	#[test]
	fn transfer_should_not_be_allowed() {
		new_test_ext().execute_with(|| {
			let from_account = AccountId::from_str(ALICE).unwrap();
			let to_account = AccountId::from_str(BOB).unwrap();
			let transfer_amount = 100;

			let call = RuntimeCall::Balances(pallet_balances::Call::transfer {
				dest: to_account.clone(),
				value: transfer_amount,
			});

			assert_err!(
				call.dispatch(RuntimeOrigin::signed(from_account)),
				frame_system::Error::<Runtime>::CallFiltered
			);
		});
	}

	#[test]
	fn transfer_all_should_not_be_allowed() {
		new_test_ext().execute_with(|| {
			let from_account = AccountId::from_str(ALICE).unwrap();
			let to_account = AccountId::from_str(BOB).unwrap();

			let call = RuntimeCall::Balances(pallet_balances::Call::transfer_all {
				dest: to_account.clone(),
				keep_alive: false,
			});

			assert_err!(
				call.dispatch(RuntimeOrigin::signed(from_account)),
				frame_system::Error::<Runtime>::CallFiltered
			);
		});
	}

	#[test]
	fn transfer_keep_alive_should_not_be_allowed() {
		new_test_ext().execute_with(|| {
			let from_account = AccountId::from_str(ALICE).unwrap();
			let to_account = AccountId::from_str(BOB).unwrap();
			let transfer_amount = 100;

			let call = RuntimeCall::Balances(pallet_balances::Call::transfer_keep_alive {
				dest: to_account.clone(),
				value: transfer_amount,
			});

			assert_err!(
				call.dispatch(RuntimeOrigin::signed(from_account)),
				frame_system::Error::<Runtime>::CallFiltered
			);
		});
	}

	#[test]
	fn transfer_allow_death_should_not_be_allowed() {
		new_test_ext().execute_with(|| {
			let from_account = AccountId::from_str(ALICE).unwrap();
			let to_account = AccountId::from_str(BOB).unwrap();
			let transfer_amount = 100;

			let call = RuntimeCall::Balances(pallet_balances::Call::transfer_allow_death {
				dest: to_account.clone(),
				value: transfer_amount,
			});

			assert_err!(
				call.dispatch(RuntimeOrigin::signed(from_account)),
				frame_system::Error::<Runtime>::CallFiltered
			);
		});
	}

	#[test]
	fn vested_transfer_should_not_be_allowed() {
		new_test_ext().execute_with(|| {
			let from_account = AccountId::from_str(ALICE).unwrap();
			let to_account = AccountId::from_str(BOB).unwrap();
			let transfer_amount = 1000;
			let per_block = 10;
			let starting_block = 100;

			let vesting_schedule =
				pallet_vesting::VestingInfo::new(transfer_amount, per_block, starting_block);

			let call = RuntimeCall::Vesting(pallet_vesting::Call::vested_transfer {
				target: to_account.clone(),
				schedule: vesting_schedule,
			});

			assert_err!(
				call.dispatch(RuntimeOrigin::signed(from_account)),
				frame_system::Error::<Runtime>::CallFiltered
			);
		});
	}

	#[test]
	fn join_candidates_should_not_be_allowed() {
		new_test_ext().execute_with(|| {
			let account = AccountId::from_str(ALICE).unwrap();
			let stake = 100_000;

			let call =
				RuntimeCall::ParachainStaking(pallet_parachain_staking::Call::join_candidates {
					bond: stake,
					candidate_count: 32,
				});

			assert_err!(
				call.dispatch(RuntimeOrigin::signed(account)),
				frame_system::Error::<Runtime>::CallFiltered
			);
		});
	}

	#[test]
	fn evm_create_should_not_be_allowed() {
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
				frame_system::Error::<Runtime>::CallFiltered
			);

			let call_2 = RuntimeCall::EVM(pallet_evm::Call::create2 {
				source: H160::from(account.0),
				init: vec![],
				salt: U256::zero(),
				gas_limit: 100_000,
				max_fee_per_gas: U256::from(100_000),
				max_priority_fee_per_gas: None,
				nonce: None,
				access_list: vec![],
				value: U256::zero(),
			});

			assert_err!(
				call_2.dispatch(RuntimeOrigin::signed(account)),
				frame_system::Error::<Runtime>::CallFiltered
			);
		});
	}

	#[test]
	fn evm_call_should_not_be_allowed() {
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
				frame_system::Error::<Runtime>::CallFiltered
			);
		});
	}

	#[test]
	fn evm_withdraw_should_not_be_allowed() {
		new_test_ext().execute_with(|| {
			let call =
				RuntimeCall::EVM(pallet_evm::Call::withdraw { address: H160([0x2; 20]), value: 0 });

			assert_err!(
				call.dispatch(RuntimeOrigin::signed(account)),
				frame_system::Error::<Runtime>::CallFiltered
			);
		});
	}
}
