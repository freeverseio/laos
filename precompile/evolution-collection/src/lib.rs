#![cfg_attr(not(feature = "std"), no_std)]
use fp_evm::ExitError;
use frame_support::DefaultNoBound;
use pallet_evm::GasWeightMapping;
use pallet_laos_evolution::{
	address_to_collection_id,
	types::CollectionId,
	weights::{SubstrateWeight as LaosEvolutionWeights, WeightInfo},
	Config, EvolutionCollection, Pallet as LaosEvolution, Slot, TokenId,
};
use parity_scale_codec::Encode;
use precompile_utils::{
	keccak256,
	prelude::{
		log1, log2, log3, revert, Address, DiscriminantResult, EvmResult, LogExt, PrecompileHandle,
		RuntimeHelper,
	},
	solidity::{self, codec::UnboundedString},
	substrate::TryDispatchError,
};
use sp_core::{H160, U256};
use sp_runtime::{traits::PhantomData, BoundedVec};

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
	R: pallet_evm::Config + pallet_laos_evolution::Config + frame_system::Config,
	R::AccountId: From<H160> + Into<H160> + Encode,
{
	#[precompile::discriminant]
	fn discriminant(address: H160, gas: u64) -> DiscriminantResult<CollectionId> {
		let extra_cost = RuntimeHelper::<R>::db_read_gas_cost();
		if gas < extra_cost {
			return DiscriminantResult::OutOfGas;
		}

		// maybe here we could avoid the extra_cost calculation cause there's no db read
		match address_to_collection_id(address) {
			Ok(id) => DiscriminantResult::Some(id, extra_cost),
			Err(_) => DiscriminantResult::None(extra_cost),
		}
	}

	#[precompile::public("owner()")]
	#[precompile::view]
	fn owner(
		collection_id: CollectionId,
		_handle: &mut impl PrecompileHandle,
	) -> EvmResult<Address> {
		if let Some(owner) = LaosEvolution::<R>::collection_owner(collection_id) {
			// TODO: handle the cost
			// handle.record_cost(GasCalculator::<Runtime>::db_read_gas_cost(1))?;

			Ok(Address(owner.into()))
		} else {
			Err(revert("collection does not exist"))
		}
	}

	// TODO use custom type for slot, it needs to be uint96, otherwise test file
	#[precompile::public("mintWithExternalURI(address,uint128,string)")]
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
				let consumed_weight = LaosEvolutionWeights::<R>::mint_with_external_uri(
					token_uri_bounded.len() as u32,
				);

				log2(
					handle.context().address,
					SELECTOR_LOG_MINTED_WITH_EXTERNAL_TOKEN_URI,
					to,
					solidity::encode_event_data((slot, token_id, token_uri)), // TODO token_uri_bounded
				)
				.record(handle)?;

				// Record EVM cost
				handle.record_cost(<R as pallet_evm::Config>::GasWeightMapping::weight_to_gas(
					consumed_weight,
				))?;

				// Record Substrate related costs
				// TODO: Add `ref_time` when precompiles are benchmarked
				handle.record_external_cost(None, Some(consumed_weight.proof_size()))?;

				Ok(token_id)
			},
			Err(err) => Err(TryDispatchError::Substrate(err).into()),
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
				let consumed_weight = LaosEvolutionWeights::<R>::evolve_with_external_uri(
					token_uri_bounded.len() as u32,
				);

				let mut token_id_bytes = [0u8; 32];
				token_id.to_big_endian(&mut token_id_bytes);

				log2(
					handle.context().address,
					SELECTOR_LOG_EVOLVED_WITH_EXTERNAL_TOKEN_URI,
					token_id_bytes,
					solidity::encode_event_data(token_uri),
				)
				.record(handle)?;

				// Record EVM cost
				handle.record_cost(<R as pallet_evm::Config>::GasWeightMapping::weight_to_gas(
					consumed_weight,
				))?;

				// Record Substrate related costs
				// TODO: Add `ref_time` when precompiles are benchmarked
				handle.record_external_cost(None, Some(consumed_weight.proof_size()))?;

				Ok(())
			},
			Err(err) => Err(TryDispatchError::Substrate(err).into()),
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
		.map_err(|err| TryDispatchError::Substrate(err))?;

		let consumed_weight = LaosEvolutionWeights::<R>::transfer_ownership();

		log3(
			handle.context().address,
			SELECTOR_LOG_OWNERSHIP_TRANSFERRED,
			handle.context().caller,
			to,
			solidity::encode_event_data(()),
		)
		.record(handle)?;

		// Record EVM cost
		handle.record_cost(<R as pallet_evm::Config>::GasWeightMapping::weight_to_gas(
			consumed_weight,
		))?;

		// Record Substrate related costs
		handle.record_external_cost(None, Some(consumed_weight.proof_size()))?;
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
				let consumed_weight = LaosEvolutionWeights::<R>::enable_public_minting();

				log1(
					handle.context().address,
					SELECTOR_LOG_ENABLED_PUBLIC_MINTING,
					solidity::encode_event_data(()),
				)
				.record(handle)?;

				// Record EVM cost
				handle.record_cost(<R as pallet_evm::Config>::GasWeightMapping::weight_to_gas(
					consumed_weight,
				))?;

				// Record Substrate related costs
				// TODO: Add `ref_time` when precompiles are benchmarked
				handle.record_external_cost(None, Some(consumed_weight.proof_size()))?;

				Ok(())
			},
			Err(err) => Err(TryDispatchError::Substrate(err).into()),
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
				let consumed_weight = LaosEvolutionWeights::<R>::disable_public_minting();

				log1(
					handle.context().address,
					SELECTOR_LOG_DISABLED_PUBLIC_MINTING,
					solidity::encode_event_data(()),
				)
				.record(handle)?;

				// Record EVM cost
				handle.record_cost(<R as pallet_evm::Config>::GasWeightMapping::weight_to_gas(
					consumed_weight,
				))?;

				// Record Substrate related costs
				// TODO: Add `ref_time` when precompiles are benchmarked
				handle.record_external_cost(None, Some(consumed_weight.proof_size()))?;

				Ok(())
			},
			Err(err) => Err(TryDispatchError::Substrate(err).into()),
		}
	}

	#[precompile::public("isPublicMintingEnabled()")]
	#[precompile::view]
	fn is_public_minting_enabled(
		collection_id: CollectionId,
		handle: &mut impl PrecompileHandle,
	) -> EvmResult<bool> {
		let is_enabled = LaosEvolution::<R>::is_public_minting_enabled(collection_id);
		let consumed_gas: u64 = RuntimeHelper::<R>::db_read_gas_cost();
		handle.record_cost(consumed_gas)?;
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

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;
