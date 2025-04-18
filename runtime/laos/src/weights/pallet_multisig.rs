
//! Autogenerated weights for `pallet_multisig`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 43.0.0
//! DATE: 2025-03-10, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
		// Minimum execution time: 5_650_000 picoseconds.
		Weight::from_parts(6_104_679, 0)
			.saturating_add(Weight::from_parts(0, 0))
			// Standard Error: 2
			.saturating_add(Weight::from_parts(280, 0).saturating_mul(z.into()))
	}
	/// Storage: `Multisig::Multisigs` (r:1 w:1)
	/// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(521), added: 2996, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[2, 20]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_create(s: u32, z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `237`
		//  Estimated: `3986`
		// Minimum execution time: 32_050_000 picoseconds.
		Weight::from_parts(29_746_105, 0)
			.saturating_add(Weight::from_parts(0, 3986))
			// Standard Error: 4_435
			.saturating_add(Weight::from_parts(173_316, 0).saturating_mul(s.into()))
			// Standard Error: 8
			.saturating_add(Weight::from_parts(1_050, 0).saturating_mul(z.into()))
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
		// Minimum execution time: 17_162_000 picoseconds.
		Weight::from_parts(15_566_248, 0)
			.saturating_add(Weight::from_parts(0, 3986))
			// Standard Error: 2_994
			.saturating_add(Weight::from_parts(132_739, 0).saturating_mul(s.into()))
			// Standard Error: 5
			.saturating_add(Weight::from_parts(1_048, 0).saturating_mul(z.into()))
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
		// Minimum execution time: 34_053_000 picoseconds.
		Weight::from_parts(32_209_047, 0)
			.saturating_add(Weight::from_parts(0, 3986))
			// Standard Error: 5_505
			.saturating_add(Weight::from_parts(178_291, 0).saturating_mul(s.into()))
			// Standard Error: 10
			.saturating_add(Weight::from_parts(961, 0).saturating_mul(z.into()))
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
		// Minimum execution time: 27_412_000 picoseconds.
		Weight::from_parts(28_345_568, 0)
			.saturating_add(Weight::from_parts(0, 3986))
			// Standard Error: 3_734
			.saturating_add(Weight::from_parts(177_883, 0).saturating_mul(s.into()))
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
		// Minimum execution time: 14_091_000 picoseconds.
		Weight::from_parts(14_797_849, 0)
			.saturating_add(Weight::from_parts(0, 3986))
			// Standard Error: 1_806
			.saturating_add(Weight::from_parts(121_036, 0).saturating_mul(s.into()))
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
		// Minimum execution time: 27_877_000 picoseconds.
		Weight::from_parts(28_731_684, 0)
			.saturating_add(Weight::from_parts(0, 3986))
			// Standard Error: 3_462
			.saturating_add(Weight::from_parts(194_761, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
