use crate::{AccountId, PhantomData};
use frame_support::traits::{tokens::currency::Currency, Imbalance, OnUnbalanced};

pub struct DealWithFees<R>(PhantomData<R>);

impl<R> OnUnbalanced<pallet_balances::NegativeImbalance<R>> for DealWithFees<R>
where
	R: pallet_balances::Config + pallet_authorship::Config,
{
	// Handles unbalanced fees and tips from substrate-based transactions
	fn on_unbalanceds<B>(
		mut fees_then_tips: impl Iterator<Item = pallet_balances::NegativeImbalance<R>>,
	) {
		if let Some(fees) = fees_then_tips.next() {
			let (mut author_amount, _) = fees.ration(100, 0);

			if let Some(tips) = fees_then_tips.next() {
				tips.merge_into(&mut author_amount);
			}

			<ToAuthor<R> as OnUnbalanced<_>>::on_unbalanced(author_amount);
		}
	}

	// Handles unbalanced fees from Ethereum-based transactions
	fn on_nonzero_unbalanced(amount: pallet_balances::NegativeImbalance<R>) {
		<ToAuthor<R> as OnUnbalanced<_>>::on_unbalanced(amount);
	}
}

/// Logic for the author to get a portion of fees.
pub struct ToAuthor<R>(PhantomData<R>);
impl<R> OnUnbalanced<pallet_balances::NegativeImbalance<R>> for ToAuthor<R>
where
	R: pallet_balances::Config + pallet_authorship::Config,
{
	fn on_nonzero_unbalanced(amount: pallet_balances::NegativeImbalance<R>) {
		if let Some(author) = <pallet_authorship::Pallet<R>>::author() {
			<pallet_balances::Pallet<R>>::resolve_creating(&author, amount);
		}
	}
}

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;
