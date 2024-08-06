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

use crate::{AuraId, Runtime, MILLISECS_PER_BLOCK};
use frame_support::parameter_types;
use sp_core::ConstU64;

parameter_types! {
	pub const MaxAuthorities : u32 = 100_000;
	pub const AllowMultipleBlocksPerSlot: bool = false;
}

impl pallet_aura::Config for Runtime {
	type AuthorityId = AuraId;
	type DisabledValidators = ();
	type MaxAuthorities = MaxAuthorities;
	type AllowMultipleBlocksPerSlot = AllowMultipleBlocksPerSlot;
	type SlotDuration = ConstU64<MILLISECS_PER_BLOCK>;
}
