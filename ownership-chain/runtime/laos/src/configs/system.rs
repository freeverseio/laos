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
				// Staking is not allowed.
				join_candidates { .. } => false,
				_ => true,
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
	use frame_support::{
		assert_noop, assert_ok,
		traits::{fungible::Balanced, tokens::Precision},
	};
	use sp_runtime::{traits::Dispatchable, DispatchErrorWithPostInfo, DispatchError};

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

			
			let result = call.dispatch(RuntimeOrigin::signed(from_account));

			assert!(result.is_err());

			match result.unwrap_err().error {
				DispatchError::Module(module_error) => {
					assert_eq!(module_error.message, Some("CallFiltered"));
				},
				_ => panic!("Expected ModuleError with message 'CallFiltered'."),
			}
		});
	}
}
