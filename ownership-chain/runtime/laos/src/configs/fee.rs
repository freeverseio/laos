use crate::{PhantomData};
use frame_support::traits::{
	fungible::{Balanced, Credit},
	OnUnbalanced,
};

/// This is a dummy implementation for `OnUnbalanced` trait.
pub struct DealWithFees<R>(PhantomData<R>);

impl<R> OnUnbalanced<pallet_balances::NegativeImbalance<R>> for DealWithFees<R>
where
	R: pallet_balances::Config + pallet_parachain_staking::Config,
{
	fn on_nonzero_unbalanced(amount: pallet_balances::NegativeImbalance<R>) {
		// <ToRewardsAccount<R> as OnUnbalanced<_>>::on_unbalanced(amount);
	}
}

impl<R> OnUnbalanced<Credit<R::AccountId, pallet_balances::Pallet<R>>> for DealWithFees<R>
where
	R: pallet_balances::Config + pallet_block_rewards_source::Config,
{
	// this is called from pallet_evm for Ethereum-based transactions
	fn on_nonzero_unbalanced(amount: Credit<R::AccountId, pallet_balances::Pallet<R>>) {
		if let Some(rewards_account) =
			pallet_block_rewards_source::pallet::Pallet::<R>::rewards_account()
		{
			let _ = <pallet_balances::Pallet<R>>::resolve(&rewards_account, amount);
		}
	}
}
