//! Core primitives of the Evochain.

#![warn(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::weights::{constants::WEIGHT_REF_TIME_PER_SECOND, IdentityFee, Weight};
use frame_system::limits;
use sp_core::Hasher as HasherT;
use sp_runtime::{
	traits::{BlakeTwo256, IdentifyAccount, Verify},
	MultiSignature, MultiSigner, Perbill,
};

/// Maximum weight of single Evochain block.
///
/// This represents 0.5 seconds of compute assuming a target block time of six seconds.
///
/// Max PoV size is set to max value, since it isn't important for relay/standalone chains.
pub const MAXIMUM_BLOCK_WEIGHT: Weight =
	Weight::from_parts(WEIGHT_REF_TIME_PER_SECOND.saturating_div(2), u64::MAX);

/// Represents the portion of a block that will be used by Normal extrinsics.
pub const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);

/// Re-export `time_units` to make usage easier.
pub use time_units::*;

/// Human readable time units defined in terms of number of blocks.
pub mod time_units {
	use super::BlockNumber;

	/// Milliseconds between Evochain chain blocks.
	pub const MILLISECS_PER_BLOCK: u64 = 6000;
	/// Slot duration in Evochain chain consensus algorithms.
	pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;

	/// A minute, expressed in Evochain chain blocks.
	pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
	/// A hour, expressed in Evochain chain blocks.
	pub const HOURS: BlockNumber = MINUTES * 60;
	/// A day, expressed in Evochain chain blocks.
	pub const DAYS: BlockNumber = HOURS * 24;
}

/// Block number type used in Evochain.
pub type BlockNumber = u32;

/// Hash type used in Evochain.
pub type Hash = <BlakeTwo256 as HasherT>::Out;

/// Type of object that can produce hashes on Evochain.
pub type Hasher = BlakeTwo256;

/// The header type used by Evochain.
pub type Header = sp_runtime::generic::Header<BlockNumber, Hasher>;

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;

/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

/// The address format for describing accounts.
pub type Address = sp_runtime::MultiAddress<AccountId, ()>;

/// Public key of the chain account that may be used to verify signatures.
pub type AccountSigner = MultiSigner;

/// Balance of an account.
pub type Balance = u64;

/// Nonce of a transaction in the chain.
pub type Nonce = u32;

/// Weight-to-Fee type used by Evochain.
pub type WeightToFee = IdentityFee<Balance>;

frame_support::parameter_types! {
	/// Size limit of the Evochain blocks.
	pub BlockLength: limits::BlockLength =
		limits::BlockLength::max_with_normal_ratio(2 * 1024 * 1024, NORMAL_DISPATCH_RATIO);
	/// Weight limit of the Evochain blocks.
	pub BlockWeights: limits::BlockWeights =
		limits::BlockWeights::with_sensible_defaults(MAXIMUM_BLOCK_WEIGHT, NORMAL_DISPATCH_RATIO);
}
