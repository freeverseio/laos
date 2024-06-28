
//! Autogenerated weights for `pallet_laos_evolution`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-06-27, STEPS: `2`, REPEAT: `2`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `titan`, CPU: `12th Gen Intel(R) Core(TM) i7-1260P`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: 1024

// Executed Command:
// ./target/release/laos
// benchmark
// pallet
// --chain=dev
// --steps=2
// --repeat=2
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
		// Minimum execution time: 250_000 picoseconds.
		Weight::from_parts(1_831_000, 0)
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
		//  Measured:  `375`
		//  Estimated: `3840`
		// Minimum execution time: 22_157_000 picoseconds.
		Weight::from_parts(26_439_000, 0)
			.saturating_add(Weight::from_parts(0, 3840))
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
		//  Measured:  `139`
		//  Estimated: `4051`
		// Minimum execution time: 14_067_000 picoseconds.
		Weight::from_parts(15_592_000, 0)
			.saturating_add(Weight::from_parts(0, 4051))
			// Standard Error: 4_648
			.saturating_add(Weight::from_parts(4_164, 0).saturating_mul(s.into()))
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
		//  Measured:  `234`
		//  Estimated: `4051`
		// Minimum execution time: 15_373_000 picoseconds.
		Weight::from_parts(16_630_000, 0)
			.saturating_add(Weight::from_parts(0, 4051))
			// Standard Error: 6_004
			.saturating_add(Weight::from_parts(24_649, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `LaosEvolution::CollectionOwner` (r:1 w:1)
	/// Proof: `LaosEvolution::CollectionOwner` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	fn precompile_transfer_ownership() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `139`
		//  Estimated: `3509`
		// Minimum execution time: 9_930_000 picoseconds.
		Weight::from_parts(12_728_000, 0)
			.saturating_add(Weight::from_parts(0, 3509))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `LaosEvolution::CollectionOwner` (r:1 w:0)
	/// Proof: `LaosEvolution::CollectionOwner` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	/// Storage: `LaosEvolution::CollectionPublicMintingEnabled` (r:0 w:1)
	/// Proof: `LaosEvolution::CollectionPublicMintingEnabled` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	fn precompile_enable_public_minting() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `139`
		//  Estimated: `3509`
		// Minimum execution time: 10_909_000 picoseconds.
		Weight::from_parts(13_065_000, 0)
			.saturating_add(Weight::from_parts(0, 3509))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `LaosEvolution::CollectionOwner` (r:1 w:0)
	/// Proof: `LaosEvolution::CollectionOwner` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	/// Storage: `LaosEvolution::CollectionPublicMintingEnabled` (r:0 w:1)
	/// Proof: `LaosEvolution::CollectionPublicMintingEnabled` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	fn precompile_disable_public_minting() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `139`
		//  Estimated: `3509`
		// Minimum execution time: 11_305_000 picoseconds.
		Weight::from_parts(13_622_000, 0)
			.saturating_add(Weight::from_parts(0, 3509))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `LaosEvolution::CollectionOwner` (r:1 w:0)
	/// Proof: `LaosEvolution::CollectionOwner` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	fn precompile_owner() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `139`
		//  Estimated: `3509`
		// Minimum execution time: 5_029_000 picoseconds.
		Weight::from_parts(6_714_000, 0)
			.saturating_add(Weight::from_parts(0, 3509))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	/// Storage: `LaosEvolution::CollectionPublicMintingEnabled` (r:1 w:0)
	/// Proof: `LaosEvolution::CollectionPublicMintingEnabled` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	fn precompile_is_public_minting_enabled() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `107`
		//  Estimated: `3489`
		// Minimum execution time: 4_117_000 picoseconds.
		Weight::from_parts(7_739_000, 0)
			.saturating_add(Weight::from_parts(0, 3489))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	/// Storage: `LaosEvolution::TokenURI` (r:1 w:0)
	/// Proof: `LaosEvolution::TokenURI` (`max_values`: None, `max_size`: Some(586), added: 3061, mode: `MaxEncodedLen`)
	fn precompile_token_uri() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `306`
		//  Estimated: `4051`
		// Minimum execution time: 6_679_000 picoseconds.
		Weight::from_parts(21_914_000, 0)
			.saturating_add(Weight::from_parts(0, 4051))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	/// Storage: `LaosEvolution::CollectionCounter` (r:1 w:1)
	/// Proof: `LaosEvolution::CollectionCounter` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `LaosEvolution::CollectionOwner` (r:0 w:1)
	/// Proof: `LaosEvolution::CollectionOwner` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	fn create_collection() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `1493`
		// Minimum execution time: 8_705_000 picoseconds.
		Weight::from_parts(12_141_000, 0)
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
		//  Measured:  `139`
		//  Estimated: `4051`
		// Minimum execution time: 15_738_000 picoseconds.
		Weight::from_parts(17_520_000, 0)
			.saturating_add(Weight::from_parts(0, 4051))
			// Standard Error: 4_767
			.saturating_add(Weight::from_parts(4_957, 0).saturating_mul(s.into()))
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
		//  Measured:  `234`
		//  Estimated: `4051`
		// Minimum execution time: 15_378_000 picoseconds.
		Weight::from_parts(16_681_500, 0)
			.saturating_add(Weight::from_parts(0, 4051))
			// Standard Error: 5_205
			.saturating_add(Weight::from_parts(4_646, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `LaosEvolution::CollectionOwner` (r:1 w:0)
	/// Proof: `LaosEvolution::CollectionOwner` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	/// Storage: `LaosEvolution::CollectionPublicMintingEnabled` (r:0 w:1)
	/// Proof: `LaosEvolution::CollectionPublicMintingEnabled` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	fn enable_public_minting() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `139`
		//  Estimated: `3509`
		// Minimum execution time: 10_626_000 picoseconds.
		Weight::from_parts(13_164_000, 0)
			.saturating_add(Weight::from_parts(0, 3509))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `LaosEvolution::CollectionOwner` (r:1 w:0)
	/// Proof: `LaosEvolution::CollectionOwner` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	/// Storage: `LaosEvolution::CollectionPublicMintingEnabled` (r:0 w:1)
	/// Proof: `LaosEvolution::CollectionPublicMintingEnabled` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	fn disable_public_minting() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `172`
		//  Estimated: `3509`
		// Minimum execution time: 11_940_000 picoseconds.
		Weight::from_parts(13_964_000, 0)
			.saturating_add(Weight::from_parts(0, 3509))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `LaosEvolution::CollectionOwner` (r:1 w:1)
	/// Proof: `LaosEvolution::CollectionOwner` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	fn transfer_ownership() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `139`
		//  Estimated: `3509`
		// Minimum execution time: 9_846_000 picoseconds.
		Weight::from_parts(12_509_000, 0)
			.saturating_add(Weight::from_parts(0, 3509))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
