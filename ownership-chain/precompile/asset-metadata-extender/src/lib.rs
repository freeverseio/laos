#![cfg_attr(not(feature = "std"), no_std)]
use fp_evm::{Precompile, PrecompileHandle, PrecompileOutput};
use laos_precompile_utils::{
	keccak256, revert_dispatch_error, succeed, Address, Bytes, EvmDataReader, EvmDataWriter,
	EvmResult, FunctionModifier, GasCalculator, LogExt, LogsBuilder, PrecompileHandleExt,
};
use pallet_asset_metadata_extender::{
	traits::AssetMetadataExtender as AssetMetadataExtenderT,
	weights::{SubstrateWeight as AssetMetadataExtenderWeights, WeightInfo},
	Pallet as AssetMetadataExtender,
};
use parity_scale_codec::Encode;
use precompile_utils::solidity::revert::revert;

use sp_core::{Get, H160};
use sp_io::hashing::keccak_256;
use sp_runtime::BoundedVec;
use sp_std::{fmt::Debug, marker::PhantomData};

/// Solidity selector of the TokenURIExtended log, which is the Keccak of the Log signature.
pub const SELECTOR_LOG_TOKEN_URI_EXTENDED: [u8; 32] =
	keccak256!("TokenURIExtended(address,bytes32,string,string)");
/// Solidity selector of the ExtendedTokenURIUpdated log, which is the Keccak of the Log signature.
pub const SELECTOR_LOG_EXTENDED_TOKEN_URI_UPDATED: [u8; 32] =
	keccak256!("ExtendedTokenURIUpdated(address,bytes32,string,string)");

#[laos_precompile_utils_macro::generate_function_selector]
#[derive(Debug, PartialEq)]
pub enum Action {
	/// Extend asset metadata (token uri)
	Extend = "extendTokenURI(string,string)",
	/// Get extensions balance for a given universal location
	Balance = "balanceOfUL(string)",
	/// Get claimer of a given universal location using indexation
	Claimer = "claimerOfULByIndex(string,uint32)",
	/// Get token uri of a given universal location using indexation
	Extension = "extensionOfULByIndex(string,uint32)", // TODO rename `extension` for `tokenURI`?
	/// Update token uri of a given universal location using indexation
	Update = "updateTokenURI(string,string)",
	/// Get claimer of a given universal location using claimer
	ExtensionOfULByClaimer = "extensionOfULByClaimer(string,address)",
	/// Check if a given universal location has an extension
	HasExtension = "hasExtensionByClaimer(string,address)",
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
			Action::ExtensionOfULByClaimer => FunctionModifier::View,
			Action::HasExtension => FunctionModifier::View,
		})?;

		match selector {
			Action::Extend => Self::extend(handle),
			Action::Balance => Self::balance_of(handle),
			Action::Claimer => Self::claimer_by_index(handle),
			Action::Extension => Self::extension_by_index(handle),
			Action::Update => Self::update(handle),
			Action::ExtensionOfULByClaimer => Self::extension_by_location_and_claimer(handle),
			Action::HasExtension => Self::has_extension_by_claimer(handle),
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
		let universal_location = Self::read_bounded_vec(&mut input)
			.map_err(|_| revert("invalid universal location length"))?;

		// get token uri from input
		let token_uri =
			Self::read_bounded_vec(&mut input).map_err(|_| revert("invalid token uri length"))?;

		AssetMetadataExtender::<Runtime>::create_token_uri_extension(
			claimer.into(),
			universal_location.clone(),
			token_uri.clone(),
		)
		.map_err(revert_dispatch_error)?;

		let universal_location_raw = universal_location.into_inner();
		let token_uri_raw = token_uri.into_inner();

		let consumed_weight = AssetMetadataExtenderWeights::<Runtime>::create_token_uri_extension(
			universal_location_raw.len() as u32,
			token_uri_raw.len() as u32,
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

		let mut input = handle.read_input()?;
		input.expect_arguments(2)?;

		let universal_location = Self::read_bounded_vec(&mut input)
			.map_err(|_| revert("invalid universal location length"))?;

		let token_uri =
			Self::read_bounded_vec(&mut input).map_err(|_| revert("invalid token uri length"))?;

		let claimer = context.caller;
		let universal_location_hash = keccak_256(&universal_location);

		AssetMetadataExtender::<Runtime>::update_token_uri_extension(
			claimer.into(),
			universal_location.clone(),
			token_uri.clone(),
		)
		.map_err(revert_dispatch_error)?;

		let universal_location_raw = universal_location.into_inner();
		let token_uri_raw = token_uri.into_inner();
		let consumed_weight = AssetMetadataExtenderWeights::<Runtime>::update_token_uri_extension(
			universal_location_raw.len() as u32,
			token_uri_raw.len() as u32,
		);

		LogsBuilder::new(context.address)
			.log3(
				SELECTOR_LOG_EXTENDED_TOKEN_URI_UPDATED,
				claimer,
				universal_location_hash,
				EvmDataWriter::new()
					.write(Bytes(universal_location_raw))
					.write(Bytes(token_uri_raw))
					.build(),
			)
			.record(handle)?;

		// Record EVM cost
		handle.record_cost(GasCalculator::<Runtime>::weight_to_gas(consumed_weight))?;

		Ok(succeed(sp_std::vec![]))
	}

	fn balance_of(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		let mut input = handle.read_input()?;
		input.expect_arguments(1)?;

		let universal_location = Self::read_bounded_vec(&mut input)
			.map_err(|_| revert("invalid universal location length"))?;

		let balance = AssetMetadataExtender::<Runtime>::balance_of(universal_location);

		Ok(succeed(balance.to_be_bytes()))
	}

	fn claimer_by_index(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		let mut input = handle.read_input()?;
		input.expect_arguments(2)?;

		let universal_location = Self::read_bounded_vec(&mut input)
			.map_err(|_| revert("invalid universal location length"))?;

		let index = input.read::<u32>()?;

		if AssetMetadataExtender::balance_of(universal_location.clone()) <= index {
			return Err(revert("invalid index"));
		}

		let claimer =
			AssetMetadataExtender::<Runtime>::claimer_by_index(universal_location.clone(), index)
				.ok_or_else(|| revert("invalid ul"))?;

		Ok(succeed(Address(claimer.into()).0 .0))
	}

	fn extension_by_index(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		let mut input = handle.read_input()?;
		input.expect_arguments(2)?;

		let universal_location = Self::read_bounded_vec(&mut input)
			.map_err(|_| revert("invalid universal location length"))?;

		let index = input.read::<u32>()?;

		if AssetMetadataExtender::balance_of(universal_location.clone()) <= index {
			return Err(revert("invalid index"));
		}

		let token_uri = AssetMetadataExtender::<Runtime>::token_uri_extension_by_index(
			universal_location.clone(),
			index,
		)
		.ok_or_else(|| revert("invalid ul"))?;

		Ok(succeed(token_uri.into_inner()))
	}

	/// Generic function to read a bounded vector from the input.
	fn read_bounded_vec<Bound: Get<u32>>(
		input: &mut EvmDataReader,
	) -> Result<BoundedVec<u8, Bound>, ()> {
		let raw_vec = input.read::<Bytes>().map_err(|_| ())?.0;
		raw_vec.try_into().map_err(|_| ())
	}

	fn extension_by_location_and_claimer(
		handle: &mut impl PrecompileHandle,
	) -> EvmResult<PrecompileOutput> {
		let mut input = handle.read_input()?;
		input.expect_arguments(2)?;

		let universal_location = Self::read_bounded_vec(&mut input)
			.map_err(|_| revert("invalid universal location length"))?;

		let claimer = input.read::<Address>().map_err(|_| revert("invalid claimer"))?.0;

		let token_uri = AssetMetadataExtender::<Runtime>::extension_by_location_and_claimer(
			claimer.into(),
			universal_location.clone(),
		)
		.ok_or_else(|| revert("invalid ul"))?;

		Ok(succeed(token_uri.into_inner()))
	}

	fn has_extension_by_claimer(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		let mut input = handle.read_input()?;
		input.expect_arguments(2)?;

		let universal_location = Self::read_bounded_vec(&mut input)
			.map_err(|_| revert("invalid universal location length"))?;

		let claimer = input.read::<Address>().map_err(|_| revert("invalid claimer"))?.0;

		let has_extension = AssetMetadataExtender::<Runtime>::has_extension(
			universal_location.clone(),
			claimer.into(),
		);

		Ok(succeed(bool_to_32_bytes(has_extension)))	
	}
}

fn bool_to_32_bytes(value: bool) -> [u8; 32] {
    let mut output = [0u8; 32]; // Create a 32-byte array initialized with zeros
    output[31] = value as u8;   // Set the last byte to the boolean value
    output
}

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;
