
//! Autogenerated weights for `pallet_elections_phragmen`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 42.0.1
//! DATE: 2024-12-12, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
// --pallet=pallet_elections_phragmen
// --extrinsic=*
// --wasm-execution=compiled
// --output=./runtime/laos/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_elections_phragmen`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_elections_phragmen::WeightInfo for WeightInfo<T> {
	/// Storage: `Elections::Candidates` (r:1 w:0)
	/// Proof: `Elections::Candidates` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::Members` (r:1 w:0)
	/// Proof: `Elections::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::RunnersUp` (r:1 w:0)
	/// Proof: `Elections::RunnersUp` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::Voting` (r:1 w:1)
	/// Proof: `Elections::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1287), added: 3762, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(887), added: 3362, mode: `MaxEncodedLen`)
	/// The range of component `v` is `[1, 8]`.
	fn vote_equal(v: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `438 + v * (56 ±0)`
		//  Estimated: `4752 + v * (56 ±0)`
		// Minimum execution time: 33_053_000 picoseconds.
		Weight::from_parts(34_796_352, 0)
			.saturating_add(Weight::from_parts(0, 4752))
			// Standard Error: 6_517
			.saturating_add(Weight::from_parts(37_472, 0).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(Weight::from_parts(0, 56).saturating_mul(v.into()))
	}
	/// Storage: `Elections::Candidates` (r:1 w:0)
	/// Proof: `Elections::Candidates` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::Members` (r:1 w:0)
	/// Proof: `Elections::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::RunnersUp` (r:1 w:0)
	/// Proof: `Elections::RunnersUp` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::Voting` (r:1 w:1)
	/// Proof: `Elections::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1287), added: 3762, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(887), added: 3362, mode: `MaxEncodedLen`)
	/// The range of component `v` is `[2, 8]`.
	fn vote_more(v: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `418 + v * (56 ±0)`
		//  Estimated: `4752 + v * (56 ±0)`
		// Minimum execution time: 46_208_000 picoseconds.
		Weight::from_parts(48_033_478, 0)
			.saturating_add(Weight::from_parts(0, 4752))
			// Standard Error: 14_262
			.saturating_add(Weight::from_parts(144_891, 0).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(Weight::from_parts(0, 56).saturating_mul(v.into()))
	}
	/// Storage: `Elections::Candidates` (r:1 w:0)
	/// Proof: `Elections::Candidates` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::Members` (r:1 w:0)
	/// Proof: `Elections::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::RunnersUp` (r:1 w:0)
	/// Proof: `Elections::RunnersUp` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::Voting` (r:1 w:1)
	/// Proof: `Elections::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1287), added: 3762, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(887), added: 3362, mode: `MaxEncodedLen`)
	/// The range of component `v` is `[2, 8]`.
	fn vote_less(v: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `438 + v * (56 ±0)`
		//  Estimated: `4752 + v * (56 ±0)`
		// Minimum execution time: 46_779_000 picoseconds.
		Weight::from_parts(48_105_280, 0)
			.saturating_add(Weight::from_parts(0, 4752))
			// Standard Error: 18_390
			.saturating_add(Weight::from_parts(173_146, 0).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(Weight::from_parts(0, 56).saturating_mul(v.into()))
	}
	/// Storage: `Elections::Voting` (r:1 w:1)
	/// Proof: `Elections::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1287), added: 3762, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(887), added: 3362, mode: `MaxEncodedLen`)
	fn remove_voter() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `608`
		//  Estimated: `4752`
		// Minimum execution time: 45_326_000 picoseconds.
		Weight::from_parts(47_120_000, 0)
			.saturating_add(Weight::from_parts(0, 4752))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Elections::Candidates` (r:1 w:1)
	/// Proof: `Elections::Candidates` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::Members` (r:1 w:0)
	/// Proof: `Elections::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::RunnersUp` (r:1 w:0)
	/// Proof: `Elections::RunnersUp` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `c` is `[1, 30]`.
	fn submit_candidacy(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1825 + c * (36 ±0)`
		//  Estimated: `3301 + c * (37 ±0)`
		// Minimum execution time: 33_944_000 picoseconds.
		Weight::from_parts(36_542_922, 0)
			.saturating_add(Weight::from_parts(0, 3301))
			// Standard Error: 4_523
			.saturating_add(Weight::from_parts(64_193, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_parts(0, 37).saturating_mul(c.into()))
	}
	/// Storage: `Elections::Candidates` (r:1 w:1)
	/// Proof: `Elections::Candidates` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `c` is `[1, 30]`.
	fn renounce_candidacy_candidate(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `408 + c * (38 ±0)`
		//  Estimated: `1890 + c * (39 ±0)`
		// Minimum execution time: 28_755_000 picoseconds.
		Weight::from_parts(30_016_242, 0)
			.saturating_add(Weight::from_parts(0, 1890))
			// Standard Error: 5_035
			.saturating_add(Weight::from_parts(73_818, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_parts(0, 39).saturating_mul(c.into()))
	}
	/// Storage: `Elections::Members` (r:1 w:1)
	/// Proof: `Elections::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::RunnersUp` (r:1 w:1)
	/// Proof: `Elections::RunnersUp` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Prime` (r:1 w:1)
	/// Proof: `Council::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Proposals` (r:1 w:0)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Members` (r:0 w:1)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn renounce_candidacy_members() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1987`
		//  Estimated: `3472`
		// Minimum execution time: 39_875_000 picoseconds.
		Weight::from_parts(41_529_000, 0)
			.saturating_add(Weight::from_parts(0, 3472))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `Elections::RunnersUp` (r:1 w:1)
	/// Proof: `Elections::RunnersUp` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn renounce_candidacy_runners_up() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1442`
		//  Estimated: `2927`
		// Minimum execution time: 29_997_000 picoseconds.
		Weight::from_parts(31_119_000, 0)
			.saturating_add(Weight::from_parts(0, 2927))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Benchmark::Override` (r:0 w:0)
	/// Proof: `Benchmark::Override` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn remove_member_without_replacement() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 500_000_000_000 picoseconds.
		Weight::from_parts(500_000_000_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	/// Storage: `Elections::Members` (r:1 w:1)
	/// Proof: `Elections::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// Storage: `Elections::RunnersUp` (r:1 w:1)
	/// Proof: `Elections::RunnersUp` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Prime` (r:1 w:1)
	/// Proof: `Council::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Proposals` (r:1 w:0)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Members` (r:0 w:1)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn remove_member_with_replacement() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2149`
		//  Estimated: `6172`
		// Minimum execution time: 58_330_000 picoseconds.
		Weight::from_parts(60_716_000, 0)
			.saturating_add(Weight::from_parts(0, 6172))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: `Elections::Voting` (r:101 w:100)
	/// Proof: `Elections::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::Members` (r:1 w:0)
	/// Proof: `Elections::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::RunnersUp` (r:1 w:0)
	/// Proof: `Elections::RunnersUp` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::Candidates` (r:1 w:0)
	/// Proof: `Elections::Candidates` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Balances::Locks` (r:100 w:100)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1287), added: 3762, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:100 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(887), added: 3362, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:100 w:100)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// The range of component `v` is `[100, 200]`.
	/// The range of component `d` is `[0, 100]`.
	fn clean_defunct_voters(v: u32, d: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + d * (427 ±0) + v * (64 ±0)`
		//  Estimated: `12070 + d * (3762 ±0) + v * (24 ±0)`
		// Minimum execution time: 4_729_000 picoseconds.
		Weight::from_parts(5_541_000, 0)
			.saturating_add(Weight::from_parts(0, 12070))
			// Standard Error: 19_044
			.saturating_add(Weight::from_parts(259_372, 0).saturating_mul(v.into()))
			// Standard Error: 41_500
			.saturating_add(Weight::from_parts(47_890_937, 0).saturating_mul(d.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().reads((4_u64).saturating_mul(d.into())))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(d.into())))
			.saturating_add(Weight::from_parts(0, 3762).saturating_mul(d.into()))
			.saturating_add(Weight::from_parts(0, 24).saturating_mul(v.into()))
	}
	/// Storage: `Elections::Candidates` (r:1 w:1)
	/// Proof: `Elections::Candidates` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::Members` (r:1 w:1)
	/// Proof: `Elections::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::RunnersUp` (r:1 w:1)
	/// Proof: `Elections::RunnersUp` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Elections::Voting` (r:201 w:0)
	/// Proof: `Elections::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Proposals` (r:1 w:0)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:4 w:4)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// Storage: `Elections::ElectionRounds` (r:1 w:1)
	/// Proof: `Elections::ElectionRounds` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Members` (r:0 w:1)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Prime` (r:0 w:1)
	/// Proof: `Council::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `c` is `[1, 30]`.
	/// The range of component `v` is `[1, 200]`.
	/// The range of component `e` is `[200, 1600]`.
	fn election_phragmen(c: u32, v: u32, e: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + c * (7 ±0) + e * (17 ±0) + v * (233 ±0)`
		//  Estimated: `82212 + c * (208 ±3) + e * (6 ±0) + v * (2427 ±2)`
		// Minimum execution time: 322_763_000 picoseconds.
		Weight::from_parts(329_385_000, 0)
			.saturating_add(Weight::from_parts(0, 82212))
			// Standard Error: 1_591_848
			.saturating_add(Weight::from_parts(8_896_163, 0).saturating_mul(c.into()))
			// Standard Error: 237_751
			.saturating_add(Weight::from_parts(9_151_221, 0).saturating_mul(v.into()))
			// Standard Error: 31_315
			.saturating_add(Weight::from_parts(301_304, 0).saturating_mul(e.into()))
			.saturating_add(T::DbWeight::get().reads(19))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(v.into())))
			.saturating_add(T::DbWeight::get().writes(8))
			.saturating_add(Weight::from_parts(0, 208).saturating_mul(c.into()))
			.saturating_add(Weight::from_parts(0, 6).saturating_mul(e.into()))
			.saturating_add(Weight::from_parts(0, 2427).saturating_mul(v.into()))
	}
}
