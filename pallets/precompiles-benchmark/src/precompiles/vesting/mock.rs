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

use core::str::FromStr;
use sp_runtime::BuildStorage;

use precompile_utils::precompile_set::{AddressU64, PrecompileAt, PrecompileSetBuilder};

use crate::{
	pallet,
	precompiles::vesting::{VestingPrecompile, VestingPrecompileCall},
};
use frame_support::{
	derive_impl, parameter_types,
	traits::{FindAuthor, OnFinalize, OnInitialize, WithdrawReasons},
	weights::constants::RocksDbWeight,
};
use frame_system::pallet_prelude::BlockNumberFor;
use sp_core::{H160, U256};
use sp_runtime::{
	traits::{ConvertInto, IdentityLookup},
	ConsensusEngineId,
};
pub use test_utils::*;

type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test {
		System: frame_system::{Pallet, Call, Config<T>, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		Vesting: pallet_vesting,
		VestingWrapper: pallet,
		Timestamp: pallet_timestamp::{Pallet, Call, Storage},
		EVM: pallet_evm::{Pallet, Call, Storage, Config<T>, Event<T>},
	}
);

pub type AccountId = H160;
type Balance = u128;

#[derive_impl(frame_system::config_preludes::TestDefaultConfig as frame_system::DefaultConfig)]
impl frame_system::Config for Test {
	type Block = Block;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type AccountData = pallet_balances::AccountData<Balance>;
	type DbWeight = RocksDbWeight;
}

parameter_types! {
	pub const MaxTokenUriLength: u32 = 512;
}

#[derive_impl(pallet::config_preludes::TestDefaultConfig as pallet::DefaultConfig)]
impl pallet::Config for Test {
	type GasWeightMapping = pallet_evm::FixedGasWeightMapping<Self>;
}

parameter_types! {
	pub const ExistentialDeposit: u64 = 0;
}

#[derive_impl(pallet_balances::config_preludes::TestDefaultConfig as pallet_balances::DefaultConfig)]
impl pallet_balances::Config for Test {
	type Balance = Balance;
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type RuntimeHoldReason = ();
	type DustRemoval = ();
}

parameter_types! {
	pub const MinVestedTransfer: u64 = 1;
	pub UnvestedFundsAllowedWithdrawReasons: WithdrawReasons =
		WithdrawReasons::except(WithdrawReasons::TRANSFER | WithdrawReasons::RESERVE);
}

impl pallet_vesting::Config for Test {
	type BlockNumberToBalance = ConvertInto;
	type Currency = Balances;
	type MinVestedTransfer = MinVestedTransfer;
	type RuntimeEvent = RuntimeEvent;
	type UnvestedFundsAllowedWithdrawReasons = UnvestedFundsAllowedWithdrawReasons;
	type BlockNumberProvider = System;
	type WeightInfo = ();
	const MAX_VESTING_SCHEDULES: u32 = 100;
}

parameter_types! {
	pub const MinimumPeriod: u64 = 1000;
}

impl pallet_timestamp::Config for Test {
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = MinimumPeriod;
	type WeightInfo = ();
}

pub struct FindAuthorTruncated;
impl FindAuthor<H160> for FindAuthorTruncated {
	fn find_author<'a, I>(_digests: I) -> Option<H160>
	where
		I: 'a + IntoIterator<Item = (ConsensusEngineId, &'a [u8])>,
	{
		Some(H160::from_str("1234500000000000000000000000000000000000").unwrap())
	}
}

pub const BLOCK_GAS_LIMIT: u64 = 15_000_000;
pub const MAX_POV_SIZE: u64 = 5 * 1024 * 1024;

pub type PrecompileCall = VestingPrecompileCall<Test>;

pub type LaosPrecompiles<Test> =
	PrecompileSetBuilder<Test, (PrecompileAt<AddressU64<1>, VestingPrecompile<Test>>,)>;

frame_support::parameter_types! {
	pub BlockGasLimit: U256 = U256::from(BLOCK_GAS_LIMIT);
	pub const GasLimitPovSizeRatio: u64 = BLOCK_GAS_LIMIT.saturating_div(MAX_POV_SIZE);
	/// 1 weight to 1 gas, for testing purposes
	pub WeightPerGas: frame_support::weights::Weight = frame_support::weights::Weight::from_parts(1, 0);
	pub PrecompilesInstance: LaosPrecompiles<Test> = LaosPrecompiles::new();
}

impl pallet_evm::Config for Test {
	type FeeCalculator = ();
	type GasWeightMapping = pallet_evm::FixedGasWeightMapping<Self>;
	type WeightPerGas = WeightPerGas;
	type BlockHashMapping = pallet_evm::SubstrateBlockHashMapping<Self>;
	type CallOrigin = pallet_evm::EnsureAddressRoot<Self::AccountId>;
	type WithdrawOrigin = pallet_evm::EnsureAddressNever<Self::AccountId>;
	type AddressMapping = pallet_evm::IdentityAddressMapping;
	type Currency = Balances;
	type RuntimeEvent = RuntimeEvent;
	type PrecompilesType = LaosPrecompiles<Self>;
	type PrecompilesValue = PrecompilesInstance;
	type ChainId = ();
	type BlockGasLimit = BlockGasLimit;
	type Runner = pallet_evm::runner::stack::Runner<Self>;
	type OnChargeTransaction = ();
	type OnCreate = ();
	type FindAuthor = FindAuthorTruncated;
	type GasLimitPovSizeRatio = GasLimitPovSizeRatio;
	type Timestamp = Timestamp;
	type WeightInfo = ();
	type SuicideQuickClearLimit = ();
}

#[derive(Default)]
pub(crate) struct ExtBuilder {
	/// Endowed accounts with balances
	balances: Vec<(AccountId, Balance)>,
}

impl ExtBuilder {
	/// Fund some accounts before starting the test
	pub(crate) fn with_balances(mut self, balances: Vec<(AccountId, Balance)>) -> Self {
		self.balances = balances;
		self
	}

	/// Build the test externalities for use in tests
	pub(crate) fn build(self) -> sp_io::TestExternalities {
		let mut t = frame_system::GenesisConfig::<Test>::default()
			.build_storage()
			.expect("Frame system builds valid default genesis config");

		pallet_balances::GenesisConfig::<Test> { balances: self.balances.clone() }
			.assimilate_storage(&mut t)
			.expect("Pallet balances storage can be assimilated");

		let mut ext = sp_io::TestExternalities::new(t);
		ext.execute_with(|| {
			System::set_block_number(1);
		});
		ext
	}
}

pub type BlockNumber = BlockNumberFor<Test>;

/// Rolls to the desired block. Returns the number of blocks played.
pub(crate) fn roll_to(n: BlockNumber) -> BlockNumber {
	let mut num_blocks = 0;
	let mut block = System::block_number();
	while block < n {
		roll_one_block!(false);
		block = System::block_number();
		num_blocks += 1;
	}
	num_blocks
}
