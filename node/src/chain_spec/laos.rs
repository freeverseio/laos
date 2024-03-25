use super::{get_collator_keys_from_seed, predefined_accounts, Extensions, SAFE_XCM_VERSION};
use fp_evm::GenesisAccount;
use laos_runtime::{
	configs::system::SS58Prefix,
	currency::{DECIMALS, UNIT},
	AuraId, Precompiles, REVERT_BYTECODE,
};
use sc_service::ChainType;
use sp_runtime::Perbill;

pub(crate) type ChainSpec =
	sc_service::GenericChainSpec<laos_runtime::RuntimeGenesisConfig, Extensions>;

pub(crate) fn development_config() -> ChainSpec {
	generic_chain_config("Development", "dev", ChainType::Development, None)
}

pub(crate) fn local_testnet_config() -> ChainSpec {
	generic_chain_config(
		"Local Testnet",
		"laos_local_testnet",
		ChainType::Local,
		Some("template-local"),
	)
}

const PARA_ID: u32 = 2001;
const EVM_CHAIN_ID: u32 = 667;

/// Generate the session keys from individual elements.
///
/// The input must be a tuple of individual keys (a single arg for now since we have just one key).
fn template_session_keys(keys: AuraId) -> laos_runtime::SessionKeys {
	laos_runtime::SessionKeys { aura: keys }
}

// function for properties
fn properties() -> sc_chain_spec::Properties {
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "UNIT".into());
	properties.insert("tokenDecimals".into(), DECIMALS.into());
	properties.insert("ss58Format".into(), SS58Prefix::get().into());
	properties
}

fn generic_chain_config(
	name: &str,
	id: &str,
	chain_type: ChainType,
	protocol_id: Option<&str>,
) -> ChainSpec {
	ChainSpec::from_genesis(
		name,
		id,
		chain_type,
		move || create_test_genesis_config(),
		Vec::new(),
		None,
		protocol_id,
		None,
		Some(properties()),
		Extensions { relay_chain: "rococo-local".into(), para_id: PARA_ID.into() },
	)
}

fn create_test_genesis_config() -> laos_runtime::RuntimeGenesisConfig {
	laos_runtime::RuntimeGenesisConfig {
		system: laos_runtime::SystemConfig {
			code: laos_runtime::WASM_BINARY
				.expect("WASM binary was not build, please build it!")
				.to_vec(),
			..Default::default()
		},
		balances: laos_runtime::BalancesConfig {
			balances: vec![
				(predefined_accounts::ALITH.into(), 800000000 * UNIT),
				(predefined_accounts::BALTATHAR.into(), 150000000 * UNIT),
				(predefined_accounts::FAITH.into(), 50000000 * UNIT),
			],
		},
		parachain_info: laos_runtime::ParachainInfoConfig {
			parachain_id: PARA_ID.into(),
			..Default::default()
		},
		session: laos_runtime::SessionConfig {
			keys: vec![(
				predefined_accounts::ALITH.into(),
				predefined_accounts::ALITH.into(),
				template_session_keys(get_collator_keys_from_seed("Alice")),
			)],
		},
		polkadot_xcm: laos_runtime::PolkadotXcmConfig {
			safe_xcm_version: Some(SAFE_XCM_VERSION),
			..Default::default()
		},
		sudo: laos_runtime::SudoConfig { key: Some(predefined_accounts::ALITH.into()) },
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
		evm_chain_id: laos_runtime::EVMChainIdConfig {
			chain_id: EVM_CHAIN_ID.into(),
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
