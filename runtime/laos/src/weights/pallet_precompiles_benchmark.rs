
//! Autogenerated weights for `pallet_precompiles_benchmark`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 42.0.1
//! DATE: 2025-01-22, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `MBP-de-Tomas.home`, CPU: `<UNKNOWN>`
//! WASM-EXECUTION: `Compiled`, CHAIN: `None`, DB CACHE: 1024

// Executed Command:
// ./target/release/laos
// benchmark
// pallet
// --pallet=pallet_precompiles_benchmark
// --runtime
// ./target/release/wbuild/laos-runtime/laos_runtime.compact.compressed.wasm
// --genesis-builder=runtime
// --extrinsic=*
// --wasm-execution=compiled
// --steps=50
// --repeat=20
// --output=./runtime/laos/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_precompiles_benchmark`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_precompiles_benchmark::WeightInfo for WeightInfo<T> {
	/// Storage: `Vesting::Vesting` (r:1 w:1)
	/// Proof: `Vesting::Vesting` (`max_values`: None, `max_size`: Some(1045), added: 3520, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1287), added: 3762, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(887), added: 3362, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	fn precompile_vesting_vest() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `182`
		//  Estimated: `4752`
		// Minimum execution time: 58_000_000 picoseconds.
		Weight::from_parts(59_000_000, 0)
			.saturating_add(Weight::from_parts(0, 4752))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Vesting::Vesting` (r:1 w:1)
	/// Proof: `Vesting::Vesting` (`max_values`: None, `max_size`: Some(1045), added: 3520, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1287), added: 3762, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(887), added: 3362, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	fn precompile_vesting_vest_other() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `182`
		//  Estimated: `4752`
		// Minimum execution time: 58_000_000 picoseconds.
		Weight::from_parts(59_000_000, 0)
			.saturating_add(Weight::from_parts(0, 4752))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Vesting::Vesting` (r:1 w:0)
	/// Proof: `Vesting::Vesting` (`max_values`: None, `max_size`: Some(1045), added: 3520, mode: `MaxEncodedLen`)
	/// The range of component `m` is `[0, 28]`.
	fn precompile_vesting_vesting(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `45 + m * (36 ±0)`
		//  Estimated: `4510`
		// Minimum execution time: 3_000_000 picoseconds.
		Weight::from_parts(7_065_944, 0)
			.saturating_add(Weight::from_parts(0, 4510))
			// Standard Error: 5_944
			.saturating_add(Weight::from_parts(53_317, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(1))
	}
}
