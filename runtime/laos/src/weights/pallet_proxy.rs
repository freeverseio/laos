
//! Autogenerated weights for `pallet_proxy`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 42.0.1
//! DATE: 2024-12-17, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `giove`, CPU: `AMD Ryzen 7 5700G with Radeon Graphics`
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
		// Minimum execution time: 14_457_000 picoseconds.
		Weight::from_parts(15_354_373, 0)
			.saturating_add(Weight::from_parts(0, 4310))
			// Standard Error: 1_137
			.saturating_add(Weight::from_parts(22_002, 0).saturating_mul(p.into()))
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
		// Minimum execution time: 37_644_000 picoseconds.
		Weight::from_parts(37_958_386, 0)
			.saturating_add(Weight::from_parts(0, 5302))
			// Standard Error: 1_736
			.saturating_add(Weight::from_parts(214_727, 0).saturating_mul(a.into()))
			// Standard Error: 1_793
			.saturating_add(Weight::from_parts(21_736, 0).saturating_mul(p.into()))
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
		// Minimum execution time: 26_610_000 picoseconds.
		Weight::from_parts(27_733_117, 0)
			.saturating_add(Weight::from_parts(0, 5302))
			// Standard Error: 1_382
			.saturating_add(Weight::from_parts(205_253, 0).saturating_mul(a.into()))
			// Standard Error: 1_428
			.saturating_add(Weight::from_parts(985, 0).saturating_mul(p.into()))
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
		// Minimum execution time: 27_098_000 picoseconds.
		Weight::from_parts(27_692_691, 0)
			.saturating_add(Weight::from_parts(0, 5302))
			// Standard Error: 1_350
			.saturating_add(Weight::from_parts(207_859, 0).saturating_mul(a.into()))
			// Standard Error: 1_395
			.saturating_add(Weight::from_parts(364, 0).saturating_mul(p.into()))
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
		// Minimum execution time: 34_572_000 picoseconds.
		Weight::from_parts(34_272_338, 0)
			.saturating_add(Weight::from_parts(0, 5302))
			// Standard Error: 1_413
			.saturating_add(Weight::from_parts(200_970, 0).saturating_mul(a.into()))
			// Standard Error: 1_460
			.saturating_add(Weight::from_parts(24_567, 0).saturating_mul(p.into()))
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
		// Minimum execution time: 26_609_000 picoseconds.
		Weight::from_parts(27_565_495, 0)
			.saturating_add(Weight::from_parts(0, 4310))
			// Standard Error: 1_285
			.saturating_add(Weight::from_parts(30_791, 0).saturating_mul(p.into()))
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
		// Minimum execution time: 26_540_000 picoseconds.
		Weight::from_parts(27_380_695, 0)
			.saturating_add(Weight::from_parts(0, 4310))
			// Standard Error: 1_241
			.saturating_add(Weight::from_parts(45_046, 0).saturating_mul(p.into()))
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
		// Minimum execution time: 24_374_000 picoseconds.
		Weight::from_parts(25_457_200, 0)
			.saturating_add(Weight::from_parts(0, 4310))
			// Standard Error: 1_436
			.saturating_add(Weight::from_parts(39_850, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Proxy::Proxies` (r:1 w:1)
	/// Proof: `Proxy::Proxies` (`max_values`: None, `max_size`: Some(845), added: 3320, mode: `MaxEncodedLen`)
	/// The range of component `p` is `[1, 31]`.
	fn create_pure(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `127`
		//  Estimated: `4310`
		// Minimum execution time: 28_705_000 picoseconds.
		Weight::from_parts(29_952_705, 0)
			.saturating_add(Weight::from_parts(0, 4310))
			// Standard Error: 1_870
			.saturating_add(Weight::from_parts(11_887, 0).saturating_mul(p.into()))
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
		// Minimum execution time: 25_632_000 picoseconds.
		Weight::from_parts(26_793_452, 0)
			.saturating_add(Weight::from_parts(0, 4310))
			// Standard Error: 1_605
			.saturating_add(Weight::from_parts(33_364, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
