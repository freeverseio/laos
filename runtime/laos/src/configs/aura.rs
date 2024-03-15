use crate::{AuraId, Runtime};
use frame_support::parameter_types;

parameter_types! {
	pub const MaxAuthorities : u32 = 100_000;
	pub const AllowMultipleBlocksPerSlot: bool = false;
}

impl pallet_aura::Config for Runtime {
	type AuthorityId = AuraId;
	type DisabledValidators = ();
	type MaxAuthorities = MaxAuthorities;
	type AllowMultipleBlocksPerSlot = AllowMultipleBlocksPerSlot;
}
