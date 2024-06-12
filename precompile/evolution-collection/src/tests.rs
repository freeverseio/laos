use super::*;
use mock::*;
use precompile_utils::testing::*;
use sp_core::H160;
use std::str::FromStr;

const ALICE: &str = "0xf24FF3a9CF04c71Dbc94D0b566f7A27B94566cac";

/// Get precompiles from the mock.
fn precompiles() -> EvolutionCollectionPrecompileSet<Test> {
	LaosPrecompiles::get()
}

#[test]
fn selectors() {
	assert!(PrecompileCall::owner_selectors().contains(&0x8DA5CB5B));
}

#[test]
fn owner_of_non_existent_collection_should_revert() {
	new_test_ext().execute_with(|| {
		let alice = H160::from_str(ALICE).unwrap();
		let collection_address =
			H160::from_str("fffffffffffffffffffffffe0000000000000000").unwrap();

		precompiles()
			.prepare_test(alice, collection_address, PrecompileCall::owner {})
			.execute_reverts(|r| r == b"collection does not exist");
	})
}
