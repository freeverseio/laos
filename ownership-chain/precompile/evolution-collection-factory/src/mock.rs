use core::str::FromStr;

use fp_evm::{FeeCalculator, Precompile, PrecompileHandle};
use sp_runtime::BuildStorage;

use crate::EvolutionCollectionFactoryPrecompile;

use frame_support::{
	parameter_types,
	traits::{ConstU16, ConstU64, FindAuthor},
	weights::Weight,
};
use pallet_balances::AccountData;
use sp_core::{H160, H256, U256};
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup},
	ConsensusEngineId,
};

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

impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Nonce = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = AccountData<u64>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
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
	type WeightInfo = ();
}

parameter_types! {
	pub const ExistentialDeposit: u64 = 0;
}
impl pallet_balances::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	type Balance = u64;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type ReserveIdentifier = ();
	type RuntimeHoldReason = ();
	type FreezeIdentifier = ();
	type MaxLocks = ();
	type MaxReserves = ();
	type MaxHolds = ();
	type MaxFreezes = ();
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

pub struct FixedGasPrice;
impl FeeCalculator for FixedGasPrice {
	fn min_gas_price() -> (U256, Weight) {
		// Return some meaningful gas price and weight
		(1_000_000_000u128.into(), Weight::from_parts(7u64, 0))
	}
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
pub const BLOCK_GAS_LIMIT: u64 = 150_000_000;
pub const MAX_POV_SIZE: u64 = 5 * 1024 * 1024;

frame_support::parameter_types! {
	pub BlockGasLimit: U256 = U256::from(crate::mock::BLOCK_GAS_LIMIT);
	pub const GasLimitPovSizeRatio: u64 = crate::mock::BLOCK_GAS_LIMIT.saturating_div(crate::mock::MAX_POV_SIZE);
	pub WeightPerGas: frame_support::weights::Weight = frame_support::weights::Weight::from_parts(20_000, 0);
	pub MockPrecompiles: MockPrecompileSet<Test> = MockPrecompileSet::<_>::new();
}

impl pallet_evm::Config for Test {
	type FeeCalculator = FixedGasPrice;
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

pub type MockEvolutionCollectionFactoryPrecompile = EvolutionCollectionFactoryPrecompile<Test>;

impl<Test> MockPrecompileSet<Test>
where
	Test: pallet_evm::Config,
{
	pub fn new() -> Self {
		Self(Default::default())
	}
}

impl<Test> fp_evm::PrecompileSet for MockPrecompileSet<Test>
where
	Test: pallet_evm::Config + pallet_laos_evolution::Config,
{
	fn execute(&self, handle: &mut impl PrecompileHandle) -> Option<fp_evm::PrecompileResult> {
		Some(MockEvolutionCollectionFactoryPrecompile::execute(handle))
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
