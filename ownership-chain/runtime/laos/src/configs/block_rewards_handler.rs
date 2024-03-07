use crate::{Balances, Runtime};
use frame_support::{ensure, weights::Weight};
use pallet_block_rewards_handler::{BalanceOf, WeightInfo};
use pallet_parachain_staking::PayoutReward;
use sp_runtime::{traits::Zero, DispatchError};
use sp_std::marker::PhantomData;
impl pallet_block_rewards_handler::Config for Runtime {
	type WeightInfo = pallet_block_rewards_handler::weights::SubstrateWeight<Runtime>;
	type Currency = Balances;
}
pub struct BlockRewardsHandlerAdapter<Runtime>(PhantomData<Runtime>);

impl<Runtime: pallet_parachain_staking::Config + pallet_block_rewards_handler::Config>
	PayoutReward<Runtime, BalanceOf<Runtime>> for BlockRewardsHandlerAdapter<Runtime>
{
	fn payout_collator_rewards(
		_round_index: pallet_parachain_staking::RoundIndex,
		collator: Runtime::AccountId,
		amount: pallet_block_rewards_handler::BalanceOf<Runtime>,
	) -> Weight {
		match pallet_block_rewards_handler::Pallet::<Runtime>::send_rewards(collator, amount.into())
		{
			Ok(amount) if amount.is_zero() => Weight::zero(),
			// TODO: In case of a failure in sending rewards, we should return a weight,
			// since at least one read operation is performed. Additionally, this situation
			// incurs an extra write operation.
			_ => <Runtime as pallet_block_rewards_handler::Config>::WeightInfo::send_rewards(),
		}
	}

	fn payout(
		destination: &Runtime::AccountId,
		amount: pallet_block_rewards_handler::BalanceOf<Runtime>,
	) -> Result<pallet_block_rewards_handler::BalanceOf<Runtime>, DispatchError> {
		ensure!(
			frame_system::Account::<Runtime>::contains_key(destination),
			"Account does not exist"
		);
		pallet_block_rewards_handler::Pallet::<Runtime>::send_rewards(destination.clone(), amount)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{tests::ExtBuilder, AccountId};

	#[test]
	fn payout_collator_rewards_when_source_account_is_none() {
		let amount = 2;
		let collator = AccountId::from([1u8; 20]);
		ExtBuilder::default().build().execute_with(|| {
			assert_eq!(
				BlockRewardsHandlerAdapter::<Runtime>::payout_collator_rewards(0, collator, amount),
				Weight::zero()
			);
		});
	}

	#[test]
	fn payout_collator_rewards_when_source_account_has_no_enough_funds() {
		let amount = 2;
		let collator = AccountId::from([1u8; 20]);
		let source = AccountId::from([0u8; 20]);
		ExtBuilder::default().with_rewards_account(source).build().execute_with(|| {
			assert_ne!(
				BlockRewardsHandlerAdapter::<Runtime>::payout_collator_rewards(0, collator, amount),
				Weight::zero()
			);
		});
	}

	#[test]
	fn payout_collator_rewards_when_send_rewards_works() {
		let source = AccountId::from([0u8; 20]);
		let amount = 2;
		let collator = AccountId::from([1u8; 20]);
		ExtBuilder::default()
			.with_balances(vec![(source, 100)])
			.with_rewards_account(source)
			.build()
			.execute_with(|| {
				assert_ne!(
					BlockRewardsHandlerAdapter::<Runtime>::payout_collator_rewards(
						0, collator, amount
					),
					Weight::zero()
				);
			});
	}

	#[test]
	fn payout_when_source_account_is_none() {
		let amount = 2;
		let destination = AccountId::from([1u8; 20]);
		ExtBuilder::default().build().execute_with(|| {
			assert_eq!(
				BlockRewardsHandlerAdapter::<Runtime>::payout(&destination, amount).unwrap(),
				0
			);
		});
	}

	#[test]
	fn payout_when_send_rewards_works() {
		let source = AccountId::from([0u8; 20]);
		let amount = 2;
		let destination = AccountId::from([1u8; 20]);
		ExtBuilder::default()
			.with_balances(vec![(source, 100)])
			.with_rewards_account(source)
			.build()
			.execute_with(|| {
				assert_eq!(
					BlockRewardsHandlerAdapter::<Runtime>::payout(&destination, amount).unwrap(),
					amount
				);
			});
	}

	#[test]
	fn payout_when_destination_does_not_exist() {
		let source = AccountId::from([0u8; 20]);
		let amount = 2;
		let destination = AccountId::from([11u8; 20]);
		ExtBuilder::default()
			.with_balances(vec![(source, 100)])
			.with_rewards_account(source)
			.build()
			.execute_with(|| {
				assert_eq!(
					BlockRewardsHandlerAdapter::<Runtime>::payout(&destination, amount).unwrap_err(),
					DispatchError::from("Account does not exist")
				);
			});
	}
}
