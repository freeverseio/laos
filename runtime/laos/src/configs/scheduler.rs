use crate::{
	weights, AccountId, OriginCaller, Preimage, Runtime, RuntimeCall, RuntimeEvent, RuntimeOrigin,
};
use frame_support::{
	parameter_types,
	traits::{ConstU32, EqualPrivilegeOnly},
	weights::Weight,
};
use frame_system::EnsureRoot;
use laos_primitives::{RuntimeBlockWeights, NORMAL_DISPATCH_RATIO};

parameter_types! {
	pub MaximumSchedulerWeight: Weight = NORMAL_DISPATCH_RATIO * RuntimeBlockWeights::get().max_block;
}

impl pallet_scheduler::Config for Runtime {
	#[cfg(feature = "runtime-benchmarks")]
	type MaxScheduledPerBlock = ConstU32<512>;
	#[cfg(not(feature = "runtime-benchmarks"))]
	type MaxScheduledPerBlock = ConstU32<50>;
	type MaximumWeight = MaximumSchedulerWeight;
	type OriginPrivilegeCmp = EqualPrivilegeOnly;
	type PalletsOrigin = OriginCaller;
	type Preimages = Preimage;
	type RuntimeCall = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type RuntimeOrigin = RuntimeOrigin;
	type ScheduleOrigin = EnsureRoot<AccountId>;
	type WeightInfo = weights::pallet_scheduler::WeightInfo<Runtime>;
}
