mod asset_metadata_extender;
mod aura;
mod authorship;
mod balances;
mod base_fee;
mod block_rewards_handler;
mod cumulus_aura_ext;
mod cumulus_dmp_queue;
mod cumulus_parachain_system;
mod cumulus_xcmp_queue;
mod ethereum;
mod evm;
mod evm_chain_id;
mod laos_evolution;
mod multisig;
mod parachain_info;
pub mod parachain_staking;
mod proxy;
mod session;
mod sudo;
mod system;
mod timestamp;
mod transaction_payment;
mod utility;
mod vesting;

use frame_support::parameter_types;

parameter_types! {
	/// Max length of the `TokenUri`
	pub const MaxTokenUriLength: u32 = 512;
}
