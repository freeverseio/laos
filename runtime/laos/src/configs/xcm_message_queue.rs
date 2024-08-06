use crate::{Runtime, RuntimeEvent};
use frame_support::parameter_types;
use sp_weights::Weight;
use sp_runtime::Perbill;
use laos_primitives::BlockWeights;
use parachains_common::message_queue::NarrowOriginToSibling;
use cumulus_primitives_core::AggregateMessageOrigin;

parameter_types! {
	/// The amount of weight (if any) which should be provided to the message queue for
	/// servicing enqueued items.
	///
	/// This may be legitimately `None` in the case that you will call
	/// `ServiceQueues::service_queues` manually.
	pub MessageQueueServiceWeight: Weight =
		Perbill::from_percent(25) * BlockWeights::get().max_block;
	/// The maximum number of stale pages (i.e. of overweight messages) allowed before culling
	/// can happen. Once there are more stale pages than this, then historical pages may be
	/// dropped, even if they contain unprocessed overweight messages.
	pub const MessageQueueMaxStale: u32 = 8;
	/// The size of the page; this implies the maximum message size which can be sent.
	///
	/// A good value depends on the expected message sizes, their weights, the weight that is
	/// available for processing them and the maximal needed message size. The maximal message
	/// size is slightly lower than this as defined by [`MaxMessageLenOf`].
	pub const MessageQueueHeapSize: u32 = 128 * 1048;
}

impl pallet_message_queue::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	#[cfg(feature = "runtime-benchmarks")]
	type MessageProcessor = pallet_message_queue::mock_helpers::NoopMessageProcessor<
		cumulus_primitives_core::AggregateMessageOrigin,
	>;
	#[cfg(not(feature = "runtime-benchmarks"))]
	type MessageProcessor = staging_xcm_builder::ProcessXcmMessage<AggregateMessageOrigin, (), ()>; // TODO look at moonbeam's code when enabling XCM
	type Size = u32;
	type HeapSize = MessageQueueHeapSize;
	type MaxStale = MessageQueueMaxStale;
	type ServiceWeight = MessageQueueServiceWeight;
	// The XCMP queue pallet is only ever able to handle the `Sibling(ParaId)` origin:
	type QueueChangeHandler = NarrowOriginToSibling<()>; // TODO replace `()` with `XcmpQueue` when XCM is enabled
	// NarrowOriginToSibling calls XcmpQueue's is_paused if Origin is sibling. Allows all other origins
	type QueuePausedQuery = NarrowOriginToSibling<()>;
	// TODO replace `NarrowOriginToSibling<()>` with `NarrowOriginToSibling<XcmpQueue>` when XCM is enabled.
	// Note: moonbeam has this definition -> `type QueuePausedQuery = (MaintenanceMode, NarrowOriginToSibling<XcmpQueue>);`
	type WeightInfo = pallet_message_queue::weights::SubstrateWeight<Runtime>;
	type IdleMaxServiceWeight = MessageQueueServiceWeight;
}
