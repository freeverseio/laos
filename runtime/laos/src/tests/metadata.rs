use super::ExtBuilder;
use crate::Runtime;
use frame_metadata::RuntimeMetadataPrefixed;
use parity_scale_codec::Decode;
use serde_json::Value;

#[test]
fn test_metadata_v15_contains_expected_runtime_shape() {
	ExtBuilder::default().build().execute_with(|| {
		let current_metadata_prefixed = Runtime::metadata_at_version(15).unwrap();
		let bytes = &*current_metadata_prefixed;
		let metadata: RuntimeMetadataPrefixed = Decode::decode(&mut &bytes[..]).unwrap();
		let metadata_value: Value = serde_json::to_value(metadata)
			.expect("Failed to serialize current metadata to JSON Value");
		let pallets = metadata_value
			.pointer("/1/V15/pallets")
			.and_then(Value::as_array)
			.expect("V15 metadata should contain a pallet list");

		assert!(pallets.len() >= 30, "unexpectedly small pallet list in V15 metadata");
		for pallet_name in ["System", "Timestamp", "Scheduler", "Preimage", "EVM", "Ethereum"] {
			assert!(
				pallets.iter().any(|pallet| pallet.get("name") == Some(&Value::String(pallet_name.into()))),
				"missing expected pallet `{pallet_name}` in V15 metadata"
			);
		}
	});
}
