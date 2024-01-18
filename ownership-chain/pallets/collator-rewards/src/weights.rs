
//! Autogenerated weights for `pallet_collator_rewards`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-01-18, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `MacBook-Pro.local`, CPU: `<UNKNOWN>`
//! WASM-EXECUTION: `Compiled`, CHAIN: `None`, DB CACHE: `1024`

// Executed Command:
// ./target/release/laos-ownership
// benchmark
// pallet
// --pallet
// pallet-collator-rewards
// --extrinsic=*
// --output
// ownership-chain/pallets/collator-rewards/src/weights.rs
// --wasm-execution=compiled
// --template
// ./ownership-chain/.maintain/frame-weight-template.hbs
// --steps
// 50
// --repeat
// 20

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `pallet_collator_rewards`.
pub trait WeightInfo {
	fn note_author() -> Weight;
}

/// Weights for `pallet_collator_rewards` using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	fn note_author() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `128`
		//  Estimated: `6172`
		// Minimum execution time: 62_000_000 picoseconds.
		Weight::from_parts(63_000_000, 6172)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	fn note_author() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `128`
		//  Estimated: `6172`
		// Minimum execution time: 62_000_000 picoseconds.
		Weight::from_parts(63_000_000, 6172)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
}