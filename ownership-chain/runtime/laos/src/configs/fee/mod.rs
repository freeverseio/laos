use crate::PhantomData;
use frame_support::traits::{tokens::currency::Currency, Imbalance, OnUnbalanced};

pub struct DealWithFees<R>(PhantomData<R>);

// Define constants for distribution ratios
const REWARDS_ACCOUNT_RATIO: u32 = 80;
const AUTHOR_RATIO: u32 = 20;

impl<R> DealWithFees<R>
where
	R: pallet_balances::Config + pallet_block_rewards_source::Config + pallet_authorship::Config,
{
	// Distributes the specified imbalance to the rewards account
	fn to_rewards_account(imbalance: pallet_balances::NegativeImbalance<R>) {
		if let Some(rewards_account) = <pallet_block_rewards_source::Pallet<R>>::rewards_account() {
			<pallet_balances::Pallet<R>>::resolve_creating(&rewards_account, imbalance);
		}
	}

	// Distributes the specified imbalance to the author of the block
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
	// Handles unbalanced fees and tips from substrate-based transactions
	fn on_unbalanceds<B>(
		mut fees_then_tips: impl Iterator<Item = pallet_balances::NegativeImbalance<R>>,
	) {
		if let Some(fees) = fees_then_tips.next() {
			let (rewards_account_amount, mut author_amount) =
				fees.ration(REWARDS_ACCOUNT_RATIO, AUTHOR_RATIO);

			if let Some(tips) = fees_then_tips.next() {
				tips.merge_into(&mut author_amount);
			}

			Self::to_rewards_account(rewards_account_amount);
			Self::to_author(author_amount);
		}
	}

	// Handles unbalanced fees from Ethereum-based transactions
	fn on_nonzero_unbalanced(amount: pallet_balances::NegativeImbalance<R>) {
		let (rewards_account_amount, author_amount) =
			amount.ration(REWARDS_ACCOUNT_RATIO, AUTHOR_RATIO);

		Self::to_rewards_account(rewards_account_amount);
		Self::to_author(author_amount);
	}
}
