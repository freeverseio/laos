use super::ExtBuilder;
use crate::Runtime;
use assert_json_diff::assert_json_eq;
use frame_metadata::RuntimeMetadataPrefixed;
use parity_scale_codec::Decode;
use serde_json::Value;
use std::{env, fs, path::PathBuf};

#[test]
fn test_metadata_matches_golden_json() {
	// Load the golden JSON metadata.
	let golden_json = include_str!("metadata15.golden");
	let golden_metadata: Value =
		serde_json::from_str(golden_json).expect("Failed to parse golden JSON metadata");

	ExtBuilder::default().build().execute_with(|| {
		// Obtain the current metadata.
		let current_metadata_prefixed = Runtime::metadata_at_version(15).unwrap();

		let bytes = &*current_metadata_prefixed;
		let metadata: RuntimeMetadataPrefixed = Decode::decode(&mut &bytes[..]).unwrap();

		// Serialize metadata directly to a serde_json::Value
		let metadata_value: Value = serde_json::to_value(metadata)
			.expect("Failed to serialize current metadata to JSON Value");

		if env::var_os("UPDATE_METADATA_GOLDEN").is_some() {
			let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
			path.push("src/tests/metadata15.golden");
			fs::write(
				path,
				serde_json::to_string_pretty(&metadata_value)
					.expect("Failed to format current metadata as pretty JSON"),
			)
			.expect("Failed to update golden JSON metadata");
			return;
		}

		assert_json_eq!(metadata_value, golden_metadata);
	});
}
