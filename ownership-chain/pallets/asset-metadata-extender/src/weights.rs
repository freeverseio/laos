
//! Autogenerated weights for `pallet_asset_metadata_extender`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-01-11, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `trujideb`, CPU: `12th Gen Intel(R) Core(TM) i5-12500H`
//! WASM-EXECUTION: `Compiled`, CHAIN: `None`, DB CACHE: `1024`

// Executed Command:
// ./target/release/laos-ownership
// benchmark
// pallet
// --pallet
// pallet-asset-metadata-extender
// --extrinsic=*
// --output
// ownership-chain/pallets/asset-metadata-extender/src/weights.rs
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

/// Weight functions needed for `pallet_asset_metadata_extender`.
pub trait WeightInfo {
	fn create_token_uri_extension(t: u32, u: u32, ) -> Weight;
}

/// Weights for `pallet_asset_metadata_extender` using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
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
		// Minimum execution time: 13_352_000 picoseconds.
		Weight::from_parts(14_181_394, 4545)
			// Standard Error: 85
			.saturating_add(Weight::from_parts(7_199, 0).saturating_mul(t.into()))
			// Standard Error: 85
			.saturating_add(Weight::from_parts(388, 0).saturating_mul(u.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
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
		// Minimum execution time: 13_352_000 picoseconds.
		Weight::from_parts(14_181_394, 4545)
			// Standard Error: 85
			.saturating_add(Weight::from_parts(7_199, 0).saturating_mul(t.into()))
			// Standard Error: 85
			.saturating_add(Weight::from_parts(388, 0).saturating_mul(u.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
}