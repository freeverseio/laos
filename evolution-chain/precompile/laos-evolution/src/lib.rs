//! LAOS precompile module.

#![cfg_attr(not(feature = "std"), no_std)]
use fp_evm::{Log, Precompile, PrecompileHandle, PrecompileOutput};
use pallet_laos_evolution::{traits::LaosEvolution as LaosEvolutionT, Slot};
use parity_scale_codec::Encode;
use precompile_utils::{
	keccak256, revert, revert_dispatch_error, succeed, Address, Bytes, EvmDataWriter, EvmResult,
	FunctionModifier, LogExt, LogsBuilder, PrecompileHandleExt,
};

use sp_core::{H160, H256};
use sp_std::{fmt::Debug, marker::PhantomData, vec::Vec};

/// Solidity selector of the CreateCollection log, which is the Keccak of the Log signature.
pub const SELECTOR_LOG_NEW_COLLECTION: [u8; 32] = keccak256!("NewCollection(uint64,address)");
/// Solidity selector of the Transfer log, which is the Keccak of the Log signature.
pub const SELECTOR_LOG_TRANSFER: [u8; 32] = keccak256!("Transfer(address,address,uint64,uint256)");

#[precompile_utils_macro::generate_function_selector]
#[derive(Debug, PartialEq)]
pub enum Action {
	/// Create collection
	CreateCollection = "createCollection(address)",
	/// Get owner of the collection
	OwnerOfCollection = "ownerOfCollection(uint64)",
	/// Mint token
	Mint = "mint(uint64,uint96,address,string)",
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

impl<AddressMapping, AccountId, TokenUri, LaosEvolution>
	LaosEvolutionPrecompile<AddressMapping, AccountId, TokenUri, LaosEvolution>
where
	AddressMapping: pallet_evm::AddressMapping<AccountId>,
	AccountId: From<H160> + Into<H160> + Encode + Debug,
	TokenUri: TryFrom<Vec<u8>>,
	LaosEvolution: LaosEvolutionT<AccountId, TokenUri>,
{
	fn inner_execute(
		handle: &mut impl PrecompileHandle,
		action: &Action,
	) -> EvmResult<PrecompileOutput> {
		let mut input = handle.read_input()?;
		let context = handle.context();

		match action {
			Action::CreateCollection => {
				input.expect_arguments(1)?;

				let owner = input.read::<Address>()?.0;

				match LaosEvolution::create_collection(owner.into()) {
					Ok(collection_id) => {
						LogsBuilder::new(context.address)
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
				input.expect_arguments(1)?;

				let collection_id = input.read::<u64>()?;

				if let Some(owner) = LaosEvolution::collection_owner(collection_id) {
					Ok(succeed(EvmDataWriter::new().write(Address(owner.into())).build()))
				} else {
					Err(revert("collection does not exist"))
				}
			},
			Action::Mint => {
				let caller = context.caller;

				input.expect_arguments(4)?;

				let collection_id = input.read::<u64>()?;
				let slot = input.read::<Slot>()?;
				let to = input.read::<Address>()?.0;
				let token_uri_raw = input.read::<Bytes>()?.0;
				let token_uri =
					token_uri_raw.try_into().map_err(|_| revert("invalid token uri length"))?;

				match LaosEvolution::mint_with_external_uri(
					caller.into(),
					collection_id,
					slot,
					to.into(),
					token_uri,
				) {
					Ok(token_id) => {
						let mut token_id_bytes = [0u8; 32];
						token_id.to_big_endian(&mut token_id_bytes);

						Log {
							address: context.address,
							topics: sp_std::vec![
								H256(SELECTOR_LOG_TRANSFER),
								H256::from(H160::zero()),
								H256::from(to),
								H256::from_low_u64_be(collection_id),
								H256(token_id_bytes),
							],
							data: Vec::new(),
						}
						.record(handle)?;

						Ok(succeed(EvmDataWriter::new().write(token_id).build()))
					},
					Err(err) => Err(revert_dispatch_error(err)),
				}
			},
		}
	}
}

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
			Action::Mint => FunctionModifier::NonPayable,
			Action::OwnerOfCollection => FunctionModifier::View,
		})?;

		Self::inner_execute(handle, &selector)
	}
}

#[cfg(test)]
mod tests;
