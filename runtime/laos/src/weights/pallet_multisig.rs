
//! Autogenerated weights for `pallet_multisig`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-08-22, STEPS: `2`, REPEAT: `1`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `titan`, CPU: `12th Gen Intel(R) Core(TM) i7-1260P`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: 1024

// Executed Command:
// ./target/release/laos
// benchmark
// pallet
// --chain=dev
// --steps=2
// --repeat=1
// --pallet=pallet_multisig
// --extrinsic=*
// --wasm-execution=compiled
// --output=./runtime/laos/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_multisig`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_multisig::WeightInfo for WeightInfo<T> {
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_threshold_1(_z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 11_272_000 picoseconds.
		Weight::from_parts(24_058_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	/// Storage: `Multisig::Multisigs` (r:1 w:1)
	/// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(521), added: 2996, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[2, 20]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_create(s: u32, z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `204`
		//  Estimated: `3986`
		// Minimum execution time: 45_802_000 picoseconds.
		Weight::from_parts(44_560_888, 0)
			.saturating_add(Weight::from_parts(0, 3986))
			// Standard Error: 49_940
			.saturating_add(Weight::from_parts(62_055, 0).saturating_mul(s.into()))
			// Standard Error: 89
			.saturating_add(Weight::from_parts(2_067, 0).saturating_mul(z.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Multisig::Multisigs` (r:1 w:1)
	/// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(521), added: 2996, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[3, 20]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_approve(s: u32, z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `246`
		//  Estimated: `3986`
		// Minimum execution time: 29_450_000 picoseconds.
		Weight::from_parts(21_977_058, 0)
			.saturating_add(Weight::from_parts(0, 3986))
			// Standard Error: 348_345
			.saturating_add(Weight::from_parts(373_647, 0).saturating_mul(s.into()))
			// Standard Error: 592
			.saturating_add(Weight::from_parts(1_928, 0).saturating_mul(z.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Multisig::Multisigs` (r:1 w:1)
	/// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(521), added: 2996, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[2, 20]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_complete(s: u32, z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `334 + s * (20 ±0)`
		//  Estimated: `3986`
		// Minimum execution time: 49_624_000 picoseconds.
		Weight::from_parts(48_519_000, 0)
			.saturating_add(Weight::from_parts(0, 3986))
			// Standard Error: 95_503
			.saturating_add(Weight::from_parts(55_250, 0).saturating_mul(s.into()))
			// Standard Error: 171
			.saturating_add(Weight::from_parts(2_157, 0).saturating_mul(z.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Multisig::Multisigs` (r:1 w:1)
	/// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(521), added: 2996, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[2, 20]`.
	fn approve_as_multi_create(_s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `204`
		//  Estimated: `3986`
		// Minimum execution time: 41_840_000 picoseconds.
		Weight::from_parts(49_805_000, 0)
			.saturating_add(Weight::from_parts(0, 3986))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Multisig::Multisigs` (r:1 w:1)
	/// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(521), added: 2996, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[2, 20]`.
	fn approve_as_multi_approve(_s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `246`
		//  Estimated: `3986`
		// Minimum execution time: 23_223_000 picoseconds.
		Weight::from_parts(23_370_000, 0)
			.saturating_add(Weight::from_parts(0, 3986))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Multisig::Multisigs` (r:1 w:1)
	/// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(521), added: 2996, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[2, 20]`.
	fn cancel_as_multi(_s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `374`
		//  Estimated: `3986`
		// Minimum execution time: 44_832_000 picoseconds.
		Weight::from_parts(45_885_000, 0)
			.saturating_add(Weight::from_parts(0, 3986))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
