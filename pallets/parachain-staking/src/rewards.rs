use crate::{traits::PayoutReward, BalanceOf, *};
use frame_support::{
	ensure,
	pallet_prelude::Weight,
	traits::{
		tokens::{currency::Currency, ExistenceRequirement},
		Imbalance,
	},
};
use sp_runtime::{traits::Zero, ArithmeticError, DispatchError};

pub struct MintingRewards;
impl<Runtime: crate::Config> PayoutReward<Runtime, BalanceOf<Runtime>> for MintingRewards {
	fn payout_collator_rewards(
		for_round: crate::RoundIndex,
		collator_id: Runtime::AccountId,
		amount: crate::BalanceOf<Runtime>,
	) -> Weight {
		crate::Pallet::<Runtime>::mint_collator_reward(for_round, collator_id, amount)
	}

	fn payout(
		delegator_id: &Runtime::AccountId,
		amount: crate::BalanceOf<Runtime>,
	) -> Result<crate::BalanceOf<Runtime>, DispatchError> {
		Runtime::Currency::deposit_into_existing(delegator_id, amount)
			.map(|imbalance| imbalance.peek())
	}
}

pub struct TransferFromRewardsAccount;
impl<Runtime: crate::Config> PayoutReward<Runtime, BalanceOf<Runtime>>
	for TransferFromRewardsAccount
{
	fn payout_collator_rewards(
		for_round: crate::RoundIndex,
		collator_id: Runtime::AccountId,
		amount: crate::BalanceOf<Runtime>,
	) -> Weight {
		crate::Pallet::<Runtime>::send_collator_reward(for_round, collator_id, amount)
	}

	fn payout(
		delegator_id: &Runtime::AccountId,
		amount: crate::BalanceOf<Runtime>,
	) -> Result<crate::BalanceOf<Runtime>, DispatchError> {
		ensure!(
			frame_system::Account::<Runtime>::contains_key(delegator_id),
			"Account does not exist"
		);

		let rewards_account = RewardsAccount::<Runtime>::get().unwrap();

		Runtime::Currency::transfer(
			&rewards_account,
			&delegator_id,
			amount,
			ExistenceRequirement::KeepAlive,
		)
		.map(|_| amount)
		.or_else(|e| match e {
			DispatchError::Arithmetic(ArithmeticError::Underflow) => Ok(Zero::zero()),
			_ => Err(e),
		})
	}
}

impl<T: Config> Pallet<T> {
	/// Mint a specified reward amount to the collator's account. Emits the [Rewarded] event.
	pub(crate) fn mint_collator_reward(
		_paid_for_round: RoundIndex,
		collator_id: T::AccountId,
		amt: BalanceOf<T>,
	) -> Weight {
		if let Ok(amount_transferred) = T::PayoutReward::payout(&collator_id, amt) {
			Self::deposit_event(Event::Rewarded {
				account: collator_id.clone(),
				rewards: amount_transferred,
			});
		}
		T::WeightInfo::mint_collator_reward()
	}

	pub fn send_collator_reward(
		_paid_for_round: RoundIndex,
		collator_id: T::AccountId,
		amt: BalanceOf<T>,
	) -> Weight {
		if T::Currency::transfer(
			&RewardsAccount::<T>::get().unwrap(),
			&collator_id,
			amt,
			ExistenceRequirement::KeepAlive,
		)
		.is_ok()
		{
			Self::deposit_event(Event::Rewarded { account: collator_id.clone(), rewards: amt });
		}
		Weight::zero() // TODO: weight
	}
}

// tests
#[cfg(test)]
mod tests {
	use super::*;
	use crate as pallet_parachain_staking;
	use frame_support::{derive_impl, parameter_types};

	type Block = frame_system::mocking::MockBlock<Test>;
	pub type Balance = u128;
	pub type AccountId = u64;

	#[derive_impl(frame_system::config_preludes::TestDefaultConfig as frame_system::DefaultConfig)]
	impl frame_system::Config for Test {
		type Block = Block;
		type AccountData = pallet_balances::AccountData<Balance>;
	}

	impl pallet_balances::Config for Test {
		type MaxReserves = ();
		type ReserveIdentifier = [u8; 4];
		type MaxLocks = ();
		type Balance = Balance;
		type RuntimeEvent = RuntimeEvent;
		type DustRemoval = ();
		type ExistentialDeposit = ();
		type AccountStore = System;
		type WeightInfo = ();
		type RuntimeHoldReason = ();
		type FreezeIdentifier = ();
		type MaxHolds = ();
		type MaxFreezes = ();
	}

	parameter_types! {
		pub const MinBlocksPerRound: u32 = 3;
		pub const MaxOfflineRounds: u32 = 1;
		pub const LeaveCandidatesDelay: u32 = 2;
		pub const CandidateBondLessDelay: u32 = 2;
		pub const LeaveDelegatorsDelay: u32 = 2;
		pub const RevokeDelegationDelay: u32 = 2;
		pub const DelegationBondLessDelay: u32 = 2;
		pub const RewardPaymentDelay: u32 = 2;
		pub const MinSelectedCandidates: u32 = 1;
		pub const MaxTopDelegationsPerCandidate: u32 = 4;
		pub const MaxBottomDelegationsPerCandidate: u32 = 4;
		pub const MaxDelegationsPerDelegator: u32 = 4;
		pub const MinCandidateStk: u32 = 10;
		pub const MinDelegation: u32 = 3;
		pub const MaxCandidates: u32 = 200;
	}

	impl Config for Test {
		type RuntimeEvent = RuntimeEvent;
		type Currency = Balances;
		type MonetaryGovernanceOrigin = frame_system::EnsureRoot<AccountId>;
		type MinBlocksPerRound = MinBlocksPerRound;
		type MaxOfflineRounds = MaxOfflineRounds;
		type LeaveCandidatesDelay = LeaveCandidatesDelay;
		type CandidateBondLessDelay = CandidateBondLessDelay;
		type LeaveDelegatorsDelay = LeaveDelegatorsDelay;
		type RevokeDelegationDelay = RevokeDelegationDelay;
		type DelegationBondLessDelay = DelegationBondLessDelay;
		type RewardPaymentDelay = RewardPaymentDelay;
		type MinSelectedCandidates = MinSelectedCandidates;
		type MaxTopDelegationsPerCandidate = MaxTopDelegationsPerCandidate;
		type MaxBottomDelegationsPerCandidate = MaxBottomDelegationsPerCandidate;
		type MaxDelegationsPerDelegator = MaxDelegationsPerDelegator;
		type MinCandidateStk = MinCandidateStk;
		type MinDelegation = MinDelegation;
		type BlockAuthor = ();
		type OnCollatorPayout = ();
		type PayoutReward = TransferFromRewardsAccount;
		type OnInactiveCollator = ();
		type OnNewRound = ();
		type SlotProvider = ();
		type WeightInfo = ();
		type MaxCandidates = MaxCandidates;
		type SlotsPerYear = frame_support::traits::ConstU32<{ 31_557_600 / 6 }>;
	}

	frame_support::construct_runtime!(
		pub enum Test
		{
			System: frame_system,
			Balances: pallet_balances,
			ParachainStaking: pallet_parachain_staking,
		}
	);
}
