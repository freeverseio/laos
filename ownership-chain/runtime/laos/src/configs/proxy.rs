use super::calculate_deposit;
use crate::{Balance, Balances, Runtime, RuntimeCall, RuntimeEvent};
use frame_support::{parameter_types, traits::InstanceFilter};
use parity_scale_codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_runtime::{traits::BlakeTwo256, RuntimeDebug};

parameter_types! {
	// One storage item; key size 32, value size 8
	pub const ProxyDepositBase: Balance = calculate_deposit(1, 8);
	// Additional storage item size of 21 bytes (20 bytes AccountId + 1 byte sizeof(ProxyType)).
	pub const ProxyDepositFactor: Balance = calculate_deposit(0, 21);
	pub const MaxProxies: u16 = 32;
	pub const MaxPending: u16 = 32;
	pub const AnnouncementDepositBase: Balance = calculate_deposit(1, 8);
	// Additional storage item size of 56 bytes:
	// - 20 bytes AccountId
	// - 32 bytes Hasher (Blake2256)
	// - 4 bytes BlockNumber (u32)
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
	Any = 0,
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
