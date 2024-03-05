use crate::PhantomData;
use frame_support::traits::{tokens::currency::Currency, Imbalance, OnUnbalanced};

pub struct DealWithFees<R>(PhantomData<R>);

impl<R> DealWithFees<R>
where
	R: pallet_balances::Config + pallet_block_rewards_source::Config + pallet_authorship::Config,
{
	fn to_rewards_account(imbalance: pallet_balances::NegativeImbalance<R>) {
		if let Some(rewards_account) = <pallet_block_rewards_source::Pallet<R>>::rewards_account() {
			<pallet_balances::Pallet<R>>::resolve_creating(&rewards_account, imbalance);
		}
	}

	fn to_author(imbalance: pallet_balances::NegativeImbalance<R>) {
		if let Some(author) = <pallet_authorship::Pallet<R>>::author() {
			<pallet_balances::Pallet<R>>::resolve_creating(&author, imbalance);
		}
	}
}

impl<R> OnUnbalanced<pallet_balances::NegativeImbalance<R>> for DealWithFees<R>
where
	R: pallet_balances::Config + pallet_block_rewards_source::Config + pallet_authorship::Config,
{
	// this seems to be called for substrate-based transactions
	fn on_unbalanceds<B>(
		mut fees_then_tips: impl Iterator<Item = pallet_balances::NegativeImbalance<R>>,
	) {
		if let Some(fees) = fees_then_tips.next() {
			// 80% to rewards account, 20% to author
			let (rewards_account_amount, mut author_amount) = fees.ration(80, 20);

			if let Some(tips) = fees_then_tips.next() {
				// for tips, if any, 100% to author
				tips.merge_into(&mut author_amount);
			}

			Self::to_rewards_account(rewards_account_amount);
			Self::to_author(author_amount);
		}
	}

	// this is called from pallet_evm for Ethereum-based transactions
	// (technically, it calls on_unbalanced, which calls this when non-zero)
	fn on_nonzero_unbalanced(amount: pallet_balances::NegativeImbalance<R>) {
		// 80% to rewards account, 20% to author
		let (rewards_account_amount, author_amount) = amount.ration(80, 20);

		Self::to_rewards_account(rewards_account_amount);
		Self::to_author(author_amount);
	}
}
