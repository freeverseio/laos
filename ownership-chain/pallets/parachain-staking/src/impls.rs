//! Mostly session related trait implementations

use crate::*;
use frame_support::traits::EstimateNextSessionRotation;
use frame_system::pallet_prelude::BlockNumberFor;
use sp_runtime::{
	traits::{Get, Saturating},
	Permill,
};
use sp_staking::SessionIndex;

/// Get the selected candidates
impl<T: Config> Get<Vec<AccountIdOf<T>>> for Pallet<T> {
	fn get() -> Vec<AccountIdOf<T>> {
		Self::selected_candidates().into_inner()
	}
}

impl<T> pallet_authorship::EventHandler<AccountIdOf<T>, BlockNumberFor<T>> for Pallet<T>
where
	T: Config + pallet_authorship::Config,
{
	fn note_author(author: T::AccountId) {
		// If the author is a candidate, then we should update the last block number
		Pallet::<T>::award_points_to_block_author(&author);
	}
}

impl<T: Config> pallet_session::SessionManager<AccountIdOf<T>> for Pallet<T> {
	/// 1. A new session starts
	fn new_session(new_index: SessionIndex) -> Option<Vec<AccountIdOf<T>>> {
		log::debug!(
			target: "parachain-staking",
			"Assembling new set of collators for session {} at block {:?}",
			new_index,
			frame_system::Pallet::<T>::block_number()
		);

		let collators = Self::selected_candidates().into_inner();

		// don't return empty set of collators
		if collators.is_empty() {
			log::warn!(target: "parachain-staking", "Collators from previous session will be used since empty set of collators was provided.");
			None
		} else {
			Some(collators)
		}
	}

	fn end_session(end_index: SessionIndex) {
		// We don't need to do anything here
	}

	fn start_session(start_index: SessionIndex) {
		// We don't need to do anything here
	}
}

impl<T: Config> pallet_session::ShouldEndSession<BlockNumberFor<T>> for Pallet<T> {
	fn should_end_session(now: BlockNumberFor<T>) -> bool {
		// This function is called in the `on_initialize` hook, so it doesn't
		// account the weight unless we explicitly register it.
		<frame_system::Pallet<T>>::register_extra_weight_unchecked(
			T::DbWeight::get().reads(1),
			frame_support::pallet_prelude::DispatchClass::Mandatory,
		);

		Round::<T>::get().should_update(now)
	}
}

impl<T: Config> EstimateNextSessionRotation<BlockNumberFor<T>> for Pallet<T> {
	fn average_session_length() -> BlockNumberFor<T> {
		Round::<T>::get().length
	}

	fn estimate_current_session_progress(
		now: BlockNumberFor<T>,
	) -> (Option<sp_runtime::Permill>, frame_support::weights::Weight) {
		let round = Round::<T>::get();
		let passed_blocks = now.saturating_sub(round.first);

		(
			Some(Permill::from_rational(passed_blocks, round.length)),
			// For reaading the current round
			T::DbWeight::get().reads(1),
		)
	}

	fn estimate_next_session_rotation(
		_now: BlockNumberFor<T>,
	) -> (Option<BlockNumberFor<T>>, frame_support::weights::Weight) {
		let round = Round::<T>::get();

		(
			Some(round.first + round.length),
			// For reaading the current round
			T::DbWeight::get().reads(1),
		)
	}
}
