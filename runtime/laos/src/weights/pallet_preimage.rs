
//! Autogenerated weights for `pallet_preimage`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 42.0.1
//! DATE: 2024-12-17, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `giove`, CPU: `AMD Ryzen 7 5700G with Radeon Graphics`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: 1024

// Executed Command:
// ./target/release/laos
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_preimage
// --extrinsic=*
// --wasm-execution=compiled
// --output=./runtime/laos/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_preimage`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_preimage::WeightInfo for WeightInfo<T> {
	/// Storage: `Preimage::StatusFor` (r:1 w:0)
	/// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(79), added: 2554, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::RequestStatusFor` (r:1 w:1)
	/// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(79), added: 2554, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(55), added: 2530, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::PreimageFor` (r:0 w:1)
	/// Proof: `Preimage::PreimageFor` (`max_values`: None, `max_size`: Some(4194344), added: 4196819, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 4194304]`.
	fn note_preimage(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `76`
		//  Estimated: `3544`
		// Minimum execution time: 61_460_000 picoseconds.
		Weight::from_parts(62_019_000, 0)
			.saturating_add(Weight::from_parts(0, 3544))
			// Standard Error: 3
			.saturating_add(Weight::from_parts(1_997, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Preimage::StatusFor` (r:1 w:0)
	/// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(79), added: 2554, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::RequestStatusFor` (r:1 w:1)
	/// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(79), added: 2554, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::PreimageFor` (r:0 w:1)
	/// Proof: `Preimage::PreimageFor` (`max_values`: None, `max_size`: Some(4194344), added: 4196819, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 4194304]`.
	fn note_requested_preimage(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `140`
		//  Estimated: `3544`
		// Minimum execution time: 18_020_000 picoseconds.
		Weight::from_parts(18_439_000, 0)
			.saturating_add(Weight::from_parts(0, 3544))
			// Standard Error: 4
			.saturating_add(Weight::from_parts(2_005, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Preimage::StatusFor` (r:1 w:0)
	/// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(79), added: 2554, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::RequestStatusFor` (r:1 w:1)
	/// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(79), added: 2554, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::PreimageFor` (r:0 w:1)
	/// Proof: `Preimage::PreimageFor` (`max_values`: None, `max_size`: Some(4194344), added: 4196819, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 4194304]`.
	fn note_no_deposit_preimage(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `140`
		//  Estimated: `3544`
		// Minimum execution time: 17_880_000 picoseconds.
		Weight::from_parts(18_718_000, 0)
			.saturating_add(Weight::from_parts(0, 3544))
			// Standard Error: 2
			.saturating_add(Weight::from_parts(1_980, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Preimage::StatusFor` (r:1 w:0)
	/// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(79), added: 2554, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::RequestStatusFor` (r:1 w:1)
	/// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(79), added: 2554, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(55), added: 2530, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::PreimageFor` (r:0 w:1)
	/// Proof: `Preimage::PreimageFor` (`max_values`: None, `max_size`: Some(4194344), added: 4196819, mode: `MaxEncodedLen`)
	fn unnote_preimage() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `253`
		//  Estimated: `3544`
		// Minimum execution time: 76_477_000 picoseconds.
		Weight::from_parts(79_201_000, 0)
			.saturating_add(Weight::from_parts(0, 3544))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Preimage::StatusFor` (r:1 w:0)
	/// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(79), added: 2554, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::RequestStatusFor` (r:1 w:1)
	/// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(79), added: 2554, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::PreimageFor` (r:0 w:1)
	/// Proof: `Preimage::PreimageFor` (`max_values`: None, `max_size`: Some(4194344), added: 4196819, mode: `MaxEncodedLen`)
	fn unnote_no_deposit_preimage() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `178`
		//  Estimated: `3544`
		// Minimum execution time: 40_088_000 picoseconds.
		Weight::from_parts(41_975_000, 0)
			.saturating_add(Weight::from_parts(0, 3544))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Preimage::StatusFor` (r:1 w:0)
	/// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(79), added: 2554, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::RequestStatusFor` (r:1 w:1)
	/// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(79), added: 2554, mode: `MaxEncodedLen`)
	fn request_preimage() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `210`
		//  Estimated: `3544`
		// Minimum execution time: 31_918_000 picoseconds.
		Weight::from_parts(32_965_000, 0)
			.saturating_add(Weight::from_parts(0, 3544))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Preimage::StatusFor` (r:1 w:0)
	/// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(79), added: 2554, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::RequestStatusFor` (r:1 w:1)
	/// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(79), added: 2554, mode: `MaxEncodedLen`)
	fn request_no_deposit_preimage() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `178`
		//  Estimated: `3544`
		// Minimum execution time: 26_191_000 picoseconds.
		Weight::from_parts(27_587_000, 0)
			.saturating_add(Weight::from_parts(0, 3544))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Preimage::StatusFor` (r:1 w:0)
	/// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(79), added: 2554, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::RequestStatusFor` (r:1 w:1)
	/// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(79), added: 2554, mode: `MaxEncodedLen`)
	fn request_unnoted_preimage() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `76`
		//  Estimated: `3544`
		// Minimum execution time: 16_342_000 picoseconds.
		Weight::from_parts(18_508_000, 0)
			.saturating_add(Weight::from_parts(0, 3544))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Preimage::StatusFor` (r:1 w:0)
	/// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(79), added: 2554, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::RequestStatusFor` (r:1 w:1)
	/// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(79), added: 2554, mode: `MaxEncodedLen`)
	fn request_requested_preimage() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `140`
		//  Estimated: `3544`
		// Minimum execution time: 12_362_000 picoseconds.
		Weight::from_parts(12_991_000, 0)
			.saturating_add(Weight::from_parts(0, 3544))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Preimage::StatusFor` (r:1 w:0)
	/// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(79), added: 2554, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::RequestStatusFor` (r:1 w:1)
	/// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(79), added: 2554, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::PreimageFor` (r:0 w:1)
	/// Proof: `Preimage::PreimageFor` (`max_values`: None, `max_size`: Some(4194344), added: 4196819, mode: `MaxEncodedLen`)
	fn unrequest_preimage() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `178`
		//  Estimated: `3544`
		// Minimum execution time: 35_409_000 picoseconds.
		Weight::from_parts(37_644_000, 0)
			.saturating_add(Weight::from_parts(0, 3544))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Preimage::StatusFor` (r:1 w:0)
	/// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(79), added: 2554, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::RequestStatusFor` (r:1 w:1)
	/// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(79), added: 2554, mode: `MaxEncodedLen`)
	fn unrequest_unnoted_preimage() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `140`
		//  Estimated: `3544`
		// Minimum execution time: 12_432_000 picoseconds.
		Weight::from_parts(13_061_000, 0)
			.saturating_add(Weight::from_parts(0, 3544))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Preimage::StatusFor` (r:1 w:0)
	/// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(79), added: 2554, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::RequestStatusFor` (r:1 w:1)
	/// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(79), added: 2554, mode: `MaxEncodedLen`)
	fn unrequest_multi_referenced_preimage() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `140`
		//  Estimated: `3544`
		// Minimum execution time: 12_012_000 picoseconds.
		Weight::from_parts(12_920_000, 0)
			.saturating_add(Weight::from_parts(0, 3544))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Preimage::StatusFor` (r:1023 w:1023)
	/// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(79), added: 2554, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1023 w:1023)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1023 w:1023)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(55), added: 2530, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::RequestStatusFor` (r:0 w:1023)
	/// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(79), added: 2554, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 1024]`.
	fn ensure_updated(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `14 + n * (203 ±0)`
		//  Estimated: `990 + n * (2591 ±0)`
		// Minimum execution time: 67_327_000 picoseconds.
		Weight::from_parts(68_444_000, 0)
			.saturating_add(Weight::from_parts(0, 990))
			// Standard Error: 32_793
			.saturating_add(Weight::from_parts(65_362_555, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes((4_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_parts(0, 2591).saturating_mul(n.into()))
	}
}
