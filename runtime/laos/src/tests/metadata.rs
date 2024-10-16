use crate::Runtime;
use serde_json::Value;
use super::ExtBuilder;

#[test]
fn test_metadata_matches_golden_json() {
	// Load the golden JSON metadata.
	let golden_json = include_str!("metadata15.golden");
	let golden_metadata: Value =
		serde_json::from_str(&golden_json).expect("Failed to parse golden JSON metadata");

	ExtBuilder::default().build().execute_with(|| {
		// Obtain the current metadata.
		let current_metadata_prefixed = Runtime::metadata();

		// Serialize current metadata to JSON.
		let current_json = serde_json::to_string_pretty(&current_metadata_prefixed)
			.expect("Failed to serialize current metadata to JSON");
		let current_metadata: Value =
			serde_json::from_str(&current_json).expect("Failed to parse current JSON metadata");

		// Compare the JSON metadata.
		assert_eq!(
			golden_metadata, current_metadata,
			"Runtime JSON metadata does not match the golden file"
		);
	});
}
