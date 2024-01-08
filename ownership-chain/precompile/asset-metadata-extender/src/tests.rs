use super::*;

#[test]
fn check_log_selectors() {
	assert_eq!(
		hex::encode(SELECTOR_LOG_TOKEN_URI_EXTENDED),
		"f46176c5100037c043791a91ccf3a737b3c8b2d240c695679518d3b5efa72680"
	);
}

#[test]
fn function_selectors() {
	assert_eq!(Action::Extend as u32, 0xB5A72BFF);
	assert_eq!(Action::Balance as u32, 0x7B65DED5);
	assert_eq!(Action::Claimer as u32, 0xC050D9BE);
	assert_eq!(Action::Extension as u32, 0x4322DE03);
}

#[test]
#[ignore]
fn call_unexistent_selector_should_fail() {
	todo!();
}

mod create_metadata_extension {
	#[test]
	#[ignore]
	fn does_not_return_anything() {
		todo!();
	}

	#[test]
	#[ignore]
	fn reverts_when_claimer_already_has_metadata_extension_for_universal_location() {
		todo!();
	}

	#[test]
	#[ignore]
	fn on_mock_with_nonzero_value_fails() {
		// duda onmock?
		todo!();
	}

	#[test]
	#[ignore]
	fn it_is_expected_to_have_a_cost() {
		todo!();
	}
}

mod balance_of_ul {
	#[test]
	#[ignore]
	fn given_unexistent_ul_returns_zero() {
		todo!();
	}
	#[test]
	#[ignore]
	fn returns_number_of_extensions() {
		todo!();
	}

	#[test]
	#[ignore]
	fn it_is_expected_to_have_a_cost() {
		todo!();
	}
}

mod claimer_of_ul_by_index {
	#[test]
	#[ignore]
	fn claimer_of_ul_by_index_given_unexistent_index_returns_empty_address() {
		todo!();
	}

	#[test]
	#[ignore]
	fn claimer_of_ul_by_index_returns_claimer() {
		todo!();
	}

	#[test]
	#[ignore]
	fn claimer_of_ul_by_index_it_is_expected_to_have_a_cost() {
		todo!();
	}
}

mod extension_of_ul_by_index {
	#[test]
	#[ignore]
	fn it_is_expected_to_have_a_cost() {
		todo!();
	}

	#[test]
	#[ignore]
	fn returns_extension() {
		todo!();
	}

	#[test]
	#[ignore]
	fn given_unexistent_ul_returns_empty_string() {
		todo!();
	}

	#[test]
	#[ignore]
	fn given_unexistent_index_returns_empty_string() {
		todo!();
	}
}
