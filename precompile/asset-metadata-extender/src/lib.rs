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

#![cfg_attr(not(feature = "std"), no_std)]
use fp_evm::{Precompile, PrecompileHandle, PrecompileOutput};
use pallet_asset_metadata_extender::{
	traits::AssetMetadataExtender as AssetMetadataExtenderT,
	weights::{SubstrateWeight as AssetMetadataExtenderWeights, WeightInfo},
	Config, Pallet as AssetMetadataExtender,
};
use parity_scale_codec::Encode;
use precompile_utils::{
	prelude::{keccak256, log3, Address, EvmResult, LogExt},
	solidity::{self, codec::UnboundedString, revert::revert},
};

use pallet_evm::GasWeightMapping;
use scale_info::prelude::{format, string::String};
use sp_core::{Get, H160};
use sp_io::hashing::keccak_256;
use sp_runtime::{BoundedVec, DispatchError};
use sp_std::{fmt::Debug, marker::PhantomData};

/// Solidity selector of the ExtendedULWithExternalURI log, which is the Keccak of the Log
/// signature.
pub const SELECTOR_LOG_EXTENDED_UL_WITH_EXTERNAL_URI: [u8; 32] =
	keccak256!("ExtendedULWithExternalURI(address,bytes32,string,string)");
/// Solidity selector of the UpdatedExtendedULWithExternalURI log, which is the Keccak of the Log
/// signature.
pub const SELECTOR_LOG_UPDATED_EXTENDED_UL_WITH_EXTERNAL_URI: [u8; 32] =
	keccak256!("UpdatedExtendedULWithExternalURI(address,bytes32,string,string)");

// #[laos_precompile_utils_macro::generate_function_selector]
// #[derive(Debug, PartialEq)]
// pub enum Action {
// 	/// Extend asset metadata (token uri)
// 	Extend = "extendULWithExternalURI(string,string)",
// 	/// Get extensions balance for a given universal location
// 	Balance = "balanceOfUL(string)",
// 	/// Get claimer of a given universal location using indexation
// 	Claimer = "claimerOfULByIndex(string,uint32)",
// 	/// Get token uri of a given universal location using indexation
// 	Extension = "extensionOfULByIndex(string,uint32)", // TODO rename `extension` for `tokenURI`?
// 	/// Update token uri of a given universal location using indexation
// 	Update = "updateExtendedULWithExternalURI(string,string)",
// 	/// Get extension of a given universal location using claimer
// 	ExtensionOfULByClaimer = "extensionOfULByClaimer(string,address)",
// 	/// Check if a given universal location has an extension
// 	HasExtension = "hasExtensionByClaimer(string,address)",
// }

pub struct AssetMetadataExtenderPrecompile<Runtime>(PhantomData<Runtime>);

impl<Runtime> AssetMetadataExtenderPrecompile<Runtime> {
	pub fn new() -> Self {
		Self(PhantomData)
	}
}

#[precompile_utils::precompile]
impl<Runtime> AssetMetadataExtenderPrecompile<Runtime>
where
	Runtime: pallet_evm::Config + pallet_asset_metadata_extender::Config,
	Runtime::AccountId: From<H160> + Into<H160> + Encode + Debug,
	AssetMetadataExtender<Runtime>: AssetMetadataExtenderT<Runtime>,
{
	#[precompile::public("extendULWithExternalURI(string,string)")]
	fn extend(
		handle: &mut impl PrecompileHandle,
		universal_location: UnboundedString,
		token_uri: UnboundedString,
	) -> EvmResult<()> {
		// TODO this might be remove when we have the bounded string as param
		let universal_location_bounded: BoundedVec<
			u8,
			<Runtime as Config>::MaxUniversalLocationLength,
		> = universal_location
			.as_bytes()
			.to_vec()
			.try_into()
			.map_err(|_| revert("invalid universal location length"))?;

		// TODO this might be remove when we have the bounded string as param
		let token_uri_bounded: BoundedVec<u8, <Runtime as Config>::MaxTokenUriLength> = token_uri
			.as_bytes()
			.to_vec()
			.try_into()
			.map_err(|_| revert("invalid token uri length"))?;

		AssetMetadataExtender::<Runtime>::create_token_uri_extension(
			handle.context().caller.into(),
			universal_location_bounded.clone(),
			token_uri_bounded.clone(),
		)
		.map_err(|err| revert(convert_dispatch_error_to_string(err)))?;

		let consumed_weight = AssetMetadataExtenderWeights::<Runtime>::create_token_uri_extension(
			universal_location.as_bytes().to_vec().len() as u32,
			token_uri.as_bytes().to_vec().len() as u32,
		);

		let ul_hash = keccak_256(&universal_location.as_bytes().to_vec()); // TODO see if this can be improved

		log3(
			handle.context().address,
			SELECTOR_LOG_EXTENDED_UL_WITH_EXTERNAL_URI,
			handle.context().caller,
			ul_hash,
			solidity::encode_event_data((universal_location, token_uri)),
		)
		.record(handle)?;

		// Record EVM cost
		handle.record_cost(<Runtime as pallet_evm::Config>::GasWeightMapping::weight_to_gas(
			consumed_weight,
		))?;

		// Record Substrate related costs
		// TODO: Add `ref_time` when precompiles are benchmarked
		handle.record_external_cost(None, Some(consumed_weight.proof_size()))?;

		Ok(())
	}

	// fn update(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
	// 	let context = handle.context();

	// 	let mut input = handle.read_input()?;
	// 	input.expect_arguments(2)?;

	// 	let universal_location = Self::read_bounded_vec(&mut input)
	// 		.map_err(|_| revert("invalid universal location length"))?;

	// 	let token_uri =
	// 		Self::read_bounded_vec(&mut input).map_err(|_| revert("invalid token uri length"))?;

	// 	let claimer = context.caller;
	// 	let universal_location_hash = keccak_256(&universal_location);

	// 	AssetMetadataExtender::<Runtime>::update_token_uri_extension(
	// 		claimer.into(),
	// 		universal_location.clone(),
	// 		token_uri.clone(),
	// 	)
	// 	.map_err(revert_dispatch_error)?;

	// 	let universal_location_raw = universal_location.into_inner();
	// 	let token_uri_raw = token_uri.into_inner();
	// 	let consumed_weight = AssetMetadataExtenderWeights::<Runtime>::update_token_uri_extension(
	// 		universal_location_raw.len() as u32,
	// 		token_uri_raw.len() as u32,
	// 	);

	// 	LogsBuilder::new(context.address)
	// 		.log3(
	// 			SELECTOR_LOG_UPDATED_EXTENDED_UL_WITH_EXTERNAL_URI,
	// 			claimer,
	// 			universal_location_hash,
	// 			EvmDataWriter::new()
	// 				.write(Bytes(universal_location_raw))
	// 				.write(Bytes(token_uri_raw))
	// 				.build(),
	// 		)
	// 		.record(handle)?;

	// 	// Record EVM cost
	// 	handle.record_cost(<Runtime as
	// pallet_evm::Config>::GasWeightMapping::weight_to_gas(consumed_weight))?;

	// 	Ok(succeed(EvmDataWriter::new().build()))
	// }

	// fn balance_of(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
	// 	let mut input = handle.read_input()?;
	// 	input.expect_arguments(1)?;

	// 129	let universal_location = Self::read_bounded_vec(&mut input)
	// 		.map_err(|_| revert("invalid universal location length"))?;

	// 	let balance = AssetMetadataExtender::<Runtime>::balance_of(universal_location);

	// 	Ok(succeed(EvmDataWriter::new().write(balance).build()))
	// }

	// fn claimer_by_index(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
	// 	let mut input = handle.read_input()?;
	// 	input.expect_arguments(2)?;

	// 	let universal_location = Self::read_bounded_vec(&mut input)
	// 		.map_err(|_| revert("invalid universal location length"))?;

	// 	let index = input.read::<u32>()?;

	// 	if AssetMetadataExtender::balance_of(universal_location.clone()) <= index {
	// 		return Err(revert("invalid index"));
	// 	}

	// 	let claimer =
	// 		AssetMetadataExtender::<Runtime>::claimer_by_index(universal_location.clone(), index)
	// 			.ok_or_else(|| revert("invalid ul"))?;

	// 	Ok(succeed(EvmDataWriter::new().write(Address(claimer.into())).build()))
	// }

	// fn extension_by_index(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
	// 	let mut input = handle.read_input()?;
	// 	input.expect_arguments(2)?;

	// 	let universal_location = Self::read_bounded_vec(&mut input)
	// 		.map_err(|_| revert("invalid universal location length"))?;

	// 	let index = input.read::<u32>()?;

	// 	if AssetMetadataExtender::balance_of(universal_location.clone()) <= index {
	// 		return Err(revert("invalid index"));
	// 	}

	// 	let token_uri = AssetMetadataExtender::<Runtime>::token_uri_extension_by_index(
	// 		universal_location.clone(),
	// 		index,
	// 	)
	// 	.ok_or_else(|| revert("invalid ul"))?;

	// 	Ok(succeed(EvmDataWriter::new().write(Bytes(token_uri.into_inner())).build()))
	// }

	// /// Generic function to read a bounded vector from the input.
	// fn read_bounded_vec<Bound: Get<u32>>(
	// 	input: &mut EvmDataReader,
	// ) -> Result<BoundedVec<u8, Bound>, ()> {
	// 	let raw_vec = input.read::<Bytes>().map_err(|_| ())?.0;
	// 	raw_vec.try_into().map_err(|_| ())
	// }

	// fn extension_by_location_and_claimer(
	// 	handle: &mut impl PrecompileHandle,
	// ) -> EvmResult<PrecompileOutput> {
	// 	let mut input = handle.read_input()?;
	// 	input.expect_arguments(2)?;

	// 	let universal_location = Self::read_bounded_vec(&mut input)
	// 		.map_err(|_| revert("invalid universal location length"))?;

	// 	let claimer = input.read::<Address>().map_err(|_| revert("invalid claimer"))?.0;

	// 	let token_uri = AssetMetadataExtender::<Runtime>::extension_by_location_and_claimer(
	// 		universal_location.clone(),
	// 		claimer.into(),
	// 	)
	// 	.ok_or_else(|| revert("invalid ul"))?;

	// 	Ok(succeed(EvmDataWriter::new().write(Bytes(token_uri.into_inner())).build()))
	// }

	// fn has_extension_by_claimer(handle: &mut impl PrecompileHandle) ->
	// EvmResult<PrecompileOutput> { 	let mut input = handle.read_input()?;
	// 	input.expect_arguments(2)?;

	// 	let universal_location = Self::read_bounded_vec(&mut input)
	// 		.map_err(|_| revert("invalid universal location length"))?;

	// 	let claimer = input.read::<Address>().map_err(|_| revert("invalid claimer"))?.0;

	// 	let has_extension = AssetMetadataExtender::<Runtime>::has_extension(
	// 		universal_location.clone(),
	// 		claimer.into(),
	// 	);

	// 	Ok(succeed(EvmDataWriter::new().write(has_extension).build()))
	// }
}

fn convert_dispatch_error_to_string(err: DispatchError) -> String {
	match err {
		DispatchError::Module(mod_err) => mod_err.message.unwrap_or("Unknown module error").into(),
		_ => format!("{:?}", err),
	}
}

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;
