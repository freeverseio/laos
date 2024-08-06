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
use frame_support::{
	derive_impl, parameter_types, traits::FindAuthor, weights::constants::RocksDbWeight,
};
use precompile_utils::precompile_set::{AddressU64, PrecompileAt, PrecompileSetBuilder};
use sp_core::{H160, U256};
use sp_runtime::{traits::IdentityLookup, BuildStorage, ConsensusEngineId};

use super::{AssetMetadataExtenderPrecompile, AssetMetadataExtenderPrecompileCall};
use crate as pallet_asset_metadata_extender;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test
	{
		System: frame_system::{Pallet, Call, Config<T>, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		Timestamp: pallet_timestamp::{Pallet, Call, Storage},
		EVM: pallet_evm::{Pallet, Call, Storage, Config<T>, Event<T>},
		AssetMetadataExtender: pallet_asset_metadata_extender,
	}
);

type AccountId = H160;
type Balance = u64;
type Block = frame_system::mocking::MockBlock<Test>;

#[derive_impl(frame_system::config_preludes::TestDefaultConfig as frame_system::DefaultConfig)]
impl frame_system::Config for Test {
	type Block = Block;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type DbWeight = RocksDbWeight;
	type AccountData = pallet_balances::AccountData<Balance>;
}

parameter_types! {
	pub const ExistentialDeposit: u64 = 0;
}

// Pallet Balances
#[derive_impl(pallet_balances::config_preludes::TestDefaultConfig as pallet_balances::DefaultConfig)]
impl pallet_balances::Config for Test {
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type RuntimeHoldReason = ();
	type DustRemoval = ();
	type Balance = Balance;
}

// Pallet Asset Metadata Extender
impl pallet_asset_metadata_extender::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type MaxUniversalLocationLength = MaxUniversalLocationLength;
	type MaxTokenUriLength = MaxTokenUriLength;
	type AccountIdToH160 = AccountIdToH160;
	type GasWeightMapping = pallet_evm::FixedGasWeightMapping<Self>;
	type WeightInfo = ();
}

pub struct AccountIdToH160;

impl sp_runtime::traits::Convert<AccountId, H160> for AccountIdToH160 {
	fn convert(account_id: AccountId) -> H160 {
		account_id
	}
}

impl sp_runtime::traits::ConvertBack<AccountId, H160> for AccountIdToH160 {
	fn convert_back(h160: H160) -> AccountId {
		h160
	}
}

parameter_types! {
	pub const MaxTokenUriLength: u32 = 512;
	pub const MaxUniversalLocationLength: u32 = 512;
}

// Pallet EVM
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

pub const BLOCK_GAS_LIMIT: u64 = 15_000_000;
pub const MAX_POV_SIZE: u64 = 5 * 1024 * 1024;

pub type PrecompileCall = AssetMetadataExtenderPrecompileCall<Test>;

pub type LaosPrecompiles<Test> = PrecompileSetBuilder<
	Test,
	(PrecompileAt<AddressU64<1>, AssetMetadataExtenderPrecompile<Test>>,),
>;

frame_support::parameter_types! {
	pub BlockGasLimit: U256 = U256::from(BLOCK_GAS_LIMIT);
	pub const GasLimitPovSizeRatio: u64 = BLOCK_GAS_LIMIT.saturating_div(MAX_POV_SIZE);
	/// 1 weight to 1 gas, for testing purposes
	pub WeightPerGas: frame_support::weights::Weight = frame_support::weights::Weight::from_parts(1, 0);
	pub PrecompilesInstance: LaosPrecompiles<Test> = LaosPrecompiles::new();
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

// Pallet Timestamp
parameter_types! {
	pub const MinimumPeriod: u64 = 1000;
}

impl pallet_timestamp::Config for Test {
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = MinimumPeriod;
	type WeightInfo = ();
}

/// New Test Ext
// Build genesis storage according to the mock runtime.
pub(crate) fn new_test_ext() -> sp_io::TestExternalities {
	frame_system::GenesisConfig::<Test>::default().build_storage().unwrap().into()
}
