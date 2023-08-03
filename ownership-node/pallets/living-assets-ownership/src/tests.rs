use crate::{mock::*, traits::CollectionManager, Event};
use frame_support::assert_ok;

type AccountId = <Test as frame_system::Config>::AccountId;
type CollectionId = <Test as crate::Config>::CollectionId;

const ALICE: AccountId = 0x1234;

#[test]
fn owner_of_unexistent_collection_is_none() {
	new_test_ext().execute_with(|| {
		assert_eq!(LivingAssetsModule::owner_of_collection(0), None);
		assert_eq!(LivingAssetsModule::owner_of_collection(1), None);
	});
}

#[test]
fn create_new_collection() {
	new_test_ext().execute_with(|| {
		assert_ok!(LivingAssetsModule::create_collection(RuntimeOrigin::signed(ALICE)));
		assert_eq!(LivingAssetsModule::owner_of_collection(0).unwrap(), ALICE);
	});
}

#[test]
fn create_new_collections_should_emit_events_with_collection_id_consecutive() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		assert_ok!(LivingAssetsModule::create_collection(RuntimeOrigin::signed(ALICE)));
		System::assert_last_event(Event::CollectionCreated { collection_id: 0, who: ALICE }.into());
		assert_ok!(LivingAssetsModule::create_collection(RuntimeOrigin::signed(ALICE)));
		System::assert_last_event(Event::CollectionCreated { collection_id: 1, who: ALICE }.into());
		assert_ok!(LivingAssetsModule::create_collection(RuntimeOrigin::signed(ALICE)));
		System::assert_last_event(Event::CollectionCreated { collection_id: 2, who: ALICE }.into());
		assert_ok!(LivingAssetsModule::create_collection(RuntimeOrigin::signed(ALICE)));
		System::assert_last_event(Event::CollectionCreated { collection_id: 3, who: ALICE }.into());
	});
}

#[test]
fn living_assets_ownership_trait_create_new_collection() {
	new_test_ext().execute_with(|| {
		let result =
			<LivingAssetsModule as CollectionManager<AccountId, CollectionId>>::create_collection(
				ALICE,
			);
		assert_ok!(result);
		assert_eq!(LivingAssetsModule::owner_of_collection(0).unwrap(), ALICE);
	});
}

#[test]
fn living_assets_ownership_trait_owner_of_unexistent_collection_is_none() {
	new_test_ext().execute_with(|| {
		assert_eq!(
			<LivingAssetsModule as CollectionManager<AccountId, CollectionId>>::owner_of_collection(
				0
			),
			None
		);
		assert_eq!(
			<LivingAssetsModule as CollectionManager<AccountId, CollectionId>>::owner_of_collection(
				1
			),
			None
		);
	});
}

#[test]
fn living_assets_ownership_trait_create_new_collection_should_emit_an_event() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		assert_ok!(
			<LivingAssetsModule as CollectionManager<AccountId, CollectionId>>::create_collection(
				ALICE
			)
		);
		System::assert_last_event(Event::CollectionCreated { collection_id: 0, who: ALICE }.into());
	});
}

#[test]
fn living_assets_ownership_trait_id_of_new_collection_should_be_consecutive() {
	new_test_ext().execute_with(|| {
		assert_eq!(
			<LivingAssetsModule as CollectionManager<AccountId, CollectionId>>::create_collection(
				ALICE
			)
			.unwrap(),
			0
		);
		assert_eq!(
			<LivingAssetsModule as CollectionManager<AccountId, CollectionId>>::create_collection(
				ALICE
			)
			.unwrap(),
			1
		);
		assert_eq!(
			<LivingAssetsModule as CollectionManager<AccountId, CollectionId>>::create_collection(
				ALICE
			)
			.unwrap(),
			2
		);
		assert_eq!(
			<LivingAssetsModule as CollectionManager<AccountId, CollectionId>>::create_collection(
				ALICE
			)
			.unwrap(),
			3
		);
		assert_eq!(
			<LivingAssetsModule as CollectionManager<AccountId, CollectionId>>::create_collection(
				ALICE
			)
			.unwrap(),
			4
		);
		assert_eq!(
			<LivingAssetsModule as CollectionManager<AccountId, CollectionId>>::create_collection(
				ALICE
			)
			.unwrap(),
			5
		);
	});
}
