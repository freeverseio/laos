// Copyright 2023-2024 LAOS Chain Foundation
// This file is part of LAOS.

// LAOS is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// LAOS is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with LAOS.  If not, see <http://www.gnu.org/licenses/>.

//! Primitives of the Laos parachain.

#![warn(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::weights::{constants::WEIGHT_REF_TIME_PER_SECOND, IdentityFee, Weight};
use frame_system::limits;
use sp_core::Hasher as HasherT;
use sp_runtime::{
	traits::{BlakeTwo256, IdentifyAccount, Verify},
	Perbill,
};

/// Authority ID used in parachain.
pub type AuraId = sp_consensus_aura::sr25519::AuthorityId;

/// Maximal weight of single LaosParachain block.
///
/// This represents 0.5 seconds of compute assuming a target block time of 12 seconds.
///
/// Max PoV size is set to `5Mb` as all Cumulus-based parachains do.
pub const MAXIMUM_BLOCK_WEIGHT: Weight = Weight::from_parts(
	WEIGHT_REF_TIME_PER_SECOND.saturating_div(2),
	cumulus_primitives_core::relay_chain::MAX_POV_SIZE as u64,
);

/// Represents the portion of a block that will be used by Normal extrinsics.
pub const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);

/// Block number type used in Laos chain.
pub type BlockNumber = u32;

/// Hash type used in Laos chain.
pub type Hash = <BlakeTwo256 as HasherT>::Out;

/// The type of object that can produce hashes on Laos chain.
pub type Hasher = BlakeTwo256;

/// The header type used by Laos chain.
pub type Header = sp_runtime::generic::Header<BlockNumber, Hasher>;

/// Signature type used by Laos chain.
pub type Signature = fp_account::EthereumSignature;

/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

/// Balance of an account.
pub type Balance = u128;

/// An instant or duration in time.
pub type Moment = u64;

/// Nonce of a transaction in the parachain.
pub type Nonce = u64;

/// Weight-to-Fee type used by Laos parachain.
pub type WeightToFee = IdentityFee<Balance>;

frame_support::parameter_types! {
	/// Size limit of the Laos parachain blocks.
	pub BlockLength: limits::BlockLength =
		limits::BlockLength::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);
	/// Weight limit of the Laos parachain blocks.
	pub BlockWeights: limits::BlockWeights =
		limits::BlockWeights::with_sensible_defaults(MAXIMUM_BLOCK_WEIGHT, NORMAL_DISPATCH_RATIO);
}
