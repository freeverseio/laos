use crate::{mock::*, traits::AssetMetadataExtender as _};
use frame_support::assert_ok;

// UL stands for Universal Location

#[test]
fn given_an_ul_and_token_uri_i_can_create_asset_extension() {
	new_test_ext().execute_with(|| {
		let universal_location = 0;
		let token_uri = 1;
		let claimer = 2;
		assert_ok!(AssetMetadataExtender::create_extension(claimer, universal_location, token_uri));

		// TODO event is emmitted
	});
}

// #[test]
// fn given_an_ul_and_token_uri_i_can_update_an_existing_extension() {
// 	new_test_ext().execute_with(|| {});
// }

#[test]
fn given_a_universal_location_after_creating_extension_counter_increases() {
	// TODO test same claimer several times and same asset with several claimers
	new_test_ext().execute_with(|| {
		let universal_location = 0;
		let another_universal_location = 1;
		let token_uri = 1;
		let claimer = 2;
		assert_eq!(
			AssetMetadataExtender::universal_location_extensions_counter(universal_location)
				.unwrap(),
			0
		);
		assert_ok!(AssetMetadataExtender::create_extension(claimer, universal_location, token_uri));
		assert_eq!(
			AssetMetadataExtender::universal_location_extensions_counter(universal_location)
				.unwrap(),
			1
		);
		assert_eq!(
			AssetMetadataExtender::universal_location_extensions_counter(
				another_universal_location
			)
			.unwrap(),
			0
		);
	});
}

// #[test]
// fn when_updating_extension_index_does_not_increase() {
// 	// test same claimer several times and same asset with several claimers
// 	new_test_ext().execute_with(|| {});
// }

#[test]
fn given_an_ul_i_can_get_all_its_token_uris() {
	new_test_ext().execute_with(|| {
		let universal_location = 0;
		let token_uri = 1;
		let claimer = 2;
		let n = 2;
		assert_ok!(AssetMetadataExtender::create_extension(claimer, universal_location, token_uri));
		assert_ok!(AssetMetadataExtender::create_extension(claimer, universal_location, token_uri));
		for i in 0..n {
			assert!(AssetMetadataExtender::universal_location_extensions(universal_location, i)
				.is_some()); // TODO get the first field of the struct
		}
	});
}

#[test]
fn given_an_ul_i_can_get_all_its_claimers() {
	new_test_ext().execute_with(|| {
		let universal_location = 0;
		let token_uri = 1;
		let claimer = 2;
		let n = 2;
		assert_ok!(AssetMetadataExtender::create_extension(claimer, universal_location, token_uri));
		assert_ok!(AssetMetadataExtender::create_extension(claimer, universal_location, token_uri));
		for i in 0..n {
			assert!(AssetMetadataExtender::universal_location_extensions(universal_location, i)
				.is_some()); // TODO get the second field of the struct
		}
	});
}

#[test]
fn given_a_claimer_and_ul_i_can_get_the_extension() {
	new_test_ext().execute_with(|| {
		let universal_location = 0;
		let token_uri = 1;
		let claimer = 2;
		assert_ok!(AssetMetadataExtender::create_extension(claimer, universal_location, token_uri));
		assert_eq!(
			AssetMetadataExtender::claimer_extensions(claimer, universal_location).unwrap(),
			universal_location
		);
	});
}
