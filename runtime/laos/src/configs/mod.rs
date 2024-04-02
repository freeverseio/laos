mod asset_metadata_extender;
mod aura;
mod authorship;
mod balances;
mod base_fee;
mod cumulus_dmp_queue;
mod cumulus_parachain_system;
mod cumulus_xcmp_queue;
mod ethereum;
mod evm;
mod laos_evolution;
mod multisig;
pub(crate) mod parachain_staking;
mod proxy;
mod session;
mod sudo;
pub mod system;
mod timestamp;
mod transaction_payment;
mod utility;
mod vesting;
mod xcm;
pub(crate) mod xcm_config;

use frame_support::parameter_types;

use crate::Runtime;

parameter_types! {
	/// Max length of the `TokenUri`
	pub const MaxTokenUriLength: u32 = 512;
}

impl cumulus_pallet_aura_ext::Config for Runtime {}
impl pallet_evm_chain_id::Config for Runtime {}
impl parachain_info::Config for Runtime {}
