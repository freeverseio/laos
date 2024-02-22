use super::{get_collator_keys_from_seed, predefined_accounts, Extensions, SAFE_XCM_VERSION};
use cumulus_primitives_core::ParaId;
use fp_evm::GenesisAccount;
use laos_ownership_runtime::{
	AccountId, AuraId, InflationInfo, Precompiles, BLOCKS_PER_YEAR, REVERT_BYTECODE, UNIT,
};
use sc_service::ChainType;
use sp_core::{H160, U256};
use sp_runtime::Perquintill;
use std::{collections::BTreeMap, str::FromStr};

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec =
	sc_service::GenericChainSpec<laos_ownership_runtime::RuntimeGenesisConfig, Extensions>;

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

	// TODO: `from_genesis` will be deprecated in May 2024, use `GenesisBuilder` instead.
	ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || {
			testnet_genesis(
				// initial collators.
				vec![(predefined_accounts::ALITH.into(), get_collator_keys_from_seed("Alice"))],
				predefined_accounts::accounts(),
				// Give Alice root privileges
				Some(predefined_accounts::ALITH.into()),
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
		laos_ownership_runtime::WASM_BINARY.expect("WASM binary was not build, please build it!"),
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
		move || {
			testnet_genesis(
				// initial collators.
				vec![(predefined_accounts::ALITH.into(), get_collator_keys_from_seed("Alice"))],
				predefined_accounts::accounts(),
				// Give Alice root privileges
				Some(predefined_accounts::ALITH.into()),
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
		laos_ownership_runtime::WASM_BINARY.expect("WASM binary was not build, please build it!"),
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
	let reward_configuration = InflationInfo::new(
		BLOCKS_PER_YEAR.into(),
		// max collator staking rate
		Perquintill::from_percent(40),
		// collator reward rate
		Perquintill::from_percent(10),
		// max delegator staking rate
		Perquintill::from_percent(10),
		// delegator reward rate
		Perquintill::from_percent(8),
	);

	laos_ownership_runtime::RuntimeGenesisConfig {
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
					predefined_accounts::FAITH.into(),
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
