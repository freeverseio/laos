use crate::{mock::*, CollectionId, Error, Event};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		// Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::do_something(RuntimeOrigin::signed(1), 42));
		// Read pallet storage and assert an expected result.
		assert_eq!(TemplateModule::something(), Some(42));
		// Assert that the correct event was deposited
		System::assert_last_event(Event::SomethingStored { something: 42, who: 1 }.into());
	});
}

#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(
			TemplateModule::cause_error(RuntimeOrigin::signed(1)),
			Error::<Test>::NoneValue
		);
	});
}

#[test]
fn owner_of_unexistent_collection() {
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
fn crete_collection_emits_event() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		// Create a collection
		assert_ok!(TemplateModule::create_collection(RuntimeOrigin::signed(1)));
		// Assert that the correct event was deposited
		System::assert_last_event(Event::CollectionCreated { collection_id: 0, who: 1 }.into());
	});
}
