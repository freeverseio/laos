#![cfg_attr(not(feature = "std"), no_std)]

use crate::{
	address_to_collection_id,
	traits::EvolutionCollection,
	types::{CollectionId, Slot},
	weights::WeightInfo,
	Config, Pallet as LaosEvolution, TokenId,
};
use fp_evm::ExitError;
use frame_support::DefaultNoBound;
use parity_scale_codec::Encode;
use precompile_utils::{
	keccak256,
	prelude::{
		log1, log2, log3, revert, Address, DiscriminantResult, EvmResult, LogExt, PrecompileHandle,
	},
	solidity::{self, codec::UnboundedString},
};
use sp_core::{H160, U256};
use sp_runtime::{traits::PhantomData, BoundedVec, DispatchError};

/// Solidity selector of the MintedWithExternalURI log, which is the Keccak of the Log signature.
pub const SELECTOR_LOG_MINTED_WITH_EXTERNAL_TOKEN_URI: [u8; 32] =
	keccak256!("MintedWithExternalURI(address,uint96,uint256,string)");

/// Solidity selector of the EvolvedWithExternalURI log, which is the Keccak of the Log signature.
pub const SELECTOR_LOG_EVOLVED_WITH_EXTERNAL_TOKEN_URI: [u8; 32] =
	keccak256!("EvolvedWithExternalURI(uint256,string)");

/// Solidity selector of the EnabledPublicMinting log, which is the Keccak of the Log signature.
pub const SELECTOR_LOG_ENABLED_PUBLIC_MINTING: [u8; 32] = keccak256!("EnabledPublicMinting()");
/// Solidity selector of the DisabledPublicMinting log, which is the Keccak of the Log signature.
pub const SELECTOR_LOG_DISABLED_PUBLIC_MINTING: [u8; 32] = keccak256!("DisabledPublicMinting()");

/// Solidity selector of the `OwnershipTransferred` log, which is the Keccak of the Log signature.
pub const SELECTOR_LOG_OWNERSHIP_TRANSFERRED: [u8; 32] =
	keccak256!("OwnershipTransferred(address,address)");

#[derive(Clone, DefaultNoBound)]
pub struct EvolutionCollectionPrecompileSet<R>(PhantomData<R>);

impl<R> EvolutionCollectionPrecompileSet<R> {
	pub fn new() -> Self {
		Self(PhantomData)
	}
}

#[precompile_utils::precompile]
#[precompile::precompile_set]
impl<R> EvolutionCollectionPrecompileSet<R>
where
	R: Config,
	R::AccountId: From<H160> + Into<H160> + Encode,
{
	#[precompile::discriminant]
	fn discriminant(address: H160, gas: u64) -> DiscriminantResult<CollectionId> {
		// maybe here we could avoid the extra_cost calculation cause there's no db read
		match address_to_collection_id(address) {
			Ok(id) => DiscriminantResult::Some(id, gas),
			Err(_) => DiscriminantResult::None(gas),
		}
	}

	#[precompile::public("owner()")]
	#[precompile::view]
	pub fn owner(
		collection_id: CollectionId,
		handle: &mut impl PrecompileHandle,
	) -> EvmResult<Address> {
		let weight = R::WeightInfo::precompile_owner();
		handle.record_external_cost(Some(weight.ref_time()), Some(weight.proof_size()))?;

		if let Some(owner) = LaosEvolution::<R>::collection_owner(collection_id) {
			Ok(Address(owner.into()))
		} else {
			Err(revert("collection does not exist"))
		}
	}

	#[precompile::public("mintWithExternalURI(address,uint96,string)")]
	fn mint(
		collection_id: CollectionId,
		handle: &mut impl PrecompileHandle,
		to: Address,
		slot: Slot,
		token_uri: UnboundedString, /* TODO use bounded vec or stringkind from solidity
		                             * BoundedString<<R as Config>::MaxTokenUriLength> */
	) -> EvmResult<U256> {
		let to: H160 = to.into();

		// TODO this might be remove when we have the bounded string as param
		let token_uri_bounded: BoundedVec<u8, <R as Config>::MaxTokenUriLength> = token_uri
			.as_bytes()
			.to_vec()
			.try_into()
			.map_err(|_| revert("invalid token uri length"))?;

		match LaosEvolution::<R>::mint_with_external_uri(
			handle.context().caller.into(),
			collection_id,
			slot,
			to.into(),
			token_uri_bounded.clone(),
		) {
			Ok(token_id) => {
				log2(
					handle.context().address,
					SELECTOR_LOG_MINTED_WITH_EXTERNAL_TOKEN_URI,
					to,
					solidity::encode_event_data((slot, token_id, token_uri)), /* TODO token_uri_bounded */
				)
				.record(handle)?;

				Ok(token_id)
			},
			Err(err) => Err(revert(convert_dispatch_error_to_string(err))),
		}
	}

	#[precompile::public("evolveWithExternalURI(uint256,string)")]
	fn evolve(
		collection_id: CollectionId,
		handle: &mut impl PrecompileHandle,
		token_id: TokenId,
		token_uri: UnboundedString, /* TODO use bounded vec or stringkind from solidity
		                             * BoundedString<<R as Config>::MaxTokenUriLength> */
	) -> EvmResult<()> {
		// TODO this might be remove when we have the bounded string as param
		let token_uri_bounded: BoundedVec<u8, <R as Config>::MaxTokenUriLength> = token_uri
			.as_bytes()
			.to_vec()
			.try_into()
			.map_err(|_| revert("invalid token uri length"))?;

		match LaosEvolution::<R>::evolve_with_external_uri(
			handle.context().caller.into(),
			collection_id,
			token_id,
			token_uri_bounded.clone(),
		) {
			Ok(()) => {
				let mut token_id_bytes = [0u8; 32];
				token_id.to_big_endian(&mut token_id_bytes);

				log2(
					handle.context().address,
					SELECTOR_LOG_EVOLVED_WITH_EXTERNAL_TOKEN_URI,
					token_id_bytes,
					solidity::encode_event_data(token_uri),
				)
				.record(handle)?;

				Ok(())
			},
			Err(err) => Err(revert(convert_dispatch_error_to_string(err))),
		}
	}

	#[precompile::public("transferOwnership(address)")]
	fn transfer_ownership(
		collection_id: CollectionId,
		handle: &mut impl PrecompileHandle,
		to: Address,
	) -> EvmResult<()> {
		let to: H160 = to.into();
		LaosEvolution::<R>::transfer_ownership(
			handle.context().caller.into(),
			to.into(),
			collection_id,
		)
		.map_err(|err| revert(convert_dispatch_error_to_string(err)))?;

		log3(
			handle.context().address,
			SELECTOR_LOG_OWNERSHIP_TRANSFERRED,
			handle.context().caller,
			to,
			solidity::encode_event_data(()),
		)
		.record(handle)?;

		Ok(())
	}

	#[precompile::public("enablePublicMinting()")]
	fn enable_public_minting(
		collection_id: CollectionId,
		handle: &mut impl PrecompileHandle,
	) -> EvmResult<()> {
		match LaosEvolution::<R>::enable_public_minting(
			handle.context().caller.into(),
			collection_id,
		) {
			Ok(()) => {
				log1(
					handle.context().address,
					SELECTOR_LOG_ENABLED_PUBLIC_MINTING,
					solidity::encode_event_data(()),
				)
				.record(handle)?;

				Ok(())
			},
			Err(err) => Err(revert(convert_dispatch_error_to_string(err))),
		}
	}

	#[precompile::public("disablePublicMinting()")]
	fn disable_public_minting(
		collection_id: CollectionId,
		handle: &mut impl PrecompileHandle,
	) -> EvmResult<()> {
		match LaosEvolution::<R>::disable_public_minting(
			handle.context().caller.into(),
			collection_id,
		) {
			Ok(()) => {
				log1(
					handle.context().address,
					SELECTOR_LOG_DISABLED_PUBLIC_MINTING,
					solidity::encode_event_data(()),
				)
				.record(handle)?;

				Ok(())
			},
			Err(err) => Err(revert(convert_dispatch_error_to_string(err))),
		}
	}

	#[precompile::public("isPublicMintingEnabled()")]
	#[precompile::view]
	fn is_public_minting_enabled(
		collection_id: CollectionId,
		_handle: &mut impl PrecompileHandle,
	) -> EvmResult<bool> {
		let is_enabled = LaosEvolution::<R>::is_public_minting_enabled(collection_id);
		Ok(is_enabled)
	}

	#[precompile::public("tokenURI(uint256)")]
	#[precompile::view]
	fn token_uri(
		collection_id: CollectionId,
		_handle: &mut impl PrecompileHandle,
		token_id: U256,
	) -> EvmResult<UnboundedString> {
		if let Some(token_uri) = LaosEvolution::<R>::token_uri(collection_id, token_id) {
			Ok(token_uri.to_vec().into())
		} else {
			Err(revert("asset does not exist"))
		}
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
