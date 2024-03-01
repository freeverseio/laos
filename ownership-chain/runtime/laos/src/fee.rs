use crate::*;

type NegativeImbalanceOfBalances<T> = pallet_balances::NegativeImbalance<T>;

/// Logic for sending fees to the collator rewards account. On every unbalanced change (f.e
/// transaction fees), the amount is transferred to the collator rewards account.
pub struct DealWithFees<R>(PhantomData<R>);


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

impl<R> OnUnbalanced<Credit<<R as frame_system::Config>::AccountId, pallet_balances::Pallet<R, ()>>>
	for DealWithFees<R>
where
	R: pallet_balances::Config + pallet_parachain_staking::Config,
{
	fn on_nonzero_unbalanced(
		_amount: Credit<<R as frame_system::Config>::AccountId, pallet_balances::Pallet<R, ()>>,
	) {
		// if let Some(account) = <pallet_parachain_staking::Pallet<R>>::collator_rewards_account()
		// { 	let result = <pallet_balances::Pallet<R>>::resolve(&account, amount);
		// 	debug_assert!(result.is_ok(), "Should not fail to transfer; qed");
		// }
	}
}