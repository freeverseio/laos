// Copyright 2023-2024 Freeverse.io
// This file is part of LAOS.

// LAOS is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// LAOS is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with LAOS.  If not, see <http://www.gnu.org/licenses/>.

//! Treasury pallet migrations.
use super::*;
use alloc::collections::BTreeSet;
#[cfg(feature = "try-runtime")]
use alloc::vec::Vec;
use core::marker::PhantomData;
use frame_support::{
	defensive,
	traits::{Get, OnRuntimeUpgrade},
};

/// The log target for this pallet.
const LOG_TARGET: &str = "runtime::treasury";

pub mod cleanup_proposals {
	use super::*;

	/// Migration to cleanup unapproved proposals to return the bonds back to the proposers.
	/// Proposals can no longer be created and the `Proposal` storage item will be removed in the
	/// future.
	///
	/// `UnreserveWeight` returns `Weight` of `unreserve_balance` operation which is perfomed during
	/// this migration.
	pub struct Migration<T, I, UnreserveWeight>(PhantomData<(T, I, UnreserveWeight)>);

	impl<T: Config<I>, I: 'static, UnreserveWeight: Get<Weight>> OnRuntimeUpgrade
		for Migration<T, I, UnreserveWeight>
	{
		fn on_runtime_upgrade() -> frame_support::weights::Weight {
			let mut approval_index = BTreeSet::new();
			for approval in Approvals::<T, I>::get().iter() {
				approval_index.insert(*approval);
			}

			let mut proposals_processed = 0;
			for (proposal_index, p) in Proposals::<T, I>::iter() {
				if !approval_index.contains(&proposal_index) {
					let err_amount = T::Currency::unreserve(&p.proposer, p.bond);
					if err_amount.is_zero() {
						Proposals::<T, I>::remove(proposal_index);
						log::info!(
							target: LOG_TARGET,
							"Released bond amount of {:?} to proposer {:?}",
							p.bond,
							p.proposer,
						);
					} else {
						defensive!(
							"err_amount is non zero for proposal {:?}",
							(proposal_index, err_amount)
						);
						Proposals::<T, I>::mutate_extant(proposal_index, |proposal| {
							proposal.value = err_amount;
						});
						log::info!(
							target: LOG_TARGET,
							"Released partial bond amount of {:?} to proposer {:?}",
							p.bond - err_amount,
							p.proposer,
						);
					}
					proposals_processed += 1;
				}
			}

			log::info!(
				target: LOG_TARGET,
				"Migration for pallet-treasury finished, released {} proposal bonds.",
				proposals_processed,
			);

			// calculate and return migration weights
			let approvals_read = 1;
			T::DbWeight::get().reads_writes(
				proposals_processed + approvals_read,
				proposals_processed,
			) + UnreserveWeight::get() * proposals_processed
		}

		#[cfg(feature = "try-runtime")]
		fn pre_upgrade() -> Result<Vec<u8>, sp_runtime::TryRuntimeError> {
			let value = (
				Proposals::<T, I>::iter_values().count() as u32,
				Approvals::<T, I>::get().len() as u32,
			);
			log::info!(
				target: LOG_TARGET,
				"Proposals and Approvals count {:?}",
				value,
			);
			Ok(value.encode())
		}

		#[cfg(feature = "try-runtime")]
		fn post_upgrade(state: Vec<u8>) -> Result<(), sp_runtime::TryRuntimeError> {
			let (old_proposals_count, old_approvals_count) =
				<(u32, u32)>::decode(&mut &state[..]).expect("Known good");
			let new_proposals_count = Proposals::<T, I>::iter_values().count() as u32;
			let new_approvals_count = Approvals::<T, I>::get().len() as u32;

			log::info!(
				target: LOG_TARGET,
				"Proposals and Approvals count {:?}",
				(new_proposals_count, new_approvals_count),
			);

			ensure!(
				new_proposals_count <= old_proposals_count,
				"Proposals after migration should be less or equal to old proposals"
			);
			ensure!(
				new_approvals_count == old_approvals_count,
				"Approvals after migration should remain the same"
			);
			Ok(())
		}
	}
}
