
//! Autogenerated weights for `pallet_asset_metadata_extender`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-06-28, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
// --pallet=pallet_asset_metadata_extender
// --extrinsic=*
// --wasm-execution=compiled
// --output=./runtime/laos/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_asset_metadata_extender`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_asset_metadata_extender::WeightInfo for WeightInfo<T> {
	/// Storage: `AssetMetadataExtender::TokenUrisByClaimerAndLocation` (r:1 w:1)
	/// Proof: `AssetMetadataExtender::TokenUrisByClaimerAndLocation` (`max_values`: None, `max_size`: Some(1080), added: 3555, mode: `MaxEncodedLen`)
	/// Storage: `AssetMetadataExtender::ExtensionsCounter` (r:1 w:1)
	/// Proof: `AssetMetadataExtender::ExtensionsCounter` (`max_values`: None, `max_size`: Some(534), added: 3009, mode: `MaxEncodedLen`)
	/// Storage: `AssetMetadataExtender::ClaimersByLocationAndIndex` (r:0 w:1)
	/// Proof: `AssetMetadataExtender::ClaimersByLocationAndIndex` (`max_values`: None, `max_size`: Some(570), added: 3045, mode: `MaxEncodedLen`)
	/// The range of component `t` is `[0, 512]`.
	/// The range of component `u` is `[0, 512]`.
	fn precompile_extend(t: u32, u: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4`
		//  Estimated: `4545`
		// Minimum execution time: 16_731_000 picoseconds.
		Weight::from_parts(17_826_853, 0)
			.saturating_add(Weight::from_parts(0, 4545))
			// Standard Error: 99
			.saturating_add(Weight::from_parts(318, 0).saturating_mul(t.into()))
			// Standard Error: 99
			.saturating_add(Weight::from_parts(11_825, 0).saturating_mul(u.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `AssetMetadataExtender::TokenUrisByClaimerAndLocation` (r:1 w:1)
	/// Proof: `AssetMetadataExtender::TokenUrisByClaimerAndLocation` (`max_values`: None, `max_size`: Some(1080), added: 3555, mode: `MaxEncodedLen`)
	/// The range of component `t` is `[0, 512]`.
	/// The range of component `u` is `[0, 512]`.
	fn precompile_update(t: u32, u: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `185 + u * (1 ±0)`
		//  Estimated: `4545`
		// Minimum execution time: 14_551_000 picoseconds.
		Weight::from_parts(15_463_865, 0)
			.saturating_add(Weight::from_parts(0, 4545))
			// Standard Error: 85
			.saturating_add(Weight::from_parts(1_638, 0).saturating_mul(t.into()))
			// Standard Error: 85
			.saturating_add(Weight::from_parts(9_881, 0).saturating_mul(u.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `AssetMetadataExtender::ExtensionsCounter` (r:1 w:0)
	/// Proof: `AssetMetadataExtender::ExtensionsCounter` (`max_values`: None, `max_size`: Some(534), added: 3009, mode: `MaxEncodedLen`)
	/// The range of component `u` is `[0, 512]`.
	fn precompile_balance_of(u: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4`
		//  Estimated: `3999`
		// Minimum execution time: 2_303_000 picoseconds.
		Weight::from_parts(2_782_864, 0)
			.saturating_add(Weight::from_parts(0, 3999))
			// Standard Error: 25
			.saturating_add(Weight::from_parts(2_064, 0).saturating_mul(u.into()))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	/// Storage: `AssetMetadataExtender::ExtensionsCounter` (r:1 w:0)
	/// Proof: `AssetMetadataExtender::ExtensionsCounter` (`max_values`: None, `max_size`: Some(534), added: 3009, mode: `MaxEncodedLen`)
	/// Storage: `AssetMetadataExtender::ClaimersByLocationAndIndex` (r:1 w:0)
	/// Proof: `AssetMetadataExtender::ClaimersByLocationAndIndex` (`max_values`: None, `max_size`: Some(570), added: 3045, mode: `MaxEncodedLen`)
	/// The range of component `u` is `[0, 512]`.
	fn precompile_claimer_by_index(u: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `167 + u * (2 ±0)`
		//  Estimated: `4035`
		// Minimum execution time: 7_992_000 picoseconds.
		Weight::from_parts(9_296_162, 0)
			.saturating_add(Weight::from_parts(0, 4035))
			// Standard Error: 74
			.saturating_add(Weight::from_parts(14_301, 0).saturating_mul(u.into()))
			.saturating_add(T::DbWeight::get().reads(2))
	}
	/// Storage: `AssetMetadataExtender::ExtensionsCounter` (r:1 w:0)
	/// Proof: `AssetMetadataExtender::ExtensionsCounter` (`max_values`: None, `max_size`: Some(534), added: 3009, mode: `MaxEncodedLen`)
	/// Storage: `AssetMetadataExtender::ClaimersByLocationAndIndex` (r:1 w:0)
	/// Proof: `AssetMetadataExtender::ClaimersByLocationAndIndex` (`max_values`: None, `max_size`: Some(570), added: 3045, mode: `MaxEncodedLen`)
	/// Storage: `AssetMetadataExtender::TokenUrisByClaimerAndLocation` (r:1 w:0)
	/// Proof: `AssetMetadataExtender::TokenUrisByClaimerAndLocation` (`max_values`: None, `max_size`: Some(1080), added: 3555, mode: `MaxEncodedLen`)
	/// The range of component `u` is `[0, 512]`.
	fn precompile_extension_by_index(u: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `314 + u * (3 ±0)`
		//  Estimated: `4545`
		// Minimum execution time: 11_398_000 picoseconds.
		Weight::from_parts(12_923_930, 0)
			.saturating_add(Weight::from_parts(0, 4545))
			// Standard Error: 89
			.saturating_add(Weight::from_parts(20_319, 0).saturating_mul(u.into()))
			.saturating_add(T::DbWeight::get().reads(3))
	}
	/// Storage: `AssetMetadataExtender::TokenUrisByClaimerAndLocation` (r:1 w:0)
	/// Proof: `AssetMetadataExtender::TokenUrisByClaimerAndLocation` (`max_values`: None, `max_size`: Some(1080), added: 3555, mode: `MaxEncodedLen`)
	/// The range of component `u` is `[0, 512]`.
	fn precompile_extension_by_location_and_claimer(u: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `259 + u * (1 ±0)`
		//  Estimated: `4545`
		// Minimum execution time: 5_867_000 picoseconds.
		Weight::from_parts(6_775_035, 0)
			.saturating_add(Weight::from_parts(0, 4545))
			// Standard Error: 48
			.saturating_add(Weight::from_parts(5_906, 0).saturating_mul(u.into()))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	/// Storage: `AssetMetadataExtender::TokenUrisByClaimerAndLocation` (r:1 w:0)
	/// Proof: `AssetMetadataExtender::TokenUrisByClaimerAndLocation` (`max_values`: None, `max_size`: Some(1080), added: 3555, mode: `MaxEncodedLen`)
	/// The range of component `u` is `[0, 512]`.
	fn precompile_has_extension_by_claimer(u: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4`
		//  Estimated: `4545`
		// Minimum execution time: 2_671_000 picoseconds.
		Weight::from_parts(3_206_362, 0)
			.saturating_add(Weight::from_parts(0, 4545))
			// Standard Error: 29
			.saturating_add(Weight::from_parts(2_078, 0).saturating_mul(u.into()))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	/// Storage: `AssetMetadataExtender::TokenUrisByClaimerAndLocation` (r:1 w:1)
	/// Proof: `AssetMetadataExtender::TokenUrisByClaimerAndLocation` (`max_values`: None, `max_size`: Some(1080), added: 3555, mode: `MaxEncodedLen`)
	/// Storage: `AssetMetadataExtender::ExtensionsCounter` (r:1 w:1)
	/// Proof: `AssetMetadataExtender::ExtensionsCounter` (`max_values`: None, `max_size`: Some(534), added: 3009, mode: `MaxEncodedLen`)
	/// Storage: `AssetMetadataExtender::ClaimersByLocationAndIndex` (r:0 w:1)
	/// Proof: `AssetMetadataExtender::ClaimersByLocationAndIndex` (`max_values`: None, `max_size`: Some(570), added: 3045, mode: `MaxEncodedLen`)
	/// The range of component `t` is `[0, 512]`.
	/// The range of component `u` is `[0, 512]`.
	fn create_token_uri_extension(t: u32, u: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4`
		//  Estimated: `4545`
		// Minimum execution time: 13_163_000 picoseconds.
		Weight::from_parts(13_722_117, 0)
			.saturating_add(Weight::from_parts(0, 4545))
			// Standard Error: 78
			.saturating_add(Weight::from_parts(805, 0).saturating_mul(t.into()))
			// Standard Error: 78
			.saturating_add(Weight::from_parts(8_846, 0).saturating_mul(u.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `AssetMetadataExtender::TokenUrisByClaimerAndLocation` (r:1 w:1)
	/// Proof: `AssetMetadataExtender::TokenUrisByClaimerAndLocation` (`max_values`: None, `max_size`: Some(1080), added: 3555, mode: `MaxEncodedLen`)
	/// The range of component `t` is `[0, 512]`.
	/// The range of component `u` is `[0, 512]`.
	fn update_token_uri_extension(t: u32, u: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `185 + u * (1 ±0)`
		//  Estimated: `4545`
		// Minimum execution time: 11_053_000 picoseconds.
		Weight::from_parts(11_893_550, 0)
			.saturating_add(Weight::from_parts(0, 4545))
			// Standard Error: 67
			.saturating_add(Weight::from_parts(1_014, 0).saturating_mul(t.into()))
			// Standard Error: 67
			.saturating_add(Weight::from_parts(7_033, 0).saturating_mul(u.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}