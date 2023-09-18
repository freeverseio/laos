use crate::{mock::*, CollectionId, Event};
use frame_support::assert_ok;

#[test]
fn owner_of_inexistent_collection() {
	new_test_ext().execute_with(|| {
		let collection_id: CollectionId = 0;
		assert_eq!(TemplateModule::collection_owner(collection_id), None);
	});
}

#[test]
fn create_collection() {
	new_test_ext().execute_with(|| {
		let collection_id: CollectionId = 0;
		assert_eq!(TemplateModule::collection_owner(collection_id), None);
		assert_ok!(TemplateModule::create_collection(RuntimeOrigin::signed(1)));
		assert_eq!(TemplateModule::collection_owner(collection_id), Some(1));
		let collection_id: CollectionId = 1;
		assert_eq!(TemplateModule::collection_owner(collection_id), None);
		assert_ok!(TemplateModule::create_collection(RuntimeOrigin::signed(2)));
		assert_eq!(TemplateModule::collection_owner(collection_id), Some(2));
	});
}

#[test]
fn counter_of_collection_increases() {
	new_test_ext().execute_with(|| {
		assert_eq!(TemplateModule::collection_counter(), 0);
		assert_ok!(TemplateModule::create_collection(RuntimeOrigin::signed(1)));
		assert_eq!(TemplateModule::collection_counter(), 1);
	})
}

#[test]
fn create_collection_emits_event() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		// Create a collection
		assert_ok!(TemplateModule::create_collection(RuntimeOrigin::signed(1)));
		// Assert that the correct event was deposited
		System::assert_last_event(Event::CollectionCreated { collection_id: 0, owner: 1 }.into());
	});
}
