use frame_support::traits::Currency;

use crate::mock::*;

#[test]
fn block_rewards_work() {
	new_test_ext().execute_with(|| {
		let reward_per_block = RewardPerBlock::get();
		let community_incentives_account = CommunityIncentivesAccountId::get();

		// community incentives account has `INITIAL_BALANCE` balance
		assert_eq!(Balances::free_balance(community_incentives_account), INITIAL_BALANCE);

		// 4 is the default author
		assert_eq!(Balances::free_balance(4), INITIAL_BALANCE);

		// trigger `note_author` for block 1
		initialize_to_block(1);

		// 4 is the default author, so he is rewarded
		assert_eq!(Balances::free_balance(4), INITIAL_BALANCE + reward_per_block);

		// check for event
		System::assert_has_event(RuntimeEvent::CollatorRewards(crate::Event::CollatorRewarded {
			collator: 4,
			amount: reward_per_block,
		}));

		// community incentives account's balance decreased
		assert_eq!(
			Balances::free_balance(community_incentives_account),
			INITIAL_BALANCE - reward_per_block
		);

		// trigger `note_author` for block 2
		initialize_to_block(2);

		// 4 is the default author, so he is rewarded
		assert_eq!(Balances::free_balance(4), INITIAL_BALANCE + reward_per_block * 2);
	})
}

#[test]
fn not_enough_reward_per_block_does_not_panic() {
	new_test_ext().execute_with(|| {
		let community_incentives_account = CommunityIncentivesAccountId::get();

		// leave less than `reward_per_block` in the community incentives account
		Balances::make_free_balance_be(&community_incentives_account, 1);

		// trigger `note_author` for block 1
		initialize_to_block(1);

		// community incentives account has `0` balance
		assert_eq!(Balances::free_balance(community_incentives_account), 0);

		// 4 was not rewarded
		assert_eq!(Balances::free_balance(4), INITIAL_BALANCE + 1);

		// trigger `note_author` for block 2
		initialize_to_block(2);

		// 4 is not rewarded because there is no balance in the community incentives account
		assert_eq!(Balances::free_balance(4), INITIAL_BALANCE + 1);
	})
}
