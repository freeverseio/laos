use crate::mock::PrecompileCall;

#[test]
fn selectors() {
	assert!(PrecompileCall::owner_selectors().contains(&0x8DA5CB5B));
}
