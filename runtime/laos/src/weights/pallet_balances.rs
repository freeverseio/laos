
//! Autogenerated weights for `pallet_balances`
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
// --pallet=pallet_balances
// --extrinsic=*
// --wasm-execution=compiled
// --output=./runtime/laos/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_balances`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_balances::WeightInfo for WeightInfo<T> {
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	fn transfer_allow_death() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `39`
		//  Estimated: `3581`
		// Minimum execution time: 44_980_000 picoseconds.
		Weight::from_parts(47_188_000, 0)
			.saturating_add(Weight::from_parts(0, 3581))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	fn transfer_keep_alive() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `39`
		//  Estimated: `3581`
		// Minimum execution time: 38_278_000 picoseconds.
		Weight::from_parts(40_159_000, 0)
			.saturating_add(Weight::from_parts(0, 3581))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	fn force_set_balance_creating() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `162`
		//  Estimated: `3581`
		// Minimum execution time: 15_661_000 picoseconds.
		Weight::from_parts(16_079_000, 0)
			.saturating_add(Weight::from_parts(0, 3581))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	fn force_set_balance_killing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `162`
		//  Estimated: `3581`
		// Minimum execution time: 21_753_000 picoseconds.
		Weight::from_parts(22_274_000, 0)
			.saturating_add(Weight::from_parts(0, 3581))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	fn force_transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `201`
		//  Estimated: `6172`
		// Minimum execution time: 50_946_000 picoseconds.
		Weight::from_parts(52_491_000, 0)
			.saturating_add(Weight::from_parts(0, 6172))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	fn transfer_all() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `39`
		//  Estimated: `3581`
		// Minimum execution time: 45_187_000 picoseconds.
		Weight::from_parts(45_946_000, 0)
			.saturating_add(Weight::from_parts(0, 3581))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	fn force_unreserve() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `162`
		//  Estimated: `3581`
		// Minimum execution time: 17_407_000 picoseconds.
		Weight::from_parts(18_018_000, 0)
			.saturating_add(Weight::from_parts(0, 3581))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `System::Account` (r:999 w:999)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// The range of component `u` is `[1, 1000]`.
	fn upgrade_accounts(u: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + u * (124 ±0)`
		//  Estimated: `990 + u * (2591 ±0)`
		// Minimum execution time: 15_222_000 picoseconds.
		Weight::from_parts(30_417_418, 0)
			.saturating_add(Weight::from_parts(0, 990))
			// Standard Error: 6_040
			.saturating_add(Weight::from_parts(12_736_523, 0).saturating_mul(u.into()))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(u.into())))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(u.into())))
			.saturating_add(Weight::from_parts(0, 2591).saturating_mul(u.into()))
	}
	fn force_adjust_total_issuance() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 4_825_000 picoseconds.
		Weight::from_parts(5_099_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	fn burn_allow_death() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 27_551_000 picoseconds.
		Weight::from_parts(28_009_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	fn burn_keep_alive() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 19_100_000 picoseconds.
		Weight::from_parts(19_409_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
}
