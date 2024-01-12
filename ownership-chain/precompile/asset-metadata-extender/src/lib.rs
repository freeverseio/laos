#![cfg_attr(not(feature = "std"), no_std)]
use fp_evm::{Precompile, PrecompileHandle, PrecompileOutput};
use laos_precompile_utils::{
	keccak256, revert_dispatch_error, succeed, Bytes, EvmDataReader, EvmDataWriter, EvmResult,
	FunctionModifier, LogExt, LogsBuilder, PrecompileHandleExt,
};
use pallet_asset_metadata_extender::{
	traits::AssetMetadataExtender as AssetMetadataExtenderT,
	types::{TokenUriOf, UniversalLocationOf},
	Pallet as AssetMetadataExtender,
};
use parity_scale_codec::Encode;
use precompile_utils::solidity::revert::revert;

use sp_core::{keccak_256, H160};
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
			Action::Balance => unimplemented!(),
			Action::Claimer => unimplemented!(),
			Action::Extension => unimplemented!(),
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

		let input = handle.read_input()?;
		input.expect_arguments(2)?;
		let universal_location = Self::get_ul_from_input(input)?;
		let token_uri = Self::get_token_uri_from_input(input)?;

		match AssetMetadataExtender::<Runtime>::create_token_uri_extension(
			context.caller.into(),
			universal_location.into(),
			token_uri.into(),
		) {
			Ok(_) => Ok(succeed(sp_std::vec![])),
			Err(err) => Err(revert_dispatch_error(err)),
		}
	}

	fn update(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		let context = handle.context();

		let input = handle.read_input()?;
		input.expect_arguments(2)?;
		let universal_location = Self::get_ul_from_input(input)?;
		let token_uri = Self::get_token_uri_from_input(input)?;
		let claimer = context.caller;

		// create a hash of the universal location of type keccak256 do not use keccak256! macro
		// because universal location is not a literal
		let universel_location_hash = keccak_256(&universal_location);

		let result = AssetMetadataExtender::<Runtime>::update_token_uri_extension(
			claimer.into(),
			universal_location.clone().into(),
			token_uri.clone().into(),
		);

		if let Err(err) = result {
			return Err(revert_dispatch_error(err));
		}

		LogsBuilder::new(context.address)
			.log3(
				SELECTOR_LOG_EXTENDED_TOKEN_URI_UPDATED,
				claimer,
				universel_location_hash,
				EvmDataWriter::new()
					.write(Bytes(universal_location.into()))
					.write(Bytes(token_uri.into()))
					.build(),
			)
			.record(handle)?;

		Ok(succeed(sp_std::vec![]))
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
