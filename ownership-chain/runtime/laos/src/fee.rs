use super::PhantomData;
use frame_support::traits::{fungible::Credit, OnUnbalanced};

/// This is a dummy implementation for `OnUnbalanced` trait.
pub struct DealWithFees<R>(PhantomData<R>);

type NegativeImbalanceOfBalances<T> = pallet_balances::NegativeImbalance<T>;
impl<R> OnUnbalanced<NegativeImbalanceOfBalances<R>> for DealWithFees<R>
where
	R: pallet_balances::Config + pallet_parachain_staking::Config,
{
	fn on_nonzero_unbalanced(_amount: NegativeImbalanceOfBalances<R>) {
		// TODO actually the fees are burned
	}
}

impl<R> OnUnbalanced<Credit<R::AccountId, pallet_balances::Pallet<R>>> for DealWithFees<R>
where
	R: pallet_balances::Config,
{
	// this is called from pallet_evm for Ethereum-based transactions
	fn on_nonzero_unbalanced(_amount: Credit<R::AccountId, pallet_balances::Pallet<R>>) {
		// TODO actually the fees are burned
	}
}
