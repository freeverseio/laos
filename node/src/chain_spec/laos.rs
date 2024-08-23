// Copyright 2023-2024 Freeverse.io
// This file is part of LAOS.

// LAOS is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// LAOS is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with LAOS.  If not, see <http://www.gnu.org/licenses/>.

use super::{get_collator_keys_from_seed, predefined_accounts, Extensions};
use crate::chain_spec::SAFE_XCM_VERSION;
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
	ChainSpec::builder(
		laos_runtime::WASM_BINARY.expect("WASM binary was not build, please build it!"),
		Extensions { relay_chain: "rococo-local".into(), para_id: PARA_ID },
	)
	.with_name("Development")
	.with_id("dev")
	.with_chain_type(ChainType::Development)
	.with_properties(properties())
	.with_genesis_config_patch(create_test_genesis_config())
	.build()
}

pub(crate) fn local_testnet_config() -> ChainSpec {
	ChainSpec::builder(
		laos_runtime::WASM_BINARY.expect("WASM binary was not build, please build it!"),
		Extensions { relay_chain: "rococo-local".into(), para_id: PARA_ID },
	)
	.with_name("Local Testnet")
	.with_id("laos_local_testnet")
	.with_chain_type(ChainType::Local)
	.with_properties(properties())
	.with_protocol_id("template-local")
	.with_genesis_config_patch(create_test_genesis_config())
	.build()
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

fn create_test_genesis_config() -> serde_json::Value {
	let config = laos_runtime::RuntimeGenesisConfig {
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
				.map(|address| {
					(
						address.into(),
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
		vesting: laos_runtime::VestingConfig {
			vesting: vec![
				(predefined_accounts::ALITH.into(), 0, 100, 1000 * UNIT),
				(predefined_accounts::ALITH.into(), 0, 200, 500 * UNIT),
			],
		},
		..Default::default()
	};
	serde_json::to_value(&config).expect("Could not build genesis config.")
}
