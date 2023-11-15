use core::str::FromStr;

use super::{hash, mock::*, FrontierPrecompiles};
use pallet_evm::{IsPrecompileResult, PrecompileSet};
use sp_core::H160;

fn is_precompile(address: H160) -> Result<bool, &'static str> {
	let p = FrontierPrecompiles::<Runtime>::new();
	match p.is_precompile(address, 0) {
		IsPrecompileResult::Answer { is_precompile, extra_cost: _ } => Ok(is_precompile),
		_ => Err("Unexpected result variant"),
	}
}

#[test]
fn null_address_is_not_precompile() {
	assert!(!is_precompile(H160::zero()).unwrap());
}

#[test]
fn ethereum_precompiled_addresses_are_precompile() {
	assert!(is_precompile(hash(1)).unwrap());
	assert!(is_precompile(hash(2)).unwrap());
	assert!(is_precompile(hash(3)).unwrap());
	assert!(is_precompile(hash(4)).unwrap());
	assert!(is_precompile(hash(5)).unwrap());
	assert!(!is_precompile(hash(1026)).unwrap());
	assert!(is_precompile(hash(1027)).unwrap());
	assert!(is_precompile(H160::from_str("0xfffffffffffffffffffffffe0000000000000005").unwrap())
		.unwrap());
}
