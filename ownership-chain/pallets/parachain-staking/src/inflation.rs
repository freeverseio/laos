// KILT Blockchain – https://botlabs.org
// Copyright (C) 2019-2024 BOTLabs GmbH

// The KILT Blockchain is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The KILT Blockchain is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// If you feel like getting in touch with us, you can do so at info@botlabs.org

//! Helper methods for computing issuance based on inflation
use crate::{pallet::Config, types::BalanceOf};
use parity_scale_codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};
use sp_runtime::{Perquintill, RuntimeDebug};
use sp_runtime::traits::{Saturating, Zero, SaturatedConversion};
#[derive(
	Eq, PartialEq, Clone, Encode, Decode, Default, RuntimeDebug, TypeInfo, Deserialize, Serialize,
)]
pub struct RewardRate {
    pub annual: u64, // Recompensa total anual fija en tokens
    pub per_block: u64, // Cantidad fija de tokens a distribuir por bloque
}

impl MaxEncodedLen for RewardRate {
	fn max_encoded_len() -> usize {
		// Perquintill is at most u128
		u128::max_encoded_len().saturating_add(u128::max_encoded_len())
	}
}

/// Convert annual reward rate to per_block.
fn annual_to_per_block(blocks_per_year: u64, rate: Perquintill) -> Perquintill {
	rate / blocks_per_year.max(1)
}



impl RewardRate {
    pub fn new(annual_reward: u64, blocks_per_year: u64) -> Self {
        let per_block_reward = annual_reward / blocks_per_year.max(1);
        RewardRate {
            annual: annual_reward,
            per_block: per_block_reward,
        }
    }
}
/// Staking info (staking rate and reward rate) for collators and delegators.

#[derive(
	Eq, PartialEq, Clone, Encode, Decode, Default, RuntimeDebug, TypeInfo, Serialize, Deserialize,
)]
pub struct StakingInfo {
    pub max_rate: Perquintill,
    pub reward_rate: RewardRate,
}

impl MaxEncodedLen for StakingInfo {
	fn max_encoded_len() -> usize {
		// Perquintill is at most u128
		RewardRate::max_encoded_len().saturating_add(u128::max_encoded_len())
	}
}

impl StakingInfo {
	pub fn new(
        max_rate: Perquintill,
        annual_reward: u64,
        blocks_per_year: u64,
    ) -> Self {
        StakingInfo {
            max_rate,
            reward_rate: RewardRate::new(annual_reward, blocks_per_year),
        }
    }

	/// Calculate newly minted rewards on coinbase, e.g.,
	/// reward = rewards_per_block * staking_rate.
	///
	/// NOTE: If we exceed the max staking rate, the reward will be reduced by
	/// max_rate / current_rate.
	/// change here
	pub fn compute_reward<T: Config>(
		&self,
		stake: BalanceOf<T>,
		current_staking_rate: Perquintill,
		authors_per_round: BalanceOf<T>,
	) -> BalanceOf<T> {
		let per_block_reward: BalanceOf<T> = self.reward_rate.per_block.saturated_into();
		let reward = per_block_reward.saturating_mul(stake).saturating_mul(authors_per_round);
		
		// Ajusta la recompensa si la tasa de apuesta actual supera la tasa máxima permitida
		if current_staking_rate > self.max_rate {
			let reduction_rate = self.max_rate / current_staking_rate;
			let reduced_reward = reward.saturating_mul(reduction_rate.deconstruct().saturated_into());
			reduced_reward
		} else {
			reward
		}
	}
}

#[derive(
	Eq,
	PartialEq,
	Clone,
	Encode,
	Decode,
	Default,
	RuntimeDebug,
	TypeInfo,
	MaxEncodedLen,
	Serialize,
	Deserialize,
)]
pub struct InflationInfo {
	pub collator: StakingInfo,
	pub delegator: StakingInfo,
}

impl InflationInfo {
	/// Create a new inflation info from the max staking rates and annual reward
	/// rates for collators and delegators.
	///
	/// Example: InflationInfo::new(Perquintill_from_percent(10), ...)
	/// //Change here
	pub fn new(
		blocks_per_year: u64,
		collator_max_rate_percentage: Perquintill,
		collator_annual_reward_absolute: u64,
		delegator_max_rate_percentage: Perquintill,
		delegator_annual_reward_absolute: u64,
	) -> Self {
		Self {
			collator: StakingInfo::new(
				collator_max_rate_percentage,
				collator_annual_reward_absolute,
				blocks_per_year,

			),
			delegator: StakingInfo::new(
				delegator_max_rate_percentage,
				delegator_annual_reward_absolute,
				blocks_per_year,
			),
		}
	}

	/// Check whether the annual reward rate is exactly the per_block reward
	/// rate multiplied with the number of blocks per year
    pub fn is_valid(&self, blocks_per_year: u64) -> bool {
        // Verifica si la recompensa anual esperada coincide con la recompensa por bloque multiplicada
        // por el número de bloques por año para collator y delegator.
        let collator_annual_expected = self.collator.reward_rate.per_block * blocks_per_year;
        let delegator_annual_expected = self.delegator.reward_rate.per_block * blocks_per_year;

        (self.collator.reward_rate.annual == collator_annual_expected) &&
        (self.delegator.reward_rate.annual == delegator_annual_expected)
    }
}

#[cfg(test)]
mod tests {
	use sp_runtime::Perbill;

	use super::*;
	use crate::mock::{almost_equal, ExtBuilder, Test, DECIMALS, MAX_COLLATOR_STAKE};

	#[test]
	fn perquintill() {
		assert_eq!(
			Perquintill::from_percent(100) * Perquintill::from_percent(50),
			Perquintill::from_percent(50)
		);
	}

	#[test]
	fn annual_to_block_rate() {
		let rate = Perquintill::one();
		assert!(almost_equal(
			rate * 10_000_000_000u128,
			Perquintill::from_parts(
				annual_to_per_block(<Test as Config>::BLOCKS_PER_YEAR, rate).deconstruct() *
					<Test as Config>::BLOCKS_PER_YEAR
			) * 10_000_000_000u128,
			Perbill::from_perthousand(1)
		));
	}

	#[test]
	fn single_block_reward_collator() {
		let inflation = InflationInfo::new(
			<Test as Config>::BLOCKS_PER_YEAR,
			Perquintill::from_percent(10),
			37500000u64,
			Perquintill::from_percent(40),
			37500000u64,
		);
		let reward = inflation.collator.compute_reward::<Test>(
			MAX_COLLATOR_STAKE,
			Perquintill::from_percent(9),
			2,
		);
		let expected = <Test as Config>::CurrencyBalance::from(15210282150733u64);
		assert!(
			almost_equal(reward, expected, Perbill::from_perthousand(1)),
			"left {:?}, right {:?}",
			reward,
			expected
		);
	}

	#[test]
	fn simple_block_reward_check() {
		let precision = Perbill::from_perthousand(1);
		ExtBuilder::default()
			.with_inflation(10, 15, 40, 10, 5)
			.with_balances(vec![(1, 10)])
			.with_collators(vec![(1, 10)])
			.build()
			.execute_with(|| {
				let inflation = InflationInfo::new(
					<Test as Config>::BLOCKS_PER_YEAR,
					Perquintill::from_percent(10),
					37500000u64,
					Perquintill::from_percent(40),
					37500000u64,
				);
				let years_u128: BalanceOf<Test> = <Test as Config>::BLOCKS_PER_YEAR as u128;
				let collator_per_block_reward_u128: u128 = inflation.collator.reward_rate.per_block as u128;
				let delegator_per_block_reward_u128: u128 = inflation.delegator.reward_rate.per_block as u128;
				let decimals_u128: u128 = DECIMALS as u128;
				// Dummy checks for correct instantiation
				assert!(inflation.is_valid(<Test as Config>::BLOCKS_PER_YEAR));
				assert_eq!(inflation.collator.max_rate, Perquintill::from_percent(10));
				assert!(
					almost_equal(
						collator_per_block_reward_u128 * decimals_u128 * 10_000,
						Perquintill::from_percent(15).deconstruct() as u128 * decimals_u128 * 10_000 / years_u128,
						precision
					),
					"left = {:?}, right = {:?}",
					collator_per_block_reward_u128 * decimals_u128 * 10_000,
					Perquintill::from_percent(15) * 10_000 * DECIMALS / years_u128,
				);
				assert_eq!(inflation.delegator.max_rate, Perquintill::from_percent(40));
				assert!(
					almost_equal(
						delegator_per_block_reward_u128 * decimals_u128 * 10_000,
						Perquintill::from_percent(10) * 10_000 * DECIMALS / years_u128,
						precision
					),
					"left = {:?}, right = {:?}",
					delegator_per_block_reward_u128 * decimals_u128 * 10_000,
					Perquintill::from_percent(10) * 10_000 * DECIMALS / years_u128,
				);

				// Check collator reward computation
				let authors_per_round = 1u128;
				let mut current_staking_rate: Perquintill = inflation.collator.max_rate;
				assert_eq!(
					inflation.collator.compute_reward::<Test>(
						0,
						current_staking_rate,
						authors_per_round
					),
					0
				);
				current_staking_rate = Perquintill::from_rational(5000u64, 100_000u64);
				assert!(
					almost_equal(
						inflation.collator.compute_reward::<Test>(
							5000 * DECIMALS,
							current_staking_rate,
							authors_per_round
						) * years_u128,
						Perquintill::from_percent(15) * 5000 * DECIMALS,
						Perbill::from_percent(1)
					),
					"left = {:?}, right = {:?}",
					inflation.collator.compute_reward::<Test>(
						5000 * DECIMALS,
						current_staking_rate,
						authors_per_round
					) * years_u128,
					Perquintill::from_percent(15) * 5000 * DECIMALS,
				);
				// Check for max_rate which is 10%
				current_staking_rate = Perquintill::from_rational(10_000u64, 100_000u64);
				assert!(
					almost_equal(
						inflation.collator.compute_reward::<Test>(
							10_000 * DECIMALS,
							current_staking_rate,
							authors_per_round
						) * years_u128,
						Perquintill::from_percent(15) * 10_000 * DECIMALS,
						Perbill::from_percent(1)
					),
					"left = {:?}, right = {:?}",
					inflation.collator.compute_reward::<Test>(
						10_000 * DECIMALS,
						current_staking_rate,
						authors_per_round
					) * years_u128,
					Perquintill::from_percent(15) * 10_000 * DECIMALS,
				);

				// Check for exceeding max_rate: 50% instead of 10%
				current_staking_rate = Perquintill::from_rational(50_000u64, 100_000u64);
				assert!(
					almost_equal(
						inflation.collator.compute_reward::<Test>(
							50_000 * DECIMALS,
							current_staking_rate,
							authors_per_round
						) * years_u128,
						Perquintill::from_percent(15) * 10_000 * DECIMALS,
						Perbill::from_percent(1)
					),
					"left = {:?}, right = {:?}",
					inflation.collator.compute_reward::<Test>(
						50_000 * DECIMALS,
						current_staking_rate,
						authors_per_round
					) * years_u128,
					Perquintill::from_percent(15) * 10_000 * DECIMALS,
				);

				// Check delegator reward computation
				current_staking_rate = inflation.delegator.max_rate;
				assert_eq!(
					inflation.delegator.compute_reward::<Test>(
						0,
						current_staking_rate,
						authors_per_round
					),
					0
				);
				current_staking_rate = Perquintill::from_rational(5000u64, 100_000u64);
				assert!(
					almost_equal(
						inflation.delegator.compute_reward::<Test>(
							5000 * DECIMALS,
							current_staking_rate,
							authors_per_round
						) * years_u128,
						Perquintill::from_percent(10) * 5000 * DECIMALS,
						Perbill::from_percent(1)
					),
					"left = {:?}, right = {:?}",
					inflation.delegator.compute_reward::<Test>(
						5000 * DECIMALS,
						current_staking_rate,
						authors_per_round
					) * years_u128,
					Perquintill::from_percent(10) * 5000 * DECIMALS,
				);
				// Check for max_rate which is 40%
				current_staking_rate = Perquintill::from_rational(40_000u64, 100_000u64);
				assert!(
					almost_equal(
						inflation.delegator.compute_reward::<Test>(
							40_000 * DECIMALS,
							current_staking_rate,
							authors_per_round
						) * years_u128,
						Perquintill::from_percent(10) * 40_000 * DECIMALS,
						Perbill::from_percent(1)
					),
					"left = {:?}, right = {:?}",
					inflation.delegator.compute_reward::<Test>(
						40_000 * DECIMALS,
						current_staking_rate,
						authors_per_round
					) * years_u128,
					Perquintill::from_percent(10) * 40_000 * DECIMALS,
				);

				// Check for exceeding max_rate: 50% instead of 40%
				current_staking_rate = Perquintill::from_rational(50_000u64, 100_000u64);
				assert!(
					almost_equal(
						inflation.delegator.compute_reward::<Test>(
							50_000 * DECIMALS,
							current_staking_rate,
							authors_per_round
						) * years_u128,
						Perquintill::from_percent(8) * 50_000 * DECIMALS,
						Perbill::from_percent(1)
					),
					"left = {:?}, right = {:?}",
					inflation.delegator.compute_reward::<Test>(
						50_000 * DECIMALS,
						current_staking_rate,
						authors_per_round
					) * years_u128,
					Perquintill::from_percent(8) * 50_000 * DECIMALS,
				);
			});
	}
}
