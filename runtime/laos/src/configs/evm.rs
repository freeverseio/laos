// Copyright 2023-2024 Freeverse.io
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

//! Pallets that enable EVM execution on Substrate
use crate::{
	precompiles::LaosPrecompiles, types::ToAuthor, weights, AccountId, Aura, Balances, BaseFee,
	EVMChainId, Runtime, RuntimeEvent, Timestamp,
};
use frame_support::{
	parameter_types,
	weights::{constants::WEIGHT_REF_TIME_PER_SECOND, Weight},
};
use laos_primitives::{MAXIMUM_BLOCK_WEIGHT, NORMAL_DISPATCH_RATIO};
use sp_core::{ConstU32, U256};

/// Current approximation of the gas/s consumption considering
/// EVM execution over compiled WASM (on 4.4Ghz CPU).
/// Given the 500ms Weight, from which 75% only are used for transactions,
/// the total EVM execution gas limit is: GAS_PER_SECOND * 0.500 * 0.75 ~= 15_000_000.
/// Note: this value has been used in production by (and is copied from) the Moonbeam parachain.
const GAS_PER_SECOND: u64 = 40_000_000;

/// Approximate ratio of the amount of Weight per Gas.
/// u64 works for approximations because Weight is a very small unit compared to gas.
const WEIGHT_PER_GAS: u64 = WEIGHT_REF_TIME_PER_SECOND / GAS_PER_SECOND;

parameter_types! {
	pub BlockGasLimit: U256 = U256::from(NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT.ref_time() / WEIGHT_PER_GAS);
	pub PrecompilesInstance: LaosPrecompiles<Runtime> = LaosPrecompiles::<_>::new();
	pub WeightPerGas: Weight = Weight::from_parts(WEIGHT_PER_GAS, 0);
	/// The amount of gas per pov. A ratio of 4 if we convert ref_time to gas and we compare
	/// it with the pov_size for a block. E.g.
	/// ceil(
	///     (max_extrinsic.ref_time() / max_extrinsic.proof_size()) / WEIGHT_PER_GAS
	/// )
	pub const GasLimitPovSizeRatio: u64 = 4;
}

impl pallet_evm::Config for Runtime {
	type AddressMapping = pallet_evm::IdentityAddressMapping;
	type BlockGasLimit = BlockGasLimit;
	type BlockHashMapping = pallet_ethereum::EthereumBlockHashMapping<Self>;
	type CallOrigin = pallet_evm::EnsureAddressRoot<AccountId>;
	type ChainId = EVMChainId;
	type Currency = Balances;
	type FeeCalculator = BaseFee;
	type FindAuthor = CustomFindAuthor<pallet_session::FindAccountFromAuthorIndex<Self, Aura>>;
	type GasLimitPovSizeRatio = GasLimitPovSizeRatio;
	type GasWeightMapping = pallet_evm::FixedGasWeightMapping<Self>;
	type OnChargeTransaction = pallet_evm::EVMFungibleAdapter<Balances, ToAuthor<Self>>;
	type OnCreate = ();
	type PrecompilesType = LaosPrecompiles<Self>;
	type PrecompilesValue = PrecompilesInstance;
	type Runner = pallet_evm::runner::stack::Runner<Self>;
	type RuntimeEvent = RuntimeEvent;
	type SuicideQuickClearLimit = ConstU32<0>;
	type Timestamp = Timestamp;
	type WeightPerGas = WeightPerGas;
	type WithdrawOrigin = pallet_evm::EnsureAddressNever<AccountId>;
	type WeightInfo = weights::pallet_evm::WeightInfo<Runtime>;
}

pub struct CustomFindAuthor<Inner>(sp_std::marker::PhantomData<Inner>);

impl<Inner> frame_support::traits::FindAuthor<sp_core::H160> for CustomFindAuthor<Inner>
where
	Inner: frame_support::traits::FindAuthor<AccountId>,
{
	fn find_author<'a, I>(digests: I) -> Option<sp_core::H160>
	where
		I: 'a + IntoIterator<Item = (frame_support::ConsensusEngineId, &'a [u8])>,
	{
		Inner::find_author(digests).map(Into::into)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use core::str::FromStr;
	use pallet_evm::{Call, FeeCalculator, GasWeightMapping};
	use pallet_laos_evolution::{
		precompiles::{
			evolution_collection::EvolutionCollectionPrecompileSetCall,
			evolution_collection_factory::EvolutionCollectionFactoryPrecompileCall,
		},
		WeightInfo,
	};
	use sp_core::H160;
	use sp_runtime::traits::Dispatchable;

	use crate::{
		currency::UNIT,
		tests::{ExtBuilder, ALICE},
		RuntimeCall, RuntimeOrigin,
	};
	use frame_support::assert_ok;
	use precompile_utils::prelude::Address;

	fn create_evm_call<R>(source: H160, target: H160, input: Vec<u8>) -> Call<R>
	where
		R: pallet_evm::Config,
	{
		Call::call {
			source,
			target,
			input,
			value: U256::zero(), // No value sent in EVM
			gas_limit: 100_000,
			max_fee_per_gas: R::FeeCalculator::min_gas_price().0,
			max_priority_fee_per_gas: Some(U256::zero()),
			nonce: None, // Use the next nonce
			access_list: Vec::new(),
		}
	}

	const WRONG_ADDRESS: u64 = 1028;

	#[test]
	fn check_block_gas_limit() {
		assert_eq!(BlockGasLimit::get(), 15000000.into());
	}

	#[test]
	fn check_min_gas_price() {
		ExtBuilder::default().build().execute_with(|| {
			let min_gas_price = <Runtime as pallet_evm::Config>::FeeCalculator::min_gas_price();
			assert_eq!(min_gas_price.0, U256::from(1_000_000_000));
			assert_eq!(min_gas_price.1, Weight::from_parts(25_000_000, 0));
		});
	}

	#[test]
	fn create_collection_precompile_call_has_a_cost() {
		let alice = AccountId::from_str(ALICE).unwrap();

		ExtBuilder::default()
			.with_balances(vec![(alice, 1000 * UNIT)])
			.build()
			.execute_with(|| {
				let precompile_call: Vec<u8> =
					EvolutionCollectionFactoryPrecompileCall::<Runtime>::create_collection {
						owner: Address(alice.into()),
					}
					.into();

				// wrong address is used to get the base cost
				let call = create_evm_call(
					H160::from(alice.0),
					H160::from_low_u64_be(WRONG_ADDRESS),
					precompile_call.clone(),
				);
				let wrong_address_call_result =
					RuntimeCall::EVM(call).dispatch(RuntimeOrigin::root()).unwrap();

				// the actual call
				let call = create_evm_call(
					H160::from(alice.0),
					H160::from_low_u64_be(1027),
					precompile_call.clone(),
				);
				let call_result = RuntimeCall::EVM(call).dispatch(RuntimeOrigin::root()).unwrap();

				// check weights
				assert_eq!(
					wrong_address_call_result.actual_weight.unwrap(),
					Weight::from_parts(428726000, 5358)
				);
				assert_eq!(
					call_result.actual_weight.unwrap(),
					Weight::from_parts(1047726000, 11548)
				);

				// check gas
				assert_eq!(
					<Runtime as pallet_evm::Config>::GasWeightMapping::weight_to_gas(
						wrong_address_call_result.actual_weight.unwrap()
					),
					17149
				);
				assert_eq!(
					<Runtime as pallet_evm::Config>::GasWeightMapping::weight_to_gas(
						call_result.actual_weight.unwrap()
					),
					41909
				);

				// check weights from benchmarking
				let weights_from_benchmarking =
					weights::pallet_laos_evolution::WeightInfo::<Runtime>::precompile_create_collection();
				assert_eq!(weights_from_benchmarking, Weight::from_parts(619016000, 3873));
				assert_eq!(
					<Runtime as pallet_evm::Config>::GasWeightMapping::weight_to_gas(
						weights_from_benchmarking
					),
					24760
				);
			});
	}

	#[test]
	fn owner_precompile_call_has_a_cost() {
		let alice = AccountId::from_str(ALICE).unwrap();

		ExtBuilder::default()
			.with_balances(vec![(alice, 1000 * UNIT)])
			.build()
			.execute_with(|| {
				// create collection
				let precompile_call: Vec<u8> =
					EvolutionCollectionFactoryPrecompileCall::<Runtime>::create_collection {
						owner: Address(alice.into()),
					}
					.into();
				let call = create_evm_call(
					H160::from(alice.0),
					H160::from_low_u64_be(WRONG_ADDRESS),
					precompile_call.clone(),
				);
				assert_ok!(RuntimeCall::EVM(call).dispatch(RuntimeOrigin::root()));

				let precompile_call: Vec<u8> =
					EvolutionCollectionPrecompileSetCall::<Runtime>::owner {}.into();

				// wrong address is used to get the base cost
				let call = create_evm_call(
					H160::from(alice.0),
					H160::from_low_u64_be(1027),
					precompile_call.clone(),
				);
				let wrong_address_call_result =
					RuntimeCall::EVM(call).dispatch(RuntimeOrigin::root()).unwrap();

				// the actual call
				let call = create_evm_call(
					H160::from(alice.0),
					H160::from_str("fffffffffffffffffffffffe0000000000000000").unwrap(),
					precompile_call.clone(),
				);
				let call_result = RuntimeCall::EVM(call).dispatch(RuntimeOrigin::root()).unwrap();

				// check weights
				assert_eq!(
					wrong_address_call_result.actual_weight.unwrap(),
					Weight::from_parts(419526000, 5266)
				);
				assert_eq!(call_result.actual_weight.unwrap(), Weight::from_parts(449426000, 5565));

				// check gas
				assert_eq!(
					<Runtime as pallet_evm::Config>::GasWeightMapping::weight_to_gas(
						wrong_address_call_result.actual_weight.unwrap()
					),
					16781
				);
				assert_eq!(
					<Runtime as pallet_evm::Config>::GasWeightMapping::weight_to_gas(
						call_result.actual_weight.unwrap()
					),
					17977
				);

				// check weights from benchmarking
				let weights_from_benchmarking =
					weights::pallet_laos_evolution::WeightInfo::<Runtime>::precompile_owner();
				assert_eq!(weights_from_benchmarking, Weight::from_parts(29765000, 3509));
				assert_eq!(
					<Runtime as pallet_evm::Config>::GasWeightMapping::weight_to_gas(
						weights_from_benchmarking
					),
					1190
				);
			});
	}
}
