//! Runtime tests

#![cfg(test)]
mod constant_tests;
mod precompile_tests;
mod xcm_mock;
mod xcm_tests;

pub use xcm_mock::ParachainXcmRouter;

use sp_runtime::BuildStorage;

// Build genesis storag e according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::<crate::Runtime>::default()
		.build_storage()
		.unwrap()
		.into();

	pallet_balances::GenesisConfig::<crate::Runtime> {
		balances: vec![
			([0u8; 20].into(), 1_000_000_000_000_000_000_000u128),
			([1u8; 20].into(), 1_000_000_000_000_000_000_000u128),
		],
	}
	.assimilate_storage(&mut t)
	.unwrap();

	t.into()
}
