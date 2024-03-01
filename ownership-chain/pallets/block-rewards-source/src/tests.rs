use crate::{mock::*, Event};
use frame_support::{assert_noop, assert_ok};
use sp_core::H160;

#[test]
fn only_sudo_can_set_rewards_account() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert!(BlockRewardsSource::rewards_account().is_none());
		let rewards_account = H160::from_low_u64_be(1);
		assert_noop!(
			BlockRewardsSource::set_rewards_account(
				RuntimeOrigin::signed(rewards_account),
				rewards_account
			),
			sp_runtime::DispatchError::BadOrigin
		);
		assert_ok!(BlockRewardsSource::set_rewards_account(RuntimeOrigin::root(), rewards_account));
		assert!(BlockRewardsSource::rewards_account().unwrap() == rewards_account);

		System::assert_has_event(Event::RewardsAccountSet(rewards_account).into());
	});
}
