//! Benchmarking setup for pallet-collator-rewards.
#![cfg(feature = "runtime-benchmarks")]

use super::*;
use crate::Pallet as CollatorRewards;
use frame_benchmarking::{account, v2::*};
use scale_info::prelude::vec;

#[benchmarks(where T: pallet_authorship::Config + pallet_session::Config)]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn note_author() {
		T::Currency::make_free_balance_be(
			&T::CommunityIncentivesAccountId::get(),
			T::RewardPerBlock::get() * 10u32.into(),
		);
		let author = account("author", 4, 0);
		let new_block: BlockNumberFor<T> = 10u32.into();

		frame_system::Pallet::<T>::set_block_number(new_block);
		assert!(T::Currency::free_balance(&author) == 0u32.into());

		#[block]
		{
			<CollatorRewards<T> as EventHandler<_, _>>::note_author(author.clone())
		}

		assert!(T::Currency::free_balance(&author) > 0u32.into());
		assert_eq!(frame_system::Pallet::<T>::block_number(), new_block);
	}
}
