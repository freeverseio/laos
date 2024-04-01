// Copyright 2023-2024 Freverse.io
// This file is part of LAOS.

// LAOS is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// LAOS is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with LAOS.  If not, see <http://www.gnu.org/licenses/>.

// Copyright 2019-2022 PureStake Inc.
// This file is part of Moonbeam.

// Moonbeam is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Moonbeam is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Moonbeam.  If not, see <http://www.gnu.org/licenses/>.

//! Helper methods for computing issuance based on inflation
use crate::pallet::{BalanceOf, Config, Pallet};
use frame_support::traits::{Currency, Get};
use parity_scale_codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};
use sp_runtime::{PerThing, Perbill, RuntimeDebug};
use substrate_fixed::{transcendental::pow as floatpow, types::I64F64};

fn rounds_per_year<T: Config>() -> u32 {
	let blocks_per_round = <Pallet<T>>::round().length;
	T::SlotsPerYear::get() / blocks_per_round
}

#[derive(
	Eq,
	PartialEq,
	Clone,
	Copy,
	Encode,
	Decode,
	Default,
	Deserialize,
	RuntimeDebug,
	MaxEncodedLen,
	Serialize,
	TypeInfo,
)]
pub struct Range<T> {
	pub min: T,
	pub ideal: T,
	pub max: T,
}

impl<T: Ord> Range<T> {
	pub fn is_valid(&self) -> bool {
		self.max >= self.ideal && self.ideal >= self.min
	}
}

impl<T: Ord + Copy> From<T> for Range<T> {
	fn from(other: T) -> Range<T> {
		Range { min: other, ideal: other, max: other }
	}
}
/// Convert an annual inflation to a round inflation
/// round = (1+annual)^(1/rounds_per_year) - 1
pub fn perbill_annual_to_perbill_round(
	annual: Range<Perbill>,
	rounds_per_year: u32,
) -> Range<Perbill> {
	let exponent = I64F64::from_num(1) / I64F64::from_num(rounds_per_year);
	let annual_to_round = |annual: Perbill| -> Perbill {
		let x = I64F64::from_num(annual.deconstruct()) / I64F64::from_num(Perbill::ACCURACY);
		let y: I64F64 = floatpow(I64F64::from_num(1) + x, exponent)
			.expect("Cannot overflow since rounds_per_year is u32 so worst case 0; QED");
		Perbill::from_parts(
			((y - I64F64::from_num(1)) * I64F64::from_num(Perbill::ACCURACY))
				.ceil()
				.to_num::<u32>(),
		)
	};
	Range {
		min: annual_to_round(annual.min),
		ideal: annual_to_round(annual.ideal),
		max: annual_to_round(annual.max),
	}
}

/// Convert an annual inflation rate to a per-round inflation rate without considering compounding.
/// The calculation is simply dividing the annual rate by the number of rounds per year.
pub fn perbill_annual_to_perbill_round_simple(
	annual: Range<Perbill>,
	rounds_per_year: u32,
) -> Range<Perbill> {
	let annual_to_round_simple = |annual: Perbill| -> Perbill {
		// Convert the annual rate from Perbill to a fractional representation.
		let x = I64F64::from_num(annual.deconstruct()) / I64F64::from_num(Perbill::ACCURACY);
		// Divide the annual rate by the number of rounds to get the per-round rate.
		let y: I64F64 = x / I64F64::from_num(rounds_per_year);
		// Convert back to Perbill, rounding as necessary.
		Perbill::from_parts((y * I64F64::from_num(Perbill::ACCURACY)).floor().to_num::<u32>())
	};
	Range {
		min: annual_to_round_simple(annual.min),
		ideal: annual_to_round_simple(annual.ideal),
		max: annual_to_round_simple(annual.max),
	}
}

/// Convert annual inflation rate range to round inflation range
pub fn annual_to_round<T: Config>(annual: Range<Perbill>) -> Range<Perbill> {
	let periods = rounds_per_year::<T>();
	perbill_annual_to_perbill_round_simple(annual, periods)
}

/// Compute round issuance range from round inflation range and current total issuance
pub fn round_issuance_range<T: Config>(round: Range<Perbill>) -> Range<BalanceOf<T>> {
	let circulating = T::Currency::total_issuance();
	Range {
		min: round.min * circulating,
		ideal: round.ideal * circulating,
		max: round.max * circulating,
	}
}

#[derive(
	Eq, PartialEq, Clone, Encode, Decode, Default, Deserialize, RuntimeDebug, Serialize, TypeInfo,
)]
pub struct InflationInfo<Balance> {
	/// Staking expectations
	pub expect: Range<Balance>,
	/// Annual inflation range
	pub annual: Range<Perbill>,
	/// Round inflation range
	pub round: Range<Perbill>,
}

impl<Balance> InflationInfo<Balance> {
	pub fn new<T: Config>(
		annual: Range<Perbill>,
		expect: Range<Balance>,
	) -> InflationInfo<Balance> {
		InflationInfo { expect, annual, round: annual_to_round::<T>(annual) }
	}
	/// Set round inflation range according to input annual inflation range
	pub fn set_round_from_annual<T: Config>(&mut self, new: Range<Perbill>) {
		self.round = annual_to_round::<T>(new);
	}
	/// Reset round inflation rate based on changes to round length
	pub fn reset_round<T: Config>(&mut self, new_length: u32) {
		let periods = T::SlotsPerYear::get() / new_length;
		self.round = perbill_annual_to_perbill_round_simple(self.annual, periods);
	}
	/// Set staking expectations
	pub fn set_expectations(&mut self, expect: Range<Balance>) {
		self.expect = expect;
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	fn mock_annual_to_round(annual: Range<Perbill>, rounds_per_year: u32) -> Range<Perbill> {
		perbill_annual_to_perbill_round(annual, rounds_per_year)
	}
	fn mock_round_issuance_range(
		// Total circulating before minting
		circulating: u128,
		// Round inflation range
		round: Range<Perbill>,
	) -> Range<u128> {
		Range {
			min: round.min * circulating,
			ideal: round.ideal * circulating,
			max: round.max * circulating,
		}
	}
	#[test]
	fn simple_issuance_conversion() {
		// 5% inflation for 10_000_0000 = 500,000 minted over the year
		// let's assume there are 10 periods in a year
		// => mint 500_000 over 10 periods => 50_000 minted per period
		let expected_round_issuance_range: Range<u128> =
			Range { min: 48_909, ideal: 48_909, max: 48_909 };
		let schedule = Range {
			min: Perbill::from_percent(5),
			ideal: Perbill::from_percent(5),
			max: Perbill::from_percent(5),
		};
		assert_eq!(
			expected_round_issuance_range,
			mock_round_issuance_range(10_000_000, mock_annual_to_round(schedule, 10))
		);
	}
	#[test]
	fn range_issuance_conversion() {
		// 3-5% inflation for 10_000_0000 = 300_000-500,000 minted over the year
		// let's assume there are 10 periods in a year
		// => mint 300_000-500_000 over 10 periods => 30_000-50_000 minted per period
		let expected_round_issuance_range: Range<u128> =
			Range { min: 29_603, ideal: 39298, max: 48_909 };
		let schedule = Range {
			min: Perbill::from_percent(3),
			ideal: Perbill::from_percent(4),
			max: Perbill::from_percent(5),
		};
		assert_eq!(
			expected_round_issuance_range,
			mock_round_issuance_range(10_000_000, mock_annual_to_round(schedule, 10))
		);
	}
	#[test]
	fn expected_parameterization() {
		let expected_round_schedule: Range<u128> = Range { min: 45, ideal: 56, max: 56 };
		let schedule = Range {
			min: Perbill::from_percent(4),
			ideal: Perbill::from_percent(5),
			max: Perbill::from_percent(5),
		};
		assert_eq!(
			expected_round_schedule,
			mock_round_issuance_range(10_000_000, mock_annual_to_round(schedule, 8766))
		);
	}
	#[test]
	fn inflation_does_not_panic_at_round_number_limit() {
		let schedule = Range {
			min: Perbill::from_percent(100),
			ideal: Perbill::from_percent(100),
			max: Perbill::from_percent(100),
		};
		mock_round_issuance_range(u32::MAX.into(), mock_annual_to_round(schedule, u32::MAX));
		mock_round_issuance_range(u64::MAX.into(), mock_annual_to_round(schedule, u32::MAX));
		mock_round_issuance_range(u128::MAX.into(), mock_annual_to_round(schedule, u32::MAX));
		mock_round_issuance_range(u32::MAX.into(), mock_annual_to_round(schedule, 1));
		mock_round_issuance_range(u64::MAX.into(), mock_annual_to_round(schedule, 1));
		mock_round_issuance_range(u128::MAX.into(), mock_annual_to_round(schedule, 1));
	}

	#[test]
	fn test_use_simple_interest_per_round_calculation() {
		let annual = Range {
			min: Perbill::from_parts(75_000_000),
			ideal: Perbill::from_parts(75_000_000),
			max: Perbill::from_parts(75_000_000),
		};
		assert_eq!(perbill_annual_to_perbill_round_simple(annual, 1).ideal.deconstruct(), 74999999);
		assert_eq!(perbill_annual_to_perbill_round_simple(annual, 2).ideal.deconstruct(), 37499999);
		assert_eq!(perbill_annual_to_perbill_round_simple(annual, 4).ideal.deconstruct(), 18749999);
		assert_eq!(perbill_annual_to_perbill_round_simple(annual, 8).ideal.deconstruct(), 9374999);
		assert_eq!(perbill_annual_to_perbill_round_simple(annual, 16).ideal.deconstruct(), 4687499);
	}
}
