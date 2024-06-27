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
use crate::{
	traits::AssetMetadataExtender as AssetMetadataExtenderT, weights::WeightInfo, Config,
	Pallet as AssetMetadataExtender,
};
use fp_evm::PrecompileHandle;
use frame_support::DefaultNoBound;
use precompile_utils::{
	prelude::{keccak256, log3, Address, EvmResult, LogExt},
	solidity::{self, codec::UnboundedString, revert::revert},
};
use scale_info::prelude::{format, string::String};
use sp_core::H160;
use sp_io::hashing::keccak_256;
use sp_runtime::{
	traits::{Convert, ConvertBack},
	BoundedVec, DispatchError,
};
use sp_std::marker::PhantomData;

/// Solidity selector of the ExtendedULWithExternalURI log, which is the Keccak of the Log
/// signature.
pub const SELECTOR_LOG_EXTENDED_UL_WITH_EXTERNAL_URI: [u8; 32] =
	keccak256!("ExtendedULWithExternalURI(address,bytes32,string,string)");
/// Solidity selector of the UpdatedExtendedULWithExternalURI log, which is the Keccak of the Log
/// signature.
pub const SELECTOR_LOG_UPDATED_EXTENDED_UL_WITH_EXTERNAL_URI: [u8; 32] =
	keccak256!("UpdatedExtendedULWithExternalURI(address,bytes32,string,string)");

#[derive(DefaultNoBound)]
pub struct AssetMetadataExtenderPrecompile<Runtime>(PhantomData<Runtime>);

impl<Runtime> AssetMetadataExtenderPrecompile<Runtime> {
	pub fn new() -> Self {
		Self(PhantomData)
	}
}

#[precompile_utils::precompile]
impl<Runtime> AssetMetadataExtenderPrecompile<Runtime>
where
	Runtime: crate::Config,
	AssetMetadataExtender<Runtime>: AssetMetadataExtenderT<Runtime>,
{
	#[precompile::public("extendULWithExternalURI(string,string)")]
	pub fn extend(
		handle: &mut impl PrecompileHandle,
		universal_location: UnboundedString,
		token_uri: UnboundedString,
	) -> EvmResult<()> {
		super::register_cost::<Runtime>(
			handle,
			Runtime::WeightInfo::precompile_extend(
				token_uri.as_bytes().len().try_into().unwrap(),
				universal_location.as_bytes().len().try_into().unwrap(),
			),
		)?;

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
			Runtime::AccountIdToH160::convert_back(handle.context().caller),
			universal_location_bounded.clone(),
			token_uri_bounded.clone(),
		)
		.map_err(|err| revert(convert_dispatch_error_to_string(err)))?;

		let ul_hash = keccak_256(universal_location.as_bytes());
		log3(
			handle.context().address,
			SELECTOR_LOG_EXTENDED_UL_WITH_EXTERNAL_URI,
			handle.context().caller,
			ul_hash,
			solidity::encode_event_data((universal_location, token_uri)),
		)
		.record(handle)?;

		Ok(())
	}

	#[precompile::public("updateExtendedULWithExternalURI(string,string)")]
	pub fn update(
		handle: &mut impl PrecompileHandle,
		universal_location: UnboundedString,
		token_uri: UnboundedString,
	) -> EvmResult<()> {
		super::register_cost::<Runtime>(
			handle,
			Runtime::WeightInfo::precompile_update(
				token_uri.as_bytes().len().try_into().unwrap(),
				universal_location.as_bytes().len().try_into().unwrap(),
			),
		)?;

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

		AssetMetadataExtender::<Runtime>::update_token_uri_extension(
			Runtime::AccountIdToH160::convert_back(handle.context().caller),
			universal_location_bounded.clone(),
			token_uri_bounded.clone(),
		)
		.map_err(|err| revert(convert_dispatch_error_to_string(err)))?;

		let ul_hash = keccak_256(universal_location.as_bytes());
		log3(
			handle.context().address,
			SELECTOR_LOG_UPDATED_EXTENDED_UL_WITH_EXTERNAL_URI,
			handle.context().caller,
			ul_hash,
			solidity::encode_event_data((universal_location, token_uri)),
		)
		.record(handle)?;

		Ok(())
	}

	#[precompile::public("balanceOfUL(string)")]
	pub fn balance_of(
		handle: &mut impl PrecompileHandle,
		universal_location: UnboundedString,
	) -> EvmResult<u32> {
		super::register_cost::<Runtime>(
			handle,
			Runtime::WeightInfo::precompile_balance_of(
				universal_location.as_bytes().len().try_into().unwrap(),
			),
		)?;

		// TODO this might be remove when we have the bounded string as param
		let universal_location_bounded: BoundedVec<
			u8,
			<Runtime as Config>::MaxUniversalLocationLength,
		> = universal_location
			.as_bytes()
			.to_vec()
			.try_into()
			.map_err(|_| revert("invalid universal location length"))?;

		let balance = AssetMetadataExtender::<Runtime>::balance_of(universal_location_bounded);

		Ok(balance)
	}

	#[precompile::public("claimerOfULByIndex(string,uint32)")]
	pub fn claimer_by_index(
		handle: &mut impl PrecompileHandle,
		universal_location: UnboundedString,
		index: u32,
	) -> EvmResult<Address> {
		super::register_cost::<Runtime>(
			handle,
			Runtime::WeightInfo::precompile_claimer_by_index(
				universal_location.as_bytes().len().try_into().unwrap(),
			),
		)?;

		// TODO this might be remove when we have the bounded string as param
		let universal_location_bounded: BoundedVec<
			u8,
			<Runtime as Config>::MaxUniversalLocationLength,
		> = universal_location
			.as_bytes()
			.to_vec()
			.try_into()
			.map_err(|_| revert("invalid universal location length"))?;

		if AssetMetadataExtender::balance_of(universal_location_bounded.clone()) <= index {
			return Err(revert("invalid index"));
		}

		let claimer = AssetMetadataExtender::<Runtime>::claimer_by_index(
			universal_location_bounded.clone(),
			index,
		)
		.ok_or_else(|| revert("invalid ul"))?;

		Ok(Address(Runtime::AccountIdToH160::convert(claimer)))
	}

	#[precompile::public("extensionOfULByIndex(string,uint32)")]
	pub fn extension_by_index(
		handle: &mut impl PrecompileHandle,
		universal_location: UnboundedString,
		index: u32,
	) -> EvmResult<UnboundedString> {
		super::register_cost::<Runtime>(
			handle,
			Runtime::WeightInfo::precompile_extension_by_index(
				universal_location.as_bytes().len().try_into().unwrap(),
			),
		)?;

		let universal_location_bounded: BoundedVec<
			u8,
			<Runtime as Config>::MaxUniversalLocationLength,
		> = universal_location
			.as_bytes()
			.to_vec()
			.try_into()
			.map_err(|_| revert("invalid universal location length"))?;

		if AssetMetadataExtender::balance_of(universal_location_bounded.clone()) <= index {
			return Err(revert("invalid index"));
		}

		let token_uri = AssetMetadataExtender::<Runtime>::token_uri_extension_by_index(
			universal_location_bounded.clone(),
			index,
		)
		.ok_or_else(|| revert("invalid ul"))?;

		Ok(token_uri.to_vec().into())
	}

	#[precompile::public("extensionOfULByClaimer(string,address)")]
	pub fn extension_by_location_and_claimer(
		handle: &mut impl PrecompileHandle,
		universal_location: UnboundedString,
		claimer: Address,
	) -> EvmResult<UnboundedString> {
		super::register_cost::<Runtime>(
			handle,
			Runtime::WeightInfo::precompile_extension_by_location_and_claimer(
				universal_location.as_bytes().len().try_into().unwrap(),
			),
		)?;

		let claimer: H160 = claimer.into();
		let universal_location_bounded: BoundedVec<
			u8,
			<Runtime as Config>::MaxUniversalLocationLength,
		> = universal_location
			.as_bytes()
			.to_vec()
			.try_into()
			.map_err(|_| revert("invalid universal location length"))?;

		let token_uri = AssetMetadataExtender::<Runtime>::extension_by_location_and_claimer(
			universal_location_bounded.clone(),
			Runtime::AccountIdToH160::convert_back(claimer),
		)
		.ok_or_else(|| revert("invalid ul"))?;

		Ok(token_uri.to_vec().into())
	}

	#[precompile::public("hasExtensionByClaimer(string,address)")]
	pub fn has_extension_by_claimer(
		handle: &mut impl PrecompileHandle,
		universal_location: UnboundedString,
		claimer: Address,
	) -> EvmResult<bool> {
		super::register_cost::<Runtime>(
			handle,
			Runtime::WeightInfo::precompile_has_extension_by_claimer(
				universal_location.as_bytes().len().try_into().unwrap(),
			),
		)?;

		let claimer: H160 = claimer.into();
		let universal_location_bounded: BoundedVec<
			u8,
			<Runtime as Config>::MaxUniversalLocationLength,
		> = universal_location
			.as_bytes()
			.to_vec()
			.try_into()
			.map_err(|_| revert("invalid universal location length"))?;

		let has_extension = AssetMetadataExtender::<Runtime>::has_extension(
			universal_location_bounded.clone(),
			Runtime::AccountIdToH160::convert_back(claimer),
		);

		Ok(has_extension)
	}
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
