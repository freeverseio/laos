// Copyright 2023-2024 Freeverse.io
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
pub use parachains_common::NORMAL_DISPATCH_RATIO;
use sp_core::Hasher as HasherT;
use sp_runtime::traits::{BlakeTwo256, IdentifyAccount, Verify};
pub use parachains_common::MAXIMUM_BLOCK_WEIGHT;

// /// We allow for 2 seconds of compute with a 6 second average block.
// pub const MAXIMUM_BLOCK_WEIGHT: Weight = Weight::from_parts(
// 	WEIGHT_REF_TIME_PER_SECOND.saturating_mul(2),
// 	cumulus_primitives_core::relay_chain::MAX_POV_SIZE as u64,
// );

/// Authority ID used in parachain.
pub type AuraId = sp_consensus_aura::sr25519::AuthorityId;

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
pub type Nonce = u32;

/// Weight-to-Fee type used by Laos parachain.
pub type WeightToFee = IdentityFee<Balance>;

frame_support::parameter_types! {
	/// Size limit of the Laos parachain blocks.
	pub RuntimeBlockLength: limits::BlockLength =
		limits::BlockLength::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);
	/// Weight limit of the Laos parachain blocks.
	pub RuntimeBlockWeights: limits::BlockWeights =
		limits::BlockWeights::with_sensible_defaults(MAXIMUM_BLOCK_WEIGHT, NORMAL_DISPATCH_RATIO);
}

#[cfg(test)]
mod tests {
	use super::*;
	use frame_support::{dispatch::DispatchClass, weights::Weight};

	#[test]
	fn test_block_weights() {
		let weights = RuntimeBlockWeights::get();

		assert_eq!(weights.base_block, Weight::from_parts(453383000, 0));
		assert_eq!(weights.max_block, Weight::from_parts(2000000000000, 5242880));

		let normal = weights.per_class.get(DispatchClass::Normal);
		assert_eq!(normal.base_extrinsic, Weight::from_parts(107074000, 0));
		assert_eq!(normal.max_extrinsic, Some(Weight::from_parts(1299892926000, 3407872)));
		assert_eq!(normal.max_total, Some(Weight::from_parts(1500000000000, 3932160)));
		assert_eq!(normal.reserved, Some(Weight::from_parts(0, 0)));

		let mandatory = weights.per_class.get(DispatchClass::Mandatory);
		assert_eq!(mandatory.base_extrinsic, Weight::from_parts(107074000, 0));
		assert_eq!(mandatory.max_extrinsic, None);
		assert_eq!(mandatory.max_total, None);
		assert_eq!(mandatory.reserved, None);

		let operational = weights.per_class.get(DispatchClass::Operational);
		assert_eq!(operational.base_extrinsic, Weight::from_parts(107074000, 0));
		assert_eq!(operational.max_extrinsic, Some(Weight::from_parts(1799892926000, 4718592)));
		assert_eq!(operational.max_total, Some(Weight::from_parts(2000000000000, 5242880)));
		assert_eq!(operational.reserved, Some(Weight::from_parts(500000000000, 1310720)));
	}
}
