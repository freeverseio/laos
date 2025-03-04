use crate::{
	currency::{MILLIUNIT, UNIT},
	weights, Balance, BlockNumber, Runtime, RuntimeEvent, Treasury,
};
use frame_support::parameter_types;
use parachains_common::{DAYS, MINUTES};
use polkadot_runtime_common::prod_or_fast;
use sp_runtime::Permill;

parameter_types! {
	pub const BountyDepositBase: Balance = UNIT;
	pub const BountyDepositPayoutDelay: BlockNumber = prod_or_fast!(7 * DAYS, MINUTES);
	pub const BountyUpdatePeriod: BlockNumber = prod_or_fast!(7 * DAYS, MINUTES);
	pub const MaximumReasonLength: u32 = 16384;
	pub const CuratorDepositMultiplier: Permill = Permill::from_percent(50);
	pub const CuratorDepositMin: Balance = 10 * UNIT;
	pub const CuratorDepositMax: Balance = 200 * UNIT;
	pub const BountyValueMinimum: Balance = 10 * UNIT;
	pub const DataDepositPerByte: Balance = 10 * MILLIUNIT;
}

impl pallet_bounties::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type BountyDepositBase = BountyDepositBase;
	type BountyDepositPayoutDelay = BountyDepositPayoutDelay;
	type BountyUpdatePeriod = BountyUpdatePeriod;
	type CuratorDepositMultiplier = CuratorDepositMultiplier;
	type CuratorDepositMin = CuratorDepositMin;
	type CuratorDepositMax = CuratorDepositMax;
	type BountyValueMinimum = BountyValueMinimum;
	type ChildBountyManager = ();
	type DataDepositPerByte = DataDepositPerByte;
	type MaximumReasonLength = MaximumReasonLength;
	type OnSlash = Treasury;
	type WeightInfo = weights::pallet_bounties::WeightInfo<Runtime>;
}
