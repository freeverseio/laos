use core::str::FromStr;

use super::*;
use sp_core::U256;

#[test]
fn asset_id_to_address_type_zero_values() {
	type TestAssetIdToInitialOwner =
		<Runtime as pallet_living_assets_ownership::Config>::AssetIdToInitialOwner;

	assert_eq!(TestAssetIdToInitialOwner::convert(U256::from(0)), AccountId::from([0u8; 20]));
}

#[test]
fn asset_id_to_address_type_max_values() {
	type TestAssetIdToInitialOwner =
		<Runtime as pallet_living_assets_ownership::Config>::AssetIdToInitialOwner;
	assert_eq!(
		TestAssetIdToInitialOwner::convert(U256::max_value()),
		AccountId::from_str("ffffffffffffffffffffffffffffffffffffffff").unwrap()
	);
}

#[test]
fn asset_id_to_address_two_assets_same_owner() {
	type TestAssetIdToInitialOwner =
		<Runtime as pallet_living_assets_ownership::Config>::AssetIdToInitialOwner;
	assert_eq!(
		TestAssetIdToInitialOwner::convert(U256::max_value()),
		AccountId::from_str("ffffffffffffffffffffffffffffffffffffffff").unwrap()
	);

	// create two different assets
	let asset1 =
		U256::from(hex::decode("01C0F0f4ab324C46e55D02D0033343B4Be8A55532d").unwrap().as_slice());
	let asset2 =
		U256::from(hex::decode("03C0F0f4ab324C46e55D02D0033343B4Be8A55532d").unwrap().as_slice());
	assert_ne!(asset1, asset2);

	// check asset in decimal format
	assert_eq!(
		U256::from_str_radix("01C0F0f4ab324C46e55D02D0033343B4Be8A55532d", 16).unwrap(),
		U256::from_dec_str("2563001357829637001682277476112176020532353127213").unwrap()
	);
	assert_eq!(
		U256::from_str_radix("03C0F0f4ab324C46e55D02D0033343B4Be8A55532d", 16).unwrap(),
		U256::from_dec_str("5486004632491442838089647141544742059844218213165").unwrap()
	);

	let mut owner = [0u8; 20];
	owner.copy_from_slice(
		hex::decode("C0F0f4ab324C46e55D02D0033343B4Be8A55532d").unwrap().as_slice(),
	);

	assert_eq!(
		TestAssetIdToInitialOwner::convert(asset1),
		AccountId::from_str("c0f0f4ab324c46e55d02d0033343b4be8a55532d").unwrap()
	);
	assert_eq!(
		TestAssetIdToInitialOwner::convert(asset2),
		AccountId::from_str("c0f0f4ab324c46e55d02d0033343b4be8a55532d").unwrap()
	);
}

const ALICE: &str = "0x0000000000000000000000000000000000000005";
const BOB: &str = "0x0000000000000000000000000000000000000005";

#[test]
fn transefer_balance() {
	new_test_ext().execute_with(||{
		let alice = AccountId::from_str(ALICE).unwrap();
		assert_eq!(Balances::total_balance(&alice), 0);
		Balances::deposit_creating(&alice, 100000000);
		Balances::force_set_balance(origin, who, new_free)
		assert_eq!(Balances::total_balance(&alice), 10000);
	})
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	frame_system::GenesisConfig::<Runtime>::default().build_storage().unwrap().into()
}