
//! Autogenerated weights for `pallet_multisig`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-09-07, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `titan`, CPU: `12th Gen Intel(R) Core(TM) i7-1260P`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: 1024

// Executed Command:
// ./target/release/laos
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
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
	fn as_multi_threshold_1(z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 5_887_000 picoseconds.
		Weight::from_parts(6_781_646, 0)
			.saturating_add(Weight::from_parts(0, 0))
			// Standard Error: 3
			.saturating_add(Weight::from_parts(390, 0).saturating_mul(z.into()))
	}
	/// Storage: `Multisig::Multisigs` (r:1 w:1)
	/// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(521), added: 2996, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[2, 20]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_create(s: u32, z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `237`
		//  Estimated: `3986`
		// Minimum execution time: 32_652_000 picoseconds.
		Weight::from_parts(28_733_391, 0)
			.saturating_add(Weight::from_parts(0, 3986))
			// Standard Error: 6_562
			.saturating_add(Weight::from_parts(313_125, 0).saturating_mul(s.into()))
			// Standard Error: 12
			.saturating_add(Weight::from_parts(1_170, 0).saturating_mul(z.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Multisig::Multisigs` (r:1 w:1)
	/// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(521), added: 2996, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[3, 20]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_approve(s: u32, z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `279`
		//  Estimated: `3986`
		// Minimum execution time: 18_153_000 picoseconds.
		Weight::from_parts(16_548_709, 0)
			.saturating_add(Weight::from_parts(0, 3986))
			// Standard Error: 2_787
			.saturating_add(Weight::from_parts(117_569, 0).saturating_mul(s.into()))
			// Standard Error: 4
			.saturating_add(Weight::from_parts(1_292, 0).saturating_mul(z.into()))
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
		//  Measured:  `367 + s * (20 ±0)`
		//  Estimated: `3986`
		// Minimum execution time: 36_187_000 picoseconds.
		Weight::from_parts(33_871_211, 0)
			.saturating_add(Weight::from_parts(0, 3986))
			// Standard Error: 4_538
			.saturating_add(Weight::from_parts(186_755, 0).saturating_mul(s.into()))
			// Standard Error: 8
			.saturating_add(Weight::from_parts(1_187, 0).saturating_mul(z.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Multisig::Multisigs` (r:1 w:1)
	/// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(521), added: 2996, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[2, 20]`.
	fn approve_as_multi_create(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `237`
		//  Estimated: `3986`
		// Minimum execution time: 26_950_000 picoseconds.
		Weight::from_parts(27_844_007, 0)
			.saturating_add(Weight::from_parts(0, 3986))
			// Standard Error: 3_484
			.saturating_add(Weight::from_parts(191_448, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Multisig::Multisigs` (r:1 w:1)
	/// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(521), added: 2996, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[2, 20]`.
	fn approve_as_multi_approve(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `279`
		//  Estimated: `3986`
		// Minimum execution time: 14_065_000 picoseconds.
		Weight::from_parts(14_588_966, 0)
			.saturating_add(Weight::from_parts(0, 3986))
			// Standard Error: 2_136
			.saturating_add(Weight::from_parts(138_507, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Multisig::Multisigs` (r:1 w:1)
	/// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(521), added: 2996, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[2, 20]`.
	fn cancel_as_multi(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `407`
		//  Estimated: `3986`
		// Minimum execution time: 28_455_000 picoseconds.
		Weight::from_parts(29_324_765, 0)
			.saturating_add(Weight::from_parts(0, 3986))
			// Standard Error: 3_393
			.saturating_add(Weight::from_parts(191_218, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
