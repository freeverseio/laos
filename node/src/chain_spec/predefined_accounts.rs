use hex_literal::hex;
use ownership_parachain_primitives::AccountId;

pub const ALITH: [u8; 20] = hex!("f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac");
pub const BALTATHAR: [u8; 20] = hex!("3Cd0A705a2DC65e5b1E1205896BaA2be8A07c6e0");
pub const CHARLETH: [u8; 20] = hex!("798d4Ba9baf0064Ec19eB4F0a1a45785ae9D6DFc");
pub const DOROTHY: [u8; 20] = hex!("773539d4Ac0e786233D90A233654ccEE26a613D9");
pub const ETHAN: [u8; 20] = hex!("Ff64d3F6efE2317EE2807d223a0Bdc4c0c49dfDB");
pub const FAITH: [u8; 20] = hex!("C0F0f4ab324C46e55D02D0033343B4Be8A55532d");

/// Returns the accounts that are predefined in the runtime.
pub fn accounts() -> Vec<AccountId> {
	vec![
		ALITH.into(),
		BALTATHAR.into(),
		CHARLETH.into(),
		DOROTHY.into(),
		ETHAN.into(),
		FAITH.into(),
	]
}
