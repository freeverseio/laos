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

use bp_laos_ownership::Signature;
use bp_polkadot_core::SuffixedCommonSignedExtensionExt;
use bp_runtime::{EncodedOrDecodedCall, StorageMapKeyProvider};
use codec::Encode;
use relay_substrate_client::{
	Chain, ChainWithBalances, ChainWithMessages, ChainWithTransactions, Error as SubstrateError,
	SignParam, UnderlyingChainProvider, UnsignedTransaction,
};
use sp_core::{ecdsa, storage::StorageKey, Pair};
use sp_runtime::generic::SignedPayload;
use std::time::Duration;

pub use codegen_runtime::api::runtime_types;

pub type RuntimeCall = runtime_types::laos_runtime::RuntimeCall;
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

/// Provides a storage key for account data.
///
/// We need to use this approach when we don't have access to the runtime.
/// The equivalent command to invoke in case full `Runtime` is known is this:
/// `let key = frame_system::Account::<Runtime>::storage_map_final_key(&account_id);`
///
/// NOTE: this is a custom impl for `AccountId = AccountId20`
/// source: [`bp_polkadot_core::AccountInfoStorageMapKeyProvider`]
pub struct AccountInfoStorageMapKeyProvider;

impl StorageMapKeyProvider for AccountInfoStorageMapKeyProvider {
	const MAP_NAME: &'static str = "Account";
	type Hasher = frame_support::Blake2_128Concat;
	type Key = bp_laos_ownership::AccountId;
	// This should actually be `AccountInfo`, but we don't use this property in order to decode the
	// data. So we use `Vec<u8>` as if we would work with encoded data.
	type Value = Vec<u8>;
}

impl AccountInfoStorageMapKeyProvider {
	/// Name of the system pallet.
	const PALLET_NAME: &'static str = "System";

	/// Return storage key for given account data.
	pub fn final_key(id: &bp_laos_ownership::AccountId) -> StorageKey {
		<Self as StorageMapKeyProvider>::final_key(Self::PALLET_NAME, id)
	}
}

impl ChainWithBalances for OwnershipParachain {
	fn account_info_storage_key(account_id: &Self::AccountId) -> StorageKey {
		AccountInfoStorageMapKeyProvider::final_key(account_id)
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
	type AccountKeyPair = ecdsa::Pair;
	type SignedTransaction = fp_self_contained::UncheckedExtrinsic<
		bp_laos_ownership::AccountId,
		EncodedOrDecodedCall<Self::Call>,
		Signature,
		bp_laos_ownership::SignedExtension,
	>;

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
		let signer = param.signer.public();
		let (call, extra, _) = raw_payload.deconstruct();

		Ok(Self::SignedTransaction::new_signed(
			call,
			signer.into(),
			Signature::new(signature),
			extra,
		))
	}

	fn is_signed(tx: &Self::SignedTransaction) -> bool {
		// this is because [`fp_self_contained::UncheckedExtrinsic`] is a wrapper around
		// [`sp_runtime::generic::UncheckedExtrinsic`] and we need to check the inner
		tx.0.signature.is_some()
	}

	fn is_signed_by(signer: &Self::AccountKeyPair, tx: &Self::SignedTransaction) -> bool {
		tx.0.signature
			.as_ref()
			.map(|(address, _, _)| *address == signer.public().into())
			.unwrap_or(false)
	}

	fn parse_transaction(tx: Self::SignedTransaction) -> Option<UnsignedTransaction<Self>> {
		let extra = &tx.0.signature.as_ref()?.2;
		Some(UnsignedTransaction::new(tx.0.function, extra.nonce()).tip(extra.tip()))
	}
}

/// OwnershipParachain signing params.
pub type SigningParams = sp_core::sr25519::Pair;

/// OwnershipParachain header type used in headers sync.
pub type SyncHeader = relay_substrate_client::SyncHeader<bp_laos_ownership::Header>;
