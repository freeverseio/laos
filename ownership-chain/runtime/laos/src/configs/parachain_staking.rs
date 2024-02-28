use crate::*;

// Polimec Blockchain – https://www.polimec.org/
// Copyright (C) Polimec 2022. All rights reserved.

// The Polimec Blockchain is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Polimec Blockchain is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use frame_support::{parameter_types, PalletId};

// Since a Round is 6 hours, one week, expresses as `RoundIndex` is 4 * 7
const WEEK_IN_ROUNDS: u32 = 4 * 7;

parameter_types! {
	pub const MinBlocksPerRound: u32 = 10;
	pub const LeaveCandidatesDelay: u32 = WEEK_IN_ROUNDS;
	pub const CandidateBondLessDelay: u32 = WEEK_IN_ROUNDS;
	pub const LeaveDelegatorsDelay: u32 = WEEK_IN_ROUNDS;
	pub const RevokeDelegationDelay: u32 = WEEK_IN_ROUNDS;
	pub const DelegationBondLessDelay: u32 = WEEK_IN_ROUNDS;
	pub const RewardPaymentDelay: u32 = 2;
	pub const MinSelectedCandidates: u32 = 5;
	pub const MaxTopDelegationsPerCandidate: u32 = 300;
	pub const MaxBottomDelegationsPerCandidate: u32 = 50;
	pub const MaxDelegationsPerDelegator: u32 = 100;
	pub const MinCandidateStk: u128 = 20_000 * UNIT;
	pub const MinDelegatorStk: u128 = 50 * UNIT;
	pub const MinDelegation: u128 = 50 * UNIT;
	pub const StakingPalletId: PalletId = PalletId(*b"plmc/stk");
}


impl pallet_parachain_staking::Config for Runtime {
	type CandidateBondLessDelay = CandidateBondLessDelay;
	type Currency = Balances;
	type DelegationBondLessDelay = DelegationBondLessDelay;
	type LeaveCandidatesDelay = LeaveCandidatesDelay;
	type LeaveDelegatorsDelay = LeaveDelegatorsDelay;
	type MaxBottomDelegationsPerCandidate = MaxBottomDelegationsPerCandidate;
	type MaxDelegationsPerDelegator = MaxDelegationsPerDelegator;
	type MaxTopDelegationsPerCandidate = MaxTopDelegationsPerCandidate;
	type MinBlocksPerRound = MinBlocksPerRound;
	type MinCandidateStk = MinCandidateStk;
	type MinDelegation = MinDelegation;
	type MinDelegatorStk = MinDelegatorStk;
	type MinSelectedCandidates = MinSelectedCandidates;
	type MonetaryGovernanceOrigin = frame_system::EnsureRoot<AccountId>;
	type OnCollatorPayout = ();
	type OnNewRound = ();
	type PayMaster = ();
	// We use the default implementation, so we leave () here.
	type PayoutCollatorReward = ();
	type RevokeDelegationDelay = RevokeDelegationDelay;
	type RewardPaymentDelay = RewardPaymentDelay;
	type RuntimeEvent = RuntimeEvent;
	type RuntimeHoldReason = RuntimeHoldReason;
	type WeightInfo = pallet_parachain_staking::weights::SubstrateWeight<Runtime>;
}