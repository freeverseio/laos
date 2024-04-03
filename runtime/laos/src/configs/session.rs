use crate::{configs, weights as laos_weights, ConvertInto, Runtime, RuntimeEvent, SessionKeys};

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
	type WeightInfo = laos_weights::pallet_session::WeightInfo<Runtime>;
}
