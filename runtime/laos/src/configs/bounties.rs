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

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{
		tests::{ExtBuilder, ALICE, BOB},
		AccountId, Balances, Bounties, RuntimeOrigin, System,
	};
	use frame_support::{
		assert_noop, assert_ok,
		traits::{Currency, OnInitialize},
	};
	use std::str::FromStr;

	#[test]
	fn test_assign_curator() {
		let alice = AccountId::from_str(ALICE).unwrap();
		let bob = AccountId::from_str(BOB).unwrap();

		ExtBuilder::default()
			.with_balances(vec![(alice, 1000 * UNIT), (bob, 1000 * UNIT)])
			.build()
			.execute_with(|| {
				System::set_block_number(1);
				Balances::make_free_balance_be(&Treasury::account_id(), 1_000_000 * UNIT);

				assert_noop!(
					Bounties::propose_curator(RuntimeOrigin::root(), 0, alice, 4),
					pallet_bounties::Error::<Runtime>::InvalidIndex
				);

				assert_ok!(Bounties::propose_bounty(
					RuntimeOrigin::signed(bob),
					23 * UNIT,
					b"12345".to_vec()
				));

				assert_ok!(Bounties::approve_bounty(RuntimeOrigin::root(), 0));
				assert_eq!(
					pallet_bounties::Bounties::<Runtime>::get(0).unwrap().get_status(),
					pallet_bounties::BountyStatus::Approved
				);

				let spending_period = <Runtime as pallet_treasury::Config>::SpendPeriod::get();
				System::set_block_number(spending_period);
				<Treasury as OnInitialize<u32>>::on_initialize(spending_period);
				assert_eq!(
					pallet_bounties::Bounties::<Runtime>::get(0).unwrap().get_status(),
					pallet_bounties::BountyStatus::Funded
				);

				let fee = 4;
				assert_ok!(Bounties::propose_curator(RuntimeOrigin::root(), 0, alice, fee));

				let bounty = pallet_bounties::Bounties::<Runtime>::get(0).unwrap();
				assert_eq!(
					bounty.get_status(),
					pallet_bounties::BountyStatus::CuratorProposed { curator: alice }
				);
			});
	}
}
