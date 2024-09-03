use crate::{Runtime, AccountId, OriginCaller, RuntimeOrigin, RuntimeEvent, RuntimeCall, Preimage};
use frame_system::EnsureRoot;   

impl pallet_scheduler::Config for Runtime {
	type MaxScheduledPerBlock = MaxScheduledPerBlock;
	type MaximumWeight = MaximumSchedulerWeight;
	type OriginPrivilegeCmp = EqualOrGreatestRootCmp;
	type PalletsOrigin = OriginCaller;
	type Preimages = Preimage;
	type RuntimeCall = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type RuntimeOrigin = RuntimeOrigin;
	type ScheduleOrigin = EnsureRoot<AccountId>;
	type WeightInfo = (); // weights::pallet_scheduler::WeightInfo<Runtime>;
}