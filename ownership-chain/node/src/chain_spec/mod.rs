use hex_literal::hex;
use ownership_parachain_primitives::{AccountId, AuraId};
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use serde::{Deserialize, Serialize};
use sp_core::{Pair, Public};

pub mod klaos;
pub mod laos;

const ALITH : [u8; 20] = hex!("f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac");
const BALTATHAR : [u8; 20] = hex!("3Cd0A705a2DC65e5b1E1205896BaA2be8A07c6e0");
const CHARLETH : [u8; 20] = hex!("798d4Ba9baf0064Ec19eB4F0a1a45785ae9D6DFc");
const DOROTHY : [u8; 20] = hex!("773539d4Ac0e786233D90A233654ccEE26a613D9");
const ETHAN : [u8; 20] = hex!("Ff64d3F6efE2317EE2807d223a0Bdc4c0c49dfDB");
const FAITH : [u8; 20] = hex!("C0F0f4ab324C46e55D02D0033343B4Be8A55532d");

/// The default XCM version to set in genesis config.
const SAFE_XCM_VERSION: u32 = staging_xcm::prelude::XCM_VERSION;

/// List of endowed accounts.
fn endowed_accounts() -> Vec<AccountId> {
	vec![
		ALITH.into(),
		BALTATHAR.into(),
		CHARLETH.into(),
		DOROTHY.into(),	
		ETHAN.into(),
		FAITH.into(),
	]
}



/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{seed}"), None)
		.expect("static values are valid; qed")
		.public()
}

/// Generate collator keys from seed.
///
/// This function's return type must always match the session keys of the chain in tuple format.
pub fn get_collator_keys_from_seed(seed: &str) -> AuraId {
	get_from_seed::<AuraId>(seed)
}

/// The extensions for the [`ChainSpec`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ChainSpecGroup, ChainSpecExtension)]
#[serde(deny_unknown_fields)]
pub struct Extensions {
	/// The relay chain of the Parachain.
	pub relay_chain: String,
	/// The id of the Parachain.
	pub para_id: u32,
}

impl Extensions {
	/// Try to get the extension from the given `ChainSpec`.
	pub fn try_get(chain_spec: &dyn sc_service::ChainSpec) -> Option<&Self> {
		sc_chain_spec::get_extension(chain_spec.extensions())
	}
}
