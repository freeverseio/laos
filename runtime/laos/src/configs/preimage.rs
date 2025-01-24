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

use crate::{
	configs::collective::{CouncilMajority, TechnicalCommitteeMajority},
	currency::calculate_deposit,
	weights, AccountId, Balance, Balances, Runtime, RuntimeEvent, RuntimeHoldReason,
};
use frame_support::{
	parameter_types,
	traits::{fungible::HoldConsideration, EitherOfDiverse, LinearStoragePrice},
};
use frame_system::EnsureRoot;
use parity_scale_codec::Encode;

parameter_types! {
	pub const PreimageBaseDeposit: Balance = calculate_deposit(2, 64);
	pub const PreimageByteDeposit: Balance = calculate_deposit(0, 1);
	pub const PreimageHoldReason: RuntimeHoldReason =
		RuntimeHoldReason::Preimage(pallet_preimage::HoldReason::Preimage);
}

impl pallet_preimage::Config for Runtime {
	type Currency = Balances;
	type ManagerOrigin = EitherOfDiverse<
		EnsureRoot<AccountId>,
		EitherOfDiverse<CouncilMajority, TechnicalCommitteeMajority>,
	>;
	type RuntimeEvent = RuntimeEvent;
	type Consideration = HoldConsideration<
		AccountId,
		Balances,
		PreimageHoldReason,
		LinearStoragePrice<PreimageBaseDeposit, PreimageByteDeposit, Balance>,
	>;
	type WeightInfo = weights::pallet_preimage::WeightInfo<Runtime>;
}

#[cfg(test)]
mod tests {

	use super::*;
	use crate::{
		tests::{ExtBuilder, ALICE, BOB},
		RuntimeCall, RuntimeOrigin, TechnicalCommittee,
	};
	use core::str::FromStr;
	use frame_support::{assert_ok, dispatch::GetDispatchInfo, traits::PreimageProvider};
	use sp_runtime::traits::Hash;

	#[test]
	fn technical_committee_can_note_preimage() {
		let alice = AccountId::from_str(ALICE).expect("ALICE is a valid H160 address; qed");
		let bob = AccountId::from_str(BOB).expect("BOB is a valid H160 address;qed");

		ExtBuilder::default().build().execute_with(|| {
			let system_remark_calldata = RuntimeCall::System(frame_system::Call::remark {
				remark: "Hello world!".as_bytes().to_vec(),
			})
			.encode();

			let hashed_calldata =
				<Runtime as frame_system::Config>::Hashing::hash(&system_remark_calldata);

			// The preimage doesn't exist
			assert!(<pallet_preimage::Pallet<Runtime> as PreimageProvider<
				<Runtime as frame_system::Config>::Hash,
			>>::get_preimage(&hashed_calldata)
			.is_none());

			let threshold = 2u32;
			// This lenghtBound is enough for a system remark
			let lenght_bound = 19u32;

			let proposal = RuntimeCall::Preimage(pallet_preimage::Call::note_preimage {
				bytes: system_remark_calldata.clone(),
			});

			assert_ok!(TechnicalCommittee::propose(
				RuntimeOrigin::signed(alice),
				threshold,
				Box::new(proposal.clone()),
				lenght_bound
			));

			let proposal_index = 0u32;
			let proposal_hash =
				pallet_collective::pallet::Proposals::<Runtime, pallet_collective::Instance2>::get(
				)[proposal_index as usize];
			let proposal_weight_bound = proposal.get_dispatch_info().weight;

			assert_ok!(TechnicalCommittee::vote(
				RuntimeOrigin::signed(alice),
				proposal_hash,
				proposal_index,
				true
			));

			assert_ok!(TechnicalCommittee::vote(
				RuntimeOrigin::signed(bob),
				proposal_hash,
				proposal_index,
				true
			));

			assert_ok!(TechnicalCommittee::close(
				RuntimeOrigin::signed(alice),
				proposal_hash,
				proposal_index,
				proposal_weight_bound,
				lenght_bound
			));

			// The preimage exists due to the technical committee has created it. Its value is
			// exactly the system remark calldata
			assert!(<pallet_preimage::Pallet<Runtime> as PreimageProvider<
				<Runtime as frame_system::Config>::Hash,
			>>::get_preimage(&hashed_calldata)
			.is_some());

			assert_eq!(
				<pallet_preimage::Pallet<Runtime> as PreimageProvider<
					<Runtime as frame_system::Config>::Hash,
				>>::get_preimage(&hashed_calldata)
				.expect("The previous assert ensures that this is Some; qed"),
				system_remark_calldata
			);
		});
	}
}
