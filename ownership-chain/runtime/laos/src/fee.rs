use super::PhantomData;
use frame_support::traits::{fungible::Credit, OnUnbalanced};

/// Logic for sending fees to the collator rewards account. On every unbalanced change (f.e
/// transaction fees), the amount is transferred to the collator rewards account.
pub struct DealWithFees<R>(PhantomData<R>);

type NegativeImbalanceOfBalances<T> = pallet_balances::NegativeImbalance<T>;
impl<R> OnUnbalanced<NegativeImbalanceOfBalances<R>> for DealWithFees<R>
where
	R: pallet_balances::Config + pallet_parachain_staking::Config,
{
	fn on_nonzero_unbalanced(_amount: NegativeImbalanceOfBalances<R>) {
		// if let Some(account) = <pallet_parachain_staking::Pallet<R>>::collator_rewards_account()
		// { 	<pallet_balances::Pallet<R>>::resolve_creating(&account, amount);
		// }
	}
}

impl<R> OnUnbalanced<Credit<R::AccountId, pallet_balances::Pallet<R>>> for DealWithFees<R>
where
	R: pallet_balances::Config,
{
	// this is called from pallet_evm for Ethereum-based transactions
	// (technically, it calls on_unbalanced, which calls this when non-zero)
	fn on_nonzero_unbalanced(_amount: Credit<R::AccountId, pallet_balances::Pallet<R>>) {
		// if let Some(account) = <pallet_parachain_staking::Pallet<R>>::collator_rewards_account()
		// { 	let result = <pallet_balances::Pallet<R>>::resolve(&account, amount);
		// 	debug_assert!(result.is_ok(), "Should not fail to transfer; qed");
		// }
	}
}
