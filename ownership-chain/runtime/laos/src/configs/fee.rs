use crate::{AccountId, PhantomData};
use frame_support::traits::{tokens::currency::Currency, OnUnbalanced};

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
mod tests {
	use super::*;
	use crate::{tests::new_test_ext, Runtime};

	#[test]
	fn all_fee_should_go_to_block_author() {
		new_test_ext().execute_with(|| {
			let fee_amount = 100;
			// get author of the block
			let author = pallet_authorship::Pallet::<Runtime>::author().unwrap();

			// initial author balance
			let initial_author_balance = pallet_balances::Pallet::<Runtime>::free_balance(author);

			// Mock the creation of a negative imbalance of 100 units
			let imbalance = pallet_balances::NegativeImbalance::new(fee_amount);

			// Distribute the fees
			ToAuthor::<Runtime>::on_unbalanceds(vec![imbalance].into_iter());

			// Assert the expected state of balances after distribution
			let author_balance = pallet_balances::Pallet::<Runtime>::free_balance(author);

			// Assuming all fees are distributed to the author
			let expected_author_balance = initial_author_balance + fee_amount;

			assert_eq!(
				author_balance, expected_author_balance,
				"Author did not receive the correct amount"
			);
		});
	}

	#[test]
	fn with_no_author_fee_should_be_burned() {
		new_test_ext().execute_with(|| {
			let fee_amount = 100;

			assert_eq!(
				pallet_authorship::Pallet::<Runtime>::author(),
				None,
				"Author should not be set"
			);

			let initial_total_issuance = pallet_balances::Pallet::<Runtime>::total_issuance();

			// Mock the creation of a negative imbalance of 100 units
			let imbalance = pallet_balances::NegativeImbalance::new(fee_amount);

			// Distribute the fees
			ToAuthor::<Runtime>::on_unbalanceds(vec![imbalance].into_iter());

			let total_issuance = pallet_balances::Pallet::<Runtime>::total_issuance();
			let expected_issuance = initial_total_issuance - fee_amount;

			assert_eq!(
				total_issuance, expected_issuance,
				"Total issuance did not decrease by the correct amount"
			);
		});
	}
}
