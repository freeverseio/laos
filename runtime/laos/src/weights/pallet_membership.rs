
//! Autogenerated weights for `pallet_membership`
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
// --pallet=pallet_membership
// --extrinsic=*
// --wasm-execution=compiled
// --output=./runtime/laos/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_membership`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_membership::WeightInfo for WeightInfo<T> {
	/// Storage: `TechnicalCommitteeMembership::Members` (r:1 w:1)
	/// Proof: `TechnicalCommitteeMembership::Members` (`max_values`: Some(1), `max_size`: Some(101), added: 596, mode: `MaxEncodedLen`)
	/// Storage: `TechnicalCommittee::Proposals` (r:1 w:0)
	/// Proof: `TechnicalCommittee::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::Members` (r:0 w:1)
	/// Proof: `TechnicalCommittee::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::Prime` (r:0 w:1)
	/// Proof: `TechnicalCommittee::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[1, 4]`.
	fn add_member(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `127 + m * (42 ±0)`
		//  Estimated: `1613 + m * (42 ±0)`
		// Minimum execution time: 12_122_000 picoseconds.
		Weight::from_parts(13_040_179, 0)
			.saturating_add(Weight::from_parts(0, 1613))
			// Standard Error: 9_096
			.saturating_add(Weight::from_parts(50_846, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 42).saturating_mul(m.into()))
	}
	/// Storage: `TechnicalCommitteeMembership::Members` (r:1 w:1)
	/// Proof: `TechnicalCommitteeMembership::Members` (`max_values`: Some(1), `max_size`: Some(101), added: 596, mode: `MaxEncodedLen`)
	/// Storage: `TechnicalCommittee::Proposals` (r:1 w:0)
	/// Proof: `TechnicalCommittee::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommitteeMembership::Prime` (r:1 w:0)
	/// Proof: `TechnicalCommitteeMembership::Prime` (`max_values`: Some(1), `max_size`: Some(20), added: 515, mode: `MaxEncodedLen`)
	/// Storage: `TechnicalCommittee::Members` (r:0 w:1)
	/// Proof: `TechnicalCommittee::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::Prime` (r:0 w:1)
	/// Proof: `TechnicalCommittee::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[2, 5]`.
	fn remove_member(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `211 + m * (41 ±0)`
		//  Estimated: `1696 + m * (41 ±0)`
		// Minimum execution time: 14_364_000 picoseconds.
		Weight::from_parts(15_316_212, 0)
			.saturating_add(Weight::from_parts(0, 1696))
			// Standard Error: 9_523
			.saturating_add(Weight::from_parts(30_499, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 41).saturating_mul(m.into()))
	}
	/// Storage: `TechnicalCommitteeMembership::Members` (r:1 w:1)
	/// Proof: `TechnicalCommitteeMembership::Members` (`max_values`: Some(1), `max_size`: Some(101), added: 596, mode: `MaxEncodedLen`)
	/// Storage: `TechnicalCommittee::Proposals` (r:1 w:0)
	/// Proof: `TechnicalCommittee::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommitteeMembership::Prime` (r:1 w:0)
	/// Proof: `TechnicalCommitteeMembership::Prime` (`max_values`: Some(1), `max_size`: Some(20), added: 515, mode: `MaxEncodedLen`)
	/// Storage: `TechnicalCommittee::Members` (r:0 w:1)
	/// Proof: `TechnicalCommittee::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::Prime` (r:0 w:1)
	/// Proof: `TechnicalCommittee::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[2, 5]`.
	fn swap_member(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `211 + m * (41 ±0)`
		//  Estimated: `1696 + m * (41 ±0)`
		// Minimum execution time: 14_191_000 picoseconds.
		Weight::from_parts(15_200_312, 0)
			.saturating_add(Weight::from_parts(0, 1696))
			// Standard Error: 10_490
			.saturating_add(Weight::from_parts(34_699, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 41).saturating_mul(m.into()))
	}
	/// Storage: `TechnicalCommitteeMembership::Members` (r:1 w:1)
	/// Proof: `TechnicalCommitteeMembership::Members` (`max_values`: Some(1), `max_size`: Some(101), added: 596, mode: `MaxEncodedLen`)
	/// Storage: `TechnicalCommittee::Proposals` (r:1 w:0)
	/// Proof: `TechnicalCommittee::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommitteeMembership::Prime` (r:1 w:0)
	/// Proof: `TechnicalCommitteeMembership::Prime` (`max_values`: Some(1), `max_size`: Some(20), added: 515, mode: `MaxEncodedLen`)
	/// Storage: `TechnicalCommittee::Members` (r:0 w:1)
	/// Proof: `TechnicalCommittee::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::Prime` (r:0 w:1)
	/// Proof: `TechnicalCommittee::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[1, 5]`.
	fn reset_members(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `207 + m * (42 ±0)`
		//  Estimated: `1693 + m * (42 ±0)`
		// Minimum execution time: 13_725_000 picoseconds.
		Weight::from_parts(14_497_518, 0)
			.saturating_add(Weight::from_parts(0, 1693))
			// Standard Error: 12_983
			.saturating_add(Weight::from_parts(347_929, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 42).saturating_mul(m.into()))
	}
	/// Storage: `TechnicalCommitteeMembership::Members` (r:1 w:1)
	/// Proof: `TechnicalCommitteeMembership::Members` (`max_values`: Some(1), `max_size`: Some(101), added: 596, mode: `MaxEncodedLen`)
	/// Storage: `TechnicalCommittee::Proposals` (r:1 w:0)
	/// Proof: `TechnicalCommittee::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommitteeMembership::Prime` (r:1 w:1)
	/// Proof: `TechnicalCommitteeMembership::Prime` (`max_values`: Some(1), `max_size`: Some(20), added: 515, mode: `MaxEncodedLen`)
	/// Storage: `TechnicalCommittee::Members` (r:0 w:1)
	/// Proof: `TechnicalCommittee::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TechnicalCommittee::Prime` (r:0 w:1)
	/// Proof: `TechnicalCommittee::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[1, 5]`.
	fn change_key(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `207 + m * (42 ±0)`
		//  Estimated: `1693 + m * (42 ±0)`
		// Minimum execution time: 14_612_000 picoseconds.
		Weight::from_parts(15_322_513, 0)
			.saturating_add(Weight::from_parts(0, 1693))
			// Standard Error: 8_463
			.saturating_add(Weight::from_parts(236_327, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(Weight::from_parts(0, 42).saturating_mul(m.into()))
	}
	/// Storage: `TechnicalCommitteeMembership::Members` (r:1 w:0)
	/// Proof: `TechnicalCommitteeMembership::Members` (`max_values`: Some(1), `max_size`: Some(101), added: 596, mode: `MaxEncodedLen`)
	/// Storage: `TechnicalCommitteeMembership::Prime` (r:0 w:1)
	/// Proof: `TechnicalCommitteeMembership::Prime` (`max_values`: Some(1), `max_size`: Some(20), added: 515, mode: `MaxEncodedLen`)
	/// Storage: `TechnicalCommittee::Prime` (r:0 w:1)
	/// Proof: `TechnicalCommittee::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[1, 5]`.
	fn set_prime(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `100 + m * (21 ±0)`
		//  Estimated: `1586 + m * (21 ±0)`
		// Minimum execution time: 6_665_000 picoseconds.
		Weight::from_parts(7_019_266, 0)
			.saturating_add(Weight::from_parts(0, 1586))
			// Standard Error: 4_372
			.saturating_add(Weight::from_parts(119_236, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(Weight::from_parts(0, 21).saturating_mul(m.into()))
	}
	/// Storage: `TechnicalCommitteeMembership::Prime` (r:0 w:1)
	/// Proof: `TechnicalCommitteeMembership::Prime` (`max_values`: Some(1), `max_size`: Some(20), added: 515, mode: `MaxEncodedLen`)
	/// Storage: `TechnicalCommittee::Prime` (r:0 w:1)
	/// Proof: `TechnicalCommittee::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn clear_prime() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 1_810_000 picoseconds.
		Weight::from_parts(1_948_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
