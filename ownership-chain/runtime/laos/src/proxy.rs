// all the use statements
use super::{deposit, Balances, Runtime, RuntimeCall, RuntimeEvent};
use frame_support::{parameter_types, traits::InstanceFilter};
pub use ownership_parachain_primitives::Balance;
use parity_scale_codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_runtime::{traits::BlakeTwo256, RuntimeDebug};
use sp_std::prelude::*;

parameter_types! {
	pub const ProxyDepositBase: Balance = deposit(1, 8);
	pub const ProxyDepositFactor: Balance = deposit(0, 21);
	pub const MaxProxies: u16 = 32;
	pub const MaxPending: u16 = 32;
	pub const AnnouncementDepositBase: Balance = deposit(1, 8);
	pub const AnnouncementDepositFactor: Balance = deposit(0, 56);
}

impl pallet_proxy::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type Currency = Balances;
	type ProxyType = ProxyType;
	// One storage item; key size 32, value size 8
	type ProxyDepositBase = ProxyDepositBase;
	// Additional storage item size of 21 bytes (20 bytes AccountId + 1 byte sizeof(ProxyType)).
	type ProxyDepositFactor = ProxyDepositFactor;
	type MaxProxies = MaxProxies;
	type MaxPending = MaxPending;
	type CallHasher = BlakeTwo256;
	type AnnouncementDepositBase = AnnouncementDepositBase;
	// Additional storage item size of 56 bytes:
	// - 20 bytes AccountId
	// - 32 bytes Hasher (Blake2256)
	// - 4 bytes BlockNumber (u32)
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
	/// All calls can be proxied. This is the trivial/most permissive filter.
	Any = 0,
}

impl Default for ProxyType {
	fn default() -> Self {
		Self::Any
	}
}

impl InstanceFilter<RuntimeCall> for ProxyType {
	fn filter(&self, _c: &RuntimeCall) -> bool {
		match self {
			ProxyType::Any => true,
		}
	}
}
