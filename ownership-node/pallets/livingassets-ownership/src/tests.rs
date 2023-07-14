use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};

#[cfg(test)]
mod test {
	use super::*;

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
			assert_ok!(LivingAssetsModule::create_collection(RuntimeOrigin::signed(1), 0));
			assert_eq!(LivingAssetsModule::owner_of_collection(0), Some(1));
		});
	}

	#[test]
	fn create_an_existing_collection_should_fail() {
		new_test_ext().execute_with(|| {
			assert_ok!(LivingAssetsModule::create_collection(RuntimeOrigin::signed(1), 0));
			assert_noop!(
				LivingAssetsModule::create_collection(RuntimeOrigin::signed(1), 0),
				Error::<Test>::CollectionAlreadyExists
			);
		});
	}

	#[test]
	fn create_new_collection_should_emit_an_event() {
		new_test_ext().execute_with(|| {
			// Go past genesis block so events get deposited
			System::set_block_number(1);

			assert_ok!(LivingAssetsModule::create_collection(RuntimeOrigin::signed(1), 0));
			System::assert_last_event(Event::CollectionCreated { collection_id: 0, who: 1 }.into());
		});
	}
}
