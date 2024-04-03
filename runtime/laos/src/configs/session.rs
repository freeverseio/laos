use crate::{
	configs::parachain_staking::ParachainStakingAdapter, weights as laos_weights, AccountId,
	Runtime, RuntimeEvent, SessionKeys,
};
use sp_runtime::traits::{ConvertInto, OpaqueKeys};

impl pallet_session::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type ValidatorId = AccountId;
	type ValidatorIdOf = ConvertInto;
	type ShouldEndSession = ParachainStakingAdapter;
	type NextSessionRotation = ParachainStakingAdapter;
	type SessionManager = ParachainStakingAdapter;
	type SessionHandler = <SessionKeys as OpaqueKeys>::KeyTypeIdProviders;
	type Keys = SessionKeys;
	type WeightInfo = laos_weights::pallet_session::WeightInfo<Runtime>;
}
