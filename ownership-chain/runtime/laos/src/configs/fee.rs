use crate::PhantomData;
use frame_support::traits::{
	fungible::{Balanced, Credit},
	OnUnbalanced,
};

// pub struct DealWithFees<R>(PhantomData<R>);

// impl<R> OnUnbalanced<pallet_balances::NegativeImbalance<R>> for DealWithFees<R>
// where
// 	R: pallet_balances::Config + pallet_parachain_staking::Config,
// {
// 	fn on_nonzero_unbalanced(_amount: pallet_balances::NegativeImbalance<R>) {}
// }

// impl<R> OnUnbalanced<Credit<R::AccountId, pallet_balances::Pallet<R>>> for DealWithFees<R>
// where
// 	R: pallet_balances::Config + pallet_block_rewards_source::Config,
// {
// 	// this is called from pallet_evm for Ethereum-based transactions
// 	fn on_nonzero_unbalanced(amount: Credit<R::AccountId, pallet_balances::Pallet<R>>) {
// 		if let Some(rewards_account) =
// 			pallet_block_rewards_source::pallet::Pallet::<R>::rewards_account()
// 		{
// 			let _ = <pallet_balances::Pallet<R>>::resolve(&rewards_account, amount);
// 		}
// 	}
// }

pub struct DealWithFees<R>(PhantomData<R>);
impl<R> OnUnbalanced<pallet_balances::NegativeImbalance<R>> for DealWithFees<R>
where
	R: pallet_balances::Config + pallet_block_rewards_source::Config,
{
	// this seems to be called for substrate-based transactions
	fn on_unbalanceds<B>(
		mut fees_then_tips: impl Iterator<Item = pallet_balances::NegativeImbalance<R>>,
	) {
		if let Some(fees) = fees_then_tips.next() {
			if let Some(tips) = fees_then_tips.next() {
			}
			// <ToStakingPot<R> as OnUnbalanced<_>>::on_unbalanced(fees);
			// <ToAuthor<R> as OnUnbalanced<_>>::on_unbalanced(split.1);
let staking_pot = <pallet_block_rewards_source::Pallet<R>>::rewards_account().unwrap(); // TODO
		let _ = <pallet_balances::Pallet<R>>::resolve(&staking_pot, fees);
		}
	}

	// this is called from pallet_evm for Ethereum-based transactions
	// (technically, it calls on_unbalanced, which calls this when non-zero)
	fn on_nonzero_unbalanced(amount: pallet_balances::NegativeImbalance<R>) {}
}


// Logic for the author to get a portion of fees.
// pub struct ToAuthor<R>(sp_std::marker::PhantomData<R>);
// impl<R> OnUnbalanced<Credit<R::AccountId, pallet_balances::Pallet<R>>> for ToAuthor<R>
// where
// 	R: pallet_balances::Config + pallet_authorship::Config,
// 	<R as frame_system::Config>::AccountId: From<AccountId>,
// 	<R as frame_system::Config>::AccountId: Into<AccountId>,
// {
// 	fn on_nonzero_unbalanced(amount: Credit<<R as frame_system::Config>::AccountId, pallet_balances::Pallet<R>>) {
// 		if let Some(author) = <pallet_authorship::Pallet<R>>::author() {
// 			let _ = <pallet_balances::Pallet<R>>::resolve(&author, amount);
// 		}
// 	}
// }

// /// Implementation of `OnUnbalanced` that deposits the fees into  the "Blockchain Operation Treasury" for later payout.
// pub struct ToStakingPot<R>(PhantomData<R>);
// impl<R> OnUnbalanced<Credit<R::AccountId, pallet_balances::Pallet<R>>> for ToStakingPot<R>
// where
// 	R: pallet_balances::Config + pallet_block_rewards_source::Config ,
// {
// 	fn on_nonzero_unbalanced(amount: Credit<<R as frame_system::Config>::AccountId, pallet_balances::Pallet<R>>) {
// 		let staking_pot = <pallet_block_rewards_source::Pallet<R>>::rewards_account().unwrap(); // TODO
// 		let _ = <pallet_balances::Pallet<R>>::resolve(&staking_pot, amount);
// 	}
// }
