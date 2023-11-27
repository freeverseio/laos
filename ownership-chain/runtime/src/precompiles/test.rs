use super::*;
use core::str::FromStr;
use evm::Context;
use precompile_utils::testing::MockHandle;

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
	assert!(!is_precompile(hash(6)).unwrap());
	assert!(!is_precompile(hash(1026)).unwrap());
	assert!(is_precompile(hash(1027)).unwrap());
	assert!(is_precompile(H160::from_str("0xfffffffffffffffffffffffe0000000000000005").unwrap())
		.unwrap());
}

/// Test to ensure that delegate calls to addresses that are not precompiles are recognized
/// correctly.
#[test]
fn delegatecall_to_non_precompile_is_recognized() {
	let precompiles = FrontierPrecompiles::<Runtime>::new();

	// Address outside the range of standard precompiles
	let code_address = hash(11);
	let context_address = hash(12);

	// Verify that the code address is not a precompile and that it's not treated as a custom
	// precompile delegate call
	assert!(!is_precompile(code_address).unwrap());
	assert!(!precompiles.is_delegatecall_to_custom_precompile(code_address, context_address));
}

/// Test to ensure that delegate calls to non-standard Ethereum precompile addresses are recognized.
#[test]
fn delegatecall_to_custom_precompile_is_recognized() {
	let precompiles = FrontierPrecompiles::<Runtime>::new();

	// Address representing a non-standard precompile
	let code_address = hash(1027);
	let context_address = hash(123456);

	// Verify that the code address is a precompile and is recognized as a custom precompile
	// delegate call
	assert!(is_precompile(code_address).unwrap());
	assert!(precompiles.is_delegatecall_to_custom_precompile(code_address, context_address));
}

/// Test to ensure that delegate calls to standard Ethereum precompile addresses are not recognized
/// as custom precompiles.
#[test]
fn delegatecall_to_standard_precompile_not_recognized_as_custom() {
	let precompiles = FrontierPrecompiles::<Runtime>::new();

	let context_address = hash(123456);

	// Iterate over standard Ethereum precompile addresses (1 to 9)
	for i in 1..=5 {
		let code_address = hash(i);

		// Verify each standard precompile address is not recognized as a custom precompile delegate
		// call
		assert!(is_precompile(code_address).unwrap());
		assert!(!precompiles.is_delegatecall_to_custom_precompile(code_address, context_address));
	}
}

#[test]
fn execute_delegate_call_on_custom_precompile_should_fail() {
	let p = FrontierPrecompiles::<Runtime>::new();

	let code_address = hash(1027);
	let context_address = hash(123456);

	// Verify that the code address is a precompile
	assert!(is_precompile(code_address).unwrap());

	// Setup the mock handle for the delegate call
	let mut handle = MockHandle::new(
		code_address,
		Context { address: context_address, caller: H160::zero(), apparent_value: 0.into() },
	);

	// Execute the precompile with the delegate call
	let result = p.execute(&mut handle);

	// Verify that the execution failed due to a delegate call to a custom precompile
	assert!(
		matches!(result, Some(Err(PrecompileFailure::Revert { exit_status: ExitRevert::Reverted, output })) if output == b"cannot be called with DELEGATECALL or CALLCODE")
	);
}
