//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as LivingAssetsOwnership;
use frame_benchmarking::v1::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
    create_collection {
        let s in 0 .. 100;
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller.clone()), s as u64)
    verify {
        assert_eq!(OwnerOfCollection::<T>::get(s as u64), Some(caller));
    }

    impl_benchmark_test_suite!(LivingAssetsOwnership, crate::mock::new_test_ext(), crate::mock::Test);
}
