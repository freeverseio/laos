//! LAOS precompile module.

#![cfg_attr(not(feature = "std"), no_std)]
use fp_evm::{Precompile, PrecompileHandle, PrecompileOutput};
use pallet_laos_evolution::traits::LaosEvolution as LaosEvolutionT;
use parity_scale_codec::Encode;
use precompile_utils::{
	keccak256, revert, revert_dispatch_error, succeed, Address, EvmDataWriter, EvmResult,
	FunctionModifier, LogExt, LogsBuilder, PrecompileHandleExt,
};

use sp_core::{H160, H256};
use sp_std::{fmt::Debug, marker::PhantomData, vec::Vec};

/// Solidity selector of the CreateCollection log, which is the Keccak of the Log signature.
pub const SELECTOR_LOG_NEW_COLLECTION: [u8; 32] = keccak256!("NewCollection(uint64,address)");

#[precompile_utils_macro::generate_function_selector]
#[derive(Debug, PartialEq)]
pub enum Action {
	/// Create collection
	CreateCollection = "createCollection(address)",
	/// Get owner of the collection
	OwnerOfCollection = "ownerOfCollection(uint64)",
	/// Get tokenURI of the token in collection
	TokenURI = "tokenURI(uint64,uint256)",
}

/// Wrapper for the precompile function.
pub struct LaosEvolutionPrecompile<AddressMapping, AccountId, TokenUri, LaosEvolution>(
	PhantomData<(AddressMapping, AccountId, TokenUri, LaosEvolution)>,
)
where
	AddressMapping: pallet_evm::AddressMapping<AccountId>,
	AccountId: Encode + Debug,
	TokenUri: TryFrom<Vec<u8>>,
	LaosEvolution: LaosEvolutionT<AccountId, TokenUri>;

impl<AddressMapping, AccountId, TokenUri, LaosEvolution> Precompile
	for LaosEvolutionPrecompile<AddressMapping, AccountId, TokenUri, LaosEvolution>
where
	AddressMapping: pallet_evm::AddressMapping<AccountId>,
	AccountId: From<H160> + Into<H160> + Encode + Debug,
	TokenUri: TryFrom<Vec<u8>>,
	LaosEvolution: LaosEvolutionT<AccountId, TokenUri>,
{
	fn execute(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		let selector = handle.read_selector()?;

		handle.check_function_modifier(match selector {
			Action::CreateCollection => FunctionModifier::NonPayable,
			Action::OwnerOfCollection => FunctionModifier::View,
			Action::TokenURI => FunctionModifier::View,
		})?;

		match selector {
			Action::CreateCollection => {
				let mut input = handle.read_input()?;

				input.expect_arguments(1)?;

				let owner = input.read::<Address>()?.0;

				match LaosEvolution::create_collection(owner.into()) {
					Ok(collection_id) => {
						LogsBuilder::new(handle.context().address)
							.log3(
								SELECTOR_LOG_NEW_COLLECTION,
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
			Action::OwnerOfCollection => {
				let mut input = handle.read_input()?;
				input.expect_arguments(1)?;

				let collection_id = input.read::<u64>()?;

				if let Some(owner) = LaosEvolution::collection_owner(collection_id) {
					Ok(succeed(EvmDataWriter::new().write(Address(owner.into())).build()))
				} else {
					Err(revert("collection does not exist"))
				}
			},
			Action::TokenURI => {
				unimplemented!()
			},
		}
	}
}

#[cfg(test)]
mod tests;
