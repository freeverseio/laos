use polkadot_service::RococoChainSpec;
use sc_cli::{
	ChainSpec, CliConfiguration, DefaultConfigurationValues, ImportParams, KeystoreParams,
	NetworkParams, Result, SharedParams, SubstrateCli,
};
use sc_service::config::{BasePath, PrometheusConfig};
use std::{net::SocketAddr, path::PathBuf};

/// Represents the command-line interface for the relay chain node.
#[derive(Debug)]
pub struct RelayChainCli {
	/// The actual relay chain CLI command.
	pub base: polkadot_cli::RunCmd,

	/// Optional chain identifier to pass to the relay chain.
	pub chain_id: Option<String>,

	/// The base path to be used by the relay chain node.
	pub base_path: Option<PathBuf>,
}

impl RelayChainCli {
	/// Constructs a new `RelayChainCli` by parsing relay chain CLI parameters.
	///
	/// This function creates a new `RelayChainCli` instance using the provided parachain
	/// configuration and relay chain arguments. It attempts to extract the relay chain ID from the
	/// parachain's chain specification extensions and sets the base path for the relay chain node
	/// accordingly.
	///
	/// # Arguments
	///
	/// * `para_config` - A reference to the parachain node's configuration.
	/// * `relay_chain_args` - An iterator over the relay chain CLI arguments.
	///
	/// # Returns
	///
	/// A new instance of `RelayChainCli`.
	pub fn new<'a>(
		para_config: &sc_service::Configuration,
		relay_chain_args: impl Iterator<Item = &'a String>,
	) -> Self {
		// Attempt to retrieve the relay chain ID from the parachain chain spec extensions.
		let chain_spec_extension = crate::chain_spec::Extensions::try_get(&*para_config.chain_spec);
		let chain_id = chain_spec_extension.map(|ext| ext.relay_chain.clone());

		// Set the base path for the relay chain node to be a subdirectory of the parachain base
		// path.
		let base_path = para_config.base_path.path().join("polkadot");

		Self {
			base_path: Some(base_path),
			chain_id,
			// Parse the relay chain CLI arguments into the base `RunCmd` structure.
			base: clap::Parser::parse_from(relay_chain_args),
		}
	}
}

impl SubstrateCli for RelayChainCli {
	/// Returns the implementation name of the node.
	fn impl_name() -> String {
		"LAOS Parachain Node".into()
	}

	/// Returns the implementation version of the node.
	fn impl_version() -> String {
		env!("SUBSTRATE_CLI_IMPL_VERSION").into()
	}

	/// Returns the description of the node.
	fn description() -> String {
		format!(
			"LAOS Parachain Node\n\n\
            The command-line arguments provided first will be \
            passed to the parachain node, while the arguments provided after '--' will be passed \
            to the relay chain node.\n\n\
            {} <parachain-args> -- <relay-chain-args>",
			Self::executable_name()
		)
	}

	/// Returns the author information.
	fn author() -> String {
		env!("CARGO_PKG_AUTHORS").into()
	}

	/// Returns the support URL.
	fn support_url() -> String {
		"https://github.com/freeverseio/laos/issues/new".into()
	}

	/// Returns the starting year for the copyright notice.
	fn copyright_start_year() -> i32 {
		2020
	}

	/// Loads the chain specification based on the given identifier.
	///
	/// This method attempts to load a chain specification matching the provided `id`.
	/// It supports custom chain specs defined in the local `specs` directory.
	///
	/// # Arguments
	///
	/// * `id` - The identifier of the chain specification to load.
	///
	/// # Returns
	///
	/// A result containing the loaded chain specification or an error message.
	fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
		match id {
			// Load the 'paseo' chain specification from the included JSON file.
			"paseo" => Ok(Box::new(RococoChainSpec::from_json_bytes(
				&include_bytes!("../../../specs/paseo.raw.json")[..],
			)?)),
			// For other identifiers, delegate to the default Polkadot CLI loader.
			_ => polkadot_cli::Cli::from_iter([RelayChainCli::executable_name()].iter())
				.load_spec(id),
		}
	}
}

impl DefaultConfigurationValues for RelayChainCli {
	fn p2p_listen_port() -> u16 {
		30334
	}

	fn rpc_listen_port() -> u16 {
		9945
	}

	fn prometheus_listen_port() -> u16 {
		9616
	}
}

impl CliConfiguration<Self> for RelayChainCli {
	fn shared_params(&self) -> &SharedParams {
		self.base.base.shared_params()
	}

	fn import_params(&self) -> Option<&ImportParams> {
		self.base.base.import_params()
	}

	fn network_params(&self) -> Option<&NetworkParams> {
		self.base.base.network_params()
	}

	fn keystore_params(&self) -> Option<&KeystoreParams> {
		self.base.base.keystore_params()
	}

	fn base_path(&self) -> Result<Option<BasePath>> {
		Ok(self
			.shared_params()
			.base_path()?
			.or_else(|| self.base_path.clone().map(Into::into)))
	}

	fn rpc_addr(&self, default_listen_port: u16) -> Result<Option<SocketAddr>> {
		self.base.base.rpc_addr(default_listen_port)
	}

	fn prometheus_config(
		&self,
		default_listen_port: u16,
		chain_spec: &Box<dyn ChainSpec>,
	) -> Result<Option<PrometheusConfig>> {
		self.base.base.prometheus_config(default_listen_port, chain_spec)
	}

	fn init<F>(
		&self,
		_support_url: &String,
		_impl_version: &String,
		_logger_hook: F,
		_config: &sc_service::Configuration,
	) -> Result<()>
	where
		F: FnOnce(&mut sc_cli::LoggerBuilder, &sc_service::Configuration),
	{
		unreachable!("PolkadotCli is never initialized; qed");
	}

	fn chain_id(&self, is_dev: bool) -> Result<String> {
		let chain_id = self.base.base.chain_id(is_dev)?;

		Ok(if chain_id.is_empty() { self.chain_id.clone().unwrap_or_default() } else { chain_id })
	}

	fn role(&self, is_dev: bool) -> Result<sc_service::Role> {
		self.base.base.role(is_dev)
	}

	fn transaction_pool(&self, is_dev: bool) -> Result<sc_service::config::TransactionPoolOptions> {
		self.base.base.transaction_pool(is_dev)
	}

	fn trie_cache_maximum_size(&self) -> Result<Option<usize>> {
		self.base.base.trie_cache_maximum_size()
	}

	fn rpc_methods(&self) -> Result<sc_service::config::RpcMethods> {
		self.base.base.rpc_methods()
	}

	fn rpc_max_connections(&self) -> Result<u32> {
		self.base.base.rpc_max_connections()
	}

	fn rpc_cors(&self, is_dev: bool) -> Result<Option<Vec<String>>> {
		self.base.base.rpc_cors(is_dev)
	}

	fn default_heap_pages(&self) -> Result<Option<u64>> {
		self.base.base.default_heap_pages()
	}

	fn force_authoring(&self) -> Result<bool> {
		self.base.base.force_authoring()
	}

	fn disable_grandpa(&self) -> Result<bool> {
		self.base.base.disable_grandpa()
	}

	fn max_runtime_instances(&self) -> Result<Option<usize>> {
		self.base.base.max_runtime_instances()
	}

	fn announce_block(&self) -> Result<bool> {
		self.base.base.announce_block()
	}

	fn telemetry_endpoints(
		&self,
		chain_spec: &Box<dyn ChainSpec>,
	) -> Result<Option<sc_telemetry::TelemetryEndpoints>> {
		self.base.base.telemetry_endpoints(chain_spec)
	}

	fn node_name(&self) -> Result<String> {
		self.base.base.node_name()
	}
}
