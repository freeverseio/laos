use crate::{Balances, Runtime};
use frame_support::weights::Weight;
use pallet_block_rewards_source::{BalanceOf, WeightInfo};
use pallet_parachain_staking::PayoutReward;
use sp_runtime::{traits::Zero, DispatchError};
use sp_std::marker::PhantomData;

impl pallet_block_rewards_source::Config for Runtime {
	type WeightInfo = pallet_block_rewards_source::weights::SubstrateWeight<Runtime>;
	type Currency = Balances;
}
pub struct BlockRewardsHandlerAdapter<Runtime>(PhantomData<Runtime>);

impl<Runtime: pallet_parachain_staking::Config + pallet_block_rewards_source::Config>
	PayoutReward<Runtime, BalanceOf<Runtime>> for BlockRewardsHandlerAdapter<Runtime>
{
	fn payout_with_computation_cost(
		_round_index: pallet_parachain_staking::RoundIndex,
		collator_id: Runtime::AccountId,
		amount: pallet_block_rewards_source::BalanceOf<Runtime>,
	) -> Weight {
		match pallet_block_rewards_source::Pallet::<Runtime>::send_rewards(
			collator_id,
			amount.into(),
		) {
			Ok(amount) if amount.is_zero() => Weight::zero(),
			// TODO: In case of a failure in sending rewards, we should return a weight,
			// since at least one read operation is performed. Additionally, this situation
			// incurs an extra write operation.
			_ => <Runtime as pallet_block_rewards_source::Config>::WeightInfo::send_rewards(),
		}
	}

	fn payout(
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
				BlockRewardsHandlerAdapter::<Runtime>::payout_with_computation_cost(
					0, collator, amount
				),
				Weight::zero()
			);
		});
	}

	#[test]
	fn payout_collator_reward_when_source_account_has_no_enough_funds() {
		let amount = 2;
		let collator = AccountId::from([1u8; 20]);
		let source = AccountId::from([0u8; 20]);
		ExtBuilder::default().with_rewards_account(source).build().execute_with(|| {
			assert_eq!(
				BlockRewardsHandlerAdapter::<Runtime>::payout_with_computation_cost(
					0, collator, amount
				),
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
					BlockRewardsHandlerAdapter::<Runtime>::payout_with_computation_cost(
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
				BlockRewardsHandlerAdapter::<Runtime>::payout(&collator, amount).unwrap(),
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
					BlockRewardsHandlerAdapter::<Runtime>::payout(&collator, amount).unwrap(),
					amount
				);
			});
	}
}
