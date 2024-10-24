use crate::{
	weights, AccountId, OriginCaller, Preimage, Runtime, RuntimeCall, RuntimeEvent, RuntimeOrigin,
};
use frame_support::{parameter_types, traits::EqualPrivilegeOnly, weights::Weight};
use frame_system::EnsureRoot;
use laos_primitives::{RuntimeBlockWeights, NORMAL_DISPATCH_RATIO};

parameter_types! {
	pub const MaxScheduledPerBlock: u32 = 50;
	pub MaximumSchedulerWeight: Weight = NORMAL_DISPATCH_RATIO * RuntimeBlockWeights::get().max_block;
}

impl pallet_scheduler::Config for Runtime {
	type MaxScheduledPerBlock = MaxScheduledPerBlock;
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
