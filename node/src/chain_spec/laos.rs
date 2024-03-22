use super::{get_collator_keys_from_seed, predefined_accounts, Extensions, SAFE_XCM_VERSION};
use cumulus_primitives_core::ParaId;
use fp_evm::GenesisAccount;
use laos_runtime::{AuraId, Precompiles, REVERT_BYTECODE};
use sc_service::ChainType;
use sp_runtime::Perbill;

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec = sc_service::GenericChainSpec<laos_runtime::RuntimeGenesisConfig, Extensions>;

/// Generate the session keys from individual elements.
///
/// The input must be a tuple of individual keys (a single arg for now since we have just one key).
pub fn template_session_keys(keys: AuraId) -> laos_runtime::SessionKeys {
	laos_runtime::SessionKeys { aura: keys }
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
		move || testnet_genesis(2001.into(), 1_000_000_000_000),
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
		"laos_local_testnet",
		ChainType::Local,
		move || testnet_genesis(2001.into(), 1_000_000_000_000_000_000),
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

pub fn zombienet_testnet_config() -> ChainSpec {
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "UNIT".into());
	properties.insert("tokenDecimals".into(), 18.into());
	properties.insert("ss58Format".into(), 42.into());

	ChainSpec::from_genesis(
		// Name
		"Local Testnet",
		// ID
		"laos_local_testnet",
		ChainType::Local,
		move || testnet_genesis(2001.into(), 1_000_000_000_000_000_000),
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

fn testnet_genesis(id: ParaId, unit: u128) -> laos_runtime::RuntimeGenesisConfig {
	laos_runtime::RuntimeGenesisConfig {
		system: laos_runtime::SystemConfig {
			code: laos_runtime::WASM_BINARY
				.expect("WASM binary was not build, please build it!")
				.to_vec(),
			..Default::default()
		},
		balances: laos_runtime::BalancesConfig {
			balances: vec![
				(predefined_accounts::ALITH.into(), 800000000 * unit),
				(predefined_accounts::BALTATHAR.into(), 150000000 * unit),
				(predefined_accounts::FAITH.into(), 50000000 * unit),
			],
		},
		parachain_info: laos_runtime::ParachainInfoConfig {
			parachain_id: id,
			..Default::default()
		},
		session: laos_runtime::SessionConfig {
			keys: vec![(
				predefined_accounts::ALITH.into(),
				predefined_accounts::ALITH.into(),
				template_session_keys(get_collator_keys_from_seed("Alice")),
			)],
		},
		// no need to pass anything to aura, in fact it will panic if we do. Session will take care
		// of this.
		aura: Default::default(),
		aura_ext: Default::default(),
		parachain_system: Default::default(),
		polkadot_xcm: laos_runtime::PolkadotXcmConfig {
			safe_xcm_version: Some(SAFE_XCM_VERSION),
			..Default::default()
		},
		sudo: laos_runtime::SudoConfig { key: Some(predefined_accounts::ALITH.into()) },
		transaction_payment: Default::default(),
		// EVM compatibility
		evm_chain_id: laos_runtime::EVMChainIdConfig { chain_id: 667, ..Default::default() },
		parachain_staking: laos_runtime::ParachainStakingConfig {
			blocks_per_round: 5,
			rewards_account: Some(predefined_accounts::BALTATHAR.into()),
			inflation_config: laos_runtime::InflationInfo {
				// staking expectations
				expect: laos_runtime::Range { min: 1000000, ideal: 1000000, max: 1000000 },
				// annual inflation
				annual: laos_runtime::Range {
					min: Perbill::from_perthousand(75),
					ideal: Perbill::from_perthousand(75),
					max: Perbill::from_perthousand(75),
				},
				round: laos_runtime::Range {
					min: Perbill::zero(),
					ideal: Perbill::zero(),
					max: Perbill::zero(),
				},
			},
			..Default::default()
		},
		evm: laos_runtime::EVMConfig {
			accounts: Precompiles::used_addresses()
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
				.collect(),
			..Default::default()
		},
		..Default::default()
	}
}
