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

//! Types used in the pallet
use laos_precompile_utils::{EvmData, EvmDataReader, EvmDataWriter, EvmResult};
use parity_scale_codec::{Decode, Encode, MaxEncodedLen};
use precompile_utils::solidity::{
	codec::{Reader, Writer},
	revert::{MayRevert, RevertReason},
	Codec,
};
use scale_info::{prelude::string::String, TypeInfo};
use serde::{Deserialize, Serialize};
use sp_core::U256;
use sp_runtime::{BoundedVec, RuntimeDebug};

/// Collection id type
pub type CollectionId = u64;

/// Explicit `AccountId`
pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

/// Wrapper around `BoundedVec` for `tokenUri`
pub type TokenUriOf<T> = BoundedVec<u8, <T as crate::Config>::MaxTokenUriLength>;

/// TokenId type
/// every slot is identified by a unique `asset_id = concat(slot #, owner_address)`
pub type TokenId = U256;

/// Slot type - 96-bit unsigned integer
#[derive(
	Eq,
	PartialEq,
	Clone,
	Copy,
	Encode,
	Decode,
	Default,
	Deserialize,
	RuntimeDebug,
	MaxEncodedLen,
	Serialize,
	TypeInfo,
	PartialOrd,
	Ord,
	Hash,
)]
pub struct Slot([u8; 12]);
impl Slot {
	/// Maximum value for a 96-bit unsigned integer
	pub const MAX_SLOT: Slot = Slot([0xFF; 12]);

	pub fn new(bytes: [u8; 12]) -> Self {
		Slot(bytes)
	}

	pub fn from_u128(value: u128) -> Result<Self, &'static str> {
		if value > ((1u128 << 96) - 1) {
			Err("Value exceeds 96-bit limit")
		} else {
			let bytes = value.to_be_bytes();
			Ok(Slot(bytes[4..].try_into().unwrap()))
		}
	}

	pub fn as_u128(&self) -> u128 {
		let mut bytes = [0u8; 16];
		bytes[4..].copy_from_slice(&self.0);
		u128::from_be_bytes(bytes)
	}
}

impl TryFrom<u128> for Slot {
	type Error = &'static str;

	fn try_from(num: u128) -> Result<Self, Self::Error> {
		Slot::from_u128(num)
	}
}

impl Into<u128> for Slot {
	fn into(self) -> u128 {
		self.as_u128()
	}
}

impl Codec for Slot {
	fn read(reader: &mut Reader) -> MayRevert<Self> {
		let value128 = <u128 as Codec>::read(reader)?;
		let slot = Slot::try_from(value128)
			.map_err(|_| RevertReason::read_out_of_bounds(Self::signature()))?;
		Ok(slot)
	}

	fn write(writer: &mut Writer, value: Self) {
		<u128 as Codec>::write(writer, value.into())
	}

	fn has_static_size() -> bool {
		<u128 as Codec>::has_static_size()
	}

	fn signature() -> String {
		String::from("uint96")
	}
}


/// This is a legacy type that is used for compatibility with the legacy Laos EVM Pallets
/// used in Klaos runtime.
/// 
/// NOTE: remove once the legacy pallets are removed
pub struct LegacySlot(pub Slot);

impl EvmData for LegacySlot {
	fn write(writer: &mut EvmDataWriter, value: Self) {
		<u128 as EvmData>::write(writer, value.0.into())
	}

	fn read(reader: &mut EvmDataReader) -> EvmResult<Self> {
		let value128 = <u128 as EvmData>::read(reader)?;
		let slot =
			Slot::try_from(value128).map_err(|_| RevertReason::read_out_of_bounds("uint96"))?;
		Ok(LegacySlot(slot))
	}

	fn has_static_size() -> bool {
		<u128 as EvmData>::has_static_size()
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn slot_from_u128_within_limit() {
		let value = 123456789012345678901234567u128;
		let slot = Slot::try_from(value).unwrap();
		let result: u128 = slot.into();
		assert_eq!(result, value);
	}

	#[test]
	fn slot_from_u128_exceeds_limit() {
		let value = 1u128 << 100;
		let result = Slot::try_from(value);
		assert!(result.is_err());
		assert_eq!(result.unwrap_err(), "Value exceeds 96-bit limit");
	}

	#[test]
	fn max_slot() {
		let max_value: u128 = Slot::MAX_SLOT.into();
		assert_eq!(max_value, (1u128 << 96) - 1);
	}
}
