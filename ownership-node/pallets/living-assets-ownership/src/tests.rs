use crate::{mock::*, Event, LivingAssetsOwnership};
use frame_support::assert_ok;

#[cfg(test)]
mod test {
	use super::*;

	type AccountId = <Test as frame_system::Config>::AccountId;
	type CollectionId = <Test as crate::Config>::CollectionId;

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
			assert_ok!(LivingAssetsModule::create_collection(RuntimeOrigin::signed(1)));
			assert_eq!(LivingAssetsModule::owner_of_collection(0), Some(1));
		});
	}

	#[test]
	fn create_new_collections_should_emit_events_with_collection_id_consecutive() {
		new_test_ext().execute_with(|| {
			// Go past genesis block so events get deposited
			System::set_block_number(1);

			assert_ok!(LivingAssetsModule::create_collection(RuntimeOrigin::signed(1)));
			System::assert_last_event(Event::CollectionCreated { collection_id: 0, who: 1 }.into());
			assert_ok!(LivingAssetsModule::create_collection(RuntimeOrigin::signed(1)));
			System::assert_last_event(Event::CollectionCreated { collection_id: 1, who: 1 }.into());
			assert_ok!(LivingAssetsModule::create_collection(RuntimeOrigin::signed(1)));
			System::assert_last_event(Event::CollectionCreated { collection_id: 2, who: 1 }.into());
			assert_ok!(LivingAssetsModule::create_collection(RuntimeOrigin::signed(1)));
			System::assert_last_event(Event::CollectionCreated { collection_id: 3, who: 1 }.into());
		});
	}

	#[test]
	fn living_assets_ownership_trait_create_new_collection() {
		new_test_ext().execute_with(|| {
        	let result = <LivingAssetsModule as LivingAssetsOwnership<AccountId, CollectionId>>::create_collection(1);
        	assert_ok!(result);
        	assert_eq!(LivingAssetsModule::owner_of_collection(0), Some(1));
    	});
	}

	#[test]
	fn living_assets_ownership_trait_owner_of_unexistent_collection_is_none() {
		new_test_ext().execute_with(|| {
			assert_eq!(<LivingAssetsModule as LivingAssetsOwnership<AccountId, CollectionId>>::owner_of_collection(0), None);
			assert_eq!(<LivingAssetsModule as LivingAssetsOwnership<AccountId, CollectionId>>::owner_of_collection(1), None);
		});
	}

	#[test]
	fn living_assets_ownership_trait_create_new_collection_should_emit_an_event() {
		new_test_ext().execute_with(|| {
			// Go past genesis block so events get deposited
			System::set_block_number(1);

			assert_ok!(<LivingAssetsModule as LivingAssetsOwnership<AccountId, CollectionId>>::create_collection(1));
			System::assert_last_event(Event::CollectionCreated { collection_id: 0, who: 1 }.into());
		});
	}

	#[test]
	fn living_assets_ownership_trait_id_of_new_collection_should_be_consecutive() {
		new_test_ext().execute_with(|| {
			assert_eq!(<LivingAssetsModule as LivingAssetsOwnership<AccountId, CollectionId>>::create_collection(1).unwrap(), 0);
			assert_eq!(<LivingAssetsModule as LivingAssetsOwnership<AccountId, CollectionId>>::create_collection(1).unwrap(), 1);
			assert_eq!(<LivingAssetsModule as LivingAssetsOwnership<AccountId, CollectionId>>::create_collection(1).unwrap(), 2);
			assert_eq!(<LivingAssetsModule as LivingAssetsOwnership<AccountId, CollectionId>>::create_collection(1).unwrap(), 3);
			assert_eq!(<LivingAssetsModule as LivingAssetsOwnership<AccountId, CollectionId>>::create_collection(1).unwrap(), 4);
			assert_eq!(<LivingAssetsModule as LivingAssetsOwnership<AccountId, CollectionId>>::create_collection(1).unwrap(), 5);
		});
	}
}
