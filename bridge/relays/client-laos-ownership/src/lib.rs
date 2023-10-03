// Copyright 2019-2021 Parity Technologies (UK) Ltd.
// This file is part of Parity Bridges Common.

// Parity Bridges Common is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity Bridges Common is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity Bridges Common.  If not, see <http://www.gnu.org/licenses/>.

//! Types used to connect to the Ownership-Substrate chain.

pub mod codegen_runtime;

use bp_polkadot_core::SuffixedCommonSignedExtensionExt;
use codec::Encode;
use relay_substrate_client::{
	Chain, ChainWithBalances, ChainWithMessages, ChainWithTransactions, Error as SubstrateError,
	SignParam, UnderlyingChainProvider, UnsignedTransaction,
};
use sp_core::{storage::StorageKey, Pair};
use sp_runtime::{generic::SignedPayload, traits::IdentifyAccount};
use std::time::Duration;

pub use codegen_runtime::api::runtime_types;

pub type RuntimeCall = runtime_types::laos_ownership_runtime::RuntimeCall;
pub type SudoCall = runtime_types::pallet_sudo::pallet::Call;
pub type BridgeGrandpaCall = runtime_types::pallet_bridge_grandpa::pallet::Call;

/// The address format for describing accounts.
pub type Address = bp_laos_ownership::Address;

/// Ownership parachain definition
#[derive(Debug, Clone, Copy)]
pub struct OwnershipParachain;

impl UnderlyingChainProvider for OwnershipParachain {
	type Chain = bp_laos_ownership::OwnershipParachain;
}

impl Chain for OwnershipParachain {
	const NAME: &'static str = "OwnershipParachain";
	const BEST_FINALIZED_HEADER_ID_METHOD: &'static str =
		bp_laos_ownership::BEST_FINALIZED_OWNERSHIP_PARACHAIN_HEADER_METHOD;
	const AVERAGE_BLOCK_INTERVAL: Duration = Duration::from_secs(5);

	type SignedBlock = bp_polkadot_core::SignedBlock;
	type Call = RuntimeCall;
}

impl ChainWithBalances for OwnershipParachain {
	fn account_info_storage_key(account_id: &Self::AccountId) -> StorageKey {
		bp_polkadot_core::AccountInfoStorageMapKeyProvider::final_key(account_id)
	}
}

impl ChainWithMessages for OwnershipParachain {
	// TODO (https://github.com/paritytech/parity-bridges-common/issues/1692): change the name
	const WITH_CHAIN_RELAYERS_PALLET_NAME: Option<&'static str> = Some("BridgeRelayers");
	const TO_CHAIN_MESSAGE_DETAILS_METHOD: &'static str =
		bp_laos_ownership::TO_OWNERSHIP_PARACHAIN_MESSAGE_DETAILS_METHOD;
	const FROM_CHAIN_MESSAGE_DETAILS_METHOD: &'static str =
		bp_laos_ownership::FROM_OWNERSHIP_PARACHAIN_MESSAGE_DETAILS_METHOD;
}

impl ChainWithTransactions for OwnershipParachain {
	type AccountKeyPair = sp_core::sr25519::Pair;
	type SignedTransaction =
		bp_polkadot_core::UncheckedExtrinsic<Self::Call, bp_laos_ownership::SignedExtension>;

	fn sign_transaction(
		param: SignParam<Self>,
		unsigned: UnsignedTransaction<Self>,
	) -> Result<Self::SignedTransaction, SubstrateError> {
		let raw_payload = SignedPayload::new(
			unsigned.call,
			bp_laos_ownership::SignedExtension::from_params(
				param.spec_version,
				param.transaction_version,
				unsigned.era,
				param.genesis_hash,
				unsigned.nonce,
				unsigned.tip,
				Default::default(),
			),
		)?;

		let signature = raw_payload.using_encoded(|payload| param.signer.sign(payload));
		let signer: sp_runtime::MultiSigner = param.signer.public().into();
		let (call, extra, _) = raw_payload.deconstruct();

		Ok(Self::SignedTransaction::new_signed(
			call,
			signer.into_account().into(),
			signature.into(),
			extra,
		))
	}

	fn is_signed(tx: &Self::SignedTransaction) -> bool {
		tx.signature.is_some()
	}

	fn is_signed_by(signer: &Self::AccountKeyPair, tx: &Self::SignedTransaction) -> bool {
		tx.signature
			.as_ref()
			.map(|(address, _, _)| *address == Address::Id(signer.public().into()))
			.unwrap_or(false)
	}

	fn parse_transaction(tx: Self::SignedTransaction) -> Option<UnsignedTransaction<Self>> {
		let extra = &tx.signature.as_ref()?.2;
		Some(UnsignedTransaction::new(tx.function, extra.nonce()).tip(extra.tip()))
	}
}

/// OwnershipParachain signing params.
pub type SigningParams = sp_core::sr25519::Pair;

/// OwnershipParachain header type used in headers sync.
pub type SyncHeader = relay_substrate_client::SyncHeader<bp_laos_ownership::Header>;
