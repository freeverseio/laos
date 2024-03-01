
//! Autogenerated weights for `pallet_block_rewards_source`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-03-01, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `trujideb`, CPU: `12th Gen Intel(R) Core(TM) i5-12500H`
//! WASM-EXECUTION: `Compiled`, CHAIN: `None`, DB CACHE: `1024`

// Executed Command:
// ./target/release/laos-ownership
// benchmark
// pallet
// --pallet
// pallet-block-rewards-source
// --extrinsic=*
// --output
// ownership-chain/pallets/block-rewards-source/src/weights.rs
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

/// Weight functions needed for `pallet_block_rewards_source`.
pub trait WeightInfo {
	fn set_rewards_account() -> Weight;
}

/// Weights for `pallet_block_rewards_source` using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `BlockRewardsSource::RewardsAccount` (r:0 w:1)
	/// Proof: `BlockRewardsSource::RewardsAccount` (`max_values`: Some(1), `max_size`: Some(20), added: 515, mode: `MaxEncodedLen`)
	fn set_rewards_account() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 5_017_000 picoseconds.
		Weight::from_parts(5_292_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	/// Storage: `BlockRewardsSource::RewardsAccount` (r:0 w:1)
	/// Proof: `BlockRewardsSource::RewardsAccount` (`max_values`: Some(1), `max_size`: Some(20), added: 515, mode: `MaxEncodedLen`)
	fn set_rewards_account() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 5_017_000 picoseconds.
		Weight::from_parts(5_292_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}