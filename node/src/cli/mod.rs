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

mod relay;

pub use relay::RelayChainCli;

use crate::{chain_spec, eth::EthConfiguration};
use sc_cli::SubstrateCli;

/// Sub-commands supported by the collator.
#[derive(Debug, clap::Subcommand)]
#[allow(clippy::large_enum_variant)]
pub enum Subcommand {
	/// Build a chain specification.
	BuildSpec(sc_cli::BuildSpecCmd),

	/// Validate blocks.
	CheckBlock(sc_cli::CheckBlockCmd),

	/// Export blocks.
	ExportBlocks(sc_cli::ExportBlocksCmd),

	/// Export the state of a given block into a chain spec.
	ExportState(sc_cli::ExportStateCmd),

	/// Import blocks.
	ImportBlocks(sc_cli::ImportBlocksCmd),

	/// Revert the chain to a previous state.
	Revert(sc_cli::RevertCmd),

	/// Remove the whole chain.
	PurgeChain(cumulus_client_cli::PurgeChainCmd),

	/// Export the genesis head data of the parachain.
	/// Head data is the encoded block header.
	#[command(alias = "export-genesis-state")]
	ExportGenesisHead(cumulus_client_cli::ExportGenesisHeadCommand),

	/// Export the genesis wasm of the parachain.
	ExportGenesisWasm(cumulus_client_cli::ExportGenesisWasmCommand),

	/// Sub-commands concerned with benchmarking.
	/// The pallet benchmarking moved to the `pallet` sub-command.
	#[command(subcommand)]
	Benchmark(frame_benchmarking_cli::BenchmarkCmd),

	/// Db meta columns information.
	FrontierDb(fc_cli::FrontierDbCmd),
}

#[derive(Debug, clap::Parser)]
#[command(
	propagate_version = true,
	args_conflicts_with_subcommands = true,
	subcommand_negates_reqs = true
)]
pub struct Cli {
	#[command(subcommand)]
	pub subcommand: Option<Subcommand>,

	#[command(flatten)]
	pub run: cumulus_client_cli::RunCmd,

	/// Disable automatic hardware benchmarks.
	///
	/// By default these benchmarks are automatically ran at startup and measure
	/// the CPU speed, the memory bandwidth and the disk speed.
	///
	/// The results are then printed out in the logs, and also sent as part of
	/// telemetry, if telemetry is enabled.
	#[arg(long)]
	pub no_hardware_benchmarks: bool,

	/// Relay chain arguments
	#[arg(raw = true)]
	pub relay_chain_args: Vec<String>,

	// Frontier arguments
	#[command(flatten)]
	pub eth: EthConfiguration,
}

impl SubstrateCli for Cli {
	fn impl_name() -> String {
		"LAOS Parachain Node".into()
	}

	fn impl_version() -> String {
		env!("SUBSTRATE_CLI_IMPL_VERSION").into()
	}

	fn description() -> String {
		format!(
			"LAOS Parachain Node\n\nThe command-line arguments provided first will be \
		passed to the parachain node, while the arguments provided after -- will be passed \
		to the relay chain node.\n\n\
		{} <parachain-args> -- <relay-chain-args>",
			Self::executable_name()
		)
	}

	fn author() -> String {
		env!("CARGO_PKG_AUTHORS").into()
	}

	fn support_url() -> String {
		"https://github.com/freeverseio/laos/issues/new".into()
	}

	fn copyright_start_year() -> i32 {
		2023
	}

	fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
		Ok(match id {
			"laos" | "" => Box::new(chain_spec::laos::ChainSpec::from_json_bytes(
				&include_bytes!("../../../specs/laos.raw.json")[..],
			)?),
			"laos-sigma" => Box::new(chain_spec::laos::ChainSpec::from_json_bytes(
				&include_bytes!("../../../specs/laos-sigma.raw.json")[..],
			)?),
			"laos-mercury" => Box::new(chain_spec::laos::ChainSpec::from_json_bytes(
				&include_bytes!("../../../specs/laos-mercury.raw.json")[..],
			)?),
			"dev" => Box::new(chain_spec::laos::development_config()),
			"local" => Box::new(chain_spec::laos::local_testnet_config()),
			path => {
				let chain_spec =
					chain_spec::laos::ChainSpec::from_json_file(std::path::PathBuf::from(path))?;
				if chain_spec.id().starts_with("laos") {
					Box::new(chain_spec)
				} else {
					Err(format!(
					"Unclear which chain spec to base this chain on. Provided chain spec ID: {}",
					chain_spec.id()
				))?
				}
			},
		})
	}
}
