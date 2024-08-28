
//! Autogenerated weights for `pallet_multisig`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-08-26, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
		// Minimum execution time: 6_251_000 picoseconds.
		Weight::from_parts(6_704_319, 0)
			.saturating_add(Weight::from_parts(0, 0))
			// Standard Error: 2
			.saturating_add(Weight::from_parts(357, 0).saturating_mul(z.into()))
	}
	/// Storage: `Multisig::Multisigs` (r:1 w:1)
	/// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(521), added: 2996, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[2, 20]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_create(s: u32, z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `204`
		//  Estimated: `3986`
		// Minimum execution time: 30_795_000 picoseconds.
		Weight::from_parts(28_236_560, 0)
			.saturating_add(Weight::from_parts(0, 3986))
			// Standard Error: 4_592
			.saturating_add(Weight::from_parts(196_803, 0).saturating_mul(s.into()))
			// Standard Error: 8
			.saturating_add(Weight::from_parts(1_254, 0).saturating_mul(z.into()))
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
		// Minimum execution time: 16_805_000 picoseconds.
		Weight::from_parts(15_332_493, 0)
			.saturating_add(Weight::from_parts(0, 3986))
			// Standard Error: 2_717
			.saturating_add(Weight::from_parts(116_751, 0).saturating_mul(s.into()))
			// Standard Error: 4
			.saturating_add(Weight::from_parts(1_228, 0).saturating_mul(z.into()))
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
		// Minimum execution time: 32_649_000 picoseconds.
		Weight::from_parts(28_220_263, 0)
			.saturating_add(Weight::from_parts(0, 3986))
			// Standard Error: 4_258
			.saturating_add(Weight::from_parts(287_816, 0).saturating_mul(s.into()))
			// Standard Error: 7
			.saturating_add(Weight::from_parts(1_126, 0).saturating_mul(z.into()))
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
		// Minimum execution time: 24_600_000 picoseconds.
		Weight::from_parts(25_581_898, 0)
			.saturating_add(Weight::from_parts(0, 3986))
			// Standard Error: 2_935
			.saturating_add(Weight::from_parts(148_403, 0).saturating_mul(s.into()))
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
		// Minimum execution time: 12_773_000 picoseconds.
		Weight::from_parts(13_311_572, 0)
			.saturating_add(Weight::from_parts(0, 3986))
			// Standard Error: 1_785
			.saturating_add(Weight::from_parts(119_680, 0).saturating_mul(s.into()))
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
		// Minimum execution time: 26_067_000 picoseconds.
		Weight::from_parts(26_635_415, 0)
			.saturating_add(Weight::from_parts(0, 3986))
			// Standard Error: 3_221
			.saturating_add(Weight::from_parts(177_242, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
