//! Benchmarking setup for pallet-template
#![cfg(feature = "runtime-benchmarks")]

use super::*;
use frame_benchmarking::v2::*;
use sp_runtime::traits::StaticLookup;

#[benchmarks]
mod benchmarks {
	use super::*;
	#[cfg(test)]
	use crate::pallet::Pallet as Template;
	use frame_system::RawOrigin;

	#[benchmark]
	fn fund_treasury() {
		let caller: T::AccountId = whitelisted_caller();

		let vault_account = Pallet::<T>::account_id();

		let amount = 1000_u32;
		let per_block = 1_u32;
		let starting_block = 0_u32;

		let _ = pallet_vesting::Pallet::<T>::vested_transfer(
			RawOrigin::Signed(vault_account.clone()).into(),
			T::Lookup::unlookup(vault_account.clone()),
			pallet_vesting::VestingInfo::new(
				amount.into(),
				per_block.into(),
				starting_block.into(),
			),
		);

		frame_system::Pallet::<T>::set_block_number(1001_u32.into());

		#[extrinsic_call]
		fund_treasury(RawOrigin::Signed(caller));

		// check treasury account balance
		let treasury_account = pallet_treasury::Pallet::<T>::account_id();
		assert_eq!(pallet_balances::Pallet::<T>::free_balance(&treasury_account), amount.into());
	}

	impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
}
