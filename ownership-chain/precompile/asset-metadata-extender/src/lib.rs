#![cfg_attr(not(feature = "std"), no_std)]
use fp_evm::{Precompile, PrecompileHandle, PrecompileOutput};
use laos_precompile_utils::{
	keccak256, succeed, Bytes, EvmResult, FunctionModifier, PrecompileHandleExt,
};
use pallet_asset_metadata_extender::{
	traits::AssetMetadataExtender as AssetMetadataExtenderT,
	types::{TokenUriOf, UniversalLocationOf},
	Config, Pallet as AssetMetadataExtender,
};
use precompile_utils::solidity::revert::revert;
use sp_runtime::BoundedVec;
use sp_std::{fmt::Debug, marker::PhantomData};
/// Solidity selector of the TokenURIExtended log, which is the Keccak of the Log signature.
pub const SELECTOR_LOG_TOKEN_URI_EXTENDED: [u8; 32] =
	keccak256!("TokenURIExtended(address,string,uint256)");

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
}

pub struct AssetMetadataExtenderPrecompile<Runtime>(PhantomData<Runtime>)
where
	Runtime: pallet_asset_metadata_extender::Config;

impl<Runtime> Precompile for AssetMetadataExtenderPrecompile<Runtime>
where
	Runtime: pallet_evm::Config + pallet_asset_metadata_extender::Config,
{
	fn execute(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		let selector = handle.read_selector()?;

		handle.check_function_modifier(match selector {
			Action::Extend => FunctionModifier::NonPayable,
			Action::Balance => FunctionModifier::View,
			Action::Claimer => FunctionModifier::View,
			Action::Extension => FunctionModifier::View,
		})?;

		match selector {
			Action::Extend => Self::extend(handle),
			Action::Balance => unimplemented!(),
			Action::Claimer => unimplemented!(),
			Action::Extension => unimplemented!(),
		}
	}
}

impl<Runtime> AssetMetadataExtenderPrecompile<Runtime>
where
	Runtime: pallet_evm::Config + pallet_asset_metadata_extender::Config,
	AssetMetadataExtender<Runtime>: AssetMetadataExtenderT<
		Runtime::AccountId,
		TokenUriOf<Runtime>,
		UniversalLocationOf<Runtime>,
	>,
{
	fn extend(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		let mut input = handle.read_input()?;
		let context = handle.context();
		let _caller = context.caller;

		input.expect_arguments(2)?;
		let universal_location = input.read::<Bytes>()?.0;
		let _universal_location: BoundedVec<u8, <Runtime as Config>::MaxUniversalLocationLength> =
			universal_location
				.clone()
				.try_into()
				.map_err(|_| revert("invalid universal location length"))?;
		let token_uri = input.read::<Bytes>()?.0;
		let _token_uri: BoundedVec<u8, <Runtime as Config>::MaxTokenUriLength> =
			token_uri.clone().try_into().map_err(|_| revert("invalid token uri length"))?;

		Ok(succeed(sp_std::vec![]))
	}
}

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;
