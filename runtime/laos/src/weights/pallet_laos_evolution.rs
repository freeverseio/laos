
//! Autogenerated weights for `pallet_laos_evolution`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-08-28, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `trujideb`, CPU: `12th Gen Intel(R) Core(TM) i5-12500H`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: 1024

// Executed Command:
// ./target/release/laos
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_laos_evolution
// --extrinsic=*
// --wasm-execution=compiled
// --output=./runtime/laos/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_laos_evolution`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_laos_evolution::WeightInfo for WeightInfo<T> {
	fn precompile_discriminant() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 123_000 picoseconds.
		Weight::from_parts(150_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	/// Storage: `LaosEvolution::CollectionCounter` (r:1 w:1)
	/// Proof: `LaosEvolution::CollectionCounter` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `EVM::Suicided` (r:1 w:0)
	/// Proof: `EVM::Suicided` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `EVM::AccountCodes` (r:1 w:1)
	/// Proof: `EVM::AccountCodes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// Storage: `EVM::AccountCodesMetadata` (r:0 w:1)
	/// Proof: `EVM::AccountCodesMetadata` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `LaosEvolution::CollectionOwner` (r:0 w:1)
	/// Proof: `LaosEvolution::CollectionOwner` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	fn precompile_create_collection() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `442`
		//  Estimated: `3907`
		// Minimum execution time: 16_986_000 picoseconds.
		Weight::from_parts(17_830_000, 0)
			.saturating_add(Weight::from_parts(0, 3907))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: `LaosEvolution::CollectionOwner` (r:1 w:0)
	/// Proof: `LaosEvolution::CollectionOwner` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	/// Storage: `LaosEvolution::TokenURI` (r:1 w:1)
	/// Proof: `LaosEvolution::TokenURI` (`max_values`: None, `max_size`: Some(586), added: 3061, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 512]`.
	fn precompile_mint(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `173`
		//  Estimated: `4051`
		// Minimum execution time: 11_218_000 picoseconds.
		Weight::from_parts(12_878_361, 0)
			.saturating_add(Weight::from_parts(0, 4051))
			// Standard Error: 86
			.saturating_add(Weight::from_parts(1_901, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `LaosEvolution::CollectionOwner` (r:1 w:0)
	/// Proof: `LaosEvolution::CollectionOwner` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	/// Storage: `LaosEvolution::TokenURI` (r:1 w:1)
	/// Proof: `LaosEvolution::TokenURI` (`max_values`: None, `max_size`: Some(586), added: 3061, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 512]`.
	fn precompile_evolve(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `298`
		//  Estimated: `4051`
		// Minimum execution time: 12_544_000 picoseconds.
		Weight::from_parts(13_380_829, 0)
			.saturating_add(Weight::from_parts(0, 4051))
			// Standard Error: 74
			.saturating_add(Weight::from_parts(1_183, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `LaosEvolution::CollectionOwner` (r:1 w:1)
	/// Proof: `LaosEvolution::CollectionOwner` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	fn precompile_transfer_ownership() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `173`
		//  Estimated: `3509`
		// Minimum execution time: 6_366_000 picoseconds.
		Weight::from_parts(6_721_000, 0)
			.saturating_add(Weight::from_parts(0, 3509))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `LaosEvolution::CollectionOwner` (r:1 w:0)
	/// Proof: `LaosEvolution::CollectionOwner` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	fn precompile_owner() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `173`
		//  Estimated: `3509`
		// Minimum execution time: 4_186_000 picoseconds.
		Weight::from_parts(4_430_000, 0)
			.saturating_add(Weight::from_parts(0, 3509))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	/// Storage: `LaosEvolution::TokenURI` (r:1 w:0)
	/// Proof: `LaosEvolution::TokenURI` (`max_values`: None, `max_size`: Some(586), added: 3061, mode: `MaxEncodedLen`)
	fn precompile_token_uri() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `340`
		//  Estimated: `4051`
		// Minimum execution time: 5_381_000 picoseconds.
		Weight::from_parts(5_732_000, 0)
			.saturating_add(Weight::from_parts(0, 4051))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	/// Storage: `LaosEvolution::CollectionCounter` (r:1 w:1)
	/// Proof: `LaosEvolution::CollectionCounter` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `LaosEvolution::CollectionOwner` (r:0 w:1)
	/// Proof: `LaosEvolution::CollectionOwner` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	fn create_collection() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `76`
		//  Estimated: `1493`
		// Minimum execution time: 5_103_000 picoseconds.
		Weight::from_parts(5_318_000, 0)
			.saturating_add(Weight::from_parts(0, 1493))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `LaosEvolution::CollectionOwner` (r:1 w:0)
	/// Proof: `LaosEvolution::CollectionOwner` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	/// Storage: `LaosEvolution::TokenURI` (r:1 w:1)
	/// Proof: `LaosEvolution::TokenURI` (`max_values`: None, `max_size`: Some(586), added: 3061, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 512]`.
	fn mint_with_external_uri(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `173`
		//  Estimated: `4051`
		// Minimum execution time: 11_688_000 picoseconds.
		Weight::from_parts(12_705_680, 0)
			.saturating_add(Weight::from_parts(0, 4051))
			// Standard Error: 77
			.saturating_add(Weight::from_parts(1_854, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `LaosEvolution::CollectionOwner` (r:1 w:0)
	/// Proof: `LaosEvolution::CollectionOwner` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	/// Storage: `LaosEvolution::TokenURI` (r:1 w:1)
	/// Proof: `LaosEvolution::TokenURI` (`max_values`: None, `max_size`: Some(586), added: 3061, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 512]`.
	fn evolve_with_external_uri(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `298`
		//  Estimated: `4051`
		// Minimum execution time: 11_541_000 picoseconds.
		Weight::from_parts(12_309_804, 0)
			.saturating_add(Weight::from_parts(0, 4051))
			// Standard Error: 74
			.saturating_add(Weight::from_parts(1_003, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `LaosEvolution::CollectionOwner` (r:1 w:1)
	/// Proof: `LaosEvolution::CollectionOwner` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	fn transfer_ownership() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `173`
		//  Estimated: `3509`
		// Minimum execution time: 6_507_000 picoseconds.
		Weight::from_parts(7_073_000, 0)
			.saturating_add(Weight::from_parts(0, 3509))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
