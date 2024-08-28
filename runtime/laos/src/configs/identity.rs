use crate::{currency, AccountId, Balances, Runtime, RuntimeEvent, Signature};
use frame_support::parameter_types;
use frame_system::EnsureRoot;
use parachains_common::DAYS;
use sp_core::ConstU128;

parameter_types! {
	pub const MaxSubAccounts: u32 = 100;
	pub const MaxAdditionalFields: u32 = 100;
	pub const MaxRegistrars: u32 = 20;
	pub const PendingUsernameExpiration: u32 = 7 * DAYS;
	pub const MaxSuffixLength: u32 = 7;
	pub const MaxUsernameLength: u32 = 32;
}

type IdentityForceOrigin = EnsureRoot<AccountId>;
type IdentityRegistrarOrigin = EnsureRoot<AccountId>;

impl pallet_identity::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	// Add one item in storage and take 258 bytes
	type BasicDeposit = ConstU128<{ currency::calculate_deposit(1, 258) }>;
	// Does not add any item to the storage but takes 1 bytes
	type ByteDeposit = ConstU128<{ currency::calculate_deposit(0, 1) }>;
	// Add one item in storage and take 53 bytes
	type SubAccountDeposit = ConstU128<{ currency::calculate_deposit(1, 53) }>;
	type MaxSubAccounts = MaxSubAccounts;
	type IdentityInformation = pallet_identity::legacy::IdentityInfo<MaxAdditionalFields>;
	type MaxRegistrars = MaxRegistrars;
	type Slashed = (); // TODO: Treasury;
	type ForceOrigin = IdentityForceOrigin;
	type RegistrarOrigin = IdentityRegistrarOrigin;
	type OffchainSignature = Signature;
	type SigningPublicKey = <Signature as sp_runtime::traits::Verify>::Signer;
	type UsernameAuthorityOrigin = EnsureRoot<AccountId>;
	type PendingUsernameExpiration = PendingUsernameExpiration;
	type MaxSuffixLength = MaxSuffixLength;
	type MaxUsernameLength = MaxUsernameLength;
	type WeightInfo = (); // moonbeam_weights::pallet_identity::WeightInfo<Runtime>;
}
