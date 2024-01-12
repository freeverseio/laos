#![cfg_attr(not(feature = "std"), no_std)]
use fp_evm::{Precompile, PrecompileHandle, PrecompileOutput};
use laos_precompile_utils::{
	keccak256, revert_dispatch_error, succeed, Bytes, EvmDataReader, EvmDataWriter, EvmResult,
	FunctionModifier, GasCalculator, LogExt, LogsBuilder, PrecompileHandleExt,
};
use pallet_asset_metadata_extender::{
	traits::AssetMetadataExtender as AssetMetadataExtenderT,
	weights::{SubstrateWeight as AssetMetadataExtenderWeights, WeightInfo},
	Pallet as AssetMetadataExtender, TokenUriOf, UniversalLocationOf,
};
use parity_scale_codec::Encode;
use precompile_utils::solidity::revert::revert;

use sp_core::{Get, H160};
use sp_io::hashing::keccak_256;
use sp_runtime::BoundedVec;
use sp_std::{fmt::Debug, marker::PhantomData};

/// Solidity selector of the TokenURIExtended log, which is the Keccak of the Log signature.
pub const SELECTOR_LOG_TOKEN_URI_EXTENDED: [u8; 32] =
	keccak256!("TokenURIExtended(address,string,uint256)");
/// Solidity selector of the ExtendedTokenURIUpdated log, which is the Keccak of the Log signature.
pub const SELECTOR_LOG_EXTENDED_TOKEN_URI_UPDATED: [u8; 32] =
	keccak256!("ExtendedTokenURIUpdated(address,uint256,string,string)");

#[laos_precompile_utils_macro::generate_function_selector]
#[derive(Debug, PartialEq)]
pub enum Action {
	/// Extend asset metadata (token uri)
	Extend = "extendTokenURI(string,string)",
	/// Get extensions balance for a given universal location
	Balance = "balanceOfUL(string)",
	/// Get claimer of a given universal location using indexation
	Claimer = "claimerOfULByIndex(string,uint256)",
	/// Get token uri of a given universal location using indexation
	Extension = "extensionOfULByIndex(string,uint256)", // TODO rename `extension` for `tokenURI`?
	/// Update token uri of a given universal location using indexation
	Update = "updateTokenURI(string,string)",
}

pub struct AssetMetadataExtenderPrecompile<Runtime>(PhantomData<Runtime>)
where
	Runtime: pallet_asset_metadata_extender::Config;

impl<Runtime> Precompile for AssetMetadataExtenderPrecompile<Runtime>
where
	Runtime: pallet_evm::Config + pallet_asset_metadata_extender::Config,
	Runtime::AccountId: From<H160> + Into<H160> + Encode + Debug,
	AssetMetadataExtender<Runtime>: AssetMetadataExtenderT<Runtime>,
{
	fn execute(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		let selector = handle.read_selector()?;

		handle.check_function_modifier(match selector {
			Action::Extend => FunctionModifier::NonPayable,
			Action::Balance => FunctionModifier::View,
			Action::Claimer => FunctionModifier::View,
			Action::Extension => FunctionModifier::View,
			Action::Update => FunctionModifier::NonPayable,
		})?;

		match selector {
			Action::Extend => Self::extend(handle),
			Action::Balance => Self::balance_of(handle),
			Action::Claimer => Self::claimer_by_index(handle),
			Action::Extension => Self::extension_by_index(handle),
			Action::Update => Self::update(handle),
		}
	}
}

impl<Runtime> AssetMetadataExtenderPrecompile<Runtime>
where
	Runtime: pallet_evm::Config + pallet_asset_metadata_extender::Config,
	Runtime::AccountId: From<H160> + Into<H160> + Encode + Debug,
	AssetMetadataExtender<Runtime>: AssetMetadataExtenderT<Runtime>,
{
	fn extend(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		let context = handle.context();
		let claimer = context.caller;

		let mut input = handle.read_input()?;
		input.expect_arguments(2)?;

		// get universal location from input
		let universal_location_raw = input.read::<Bytes>()?.0;
		let universal_location = Self::read_bounded_vec(&mut input)
			.map_err(|_| revert("invalid universal location length"))?;

		// get token uri from input
		let token_uri_raw = input.read::<Bytes>()?.0;
		let token_uri =
			Self::read_bounded_vec(&mut input).map_err(|_| revert("invalid token uri length"))?;

		AssetMetadataExtender::<Runtime>::create_token_uri_extension(
			claimer.into(),
			universal_location,
			token_uri,
		)
		.map_err(revert_dispatch_error)?;

		let consumed_weight = AssetMetadataExtenderWeights::<Runtime>::create_token_uri_extension(
			token_uri_raw.len() as u32,
			universal_location_raw.len() as u32,
		);

		let ul_hash = keccak_256(&universal_location_raw);

		LogsBuilder::new(context.address)
			.log3(
				SELECTOR_LOG_TOKEN_URI_EXTENDED,
				claimer,
				ul_hash,
				EvmDataWriter::new()
					.write(Bytes(universal_location_raw))
					.write(Bytes(token_uri_raw))
					.build(),
			)
			.record(handle)?;

		// Record EVM cost
		handle.record_cost(GasCalculator::<Runtime>::weight_to_gas(consumed_weight))?;

		// Record Substrate related costs
		// TODO: Add `ref_time` when precompiles are benchmarked
		handle.record_external_cost(None, Some(consumed_weight.proof_size()))?;

		Ok(succeed(sp_std::vec![]))
	}

	fn update(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		let context = handle.context();

		let input = handle.read_input()?;
		input.expect_arguments(2)?;

		let universal_location = Self::get_ul_from_input(input.clone())?;
		let token_uri = Self::get_token_uri_from_input(input)?;

		let claimer = context.caller;
		let universal_location_hash = keccak_256(&universal_location);

		AssetMetadataExtender::<Runtime>::update_token_uri_extension(
			claimer.into(),
			universal_location.clone(),
			token_uri.clone(),
		)
		.map_err(revert_dispatch_error)?;

		LogsBuilder::new(context.address)
			.log3(
				SELECTOR_LOG_EXTENDED_TOKEN_URI_UPDATED,
				claimer,
				universal_location_hash,
				EvmDataWriter::new()
					.write(Bytes(universal_location.into()))
					.write(Bytes(token_uri.into()))
					.build(),
			)
			.record(handle)?;

		Ok(succeed(sp_std::vec![]))
	}

	fn balance_of(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		let mut input = handle.read_input()?;
		input.expect_arguments(1)?;

		let universal_location = Self::read_bounded_vec(&mut input)
			.map_err(|_| revert("invalid universal location length"))?;

		let balance = AssetMetadataExtender::<Runtime>::balance_of(universal_location);

		Ok(succeed(balance.encode()))
	}

	fn claimer_by_index(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		let mut input = handle.read_input()?;
		input.expect_arguments(2)?;

		let universal_location = Self::read_bounded_vec(&mut input)
			.map_err(|_| revert("invalid universal location length"))?;

		let index = input.read::<u32>()?;

		let claimer =
			AssetMetadataExtender::<Runtime>::claimer_by_index(universal_location.clone(), index);

		Ok(succeed(claimer.encode()))
	}

	fn extension_by_index(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		let mut input = handle.read_input()?;
		input.expect_arguments(2)?;

		let universal_location = Self::read_bounded_vec(&mut input)
			.map_err(|_| revert("invalid universal location length"))?;

		let index = input.read::<u32>()?;

		let token_uri = AssetMetadataExtender::<Runtime>::token_uri_extension_by_index(
			universal_location.clone(),
			index,
		);

		Ok(succeed(token_uri.encode()))
	}

	fn read_bounded_vec<Bound: Get<u32>>(
		input: &mut EvmDataReader,
	) -> Result<BoundedVec<u8, Bound>, ()> {
		let raw_vec = input.read::<Bytes>().map_err(|_| ())?.0;
		raw_vec.try_into().map_err(|_| ())
	}

	fn get_ul_from_input(mut input: EvmDataReader) -> EvmResult<UniversalLocationOf<Runtime>> {
		let universal_location = input.read::<Bytes>()?.0;
		let universal_location = universal_location
			.clone()
			.try_into()
			.map_err(|_| revert("invalid universal location length"))?;
		Ok(universal_location)
	}

	fn get_token_uri_from_input(mut input: EvmDataReader) -> EvmResult<TokenUriOf<Runtime>> {
		let token_uri = input.read::<Bytes>()?.0;
		let token_uri =
			token_uri.clone().try_into().map_err(|_| revert("invalid token uri length"))?;
		Ok(token_uri)
	}
}

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;
