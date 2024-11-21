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

use crate::eth::EthConfiguration;
use cumulus_client_service::storage_proof_size::HostFunctions as ReclaimHostFunctions;
use cumulus_primitives_core::ParaId;
use fc_db::kv::frontier_database_dir;
use fc_rpc::StorageOverrideHandler;
use frame_benchmarking_cli::{BenchmarkCmd, SUBSTRATE_REFERENCE_HARDWARE};
use laos_runtime::Block;
use log::info;
use sc_cli::{Result, SubstrateCli};
use sc_network::NotificationMetrics;
use sc_service::{DatabaseSource, PartialComponents};
use sp_runtime::traits::AccountIdConversion;
use std::sync::Arc;

use crate::{
	chain_spec,
	cli::{Cli, RelayChainCli, Subcommand},
	eth::db_config_dir,
	service::new_partial,
};

macro_rules! construct_async_run {
	(|$components:ident, $cli:ident, $cmd:ident, $config:ident, $eth_config:ident| $( $code:tt )* ) => {{
		let runner = $cli.create_runner($cmd)?;
		runner.async_run(|$config| {
			let $components = new_partial(&$config, &$eth_config)?;
			let task_manager = $components.task_manager;
			{ $( $code )* }.map(|v| (v, task_manager))
		})
	}}
}

/// Parse command line arguments into service configuration.
pub fn run() -> Result<()> {
	let cli = Cli::from_args();
	let eth_cfg = cli.eth.clone();

	match &cli.subcommand {
		Some(Subcommand::BuildSpec(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| cmd.run(config.chain_spec, config.network))
		},
		Some(Subcommand::CheckBlock(cmd)) => {
			construct_async_run!(|components, cli, cmd, config, eth_cfg| {
				Ok(cmd.run(components.client, components.import_queue))
			})
		},
		Some(Subcommand::ExportBlocks(cmd)) => {
			construct_async_run!(|components, cli, cmd, config, eth_cfg| {
				Ok(cmd.run(components.client, config.database))
			})
		},
		Some(Subcommand::ExportState(cmd)) => {
			construct_async_run!(|components, cli, cmd, config, eth_cfg| {
				Ok(cmd.run(components.client, config.chain_spec))
			})
		},
		Some(Subcommand::ImportBlocks(cmd)) => {
			construct_async_run!(|components, cli, cmd, config, eth_cfg| {
				Ok(cmd.run(components.client, components.import_queue))
			})
		},
		Some(Subcommand::Revert(cmd)) => {
			construct_async_run!(|components, cli, cmd, config, eth_cfg| {
				Ok(cmd.run(components.client, components.backend, None))
			})
		},
		Some(Subcommand::PurgeChain(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| {
				// Remove Frontier offchain db
				let db_config_dir = db_config_dir(&config);
				match cli.eth.frontier_backend_type {
					crate::eth::BackendType::KeyValue => {
						let frontier_database_config = match config.database {
							DatabaseSource::RocksDb { .. } => DatabaseSource::RocksDb {
								path: frontier_database_dir(&db_config_dir, "db"),
								cache_size: 0,
							},
							DatabaseSource::ParityDb { .. } => DatabaseSource::ParityDb {
								path: frontier_database_dir(&db_config_dir, "paritydb"),
							},
							_ =>
								return Err(
									format!("Cannot purge `{:?}` database", config.database).into()
								),
						};
						cmd.base.run(frontier_database_config)?;
					},
					crate::eth::BackendType::Sql => {
						let db_path = db_config_dir.join("sql");
						match std::fs::remove_dir_all(&db_path) {
							Ok(_) => {
								println!("{:?} removed.", &db_path);
							},
							Err(ref err) if err.kind() == std::io::ErrorKind::NotFound => {
								eprintln!("{:?} did not exist.", &db_path);
							},
							Err(err) =>
								return Err(format!(
									"Cannot purge `{:?}` database: {:?}",
									db_path, err,
								)
								.into()),
						};
					},
				};

				let polkadot_cli = RelayChainCli::new(
					&config,
					[RelayChainCli::executable_name()].iter().chain(cli.relay_chain_args.iter()),
				);

				let polkadot_config = SubstrateCli::create_configuration(
					&polkadot_cli,
					&polkadot_cli,
					config.tokio_handle.clone(),
				)
				.map_err(|err| format!("Relay chain argument error: {}", err))?;

				cmd.run(config, polkadot_config)
			})
		},
		Some(Subcommand::ExportGenesisHead(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| {
				let partials = new_partial(&config, &eth_cfg)?;
				cmd.run(partials.client)
			})
		},
		Some(Subcommand::ExportGenesisWasm(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|_config| {
				let spec = cli.load_spec(&cmd.shared_params.chain.clone().unwrap_or_default())?;
				cmd.run(&*spec)
			})
		},
		Some(Subcommand::Benchmark(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			// Switch on the concrete benchmark sub-command-
			match cmd {
				BenchmarkCmd::Pallet(cmd) =>
					if cfg!(feature = "runtime-benchmarks") {
						runner.sync_run(|config| cmd.run_with_spec::<sp_runtime::traits::HashingFor<Block>, ReclaimHostFunctions>(Some(config.chain_spec)))
					} else {
						Err("Benchmarking wasn't enabled when building the node. \
					You can enable it with `--features runtime-benchmarks`."
							.into())
					},
				BenchmarkCmd::Block(cmd) => runner.sync_run(|config| {
					let partials = new_partial(&config, &eth_cfg)?;
					cmd.run(partials.client)
				}),
				#[cfg(not(feature = "runtime-benchmarks"))]
				BenchmarkCmd::Storage(_) =>
					return Err(sc_cli::Error::Input(
						"Compile with --features=runtime-benchmarks \
						to enable storage benchmarks."
							.into(),
					)
					.into()),
				#[cfg(feature = "runtime-benchmarks")]
				BenchmarkCmd::Storage(cmd) => runner.sync_run(|config| {
					let partials = new_partial(&config, &eth_cfg)?;
					let db = partials.backend.expose_db();
					let storage = partials.backend.expose_storage();
					cmd.run(config, partials.client.clone(), db, storage)
				}),
				BenchmarkCmd::Machine(cmd) =>
					runner.sync_run(|config| cmd.run(&config, SUBSTRATE_REFERENCE_HARDWARE.clone())),
				// NOTE: this allows the Client to leniently implement
				// new benchmark commands without requiring a companion MR.
				#[allow(unreachable_patterns)]
				_ => Err("Benchmarking sub-command unsupported".into()),
			}
		},
		Some(Subcommand::FrontierDb(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| {
				let PartialComponents { client, other, .. } =
					crate::service::new_partial(&config, &cli.eth)?;
				let (_, _, _, frontier_backend, _) = other;
				let frontier_backend = match frontier_backend {
					fc_db::Backend::KeyValue(kv) => kv,
					_ => panic!("Only fc_db::Backend::KeyValue supported"),
				};
				cmd.run(client, frontier_backend)
			})
		},
		None => {
			let runner = cli.create_runner(&cli.run.normalize())?;
			let collator_options = cli.run.collator_options();

			runner.run_node_until_exit(|config| async move {
				let hwbench = (!cli.no_hardware_benchmarks)
					.then_some(config.database.path().map(|database_path| {
						let _ = std::fs::create_dir_all(database_path);
						sc_sysinfo::gather_hwbench(Some(database_path))
					}))
					.flatten();

				let para_id = chain_spec::Extensions::try_get(&*config.chain_spec)
					.map(|e| e.para_id)
					.ok_or("Could not find parachain ID in chain-spec.")?;

				let id = ParaId::from(para_id);

				if config.chain_spec.id() == "dev" {
					info!("Starting Dev Node");
					return start_dev_node(config, id, eth_cfg).map_err(Into::into);
				}

				let polkadot_cli = RelayChainCli::new(
					&config,
					[RelayChainCli::executable_name()].iter().chain(cli.relay_chain_args.iter()),
				);

				let parachain_account =
					AccountIdConversion::<polkadot_primitives::AccountId>::into_account_truncating(
						&id,
					);

				let tokio_handle = config.tokio_handle.clone();
				let polkadot_config =
					SubstrateCli::create_configuration(&polkadot_cli, &polkadot_cli, tokio_handle)
						.map_err(|err| format!("Relay chain argument error: {}", err))?;

				info!("Parachain id: {:?}", id);
				info!("Parachain Account: {}", parachain_account);
				info!("Is collating: {}", if config.role.is_authority() { "yes" } else { "no" });

				crate::service::start_parachain_node(
					config,
					polkadot_config,
					eth_cfg,
					collator_options,
					id,
					hwbench,
				)
				.await
				.map(|r| r.0)
				.map_err(Into::into) // Convert errors into a compatible format
			})
		},
	}
}

fn start_dev_node(
	mut config: sc_service::Configuration,
	para_id: ParaId,
	eth_config: EthConfiguration,
) -> Result<sc_service::TaskManager> {
	let PartialComponents {
		client,
		backend,
		task_manager,
		import_queue,
		keystore_container,
		transaction_pool,
		other: (block_import, telemetry, telemetry_worker_handle, frontier_backend, overrides),
		..
	} = new_partial(&config, &eth_config)?;

	let net_config =
		sc_network::config::FullNetworkConfiguration::<_, _, sc_network::NetworkWorker<_, _>>::new(
			&config.network,
		);

	let (network, system_rpc_tx, tx_handler_controller, start_network, sync_service) =
		sc_service::build_network(sc_service::BuildNetworkParams {
			config: &config,
			client: client.clone(),
			transaction_pool: transaction_pool.clone(),
			net_config,
			spawn_handle: task_manager.spawn_handle(),
			import_queue,
			block_announce_validator_builder: None,
			warp_sync_params: None,
			block_relay: None,
			metrics: NotificationMetrics::new(None),
		})?;

	let frontier_backend = Arc::new(frontier_backend);
	let force_authoring = config.force_authoring;
	let backoff_authoring_blocks = None::<()>;
	let slot_duration = sc_consensus_aura::slot_duration(&*client)?;

	let proposer_factory = sc_basic_authorship::ProposerFactory::new(
		task_manager.spawn_handle(),
		client.clone(),
		transaction_pool.clone(),
		None,
		None,
	);

	let client_for_cidp = client.clone();

	let prometheus_registry = config.prometheus_registry().cloned();
	let overrides = Arc::new(StorageOverrideHandler::new(client.clone()));
	let block_data_cache = Arc::new(fc_rpc::EthBlockDataCacheTask::new(
		task_manager.spawn_handle(),
		overrides.clone(),
		eth_config.eth_log_block_cache,
		eth_config.eth_statuses_cache,
		prometheus_registry.clone(),
	));

	let pubsub_notification_sinks: fc_mapping_sync::EthereumBlockNotificationSinks<
		fc_mapping_sync::EthereumBlockNotification<Block>,
	> = Default::default();
	let pubsub_notification_sinks = Arc::new(pubsub_notification_sinks);
	// for ethereum-compatibility rpc.
	config.rpc_id_provider = Some(Box::new(fc_rpc::EthereumSubIdProvider));

	let rpc_extensions_builder = {
		let client = client.clone();
		let transaction_pool = transaction_pool.clone();
		let pubsub_notification_sinks = pubsub_notification_sinks.clone();

		Box::new(move |deny_unsafe, subscription_task_executor| {
			let deps = crate::rpc::FullDeps {
				client: client.clone(),
				pool: transaction_pool.clone(),
				deny_unsafe,
				eth: eth_rpc_params.clone(),
			};

			crate::rpc::create_full(
				deps,
				subscription_task_executor,
				pubsub_notification_sinks.clone(),
			)
			.map_err(Into::into)
		})
	};

	sc_service::spawn_tasks(sc_service::SpawnTasksParams {
		rpc_builder: Box::new(rpc_extensions_builder),
		client,
		transaction_pool,
		task_manager: &mut task_manager,
		config,
		keystore: keystore_container.keystore(),
		backend,
		network,
		sync_service,
		system_rpc_tx,
		tx_handler_controller,
		telemetry: None,
	})?;

	start_network.start_network();

	Ok(task_manager)
}
