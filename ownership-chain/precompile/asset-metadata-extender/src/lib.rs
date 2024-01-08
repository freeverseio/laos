#![cfg_attr(not(feature = "std"), no_std)]

use laos_precompile_utils::{
	keccak256, revert_dispatch_error, succeed, Address, Bytes, EvmDataWriter, EvmResult,
	FunctionModifier, GasCalculator, LogExt, LogsBuilder, PrecompileHandleExt,
};

/// Solidity selector of the TokenURIExtended log, which is the Keccak of the Log signature.
pub const SELECTOR_LOG_TOKEN_URI_EXTENDED: [u8; 32] =
	keccak256!("TokenURIExtended(address,string,uint256)");

#[cfg(test)]
mod tests;
