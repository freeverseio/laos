//! Service and ServiceFactory implementation. Specialized wrapper over substrate service.
use jsonrpsee::RpcModule;
use laos_evolution_runtime::{opaque::Block, RuntimeApi};
use sc_client_api::{Backend, BlockBackend};
use sc_consensus_aura::{CompatibilityMode, ImportQueueParams, SlotProportion, StartAuraParams};
use sc_consensus_grandpa::SharedVoterState;
pub use sc_executor::NativeElseWasmExecutor;
use sc_executor::{HeapAllocStrategy, WasmExecutor, DEFAULT_HEAP_ALLOC_STRATEGY};
use sc_service::{error::Error as ServiceError, Configuration, TaskManager};
use sc_telemetry::{Telemetry, TelemetryWorker};
use sc_transaction_pool_api::OffchainTransactionPoolFactory;
use sp_consensus_aura::sr25519::AuthorityPair as AuraPair;
use std::{sync::Arc, time::Duration};

// Our native executor instance.
pub struct ExecutorDispatch;

impl sc_executor::NativeExecutionDispatch for ExecutorDispatch {
	/// Only enable the benchmarking host functions when we actually want to benchmark.
	#[cfg(feature = "runtime-benchmarks")]
	type ExtendHostFunctions = frame_benchmarking::benchmarking::HostFunctions;
	/// Otherwise we only use the default Substrate host functions.
	#[cfg(not(feature = "runtime-benchmarks"))]
	type ExtendHostFunctions = ();

	fn dispatch(method: &str, data: &[u8]) -> Option<Vec<u8>> {
		laos_evolution_runtime::api::dispatch(method, data)
	}

	fn native_version() -> sc_executor::NativeVersion {
		laos_evolution_runtime::native_version()
	}
}

pub(crate) type FullClient =
	sc_service::TFullClient<Block, RuntimeApi, NativeElseWasmExecutor<ExecutorDispatch>>;
type FullBackend = sc_service::TFullBackend<Block>;
type FullSelectChain = sc_consensus::LongestChain<FullBackend, Block>;

#[allow(clippy::type_complexity)]
pub fn new_partial(
	config: &Configuration,
) -> Result<
	sc_service::PartialComponents<
		FullClient,
		FullBackend,
		FullSelectChain,
		sc_consensus::DefaultImportQueue<Block, FullClient>,
		sc_transaction_pool::FullPool<Block, FullClient>,
		(
			sc_consensus_grandpa::GrandpaBlockImport<
				FullBackend,
				Block,
				FullClient,
				FullSelectChain,
			>,
			sc_consensus_grandpa::LinkHalf<Block, FullClient, FullSelectChain>,
			Option<Telemetry>,
		),
	>,
	ServiceError,
> {
	let telemetry = config
		.telemetry_endpoints
		.clone()
		.filter(|x| !x.is_empty())
		.map(|endpoints| -> Result<_, sc_telemetry::Error> {
			let worker = TelemetryWorker::new(16)?;
			let telemetry = worker.handle().new_telemetry(endpoints);
			Ok((worker, telemetry))
		})
		.transpose()?;

	let heap_pages = config
		.default_heap_pages
		.map_or(DEFAULT_HEAP_ALLOC_STRATEGY, |h| HeapAllocStrategy::Static { extra_pages: h as _ });
	let executor = NativeElseWasmExecutor::<ExecutorDispatch>::new_with_wasm_executor(
		WasmExecutor::builder()
			.with_execution_method(config.wasm_method)
			.with_onchain_heap_alloc_strategy(heap_pages)
			.with_offchain_heap_alloc_strategy(heap_pages)
			.with_max_runtime_instances(config.max_runtime_instances)
			.with_runtime_cache_size(config.runtime_cache_size)
			.build(),
	);

	let (client, backend, keystore_container, task_manager) =
		sc_service::new_full_parts::<Block, RuntimeApi, _>(
			config,
			telemetry.as_ref().map(|(_, telemetry)| telemetry.handle()),
			executor,
		)?;
	let client = Arc::new(client);

	let telemetry = telemetry.map(|(worker, telemetry)| {
		task_manager.spawn_handle().spawn("telemetry", None, worker.run());
		telemetry
	});

	let select_chain = sc_consensus::LongestChain::new(backend.clone());

	let transaction_pool = sc_transaction_pool::BasicPool::new_full(
		config.transaction_pool.clone(),
		config.role.is_authority().into(),
		config.prometheus_registry(),
		task_manager.spawn_essential_handle(),
		client.clone(),
	);

	let (grandpa_block_import, grandpa_link) = sc_consensus_grandpa::block_import(
		client.clone(),
		&client,
		select_chain.clone(),
		telemetry.as_ref().map(|x| x.handle()),
	)?;

	let slot_duration = sc_consensus_aura::slot_duration(&*client)?;

	let import_queue =
		sc_consensus_aura::import_queue::<AuraPair, _, _, _, _, _>(ImportQueueParams {
			block_import: grandpa_block_import.clone(),
			justification_import: Some(Box::new(grandpa_block_import.clone())),
			client: client.clone(),
			create_inherent_data_providers: move |_, ()| async move {
				let timestamp = sp_timestamp::InherentDataProvider::from_system_time();

				let slot =
					sp_consensus_aura::inherents::InherentDataProvider::from_timestamp_and_slot_duration(
						*timestamp,
						slot_duration,
					);

				Ok((slot, timestamp))
			},
			spawner: &task_manager.spawn_essential_handle(),
			registry: config.prometheus_registry(),
			check_for_equivocation: Default::default(),
			telemetry: telemetry.as_ref().map(|x| x.handle()),
			compatibility_mode: CompatibilityMode::None,
		})?;

	Ok(sc_service::PartialComponents {
		client,
		backend,
		task_manager,
		import_queue,
		keystore_container,
		select_chain,
		transaction_pool,
		other: (grandpa_block_import, grandpa_link, telemetry),
	})
}

/// Builds a new service for a full client.
pub fn new_full(config: Configuration) -> Result<TaskManager, ServiceError> {
	use sc_network_common::sync::warp::WarpSyncParams;

	let sc_service::PartialComponents {
		client,
		backend,
		mut task_manager,
		import_queue,
		keystore_container,
		select_chain,
		transaction_pool,
		other: (block_import, grandpa_link, mut telemetry),
	} = new_partial(&config)?;

	let mut net_config = sc_network::config::FullNetworkConfiguration::new(&config.network);

	// Note: GrandPa is pushed before the Polkadot-specific protocols. This doesn't change
	// anything in terms of behaviour, but makes the logs more consistent with the other
	// Substrate nodes.
	let grandpa_protocol_name = sc_consensus_grandpa::protocol_standard_name(
		&client.block_hash(0).ok().flatten().expect("Genesis block exists; qed"),
		&config.chain_spec,
	);
	net_config.add_notification_protocol(sc_consensus_grandpa::grandpa_peers_set_config(
		grandpa_protocol_name.clone(),
	));

	let role = config.role.clone();
	let warp_sync = Arc::new(sc_consensus_grandpa::warp_proof::NetworkProvider::new(
		backend.clone(),
		grandpa_link.shared_authority_set().clone(),
		Vec::default(),
	));

	let (network, system_rpc_tx, tx_handler_controller, network_starter, sync_service) =
		sc_service::build_network(sc_service::BuildNetworkParams {
			config: &config,
			net_config,
			client: client.clone(),
			transaction_pool: transaction_pool.clone(),
			spawn_handle: task_manager.spawn_handle(),
			import_queue,
			block_announce_validator_builder: None,
			warp_sync_params: Some(WarpSyncParams::WithProvider(warp_sync)),
		})?;

	if config.offchain_worker.enabled {
		use futures::FutureExt;

		task_manager.spawn_handle().spawn(
			"offchain-workers-runner",
			"offchain-work",
			sc_offchain::OffchainWorkers::new(sc_offchain::OffchainWorkerOptions {
				runtime_api_provider: client.clone(),
				keystore: Some(keystore_container.keystore()),
				offchain_db: backend.offchain_storage(),
				transaction_pool: Some(OffchainTransactionPoolFactory::new(
					transaction_pool.clone(),
				)),
				network_provider: network.clone(),
				is_validator: role.is_authority(),
				enable_http_requests: true,
				custom_extensions: move |_| vec![],
			})
			.run(client.clone(), task_manager.spawn_handle())
			.boxed(),
		);
	}

	let force_authoring = config.force_authoring;
	let backoff_authoring_blocks: Option<()> = None;
	let name = config.network.node_name.clone();
	let enable_grandpa = !config.disable_grandpa;
	let prometheus_registry = config.prometheus_registry().cloned();
	let shared_voter_state = SharedVoterState::empty();

	let rpc_extensions_builder = {
		use sc_consensus_grandpa::FinalityProofProvider as GrandpaFinalityProofProvider;

		use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApiServer};
		use sc_consensus_grandpa_rpc::{Grandpa, GrandpaApiServer};
		use sc_rpc::DenyUnsafe;
		use substrate_frame_rpc_system::{System, SystemApiServer};

		let backend = backend.clone();
		let client = client.clone();
		let pool = transaction_pool.clone();

		let justification_stream = grandpa_link.justification_stream();
		let shared_authority_set = grandpa_link.shared_authority_set().clone();
		let shared_voter_state = shared_voter_state.clone();

		let finality_proof_provider = GrandpaFinalityProofProvider::new_for_service(
			backend.clone(),
			Some(shared_authority_set.clone()),
		);

		Box::new(move |_, subscription_executor: sc_rpc::SubscriptionTaskExecutor| {
			let mut io = RpcModule::new(());
			let map_err = |e| sc_service::Error::Other(format!("{e}"));
			io.merge(System::new(client.clone(), pool.clone(), DenyUnsafe::No).into_rpc())
				.map_err(map_err)?;
			io.merge(TransactionPayment::new(client.clone()).into_rpc()).map_err(map_err)?;
			io.merge(
				Grandpa::new(
					subscription_executor.clone(),
					shared_authority_set.clone(),
					shared_voter_state.clone(),
					justification_stream.clone(),
					finality_proof_provider.clone(),
				)
				.into_rpc(),
			)
			.map_err(map_err)?;

			Ok(io)
		})
	};

	let _rpc_handlers = sc_service::spawn_tasks(sc_service::SpawnTasksParams {
		network: network.clone(),
		client: client.clone(),
		keystore: keystore_container.keystore(),
		task_manager: &mut task_manager,
		transaction_pool: transaction_pool.clone(),
		sync_service: sync_service.clone(),
		rpc_builder: rpc_extensions_builder,
		backend: backend.clone(),
		system_rpc_tx,
		config,
		tx_handler_controller,
		telemetry: telemetry.as_mut(),
	})?;

	if role.is_authority() {
		let proposer_factory = sc_basic_authorship::ProposerFactory::new(
			task_manager.spawn_handle(),
			client.clone(),
			transaction_pool.clone(),
			prometheus_registry.as_ref(),
			telemetry.as_ref().map(|x| x.handle()),
		);

		let slot_duration = sc_consensus_aura::slot_duration(&*client)?;

		let aura = sc_consensus_aura::start_aura::<AuraPair, _, _, _, _, _, _, _, _, _, _>(
			StartAuraParams {
				slot_duration,
				client: client.clone(),
				select_chain,
				block_import,
				proposer_factory,
				create_inherent_data_providers: move |_, ()| async move {
					let timestamp = sp_timestamp::InherentDataProvider::from_system_time();

					let slot =
						sp_consensus_aura::inherents::InherentDataProvider::from_timestamp_and_slot_duration(
							*timestamp,
							slot_duration,
						);

					Ok((slot, timestamp))
				},
				force_authoring,
				backoff_authoring_blocks,
				keystore: keystore_container.keystore(),
				sync_oracle: sync_service.clone(),
				justification_sync_link: sync_service.clone(),
				block_proposal_slot_portion: SlotProportion::new(2f32 / 3f32),
				max_block_proposal_slot_portion: None,
				telemetry: telemetry.as_ref().map(|x| x.handle()),
				compatibility_mode: CompatibilityMode::None,
			},
		)?;

		// the AURA authoring task is considered essential, i.e. if it
		// fails we take down the service with it.
		task_manager
			.spawn_essential_handle()
			.spawn_blocking("aura", Some("block-authoring"), aura);
	}

	// if the node isn't actively participating in consensus then it doesn't
	// need a keystore, regardless of which protocol we use below.
	let keystore = if role.is_authority() { Some(keystore_container.keystore()) } else { None };

	let grandpa_config = sc_consensus_grandpa::Config {
		// FIXME #1578 make this available through chainspec
		gossip_duration: Duration::from_millis(333),
		justification_period: 512,
		name: Some(name),
		observer_enabled: false,
		keystore,
		local_role: role,
		telemetry: telemetry.as_ref().map(|x| x.handle()),
		protocol_name: grandpa_protocol_name,
	};

	if enable_grandpa {
		// start the full GRANDPA voter
		// NOTE: non-authorities could run the GRANDPA observer protocol, but at
		// this point the full voter should provide better guarantees of block
		// and vote data availability than the observer. The observer has not
		// been tested extensively yet and having most nodes in a network run it
		// could lead to finality stalls.
		let grandpa_config = sc_consensus_grandpa::GrandpaParams {
			config: grandpa_config,
			link: grandpa_link,
			network,
			sync: sync_service,
			voting_rule: sc_consensus_grandpa::VotingRulesBuilder::default().build(),
			prometheus_registry,
			shared_voter_state,
			telemetry: telemetry.as_ref().map(|x| x.handle()),
			offchain_tx_pool_factory: OffchainTransactionPoolFactory::new(transaction_pool),
		};

		// the GRANDPA voter task is considered infallible, i.e.
		// if it fails we take down the service with it.
		task_manager.spawn_essential_handle().spawn_blocking(
			"grandpa-voter",
			None,
			sc_consensus_grandpa::run_grandpa_voter(grandpa_config)?,
		);
	}

	network_starter.start_network();
	Ok(task_manager)
}
