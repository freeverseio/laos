use crate::PhantomData;
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
	use crate::{generic::DigestItem, H256};
	use frame_support::{
		derive_impl, parameter_types,
		traits::{fungible::Balanced, tokens::Precision},
		ConsensusEngineId,
	};
	use sp_runtime::{
		codec::{Decode, Encode},
		testing::Header,
		traits::Header as HeaderT,
		BuildStorage,
	};

	type Block = frame_system::mocking::MockBlock<Test>;

	// Configure a mock runtime to test the pallet.
	frame_support::construct_runtime!(
		pub enum Test
		{
			System: frame_system,
			Authorship: pallet_authorship,
			Balances: pallet_balances,
		}
	);

	type AccountId = u64;
	type Balance = u128;

	#[derive_impl(frame_system::config_preludes::TestDefaultConfig as frame_system::DefaultConfig)]
	impl frame_system::Config for Test {
		type Block = Block;
		type AccountData = pallet_balances::AccountData<Balance>;
	}

	parameter_types! {
		pub const MaxTokenUriLength: u32 = 512;
		pub const ExistentialDeposit: u128 = 1;
	}

	impl pallet_balances::Config for Test {
		type MaxReserves = ();
		type ReserveIdentifier = [u8; 4];
		type MaxLocks = ();
		type Balance = Balance;
		type RuntimeEvent = RuntimeEvent;
		type DustRemoval = ();
		type ExistentialDeposit = ExistentialDeposit;
		type AccountStore = System;
		type WeightInfo = ();
		type RuntimeHoldReason = ();
		type FreezeIdentifier = ();
		type MaxHolds = ();
		type MaxFreezes = ();
	}
	impl pallet_authorship::Config for Test {
		type FindAuthor = AuthorGiven;
		type EventHandler = ();
	}

	const TEST_ID: ConsensusEngineId = [1, 2, 3, 4];

	pub struct AuthorGiven;

	impl frame_support::traits::FindAuthor<u64> for AuthorGiven {
		fn find_author<'a, I>(digests: I) -> Option<u64>
		where
			I: 'a + IntoIterator<Item = (ConsensusEngineId, &'a [u8])>,
		{
			for (id, mut data) in digests {
				if id == TEST_ID {
					return u64::decode(&mut data).ok()
				}
			}

			None
		}
	}

	fn seal_header(mut header: Header, author: u64) -> Header {
		{
			let digest = header.digest_mut();
			digest.logs.push(DigestItem::PreRuntime(TEST_ID, author.encode()));
			digest.logs.push(DigestItem::Seal(TEST_ID, author.encode()));
		}

		header
	}

	fn create_header(number: u64, parent_hash: H256, state_root: H256) -> Header {
		Header::new(number, Default::default(), state_root, parent_hash, Default::default())
	}

	fn new_test_ext() -> sp_io::TestExternalities {
		let t = frame_system::GenesisConfig::<Test>::default().build_storage().unwrap();
		t.into()
	}

	fn initialize_block_and_set_author(number: u64, author: AccountId) {
		let mut header =
			seal_header(create_header(number, Default::default(), [1; 32].into()), author);

		header.digest_mut().pop(); // pop the seal off.
		System::reset_events();
		System::initialize(&number, &Default::default(), header.digest());
	}

	#[test]
	fn sets_author_lazily() {
		new_test_ext().execute_with(|| {
			let author = 42;

			initialize_block_and_set_author(1, author);
			assert_eq!(Authorship::author(), Some(author));
		});
	}

	#[test]
	fn all_fee_should_go_to_block_author() {
		new_test_ext().execute_with(|| {
			let fee_amount = 100;
			let author = 62;

			initialize_block_and_set_author(1, author);

			// initial author balance
			let initial_author_balance = pallet_balances::Pallet::<Test>::free_balance(author);

			// Mock the creation of a negative imbalance of 100 units
			let imbalance = pallet_balances::NegativeImbalance::new(fee_amount);

			// Distribute the fees
			ToAuthor::<Test>::on_unbalanceds(vec![imbalance].into_iter());

			// Assert the expected state of balances after distribution
			let author_balance = pallet_balances::Pallet::<Test>::free_balance(author);

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
				pallet_authorship::Pallet::<Test>::author(),
				None,
				"Author should not be set"
			);

			// Deposit some funds to burn
			assert!(pallet_balances::Pallet::<Test>::deposit(
				&66,
				fee_amount * 2,
				Precision::Exact
			)
			.is_ok());

			let initial_total_issuance = pallet_balances::Pallet::<Test>::total_issuance();

			// Mock the creation of a negative imbalance of 100 units
			let imbalance = pallet_balances::NegativeImbalance::new(fee_amount);

			// Distribute the fees
			ToAuthor::<Test>::on_unbalanceds(vec![imbalance].into_iter());

			let total_issuance = pallet_balances::Pallet::<Test>::total_issuance();
			let expected_issuance = initial_total_issuance - fee_amount;

			assert_eq!(
				total_issuance, expected_issuance,
				"Total issuance did not decrease by the correct amount"
			);
		});
	}
}
