//! Chain specification for Evochain.

use node_template_runtime::{
	AccountId, AuraConfig, BalancesConfig, GrandpaConfig, RuntimeGenesisConfig, Signature,
	SudoConfig, SystemConfig, WASM_BINARY,
};
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_core::{sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};

/// "Names" of the authorities accounts at local testnet.
const LOCAL_AUTHORITIES_ACCOUNTS: [&str; 5] = ["Alice", "Bob", "Charlie", "Dave", "Eve"];
/// "Names" of the authorities accounts at development testnet.
const DEV_AUTHORITIES_ACCOUNTS: [&str; 1] = [LOCAL_AUTHORITIES_ACCOUNTS[0]];
/// "Names" of all possible authorities accounts.
const ALL_AUTHORITIES_ACCOUNTS: [&str; 5] = LOCAL_AUTHORITIES_ACCOUNTS;
/// "Name" of the `sudo` account.
const SUDO_ACCOUNT: &str = "Alice";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<RuntimeGenesisConfig>;

/// The chain specification option. This is expected to come in from the CLI and
/// is little more than one of a number of alternatives which can easily be converted
/// from a string (`--chain=...`) into a `ChainSpec`.
#[derive(Clone, Debug)]
pub enum Alternative {
	/// Whatever the current runtime is, with just Alice as an auth.
	Development,
	/// Whatever the current runtime is, with simple Alice/Bob/Charlie/Dave/Eve auths.
	LocalTestnet,
}

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{seed}"), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate an authority key for Aura
pub fn get_authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
	(get_from_seed::<AuraId>(s), get_from_seed::<GrandpaId>(s))
}

impl Alternative {
	/// Get an actual chain config from one of the alternatives.
	pub(crate) fn load(self) -> ChainSpec {
		let properties = Some(
			serde_json::json!({
				"tokenDecimals": 10,
				"tokenSymbol": "EVOL"
			})
			.as_object()
			.expect("Map given; qed")
			.clone(),
		);
		match self {
			Alternative::Development => ChainSpec::from_genesis(
				"Evochain Development",
				"evochain_dev",
				sc_service::ChainType::Development,
				|| {
					testnet_genesis(
						DEV_AUTHORITIES_ACCOUNTS
							.into_iter()
							.map(get_authority_keys_from_seed)
							.collect(),
						get_account_id_from_seed::<sr25519::Public>(SUDO_ACCOUNT),
						endowed_accounts(),
						true,
					)
				},
				vec![],
				None,
				None,
				None,
				properties,
				None,
			),
			Alternative::LocalTestnet => ChainSpec::from_genesis(
				"Evochain Local",
				"evochain_local",
				sc_service::ChainType::Local,
				|| {
					testnet_genesis(
						[LOCAL_AUTHORITIES_ACCOUNTS[0], LOCAL_AUTHORITIES_ACCOUNTS[1]]
							.into_iter()
							.map(get_authority_keys_from_seed)
							.collect(),
						get_account_id_from_seed::<sr25519::Public>(SUDO_ACCOUNT),
						endowed_accounts(),
						true,
					)
				},
				vec![],
				None,
				None,
				None,
				properties,
				None,
			),
		}
	}
}

/// We're using the same set of endowed accounts on all Evochain chains (dev/local) to make
/// sure that all accounts, required for bridge to be functional (e.g. relayers fund account,
/// accounts used by relayers in our test deployments, accounts used for demonstration
/// purposes), are all available on these chains.
fn endowed_accounts() -> Vec<AccountId> {
	let all_authorities = ALL_AUTHORITIES_ACCOUNTS.iter().flat_map(|x| {
		[
			get_account_id_from_seed::<sr25519::Public>(x),
			get_account_id_from_seed::<sr25519::Public>(&format!("{x}//stash")),
		]
	});
	vec![
		// Regular (unused) accounts
		get_account_id_from_seed::<sr25519::Public>("Ferdie"),
		get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
	]
	.into_iter()
	.chain(all_authorities)
	.collect()
}

fn testnet_genesis(
	initial_authorities: Vec<(AuraId, GrandpaId)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	_enable_println: bool,
) -> RuntimeGenesisConfig {
	RuntimeGenesisConfig {
		system: SystemConfig {
			code: WASM_BINARY.expect("Evochain development WASM not available").to_vec(),
			..Default::default()
		},
		balances: BalancesConfig {
			balances: endowed_accounts.iter().cloned().map(|k| (k, 1 << 50)).collect(),
		},
		aura: AuraConfig {
			authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
		},
		grandpa: GrandpaConfig {
			authorities: initial_authorities.iter().map(|x| (x.1.clone(), 1)).collect(),
			..Default::default()
		},
		sudo: SudoConfig { key: Some(root_key) },
		..Default::default()
	}
}
