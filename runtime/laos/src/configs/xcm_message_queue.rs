use crate::{weights, Runtime, RuntimeEvent, XcmpQueue};
use cumulus_primitives_core::AggregateMessageOrigin;
use frame_support::parameter_types;
use laos_primitives::BlockWeights;
use parachains_common::message_queue::NarrowOriginToSibling;
use sp_core::ConstU32;
use sp_runtime::Perbill;
use sp_weights::Weight;

parameter_types! {
	/// The amount of weight (if any) which should be provided to the message queue for
	/// servicing enqueued items.
	///
	/// This may be legitimately `None` in the case that you will call
	/// `ServiceQueues::service_queues` manually.
	pub MessageQueueServiceWeight: Weight =
	Perbill::from_percent(25) * BlockWeights::get().max_block;
}

impl pallet_message_queue::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	#[cfg(feature = "runtime-benchmarks")]
	type MessageProcessor =
		pallet_message_queue::mock_helpers::NoopMessageProcessor<AggregateMessageOrigin>;
	#[cfg(not(feature = "runtime-benchmarks"))]
	type MessageProcessor = staging_xcm_builder::ProcessXcmMessage<
		AggregateMessageOrigin,
		staging_xcm_executor::XcmExecutor<super::xcm_config::XcmConfig>,
		crate::RuntimeCall,
	>;
	type Size = u32;
	type HeapSize = ConstU32<{ 64 * 1024 }>;
	type MaxStale = ConstU32<8>;
	type ServiceWeight = MessageQueueServiceWeight;
	// The XCMP queue pallet is only ever able to handle the `Sibling(ParaId)` origin:
	type QueueChangeHandler = NarrowOriginToSibling<XcmpQueue>;
	type QueuePausedQuery = NarrowOriginToSibling<XcmpQueue>;
	type WeightInfo = weights::pallet_message_queue::WeightInfo<Runtime>;
	type IdleMaxServiceWeight = ();
}
