use crate::{Balances, Runtime};
use frame_support::weights::Weight;
use pallet_block_rewards_source::{BalanceOf, WeightInfo};
use pallet_parachain_staking::PayoutCollatorReward;
use sp_runtime::{traits::Zero, DispatchError};
use sp_std::marker::PhantomData;

impl pallet_block_rewards_source::Config for Runtime {
	type WeightInfo = pallet_block_rewards_source::weights::SubstrateWeight<Runtime>;
	type Currency = Balances;
}
pub struct BlockRewardsSourceAdapter<Runtime>(PhantomData<Runtime>);

impl<Runtime: pallet_parachain_staking::Config + pallet_block_rewards_source::Config>
	PayoutCollatorReward<Runtime, BalanceOf<Runtime>> for BlockRewardsSourceAdapter<Runtime>
{
	fn payout_collator_reward(
		_round_index: pallet_parachain_staking::RoundIndex,
		collator_id: Runtime::AccountId,
		amount: pallet_block_rewards_source::BalanceOf<Runtime>,
	) -> Weight {
		match pallet_block_rewards_source::Pallet::<Runtime>::send_rewards(
			collator_id,
			amount.into(),
		) {
			Ok(amount) if amount.is_zero() => Weight::zero(),
			Ok(_) => <Runtime as pallet_block_rewards_source::Config>::WeightInfo::send_rewards(),
			Err(_) => Weight::zero(),
		}
	}

	fn deposit_into_existing(
		delegator_id: &Runtime::AccountId,
		amount: pallet_block_rewards_source::BalanceOf<Runtime>,
	) -> Result<pallet_block_rewards_source::BalanceOf<Runtime>, DispatchError> {
		pallet_block_rewards_source::Pallet::<Runtime>::send_rewards(delegator_id.clone(), amount)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{tests::ExtBuilder, AccountId};

	#[test]
	fn payout_collator_reward_when_source_account_is_none() {
		let amount = 2;
		let collator = AccountId::from([1u8; 20]);
		ExtBuilder::default().build().execute_with(|| {
			assert_eq!(
				BlockRewardsSourceAdapter::<Runtime>::payout_collator_reward(0, collator, amount),
				Weight::zero()
			);
		});
	}

	#[test]
	fn payout_collator_reward_when_send_rewards_works() {
		let source = AccountId::from([0u8; 20]);
		let amount = 2;
		let collator = AccountId::from([1u8; 20]);
		ExtBuilder::default()
			.with_balances(vec![(source, 100)])
			.with_rewards_account(source)
			.build()
			.execute_with(|| {
				assert_ne!(
					BlockRewardsSourceAdapter::<Runtime>::payout_collator_reward(
						0, collator, amount
					),
					Weight::zero()
				);
			});
	}

	#[test]
	fn deposit_into_existing_when_source_account_is_none() {
		let amount = 2;
		let collator = AccountId::from([1u8; 20]);
		ExtBuilder::default().build().execute_with(|| {
			assert_eq!(
				BlockRewardsSourceAdapter::<Runtime>::deposit_into_existing(&collator, amount)
					.unwrap(),
				0
			);
		});
	}

	#[test]
	fn deposit_into_existing_when_send_rewards_works() {
		let source = AccountId::from([0u8; 20]);
		let amount = 2;
		let collator = AccountId::from([1u8; 20]);
		ExtBuilder::default()
			.with_balances(vec![(source, 100)])
			.with_rewards_account(source)
			.build()
			.execute_with(|| {
				assert_eq!(
					BlockRewardsSourceAdapter::<Runtime>::deposit_into_existing(&collator, amount)
						.unwrap(),
					amount
				);
			});
	}
}
