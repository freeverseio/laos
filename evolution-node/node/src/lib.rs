mod chain_spec;
#[macro_use]
mod service;
mod cli;
mod command;
mod rpc;

/// Node run result.
pub type Result = sc_cli::Result<()>;

/// Run node.
pub fn run() -> Result {
	command::run()
}
