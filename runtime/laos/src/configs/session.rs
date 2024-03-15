use crate::{configs, ConvertInto, Runtime, RuntimeEvent, SessionKeys, HOURS};
use frame_support::parameter_types;

parameter_types! {
	pub const Period: u32 = 6 * HOURS;
	pub const Offset: u32 = 0;
}

impl pallet_session::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type ValidatorId = <Self as frame_system::Config>::AccountId;
	// we don't have stash and controller, thus we don't need the convert as well.
	type ValidatorIdOf = ConvertInto;
	type ShouldEndSession = configs::parachain_staking::ParachainStakingAdapter;
	type NextSessionRotation = configs::parachain_staking::ParachainStakingAdapter;
	type SessionManager = configs::parachain_staking::ParachainStakingAdapter;
	// Essentially just Aura, but let's be pedantic.
	type SessionHandler = <SessionKeys as sp_runtime::traits::OpaqueKeys>::KeyTypeIdProviders;
	type Keys = SessionKeys;
	type WeightInfo = pallet_session::weights::SubstrateWeight<Runtime>;
}
