
//! Autogenerated weights for `pallet_proxy`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-08-09, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
// --pallet=pallet_proxy
// --extrinsic=*
// --wasm-execution=compiled
// --output=./runtime/laos/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_proxy`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_proxy::WeightInfo for WeightInfo<T> {
	/// Storage: `Proxy::Proxies` (r:1 w:0)
	/// Proof: `Proxy::Proxies` (`max_values`: None, `max_size`: Some(845), added: 3320, mode: `MaxEncodedLen`)
	/// The range of component `p` is `[1, 31]`.
	fn proxy(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `115 + p * (25 ±0)`
		//  Estimated: `4310`
		// Minimum execution time: 9_434_000 picoseconds.
		Weight::from_parts(10_377_337, 0)
			.saturating_add(Weight::from_parts(0, 4310))
			// Standard Error: 1_342
			.saturating_add(Weight::from_parts(27_756, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	/// Storage: `Proxy::Proxies` (r:1 w:0)
	/// Proof: `Proxy::Proxies` (`max_values`: None, `max_size`: Some(845), added: 3320, mode: `MaxEncodedLen`)
	/// Storage: `Proxy::Announcements` (r:1 w:1)
	/// Proof: `Proxy::Announcements` (`max_values`: None, `max_size`: Some(1837), added: 4312, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// The range of component `a` is `[0, 31]`.
	/// The range of component `p` is `[1, 31]`.
	fn proxy_announced(a: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `367 + a * (56 ±0) + p * (25 ±0)`
		//  Estimated: `5302`
		// Minimum execution time: 25_950_000 picoseconds.
		Weight::from_parts(27_077_814, 0)
			.saturating_add(Weight::from_parts(0, 5302))
			// Standard Error: 2_535
			.saturating_add(Weight::from_parts(123_283, 0).saturating_mul(a.into()))
			// Standard Error: 2_619
			.saturating_add(Weight::from_parts(24_272, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Proxy::Announcements` (r:1 w:1)
	/// Proof: `Proxy::Announcements` (`max_values`: None, `max_size`: Some(1837), added: 4312, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// The range of component `a` is `[0, 31]`.
	/// The range of component `p` is `[1, 31]`.
	fn remove_announcement(a: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `295 + a * (56 ±0)`
		//  Estimated: `5302`
		// Minimum execution time: 18_197_000 picoseconds.
		Weight::from_parts(19_143_760, 0)
			.saturating_add(Weight::from_parts(0, 5302))
			// Standard Error: 1_692
			.saturating_add(Weight::from_parts(121_460, 0).saturating_mul(a.into()))
			// Standard Error: 1_748
			.saturating_add(Weight::from_parts(2_890, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Proxy::Announcements` (r:1 w:1)
	/// Proof: `Proxy::Announcements` (`max_values`: None, `max_size`: Some(1837), added: 4312, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// The range of component `a` is `[0, 31]`.
	/// The range of component `p` is `[1, 31]`.
	fn reject_announcement(a: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `295 + a * (56 ±0)`
		//  Estimated: `5302`
		// Minimum execution time: 18_144_000 picoseconds.
		Weight::from_parts(19_027_447, 0)
			.saturating_add(Weight::from_parts(0, 5302))
			// Standard Error: 1_764
			.saturating_add(Weight::from_parts(126_448, 0).saturating_mul(a.into()))
			// Standard Error: 1_822
			.saturating_add(Weight::from_parts(5_846, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Proxy::Proxies` (r:1 w:0)
	/// Proof: `Proxy::Proxies` (`max_values`: None, `max_size`: Some(845), added: 3320, mode: `MaxEncodedLen`)
	/// Storage: `Proxy::Announcements` (r:1 w:1)
	/// Proof: `Proxy::Announcements` (`max_values`: None, `max_size`: Some(1837), added: 4312, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// The range of component `a` is `[0, 31]`.
	/// The range of component `p` is `[1, 31]`.
	fn announce(a: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `311 + a * (56 ±0) + p * (25 ±0)`
		//  Estimated: `5302`
		// Minimum execution time: 23_398_000 picoseconds.
		Weight::from_parts(23_905_849, 0)
			.saturating_add(Weight::from_parts(0, 5302))
			// Standard Error: 2_036
			.saturating_add(Weight::from_parts(134_353, 0).saturating_mul(a.into()))
			// Standard Error: 2_103
			.saturating_add(Weight::from_parts(28_077, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Proxy::Proxies` (r:1 w:1)
	/// Proof: `Proxy::Proxies` (`max_values`: None, `max_size`: Some(845), added: 3320, mode: `MaxEncodedLen`)
	/// The range of component `p` is `[1, 31]`.
	fn add_proxy(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `115 + p * (25 ±0)`
		//  Estimated: `4310`
		// Minimum execution time: 17_162_000 picoseconds.
		Weight::from_parts(17_996_660, 0)
			.saturating_add(Weight::from_parts(0, 4310))
			// Standard Error: 1_546
			.saturating_add(Weight::from_parts(32_855, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Proxy::Proxies` (r:1 w:1)
	/// Proof: `Proxy::Proxies` (`max_values`: None, `max_size`: Some(845), added: 3320, mode: `MaxEncodedLen`)
	/// The range of component `p` is `[1, 31]`.
	fn remove_proxy(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `115 + p * (25 ±0)`
		//  Estimated: `4310`
		// Minimum execution time: 17_117_000 picoseconds.
		Weight::from_parts(18_407_845, 0)
			.saturating_add(Weight::from_parts(0, 4310))
			// Standard Error: 1_658
			.saturating_add(Weight::from_parts(30_633, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Proxy::Proxies` (r:1 w:1)
	/// Proof: `Proxy::Proxies` (`max_values`: None, `max_size`: Some(845), added: 3320, mode: `MaxEncodedLen`)
	/// The range of component `p` is `[1, 31]`.
	fn remove_proxies(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `115 + p * (25 ±0)`
		//  Estimated: `4310`
		// Minimum execution time: 17_207_000 picoseconds.
		Weight::from_parts(17_889_965, 0)
			.saturating_add(Weight::from_parts(0, 4310))
			// Standard Error: 1_490
			.saturating_add(Weight::from_parts(29_668, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Proxy::Proxies` (r:1 w:1)
	/// Proof: `Proxy::Proxies` (`max_values`: None, `max_size`: Some(845), added: 3320, mode: `MaxEncodedLen`)
	/// The range of component `p` is `[1, 31]`.
	fn create_pure(_p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `127`
		//  Estimated: `4310`
		// Minimum execution time: 18_128_000 picoseconds.
		Weight::from_parts(19_217_828, 0)
			.saturating_add(Weight::from_parts(0, 4310))
			// Standard Error: 1_577
			.saturating_add(Weight::from_parts(19_993, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Proxy::Proxies` (r:1 w:1)
	/// Proof: `Proxy::Proxies` (`max_values`: None, `max_size`: Some(845), added: 3320, mode: `MaxEncodedLen`)
	/// The range of component `p` is `[0, 30]`.
	fn kill_pure(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `140 + p * (25 ±0)`
		//  Estimated: `4310`
		// Minimum execution time: 17_564_000 picoseconds.
		Weight::from_parts(18_523_353, 0)
			.saturating_add(Weight::from_parts(0, 4310))
			// Standard Error: 1_361
			.saturating_add(Weight::from_parts(33_056, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
