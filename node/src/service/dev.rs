use crate::{
	eth::EthConfiguration,
	service::{new_partial, Block},
};
use laos_primitives::{AccountId, Balance, Nonce};

type HostFunctions = sp_io::SubstrateHostFunctions;

/// Full client type.
type FullClient<RuntimeApi> =
	sc_service::TFullClient<Block, RuntimeApi, sc_executor::WasmExecutor<HostFunctions>>;

pub trait RuntimeApiCollection:
	cumulus_primitives_aura::AuraUnincludedSegmentApi<Block>
	+ cumulus_primitives_core::CollectCollationInfo<Block>
	+ fp_rpc::ConvertTransactionRuntimeApi<Block>
	+ fp_rpc::EthereumRuntimeRPCApi<Block>
	+ pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>
	+ sp_api::ApiExt<Block>
	+ sp_api::Metadata<Block>
	+ sp_block_builder::BlockBuilder<Block>
	+ sp_consensus_aura::AuraApi<Block, <<sp_consensus_aura::sr25519::AuthorityId as sp_runtime::app_crypto::AppCrypto>::Pair as sp_core::Pair>::Public>
	+ sp_offchain::OffchainWorkerApi<Block>
	+ sp_session::SessionKeys<Block>
	+ sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block>
	+ substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Nonce>
{
}
impl<Api> RuntimeApiCollection for Api where
	Api: cumulus_primitives_aura::AuraUnincludedSegmentApi<Block>
		+ cumulus_primitives_core::CollectCollationInfo<Block>
		+ fp_rpc::ConvertTransactionRuntimeApi<Block>
		+ fp_rpc::EthereumRuntimeRPCApi<Block>
		+ pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>
		+ sp_api::ApiExt<Block>
		+ sp_api::Metadata<Block>
		+ sp_block_builder::BlockBuilder<Block>
		+ sp_consensus_aura::AuraApi<Block, <<sp_consensus_aura::sr25519::AuthorityId as sp_runtime::app_crypto::AppCrypto>::Pair as sp_core::Pair>::Public>
		+ sp_offchain::OffchainWorkerApi<Block>
		+ sp_session::SessionKeys<Block>
		+ sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block>
		+ substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Nonce>
{
}

/// Start a dev node which can seal instantly.
/// !!! WARNING: DO NOT USE ELSEWHERE
pub fn start_dev_node<RuntimeApi>(
	mut config: sc_service::Configuration,
	para_id: cumulus_primitives_core::ParaId,
	eth_rpc_config: EthConfiguration,
) -> Result<sc_service::TaskManager, sc_service::error::Error>
where
	RuntimeApi: 'static + Send + Sync + sp_api::ConstructRuntimeApi<Block, FullClient<RuntimeApi>>,
	RuntimeApi::RuntimeApi: RuntimeApiCollection,
	RuntimeApi::RuntimeApi:
		sp_consensus_aura::AuraApi<Block, sp_consensus_aura::sr25519::AuthorityId>,
{
	let sc_service::PartialComponents {
		client,
		backend,
		mut task_manager,
		import_queue,
		keystore_container,
		select_chain,
		transaction_pool,
		other:
			(frontier_backend, filter_pool, fee_history_cache, fee_history_cache_limit, _block_import),
	} = new_partial(&config, &eth_rpc_config)?;

	// let net_config = sc_network::config::FullNetworkConfiguration::new(&config.network);
	// let (network, system_rpc_tx, tx_handler_controller, start_network, sync_service) =
	// 	sc_service::build_network(sc_service::BuildNetworkParams {
	// 		config: &config,
	// 		net_config,
	// 		client: client.clone(),
	// 		transaction_pool: transaction_pool.clone(),
	// 		spawn_handle: task_manager.spawn_handle(),
	// 		import_queue,
	// 		block_announce_validator_builder: None,
	// 		warp_sync_params: None,
	// 		block_relay: None,
	// 		metrics: NotificationMetrics::new(None),
	// 	})?;

	/*

		if config.offchain_worker.enabled {
			task_manager.spawn_handle().spawn(
				"offchain-workers-runner",
				"offchain-work",
				sc_offchain::OffchainWorkers::new(sc_offchain::OffchainWorkerOptions {
					runtime_api_provider: client.clone(),
					keystore: None,
					offchain_db: backend.offchain_storage(),
					transaction_pool: Some(
						sc_transaction_pool_api::OffchainTransactionPoolFactory::new(
							transaction_pool.clone(),
						),
					),
					network_provider: network.clone(),
					is_validator: config.role.is_authority(),
					enable_http_requests: false,
					custom_extensions: move |_| Vec::new(),
				})
				.run(client.clone(), task_manager.spawn_handle())
				.boxed(),
			);
		}

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
		if config.role.is_authority() {
			let aura = sc_consensus_aura::start_aura::<
				sp_consensus_aura::sr25519::AuthorityPair,
				_,
				_,
				_,
				_,
				_,
				_,
				_,
				_,
				_,
				_,
			>(sc_consensus_aura::StartAuraParams {
				slot_duration,
				client: client.clone(),
				select_chain,
				block_import: instant_finalize::InstantFinalizeBlockImport::new(client.clone()),
				proposer_factory,
				create_inherent_data_providers: move |block, ()| {
					let maybe_current_para_block = client_for_cidp.number(block);
					let maybe_current_block_head = client_for_cidp.expect_header(block);
					let client_for_xcm = client_for_cidp.clone();
					// TODO: hack for now.
					let additional_key_values = Some(vec![(
						array_bytes::hex2bytes_unchecked(
							"1cb6f36e027abb2091cfb5110ab5087f06155b3cd9a8c9e5e9a23fd5dc13a5ed",
						),
						cumulus_primitives_aura::Slot::from_timestamp(
							sp_timestamp::Timestamp::current(),
							slot_duration,
						)
						.encode(),
					)]);
					async move {
						let current_para_block = maybe_current_para_block?
							.ok_or(sp_blockchain::Error::UnknownBlock(block.to_string()))?;
						let current_para_block_head =
							Some(polkadot_primitives::HeadData(maybe_current_block_head?.encode()));
						let timestamp = sp_timestamp::InherentDataProvider::from_system_time();
						let slot = sp_consensus_aura::inherents::InherentDataProvider::from_timestamp_and_slot_duration(
							*timestamp,
							slot_duration,
						);
						let mocked_parachain =
							cumulus_client_parachain_inherent::MockValidationDataInherentDataProvider {
								current_para_block,
								para_id,
								current_para_block_head,
								relay_offset: 1000,
								relay_blocks_per_para_block: 2,
								para_blocks_per_relay_epoch: 0,
								relay_randomness_config: (),
								xcm_config: cumulus_client_parachain_inherent::MockXcmConfig::new(
									&*client_for_xcm,
									block,
									Default::default(),
								),
								raw_downward_messages: Vec::new(),
								raw_horizontal_messages: Vec::new(),
								additional_key_values,
							};

						Ok((slot, timestamp, mocked_parachain))
					}
				},
				force_authoring,
				backoff_authoring_blocks,
				keystore: keystore_container.keystore(),
				sync_oracle: sync_service.clone(),
				justification_sync_link: sync_service.clone(),
				// We got around 500ms for proposing
				block_proposal_slot_portion: cumulus_client_consensus_aura::SlotProportion::new(
					1f32 / 24f32,
				),
				// And a maximum of 750ms if slots are skipped
				max_block_proposal_slot_portion: Some(
					cumulus_client_consensus_aura::SlotProportion::new(1f32 / 16f32),
				),
				telemetry: None,
				compatibility_mode: Default::default(),
			})?;

			// the AURA authoring task is considered essential, i.e. if it
			// fails we take down the service with it.
			task_manager
				.spawn_essential_handle()
				.spawn_blocking("aura", Some("block-authoring"), aura);
		} else {
			log::warn!("You could add --alice or --bob to make dev chain seal instantly.");
		}

		let prometheus_registry = config.prometheus_registry().cloned();
		let overrides = fc_storage::overrides_handle(client.clone());
		let block_data_cache = Arc::new(fc_rpc::EthBlockDataCacheTask::new(
			task_manager.spawn_handle(),
			overrides.clone(),
			eth_rpc_config.eth_log_block_cache,
			eth_rpc_config.eth_statuses_cache,
			prometheus_registry.clone(),
		));
		let pubsub_notification_sinks: fc_mapping_sync::EthereumBlockNotificationSinks<
			fc_mapping_sync::EthereumBlockNotification<Block>,
		> = Default::default();
		let pubsub_notification_sinks = Arc::new(pubsub_notification_sinks);
		// for ethereum-compatibility rpc.
		config.rpc_id_provider = Some(Box::new(fc_rpc::EthereumSubIdProvider));
		let tracing_requesters = frontier::spawn_tasks(
			&task_manager,
			client.clone(),
			backend.clone(),
			frontier_backend.clone(),
			filter_pool.clone(),
			overrides.clone(),
			fee_history_cache.clone(),
			fee_history_cache_limit,
			sync_service.clone(),
			pubsub_notification_sinks.clone(),
			eth_rpc_config.clone(),
			prometheus_registry,
		);
		let rpc_extensions_builder = {
			let client = client.clone();
			let pool = transaction_pool.clone();
			let network = network.clone();
			let filter_pool = filter_pool;
			let frontier_backend = frontier_backend;
			let overrides = overrides;
			let fee_history_cache = fee_history_cache;
			let max_past_logs = eth_rpc_config.max_past_logs;
			let collator = config.role.is_authority();
			let eth_rpc_config = eth_rpc_config.clone();
			let sync_service = sync_service.clone();

			let pending_create_inherent_data_providers = move |_, ()| async move {
				let timestamp = sp_timestamp::InherentDataProvider::from_system_time();
				let relay_chain_slot = Slot::from_timestamp(
					timestamp.timestamp(),
					SlotDuration::from_millis(RELAY_CHAIN_SLOT_DURATION_MILLIS as u64),
				);
				// Create a mocked parachain inherent data provider to pass all validations in the
				// parachain system.
				// Without this, the pending functionality will fail.
				let state_proof_builder = cumulus_test_relay_sproof_builder::RelayStateSproofBuilder {
					para_id,
					current_slot: relay_chain_slot,
					included_para_head: Some(polkadot_primitives::HeadData(vec![])),
					..Default::default()
				};
				let (relay_parent_storage_root, relay_chain_state) =
					state_proof_builder.into_state_root_and_proof();
				let parachain_inherent_data =
					cumulus_primitives_parachain_inherent::ParachainInherentData {
						validation_data: cumulus_primitives_core::PersistedValidationData {
							relay_parent_number: u32::MAX,
							relay_parent_storage_root,
							..Default::default()
						},
						relay_chain_state,
						downward_messages: Default::default(),
						horizontal_messages: Default::default(),
					};
				Ok((timestamp, parachain_inherent_data))
			};

			Box::new(move |deny_unsafe, subscription_task_executor| {
				let deps = crate::rpc::FullDeps {
					client: client.clone(),
					pool: pool.clone(),
					graph: pool.pool().clone(),
					deny_unsafe,
					is_authority: collator,
					network: network.clone(),
					sync: sync_service.clone(),
					filter_pool: filter_pool.clone(),
					frontier_backend: match &*frontier_backend {
						fc_db::Backend::KeyValue(bd) => bd.clone(),
						fc_db::Backend::Sql(bd) => bd.clone(),
					},
					max_past_logs,
					fee_history_cache: fee_history_cache.clone(),
					fee_history_cache_limit,
					overrides: overrides.clone(),
					block_data_cache: block_data_cache.clone(),
					forced_parent_hashes: None,
					pending_create_inherent_data_providers,
				};

				if eth_rpc_config.tracing_api.contains(&crate::cli::TracingApi::Debug) ||
					eth_rpc_config.tracing_api.contains(&crate::cli::TracingApi::Trace)
				{
					crate::rpc::create_full::<_, _, _, _, crate::rpc::DefaultEthConfig<_, _>, _>(
						deps,
						subscription_task_executor,
						pubsub_notification_sinks.clone(),
						Some(crate::rpc::TracingConfig {
							tracing_requesters: tracing_requesters.clone(),
							trace_filter_max_count: eth_rpc_config.tracing_max_count,
						}),
					)
					.map_err(Into::into)
				} else {
					crate::rpc::create_full::<_, _, _, _, crate::rpc::DefaultEthConfig<_, _>, _>(
						deps,
						subscription_task_executor,
						pubsub_notification_sinks.clone(),
						None,
					)
					.map_err(Into::into)
				}
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

	*/
	Ok(task_manager)
}
