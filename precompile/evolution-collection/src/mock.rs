// Copyright 2023-2024 Freverse.io
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

use fp_evm::{Precompile, PrecompileHandle};
use pallet_evm_evolution_collection_factory::EvolutionCollectionFactoryPrecompile;
use pallet_laos_evolution::address_to_collection_id;
use sp_runtime::BuildStorage;

use crate::EvolutionCollectionPrecompile;

use frame_support::{
	derive_impl, parameter_types, traits::FindAuthor, weights::constants::RocksDbWeight,
};
use sp_core::{H160, U256};
use sp_runtime::{traits::IdentityLookup, ConsensusEngineId};

type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test {
		System: frame_system::{Pallet, Call, Config<T>, Storage, Event<T>},
		LaosEvolutionPallet: pallet_laos_evolution,
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		Timestamp: pallet_timestamp::{Pallet, Call, Storage},
		EVM: pallet_evm::{Pallet, Call, Storage, Config<T>, Event<T>},
	}
);

pub type AccountId = H160;
type Balance = u64;

#[derive_impl(frame_system::config_preludes::TestDefaultConfig as frame_system::DefaultConfig)]
impl frame_system::Config for Test {
	type Block = Block;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type AccountData = pallet_balances::AccountData<Balance>;
	type DbWeight = RocksDbWeight;
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
	pub const MaxTokenUriLength: u32 = 512;
}

pub struct AccountIdToH160;

impl sp_runtime::traits::Convert<AccountId, H160> for AccountIdToH160 {
	fn convert(account_id: AccountId) -> H160 {
		account_id
	}
}

impl pallet_laos_evolution::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type AccountIdToH160 = AccountIdToH160;
	type MaxTokenUriLength = MaxTokenUriLength;
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

frame_support::parameter_types! {
	pub BlockGasLimit: U256 = U256::from(crate::mock::BLOCK_GAS_LIMIT);
	pub const GasLimitPovSizeRatio: u64 = crate::mock::BLOCK_GAS_LIMIT.saturating_div(crate::mock::MAX_POV_SIZE);
	/// 1 weight to 1 gas, for testing purposes
	pub WeightPerGas: frame_support::weights::Weight = frame_support::weights::Weight::from_parts(1, 0);
	pub MockPrecompiles: MockPrecompileSet<Test> = MockPrecompileSet::<_>::new();
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
	type PrecompilesType = MockPrecompileSet<Self>;
	type PrecompilesValue = MockPrecompiles;
	type ChainId = ();
	type BlockGasLimit = BlockGasLimit;
	type Runner = pallet_evm::runner::stack::Runner<Self>;
	type OnChargeTransaction = ();
	type OnCreate = ();
	type FindAuthor = FindAuthorTruncated;
	type GasLimitPovSizeRatio = GasLimitPovSizeRatio;
	type Timestamp = Timestamp;
	type WeightInfo = ();
}

pub struct MockPrecompileSet<Test>(sp_std::marker::PhantomData<Test>);

pub type MockEvolutionCollectionPrecompile = EvolutionCollectionPrecompile<Test>;
pub type MockEvolutionCollectionFactoryPrecompile = EvolutionCollectionFactoryPrecompile<Test>;

impl<Test> MockPrecompileSet<Test>
where
	Test: pallet_evm::Config,
{
	pub fn new() -> Self {
		Self(Default::default())
	}
}

/// Fixed precompile addresses for testing.
pub const EVOLUTION_FACTORY_PRECOMPILE_ADDRESS: [u8; 20] = [6u8; 20];

impl<Test> fp_evm::PrecompileSet for MockPrecompileSet<Test>
where
	Test: pallet_evm::Config + pallet_laos_evolution::Config,
{
	fn execute(&self, handle: &mut impl PrecompileHandle) -> Option<fp_evm::PrecompileResult> {
		match handle.context().address {
			a if address_to_collection_id(a).is_ok() =>
				Some(MockEvolutionCollectionPrecompile::execute(handle)),
			H160(EVOLUTION_FACTORY_PRECOMPILE_ADDRESS) =>
				Some(MockEvolutionCollectionFactoryPrecompile::execute(handle)),
			_ => return None,
		}
	}

	fn is_precompile(&self, _address: H160, _gas: u64) -> fp_evm::IsPrecompileResult {
		return fp_evm::IsPrecompileResult::Answer { is_precompile: true, extra_cost: 0 }
	}
}

/// New Test Ext
// Build genesis storage according to the mock runtime.
pub(crate) fn new_test_ext() -> sp_io::TestExternalities {
	frame_system::GenesisConfig::<Test>::default().build_storage().unwrap().into()
}
