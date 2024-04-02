#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit = "256"] // `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.

#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

pub mod apis;
pub mod configs;
pub mod currency;
mod precompiles;
mod self_contained_call;
pub mod types;
mod weights;

use core::marker::PhantomData;
use frame_support::{
	construct_runtime,
	weights::{constants::WEIGHT_REF_TIME_PER_SECOND, Weight},
};
use frame_system::EnsureRoot;
pub use laos_primitives::{
	AccountId, AuraId, Balance, BlockNumber, Hash, Header, Nonce, Signature,
};
pub use pallet_evm_evolution_collection_factory::REVERT_BYTECODE;
pub use pallet_parachain_staking::{InflationInfo, Range};
use precompiles::FrontierPrecompiles;
use sp_core::U256;
use sp_runtime::{create_runtime_str, generic, impl_opaque_keys, traits::ConvertInto, Permill};
use sp_std::prelude::*;
#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;
use staging_xcm_executor::XcmExecutor;

/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;

/// BlockId type as expected by this runtime.
pub type BlockId = generic::BlockId<Block>;

pub type Precompiles = FrontierPrecompiles<Runtime>;

/// Opaque types. These are used by the CLI to instantiate machinery that don't need to know
/// the specifics of the runtime. They can then be made to be agnostic over specific formats
/// of data like extrinsics, allowing for them to continue syncing the network through upgrades
/// to even the core data structures.
pub mod opaque {
	use super::*;
	use sp_runtime::{generic, traits::BlakeTwo256};

	pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;
	/// Opaque block header type.
	pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
	/// Opaque block type.
	pub type Block = generic::Block<Header, UncheckedExtrinsic>;
	/// Opaque block identifier type.
	pub type BlockId = generic::BlockId<Block>;
}

impl_opaque_keys! {
	pub struct SessionKeys {
		pub aura: Aura,
	}
}

/// This determines the average expected block time that we are targeting.
pub const MILLISECS_PER_BLOCK: u64 = 12000;

/// Version of the runtime
#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
	spec_name: create_runtime_str!("laos"),
	impl_name: create_runtime_str!("laos"),
	authoring_version: 1,
	spec_version: 1300,
	impl_version: 0,
	apis: apis::PUBLIC_RUNTIME_API_VERSIONS,
	transaction_version: 1,
	state_version: 1,
};

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
	NativeVersion { runtime_version: VERSION, can_author_with: Default::default() }
}

// Create the runtime by composing the FRAME pallets that were previously configured.
construct_runtime!(
	pub struct Runtime
	{
		// System support stuff.
		System: frame_system = 0,
		ParachainSystem: cumulus_pallet_parachain_system = 1,
		Timestamp: pallet_timestamp = 2,
		ParachainInfo: parachain_info = 3,
		Sudo: pallet_sudo = 4,
		Utility: pallet_utility = 5,
		Multisig: pallet_multisig = 6,
		Proxy: pallet_proxy = 7,

		// Monetary stuff.
		Balances: pallet_balances = 10,
		TransactionPayment: pallet_transaction_payment = 11,
		Vesting: pallet_vesting = 12,

		// Consensus support: the order of these 5 are important and shall not change.
		Authorship: pallet_authorship = 20,
		Session: pallet_session = 21,
		Aura: pallet_aura = 22,
		AuraExt: cumulus_pallet_aura_ext = 23,
		ParachainStaking: pallet_parachain_staking = 24,

		// XCM helpers.
		XcmpQueue: cumulus_pallet_xcmp_queue = 30,
		PolkadotXcm: pallet_xcm = 31,
		CumulusXcm: cumulus_pallet_xcm = 32,
		DmpQueue: cumulus_pallet_dmp_queue = 33,

		// Frontier
		Ethereum: pallet_ethereum = 50,
		EVM: pallet_evm = 51,
		EVMChainId: pallet_evm_chain_id = 52,
		BaseFee: pallet_base_fee = 53,

		// LAOS pallets
		LaosEvolution: pallet_laos_evolution = 100,
		AssetMetadataExtender: pallet_asset_metadata_extender = 101,
	}
);

/// The SignedExtension to the basic transaction logic.
type SignedExtra = (
	frame_system::CheckNonZeroSender<Runtime>,
	frame_system::CheckSpecVersion<Runtime>,
	frame_system::CheckTxVersion<Runtime>,
	frame_system::CheckGenesis<Runtime>,
	frame_system::CheckEra<Runtime>,
	frame_system::CheckNonce<Runtime>,
	frame_system::CheckWeight<Runtime>,
	pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
);

/// Unchecked extrinsic type as expected by this runtime.
type UncheckedExtrinsic =
	fp_self_contained::UncheckedExtrinsic<AccountId, RuntimeCall, Signature, SignedExtra>;

/// Executive: handles dispatch to the various modules.
type Executive = frame_executive::Executive<
	Runtime,
	Block,
	frame_system::ChainContext<Runtime>,
	Runtime,
	AllPalletsWithSystem,
>;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarks;
