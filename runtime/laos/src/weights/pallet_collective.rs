
//! Autogenerated weights for `pallet_collective`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 42.0.1
//! DATE: 2024-12-17, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
// --pallet=pallet_collective
// --extrinsic=*
// --wasm-execution=compiled
// --output=./runtime/laos/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_collective`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_collective::WeightInfo for WeightInfo<T> {
	/// Storage: `Council::Members` (r:1 w:1)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Proposals` (r:1 w:0)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Voting` (r:20 w:20)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Prime` (r:0 w:1)
	/// Proof: `Council::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[0, 7]`.
	/// The range of component `n` is `[0, 7]`.
	/// The range of component `p` is `[0, 20]`.
	fn set_members(m: u32, _n: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + m * (430 ±0) + p * (166 ±0)`
		//  Estimated: `3970 + m * (265 ±3) + p * (2537 ±1)`
		// Minimum execution time: 6_366_000 picoseconds.
		Weight::from_parts(6_893_000, 0)
			.saturating_add(Weight::from_parts(0, 3970))
			// Standard Error: 105_471
			.saturating_add(Weight::from_parts(3_161_046, 0).saturating_mul(m.into()))
			// Standard Error: 37_610
			.saturating_add(Weight::from_parts(3_035_423, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(m.into())))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(p.into())))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(m.into())))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(p.into())))
			.saturating_add(Weight::from_parts(0, 265).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 2537).saturating_mul(p.into()))
	}
	/// Storage: `Council::Members` (r:1 w:0)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 7]`.
	fn execute(b: u32, m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `66 + m * (20 ±0)`
		//  Estimated: `1552 + m * (20 ±0)`
		// Minimum execution time: 9_412_000 picoseconds.
		Weight::from_parts(9_565_827, 0)
			.saturating_add(Weight::from_parts(0, 1552))
			// Standard Error: 27
			.saturating_add(Weight::from_parts(1_216, 0).saturating_mul(b.into()))
			// Standard Error: 4_262
			.saturating_add(Weight::from_parts(47_864, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(Weight::from_parts(0, 20).saturating_mul(m.into()))
	}
	/// Storage: `Council::Members` (r:1 w:0)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalOf` (r:1 w:0)
	/// Proof: `Council::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 7]`.
	fn propose_execute(b: u32, m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `66 + m * (20 ±0)`
		//  Estimated: `3532 + m * (20 ±0)`
		// Minimum execution time: 11_400_000 picoseconds.
		Weight::from_parts(11_529_034, 0)
			.saturating_add(Weight::from_parts(0, 3532))
			// Standard Error: 30
			.saturating_add(Weight::from_parts(1_330, 0).saturating_mul(b.into()))
			// Standard Error: 4_709
			.saturating_add(Weight::from_parts(76_392, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(Weight::from_parts(0, 20).saturating_mul(m.into()))
	}
	/// Storage: `Council::Members` (r:1 w:0)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalOf` (r:1 w:1)
	/// Proof: `Council::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Proposals` (r:1 w:1)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalCount` (r:1 w:1)
	/// Proof: `Council::ProposalCount` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Voting` (r:0 w:1)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[2, 7]`.
	/// The range of component `p` is `[1, 20]`.
	fn propose_proposed(b: u32, m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `58 + m * (20 ±0) + p * (55 ±0)`
		//  Estimated: `3473 + m * (30 ±0) + p * (54 ±0)`
		// Minimum execution time: 15_453_000 picoseconds.
		Weight::from_parts(14_686_217, 0)
			.saturating_add(Weight::from_parts(0, 3473))
			// Standard Error: 50
			.saturating_add(Weight::from_parts(1_972, 0).saturating_mul(b.into()))
			// Standard Error: 9_322
			.saturating_add(Weight::from_parts(117_024, 0).saturating_mul(m.into()))
			// Standard Error: 2_654
			.saturating_add(Weight::from_parts(335_880, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(Weight::from_parts(0, 30).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 54).saturating_mul(p.into()))
	}
	/// Storage: `Council::Members` (r:1 w:0)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Voting` (r:1 w:1)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[5, 7]`.
	fn vote(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `634 + m * (40 ±0)`
		//  Estimated: `4099 + m * (40 ±0)`
		// Minimum execution time: 15_047_000 picoseconds.
		Weight::from_parts(16_449_933, 0)
			.saturating_add(Weight::from_parts(0, 4099))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_parts(0, 40).saturating_mul(m.into()))
	}
	/// Storage: `Council::Voting` (r:1 w:1)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Members` (r:1 w:0)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Proposals` (r:1 w:1)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalOf` (r:0 w:1)
	/// Proof: `Council::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[4, 7]`.
	/// The range of component `p` is `[1, 20]`.
	fn close_early_disapproved(m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `167 + m * (40 ±0) + p * (55 ±0)`
		//  Estimated: `3621 + m * (44 ±0) + p * (55 ±0)`
		// Minimum execution time: 16_869_000 picoseconds.
		Weight::from_parts(17_441_028, 0)
			.saturating_add(Weight::from_parts(0, 3621))
			// Standard Error: 12_107
			.saturating_add(Weight::from_parts(54_313, 0).saturating_mul(m.into()))
			// Standard Error: 2_206
			.saturating_add(Weight::from_parts(294_320, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 44).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 55).saturating_mul(p.into()))
	}
	/// Storage: `Council::Voting` (r:1 w:1)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Members` (r:1 w:0)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalOf` (r:1 w:1)
	/// Proof: `Council::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Proposals` (r:1 w:1)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 7]`.
	/// The range of component `p` is `[1, 20]`.
	fn close_early_approved(b: u32, m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `112 + b * (1 ±0) + m * (40 ±0) + p * (78 ±0)`
		//  Estimated: `3747 + b * (1 ±0) + m * (23 ±1) + p * (74 ±0)`
		// Minimum execution time: 23_810_000 picoseconds.
		Weight::from_parts(28_966_889, 0)
			.saturating_add(Weight::from_parts(0, 3747))
			// Standard Error: 100
			.saturating_add(Weight::from_parts(1_765, 0).saturating_mul(b.into()))
			// Standard Error: 5_256
			.saturating_add(Weight::from_parts(459_633, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 1).saturating_mul(b.into()))
			.saturating_add(Weight::from_parts(0, 23).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 74).saturating_mul(p.into()))
	}
	/// Storage: `Council::Voting` (r:1 w:1)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Members` (r:1 w:0)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Prime` (r:1 w:0)
	/// Proof: `Council::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Proposals` (r:1 w:1)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalOf` (r:0 w:1)
	/// Proof: `Council::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[4, 7]`.
	/// The range of component `p` is `[1, 20]`.
	fn close_disapproved(m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `184 + m * (40 ±0) + p * (55 ±0)`
		//  Estimated: `3654 + m * (39 ±0) + p * (56 ±0)`
		// Minimum execution time: 18_018_000 picoseconds.
		Weight::from_parts(18_872_801, 0)
			.saturating_add(Weight::from_parts(0, 3654))
			// Standard Error: 18_049
			.saturating_add(Weight::from_parts(27_362, 0).saturating_mul(m.into()))
			// Standard Error: 3_289
			.saturating_add(Weight::from_parts(309_974, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 39).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 56).saturating_mul(p.into()))
	}
	/// Storage: `Council::Voting` (r:1 w:1)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Members` (r:1 w:0)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Prime` (r:1 w:0)
	/// Proof: `Council::Prime` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalOf` (r:1 w:1)
	/// Proof: `Council::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Proposals` (r:1 w:1)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 7]`.
	/// The range of component `p` is `[1, 20]`.
	fn close_approved(b: u32, m: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `128 + b * (1 ±0) + m * (40 ±0) + p * (78 ±0)`
		//  Estimated: `3764 + b * (1 ±0) + m * (23 ±1) + p * (74 ±0)`
		// Minimum execution time: 26_093_000 picoseconds.
		Weight::from_parts(27_635_560, 0)
			.saturating_add(Weight::from_parts(0, 3764))
			// Standard Error: 133
			.saturating_add(Weight::from_parts(2_513, 0).saturating_mul(b.into()))
			// Standard Error: 6_961
			.saturating_add(Weight::from_parts(488_271, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 1).saturating_mul(b.into()))
			.saturating_add(Weight::from_parts(0, 23).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 74).saturating_mul(p.into()))
	}
	/// Storage: `Council::Proposals` (r:1 w:1)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Voting` (r:0 w:1)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalOf` (r:0 w:1)
	/// Proof: `Council::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `p` is `[1, 20]`.
	fn disapprove_proposal(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `226 + p * (32 ±0)`
		//  Estimated: `1711 + p * (32 ±0)`
		// Minimum execution time: 9_964_000 picoseconds.
		Weight::from_parts(10_705_872, 0)
			.saturating_add(Weight::from_parts(0, 1711))
			// Standard Error: 2_317
			.saturating_add(Weight::from_parts(221_815, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 32).saturating_mul(p.into()))
	}
}
