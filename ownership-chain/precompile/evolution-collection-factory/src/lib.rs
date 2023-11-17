//! LAOS precompile module.

#![cfg_attr(not(feature = "std"), no_std)]
use fp_evm::{Precompile, PrecompileHandle, PrecompileOutput};
use laos_precompile_utils::{
	keccak256, revert_dispatch_error, succeed, Address, EvmDataWriter, EvmResult, FunctionModifier,
	LogExt, LogsBuilder, PrecompileHandleExt,
};
use pallet_laos_evolution::{
	collection_id_to_address, traits::EvolutionCollectionFactory as EvolutionCollectionFactoryT,
};
use parity_scale_codec::Encode;

use sp_core::H160;
use sp_std::{fmt::Debug, marker::PhantomData};

/// Solidity selector of the CreateCollection log, which is the Keccak of the Log signature.
pub const SELECTOR_LOG_NEW_COLLECTION: [u8; 32] = keccak256!("NewCollection(address,address)");

#[laos_precompile_utils_macro::generate_function_selector]
#[derive(Debug, PartialEq)]
pub enum Action {
	/// Create collection
	CreateCollection = "createCollection(address)",
}

/// Wrapper for the precompile function.
pub struct EvolutionCollectionFactoryPrecompile<AddressMapping, AccountId, LaosEvolution>(
	PhantomData<(AddressMapping, AccountId, LaosEvolution)>,
)
where
	AddressMapping: pallet_evm::AddressMapping<AccountId>,
	AccountId: Encode + Debug,
	LaosEvolution: EvolutionCollectionFactoryT<AccountId>;

impl<AddressMapping, AccountId, LaosEvolution>
	EvolutionCollectionFactoryPrecompile<AddressMapping, AccountId, LaosEvolution>
where
	AddressMapping: pallet_evm::AddressMapping<AccountId>,
	AccountId: From<H160> + Into<H160> + Encode + Debug,
	LaosEvolution: EvolutionCollectionFactoryT<AccountId>,
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
						let collection_id: H160 = collection_id_to_address(collection_id);

						LogsBuilder::new(context.address)
							.log2(
								SELECTOR_LOG_NEW_COLLECTION,
								owner,
								EvmDataWriter::new().write(Address(collection_id.into())).build(),
							)
							.record(handle)?;

						Ok(succeed(
							EvmDataWriter::new().write(Address(collection_id.into())).build(),
						))
					},
					Err(err) => Err(revert_dispatch_error(err)),
				}
			},
		}
	}
}

impl<AddressMapping, AccountId, LaosEvolution> Precompile
	for EvolutionCollectionFactoryPrecompile<AddressMapping, AccountId, LaosEvolution>
where
	AddressMapping: pallet_evm::AddressMapping<AccountId>,
	AccountId: From<H160> + Into<H160> + Encode + Debug,
	LaosEvolution: EvolutionCollectionFactoryT<AccountId>,
{
	fn execute(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		let selector = handle.read_selector()?;

		handle.check_function_modifier(match selector {
			Action::CreateCollection => FunctionModifier::NonPayable,
		})?;

		Self::inner_execute(handle, &selector)
	}
}

#[cfg(test)]
mod tests;
