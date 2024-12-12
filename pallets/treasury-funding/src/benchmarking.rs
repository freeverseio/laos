//! Benchmarking setup for pallet-template
#![cfg(feature = "runtime-benchmarks")]

use super::*;
use frame_benchmarking::v2::*;
use sp_runtime::traits::StaticLookup;
 use frame_support::assert_ok;

#[benchmarks]
mod benchmarks {
	use super::*;
	#[cfg(test)]
	use crate::pallet::Pallet as Template;
	use frame_system::RawOrigin;
	use sp_runtime::traits::SaturatedConversion;

	#[benchmark]
	fn fund_treasury() {
		let caller: T::AccountId = whitelisted_caller();

		let vault_account = Pallet::<T>::account_id();

		let amount = 10000000000000000000000000_u128;
		let per_block = 1_u32;
		let starting_block = 0_u32;

		let treasury_account = pallet_treasury::Pallet::<T>::account_id();
		let treasury_amount = pallet_balances::Pallet::<T>::free_balance(&treasury_account);
		assert_eq!(treasury_amount, 10000000000000000000000000_u128.saturated_into());


		assert_ok!(pallet_vesting::Pallet::<T>::vested_transfer(
			RawOrigin::Signed(vault_account.clone()).into(),
			T::Lookup::unlookup(vault_account.clone()),
			pallet_vesting::VestingInfo::new(
				amount.saturated_into(),
				per_block.into(),
				starting_block.into(),
			),
		));

		frame_system::Pallet::<T>::set_block_number(1001_u32.into());

		#[extrinsic_call]
		fund_treasury(RawOrigin::Signed(caller));

		// check treasury account balance
		assert_eq!(pallet_balances::Pallet::<T>::free_balance(&treasury_account), treasury_amount);
	}

	impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
}
