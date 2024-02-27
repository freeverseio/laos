use crate::{Balances, Runtime, RuntimeCall, RuntimeEvent, MILLIUNIT, UNIT};
use frame_support::{parameter_types, traits::InstanceFilter};
use ownership_parachain_primitives::Balance;
use parity_scale_codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_runtime::{traits::BlakeTwo256, RuntimeDebug};

// Define storage fees as constants for clarity and reuse
const STORAGE_ITEM_FEE: Balance = 10 * UNIT;
const STORAGE_BYTE_FEE: Balance = 10 * MILLIUNIT;

/// Calculates the deposit required based on the number of items and bytes.
const fn calculate_deposit(items: u32, bytes: u32) -> Balance {
	(items as Balance) * STORAGE_ITEM_FEE + (bytes as Balance) * STORAGE_BYTE_FEE
}

parameter_types! {
	pub const ProxyDepositBase: Balance = calculate_deposit(1, 8);
	pub const ProxyDepositFactor: Balance = calculate_deposit(0, 21);
	pub const MaxProxies: u16 = 32;
	pub const MaxPending: u16 = 32;
	pub const AnnouncementDepositBase: Balance = calculate_deposit(1, 8);
	pub const AnnouncementDepositFactor: Balance = calculate_deposit(0, 56);
}

impl pallet_proxy::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type Currency = Balances;
	type ProxyType = ProxyType;
	type ProxyDepositBase = ProxyDepositBase;
	type ProxyDepositFactor = ProxyDepositFactor;
	type MaxProxies = MaxProxies;
	type MaxPending = MaxPending;
	type CallHasher = BlakeTwo256;
	type AnnouncementDepositBase = AnnouncementDepositBase;
	type AnnouncementDepositFactor = AnnouncementDepositFactor;
	type WeightInfo = pallet_proxy::weights::SubstrateWeight<Runtime>;
}

#[derive(
	Copy,
	Clone,
	Eq,
	PartialEq,
	Ord,
	PartialOrd,
	Encode,
	Decode,
	RuntimeDebug,
	MaxEncodedLen,
	TypeInfo,
)]
pub enum ProxyType {
	/// Represents a proxy type that allows any call to be proxied.
	Any,
}

impl Default for ProxyType {
	fn default() -> Self {
		Self::Any
	}
}

impl InstanceFilter<RuntimeCall> for ProxyType {
	fn filter(&self, _c: &RuntimeCall) -> bool {
		matches!(self, ProxyType::Any)
	}
}
