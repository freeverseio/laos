//! Rococo parachain specification for CLI.

use crate::cli::CliChain;
use relay_rococo_client::Rococo;
use relay_substrate_client::SimpleRuntimeVersion;

impl CliChain for Rococo {
	const RUNTIME_VERSION: Option<SimpleRuntimeVersion> = None;
}
