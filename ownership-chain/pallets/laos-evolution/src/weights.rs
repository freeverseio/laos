
//! Autogenerated weights for `pallet_laos_evolution`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-01-25, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `MacBook-Pro.local`, CPU: `<UNKNOWN>`
//! WASM-EXECUTION: `Compiled`, CHAIN: `None`, DB CACHE: `1024`

// Executed Command:
// ./target/release/laos-ownership
// benchmark
// pallet
// --pallet
// pallet-laos-evolution
// --extrinsic=*
// --output
// ownership-chain/pallets/laos-evolution/src/weights.rs
// --execution
// wasm
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

/// Weight functions needed for `pallet_laos_evolution`.
pub trait WeightInfo {
	fn create_collection() -> Weight;
	fn mint_with_external_uri(s: u32, ) -> Weight;
	fn evolve_with_external_uri(s: u32, ) -> Weight;
	fn enable_public_minting() -> Weight;
	fn disable_public_minting() -> Weight;
	fn transfer_ownership() -> Weight;
}

/// Weights for `pallet_laos_evolution` using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `LaosEvolution::CollectionCounter` (r:1 w:1)
	/// Proof: `LaosEvolution::CollectionCounter` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `LaosEvolution::CollectionOwner` (r:0 w:1)
	/// Proof: `LaosEvolution::CollectionOwner` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	fn create_collection() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `1493`
		// Minimum execution time: 10_000_000 picoseconds.
		Weight::from_parts(11_000_000, 1493)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `LaosEvolution::CollectionOwner` (r:1 w:0)
	/// Proof: `LaosEvolution::CollectionOwner` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	/// Storage: `LaosEvolution::TokenURI` (r:1 w:1)
	/// Proof: `LaosEvolution::TokenURI` (`max_values`: None, `max_size`: Some(586), added: 3061, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 512]`.
	fn mint_with_external_uri(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `139`
		//  Estimated: `4051`
		// Minimum execution time: 19_000_000 picoseconds.
		Weight::from_parts(20_203_305, 4051)
			// Standard Error: 185
			.saturating_add(Weight::from_parts(2_382, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `LaosEvolution::CollectionOwner` (r:1 w:0)
	/// Proof: `LaosEvolution::CollectionOwner` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	/// Storage: `LaosEvolution::TokenURI` (r:1 w:1)
	/// Proof: `LaosEvolution::TokenURI` (`max_values`: None, `max_size`: Some(586), added: 3061, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 512]`.
	fn evolve_with_external_uri(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `264`
		//  Estimated: `4051`
		// Minimum execution time: 18_000_000 picoseconds.
		Weight::from_parts(18_952_857, 4051)
			// Standard Error: 68
			.saturating_add(Weight::from_parts(51, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `LaosEvolution::CollectionOwner` (r:1 w:0)
	/// Proof: `LaosEvolution::CollectionOwner` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	/// Storage: `LaosEvolution::CollectionPublicMintingEnabled` (r:0 w:1)
	/// Proof: `LaosEvolution::CollectionPublicMintingEnabled` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	fn enable_public_minting() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `139`
		//  Estimated: `3509`
		// Minimum execution time: 13_000_000 picoseconds.
		Weight::from_parts(14_000_000, 3509)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `LaosEvolution::CollectionOwner` (r:1 w:0)
	/// Proof: `LaosEvolution::CollectionOwner` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	/// Storage: `LaosEvolution::CollectionPublicMintingEnabled` (r:0 w:1)
	/// Proof: `LaosEvolution::CollectionPublicMintingEnabled` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	fn disable_public_minting() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `172`
		//  Estimated: `3509`
		// Minimum execution time: 13_000_000 picoseconds.
		Weight::from_parts(13_000_000, 3509)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `LaosEvolution::CollectionOwner` (r:1 w:1)
	/// Proof: `LaosEvolution::CollectionOwner` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	fn transfer_ownership() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `139`
		//  Estimated: `3509`
		// Minimum execution time: 11_000_000 picoseconds.
		Weight::from_parts(12_000_000, 3509)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	/// Storage: `LaosEvolution::CollectionCounter` (r:1 w:1)
	/// Proof: `LaosEvolution::CollectionCounter` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `LaosEvolution::CollectionOwner` (r:0 w:1)
	/// Proof: `LaosEvolution::CollectionOwner` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	fn create_collection() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `1493`
		// Minimum execution time: 10_000_000 picoseconds.
		Weight::from_parts(11_000_000, 1493)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `LaosEvolution::CollectionOwner` (r:1 w:0)
	/// Proof: `LaosEvolution::CollectionOwner` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	/// Storage: `LaosEvolution::TokenURI` (r:1 w:1)
	/// Proof: `LaosEvolution::TokenURI` (`max_values`: None, `max_size`: Some(586), added: 3061, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 512]`.
	fn mint_with_external_uri(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `139`
		//  Estimated: `4051`
		// Minimum execution time: 19_000_000 picoseconds.
		Weight::from_parts(20_203_305, 4051)
			// Standard Error: 185
			.saturating_add(Weight::from_parts(2_382, 0).saturating_mul(s.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `LaosEvolution::CollectionOwner` (r:1 w:0)
	/// Proof: `LaosEvolution::CollectionOwner` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	/// Storage: `LaosEvolution::TokenURI` (r:1 w:1)
	/// Proof: `LaosEvolution::TokenURI` (`max_values`: None, `max_size`: Some(586), added: 3061, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 512]`.
	fn evolve_with_external_uri(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `264`
		//  Estimated: `4051`
		// Minimum execution time: 18_000_000 picoseconds.
		Weight::from_parts(18_952_857, 4051)
			// Standard Error: 68
			.saturating_add(Weight::from_parts(51, 0).saturating_mul(s.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `LaosEvolution::CollectionOwner` (r:1 w:0)
	/// Proof: `LaosEvolution::CollectionOwner` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	/// Storage: `LaosEvolution::CollectionPublicMintingEnabled` (r:0 w:1)
	/// Proof: `LaosEvolution::CollectionPublicMintingEnabled` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	fn enable_public_minting() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `139`
		//  Estimated: `3509`
		// Minimum execution time: 13_000_000 picoseconds.
		Weight::from_parts(14_000_000, 3509)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `LaosEvolution::CollectionOwner` (r:1 w:0)
	/// Proof: `LaosEvolution::CollectionOwner` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	/// Storage: `LaosEvolution::CollectionPublicMintingEnabled` (r:0 w:1)
	/// Proof: `LaosEvolution::CollectionPublicMintingEnabled` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	fn disable_public_minting() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `172`
		//  Estimated: `3509`
		// Minimum execution time: 13_000_000 picoseconds.
		Weight::from_parts(13_000_000, 3509)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `LaosEvolution::CollectionOwner` (r:1 w:1)
	/// Proof: `LaosEvolution::CollectionOwner` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	fn transfer_ownership() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `139`
		//  Estimated: `3509`
		// Minimum execution time: 11_000_000 picoseconds.
		Weight::from_parts(12_000_000, 3509)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}