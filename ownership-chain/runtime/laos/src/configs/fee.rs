use crate::PhantomData;
use frame_support::traits::{tokens::currency::Currency, Imbalance, OnUnbalanced};

pub struct DealWithFees<R>(PhantomData<R>);
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
			let (to_rewards_account, mut to_author) = fees.ration(80, 20);

			if let Some(tips) = fees_then_tips.next() {
				// for tips, if any, 100% to author
				tips.merge_into(&mut to_author);
			}

			let rewards_account =
				<pallet_block_rewards_source::Pallet<R>>::rewards_account().unwrap(); // TODO
			<pallet_balances::Pallet<R>>::resolve_creating(&rewards_account, to_rewards_account);

			if let Some(author) = <pallet_authorship::Pallet<R>>::author() {
				<pallet_balances::Pallet<R>>::resolve_creating(&author, to_author);
			};
		}
	}

	// this is called from pallet_evm for Ethereum-based transactions
	// (technically, it calls on_unbalanced, which calls this when non-zero)
	fn on_nonzero_unbalanced(amount: pallet_balances::NegativeImbalance<R>) {
		// 80% to rewards account, 20% to author
		let (to_rewards_account, to_author) = amount.ration(80, 20);

		let rewards_account = <pallet_block_rewards_source::Pallet<R>>::rewards_account().unwrap(); // TODO
		<pallet_balances::Pallet<R>>::resolve_creating(&rewards_account, to_rewards_account);

		if let Some(author) = <pallet_authorship::Pallet<R>>::author() {
			<pallet_balances::Pallet<R>>::resolve_creating(&author, to_author);
		};
	}
}
