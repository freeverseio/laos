use crate::PhantomData;
use frame_support::traits::{tokens::currency::Currency, Imbalance, OnUnbalanced};

pub struct DealWithFees<R>(PhantomData<R>);

impl<R> DealWithFees<R>
where
	R: pallet_balances::Config + pallet_authorship::Config,
{
	// Distributes the specified imbalance to the author of the block
	fn to_author(imbalance: pallet_balances::NegativeImbalance<R>) {
		if let Some(author) = <pallet_authorship::Pallet<R>>::author() {
			<pallet_balances::Pallet<R>>::resolve_creating(&author, imbalance);
		}
	}
}

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

			Self::to_author(author_amount);
		}
	}

	// Handles unbalanced fees from Ethereum-based transactions
	fn on_nonzero_unbalanced(amount: pallet_balances::NegativeImbalance<R>) {
		Self::to_author(amount);
	}
}

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;
