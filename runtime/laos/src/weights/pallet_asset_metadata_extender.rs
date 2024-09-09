
//! Autogenerated weights for `pallet_asset_metadata_extender`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-09-07, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
		// Minimum execution time: 17_630_000 picoseconds.
		Weight::from_parts(19_603_696, 0)
			.saturating_add(Weight::from_parts(0, 4545))
			// Standard Error: 137
			.saturating_add(Weight::from_parts(459, 0).saturating_mul(t.into()))
			// Standard Error: 137
			.saturating_add(Weight::from_parts(13_075, 0).saturating_mul(u.into()))
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
		// Minimum execution time: 14_229_000 picoseconds.
		Weight::from_parts(14_597_553, 0)
			.saturating_add(Weight::from_parts(0, 4545))
			// Standard Error: 114
			.saturating_add(Weight::from_parts(2_966, 0).saturating_mul(t.into()))
			// Standard Error: 114
			.saturating_add(Weight::from_parts(13_002, 0).saturating_mul(u.into()))
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
		// Minimum execution time: 3_399_000 picoseconds.
		Weight::from_parts(3_868_306, 0)
			.saturating_add(Weight::from_parts(0, 3999))
			// Standard Error: 35
			.saturating_add(Weight::from_parts(2_206, 0).saturating_mul(u.into()))
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
		// Minimum execution time: 8_822_000 picoseconds.
		Weight::from_parts(10_581_526, 0)
			.saturating_add(Weight::from_parts(0, 4035))
			// Standard Error: 110
			.saturating_add(Weight::from_parts(16_494, 0).saturating_mul(u.into()))
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
		// Minimum execution time: 12_560_000 picoseconds.
		Weight::from_parts(14_896_322, 0)
			.saturating_add(Weight::from_parts(0, 4545))
			// Standard Error: 146
			.saturating_add(Weight::from_parts(22_432, 0).saturating_mul(u.into()))
			.saturating_add(T::DbWeight::get().reads(3))
	}
	/// Storage: `AssetMetadataExtender::TokenUrisByClaimerAndLocation` (r:1 w:0)
	/// Proof: `AssetMetadataExtender::TokenUrisByClaimerAndLocation` (`max_values`: None, `max_size`: Some(1080), added: 3555, mode: `MaxEncodedLen`)
	/// The range of component `u` is `[0, 512]`.
	fn precompile_extension_by_location_and_claimer(u: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `259 + u * (1 ±0)`
		//  Estimated: `4545`
		// Minimum execution time: 6_500_000 picoseconds.
		Weight::from_parts(7_445_231, 0)
			.saturating_add(Weight::from_parts(0, 4545))
			// Standard Error: 60
			.saturating_add(Weight::from_parts(6_853, 0).saturating_mul(u.into()))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	/// Storage: `AssetMetadataExtender::TokenUrisByClaimerAndLocation` (r:1 w:0)
	/// Proof: `AssetMetadataExtender::TokenUrisByClaimerAndLocation` (`max_values`: None, `max_size`: Some(1080), added: 3555, mode: `MaxEncodedLen`)
	/// The range of component `u` is `[0, 512]`.
	fn precompile_has_extension_by_claimer(u: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4`
		//  Estimated: `4545`
		// Minimum execution time: 3_682_000 picoseconds.
		Weight::from_parts(4_273_466, 0)
			.saturating_add(Weight::from_parts(0, 4545))
			// Standard Error: 37
			.saturating_add(Weight::from_parts(2_579, 0).saturating_mul(u.into()))
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
		// Minimum execution time: 13_397_000 picoseconds.
		Weight::from_parts(14_255_288, 0)
			.saturating_add(Weight::from_parts(0, 4545))
			// Standard Error: 82
			.saturating_add(Weight::from_parts(705, 0).saturating_mul(t.into()))
			// Standard Error: 82
			.saturating_add(Weight::from_parts(9_767, 0).saturating_mul(u.into()))
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
		// Minimum execution time: 10_393_000 picoseconds.
		Weight::from_parts(11_165_619, 0)
			.saturating_add(Weight::from_parts(0, 4545))
			// Standard Error: 57
			.saturating_add(Weight::from_parts(666, 0).saturating_mul(t.into()))
			// Standard Error: 57
			.saturating_add(Weight::from_parts(8_012, 0).saturating_mul(u.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
