//! LAOS precompile module.

#![cfg_attr(not(feature = "std"), no_std)]
use fp_evm::{Precompile, PrecompileHandle, PrecompileOutput};
use pallet_laos_evolution::traits::LivingAssetsEvolution as LivingAssetsEvolutionT;
use parity_scale_codec::Encode;
use precompile_utils::{
	keccak256, revert_dispatch_error, succeed, Address, EvmDataWriter, EvmResult, FunctionModifier,
	LogExt, LogsBuilder, PrecompileHandleExt,
};

use sp_core::{H160, H256};
use sp_std::{fmt::Debug, marker::PhantomData, vec::Vec};

/// Solidity selector of the CreateCollection log, which is the Keccak of the Log signature.
pub const SELECTOR_LOG_CREATE_COLLECTION: [u8; 32] =
	keccak256!("CreateCollection(uint256,address)");

#[precompile_utils_macro::generate_function_selector]
#[derive(Debug, PartialEq)]
pub enum Action {
	/// Create collection
	CreateCollection = "createCollection(address)",
}

/// Wrapper for the precompile function.
pub struct LaosEvolutionPrecompile<AddressMapping, AccountId, TokenUri, LivingAssetsEvolution>(
	PhantomData<(AddressMapping, AccountId, TokenUri, LivingAssetsEvolution)>,
)
where
	AddressMapping: pallet_evm::AddressMapping<AccountId>,
	AccountId: Encode + Debug,
	TokenUri: TryFrom<Vec<u8>>,
	LivingAssetsEvolution: LivingAssetsEvolutionT<AccountId, TokenUri>;

impl<AddressMapping, AccountId, TokenUri, LivingAssetsEvolution> Precompile
	for LaosEvolutionPrecompile<AddressMapping, AccountId, TokenUri, LivingAssetsEvolution>
where
	AddressMapping: pallet_evm::AddressMapping<AccountId>,
	AccountId: From<H160> + Into<H160> + Encode + Debug,
	TokenUri: TryFrom<Vec<u8>>,
	LivingAssetsEvolution: LivingAssetsEvolutionT<AccountId, TokenUri>,
{
	fn execute(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		let selector = handle.read_selector()?;

		handle.check_function_modifier(match selector {
			Action::CreateCollection => FunctionModifier::NonPayable,
		})?;

		match selector {
			Action::CreateCollection => {
				let mut input = handle.read_input()?;

				input.expect_arguments(1)?;

				let owner = input.read::<Address>()?.0;

				match LivingAssetsEvolution::create_collection(owner.into()) {
					Ok(collection_id) => {
						LogsBuilder::new(handle.context().address)
							.log3(
								SELECTOR_LOG_CREATE_COLLECTION,
								H256::from_low_u64_be(collection_id.to_be()),
								owner,
								Vec::new(),
							)
							.record(handle)?;

						Ok(succeed(EvmDataWriter::new().write(collection_id).build()))
					},
					Err(err) => Err(revert_dispatch_error(err)),
				}
			},
		}
	}
}

#[cfg(test)]
mod tests;
