use cumulus_primitives_core::ParaId;
use fp_evm::GenesisAccount;
use hex_literal::hex;
use laos_ownership_runtime::{
	AccountId, AuraId, InflationInfo, Precompiles, RewardRate, StakingInfo, BLOCKS_PER_YEAR,
	REVERT_BYTECODE, UNIT,
};
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sc_service::ChainType;
use serde::{Deserialize, Serialize};
use sp_core::{Pair, Public, H160, U256};
use sp_runtime::Perquintill;
use std::{collections::BTreeMap, str::FromStr};

/// List of endowed accounts.
fn endowed_accounts() -> Vec<AccountId> {
	vec![
		// ALITH
		hex!("f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac").into(),
		// BALTATHAR
		hex!("3Cd0A705a2DC65e5b1E1205896BaA2be8A07c6e0").into(),
		// CHARLETH
		hex!("798d4Ba9baf0064Ec19eB4F0a1a45785ae9D6DFc").into(),
		// DOROTHY
		hex!("773539d4Ac0e786233D90A233654ccEE26a613D9").into(),
		// ETHAN
		hex!("Ff64d3F6efE2317EE2807d223a0Bdc4c0c49dfDB").into(),
		// FAITH
		hex!("C0F0f4ab324C46e55D02D0033343B4Be8A55532d").into(),
	]
}

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec =
	sc_service::GenericChainSpec<laos_ownership_runtime::RuntimeGenesisConfig, Extensions>;

/// The default XCM version to set in genesis config.
const SAFE_XCM_VERSION: u32 = staging_xcm::prelude::XCM_VERSION;

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{seed}"), None)
		.expect("static values are valid; qed")
		.public()
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

/// Generate collator keys from seed.
///
/// This function's return type must always match the session keys of the chain in tuple format.
pub fn get_collator_keys_from_seed(seed: &str) -> AuraId {
	get_from_seed::<AuraId>(seed)
}

/// Generate the session keys from individual elements.
///
/// The input must be a tuple of individual keys (a single arg for now since we have just one key).
pub fn template_session_keys(keys: AuraId) -> laos_ownership_runtime::SessionKeys {
	laos_ownership_runtime::SessionKeys { aura: keys }
}

pub fn development_config() -> ChainSpec {
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "UNIT".into());
	properties.insert("tokenDecimals".into(), 18.into());
	properties.insert("ss58Format".into(), 42.into());

	ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || {
			testnet_genesis(
				// initial collators.
				vec![(
					hex!("f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac").into(),
					get_collator_keys_from_seed("Alice"),
				)],
				endowed_accounts(),
				// Give Alice root privileges
				Some(hex!("f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac").into()),
				2001.into(),
			)
		},
		Vec::new(),
		None,
		None,
		None,
		None,
		Extensions {
			relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
			para_id: 2001,
		},
	)
}

pub fn local_testnet_config() -> ChainSpec {
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "UNIT".into());
	properties.insert("tokenDecimals".into(), 18.into());
	properties.insert("ss58Format".into(), 42.into());

	ChainSpec::from_genesis(
		// Name
		"Local Testnet",
		// ID
		"local_testnet",
		ChainType::Local,
		move || {
			testnet_genesis(
				// initial collators.
				vec![(
					hex!("f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac").into(),
					get_collator_keys_from_seed("Alice"),
				)],
				endowed_accounts(),
				// Give Alice root privileges
				Some(hex!("f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac").into()),
				2001.into(),
			)
		},
		// Bootnodes
		Vec::new(),
		// Telemetry
		None,
		// Protocol ID
		Some("template-local"),
		// Fork ID
		None,
		// Properties
		Some(properties),
		// Extensions
		Extensions {
			relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
			para_id: 2001,
		},
	)
}

fn testnet_genesis(
	invulnerables: Vec<(AccountId, AuraId)>,
	endowed_accounts: Vec<AccountId>,
	root_key: Option<AccountId>,
	id: ParaId,
) -> laos_ownership_runtime::RuntimeGenesisConfig {

	// Reward configuration used in the genesis config
	// This defines the rate at which rewards are distributed to collators and delegators
	let reward_configuration = InflationInfo {
		InflationInfo::new(
			BLOCKS_PER_YEAR,
			// max collator staking rate
			Perquintill::from_percent(40),
			// collator reward rate
			Perquintill::from_percent(10),
			// max delegator staking rate
			Perquintill::from_percent(10),
			// delegator reward rate
			Perquintill::from_percent(8),
		)
	};
	

	laos_ownership_runtime::RuntimeGenesisConfig {
		system: laos_ownership_runtime::SystemConfig {
			code: laos_ownership_runtime::WASM_BINARY
				.expect("WASM binary was not build, please build it!")
				.to_vec(),
			..Default::default()
		},
		balances: laos_ownership_runtime::BalancesConfig {
			balances: endowed_accounts.iter().cloned().map(|k| (k, 1e24 as u128)).collect(),
		},
		parachain_info: laos_ownership_runtime::ParachainInfoConfig {
			parachain_id: id,
			..Default::default()
		},
		session: laos_ownership_runtime::SessionConfig {
			keys: invulnerables
				.into_iter()
				.map(|(acc, aura)| {
					(
						acc,                         // account id
						acc,                         // validator id
						template_session_keys(aura), // session keys
					)
				})
				.collect(),
		},
		// no need to pass anything to aura, in fact it will panic if we do. Session will take care
		// of this.
		aura: Default::default(),
		aura_ext: Default::default(),
		parachain_system: Default::default(),
		polkadot_xcm: laos_ownership_runtime::PolkadotXcmConfig {
			safe_xcm_version: Some(SAFE_XCM_VERSION),
			..Default::default()
		},
		sudo: laos_ownership_runtime::SudoConfig { key: root_key },
		transaction_payment: Default::default(),
		// EVM compatibility
		evm_chain_id: laos_ownership_runtime::EVMChainIdConfig {
			chain_id: 667,
			..Default::default()
		},
		parachain_staking: laos_ownership_runtime::ParachainStakingConfig {
			max_candidate_stake: 10_000 * UNIT,
			inflation_config: reward_configuration,
			..Default::default()
		},
		evm: laos_ownership_runtime::EVMConfig {
			accounts: {
				let mut map: BTreeMap<_, _> = Precompiles::used_addresses()
					.iter()
					.map(|&address| {
						(
							address,
							GenesisAccount {
								nonce: Default::default(),
								balance: Default::default(),
								storage: Default::default(),
								code: REVERT_BYTECODE.into(),
							},
						)
					})
					.collect();

				map.insert(
					// H160 address of Alice dev account
					// Derived from SS58 (42 prefix) address
					// SS58: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
					// hex: 0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d
					// Using the full hex key, truncating to the first 20 bytes (the first 40 hex
					// chars)
					H160::from_str("d43593c715fdd31c61141abd04a99fd6822c8558")
						.expect("internal H160 is valid; qed"),
					fp_evm::GenesisAccount {
						balance: U256::from_str("0xffffffffffffffffffffffffffffffff")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address of CI test runner account
					H160::from_str("6be02d1d3665660d22ff9624b7be0551ee1ac91b")
						.expect("internal H160 is valid; qed"),
					fp_evm::GenesisAccount {
						balance: U256::from_str("0xffffffffffffffffffffffffffffffff")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map.insert(
					// H160 address for benchmark usage
					H160::from_str("1000000000000000000000000000000000000001")
						.expect("internal H160 is valid; qed"),
					fp_evm::GenesisAccount {
						nonce: U256::from(1),
						balance: U256::from(1_000_000_000_000_000_000_000_000u128),
						storage: Default::default(),
						code: vec![0x00],
					},
				);
				map.insert(
					// H160 address of dev account
					// Private key :
					// 0xb9d2ea9a615f3165812e8d44de0d24da9bbd164b65c4f0573e1ce2c8dbd9c8df
					H160::from_str("C0F0f4ab324C46e55D02D0033343B4Be8A55532d")
						.expect("internal H160 is valid; qed"),
					fp_evm::GenesisAccount {
						balance: U256::from_str("0xef000000000000000000000000000")
							.expect("internal U256 is valid; qed"),
						code: Default::default(),
						nonce: Default::default(),
						storage: Default::default(),
					},
				);
				map
			},
			..Default::default()
		},
		..Default::default()
	}
}
