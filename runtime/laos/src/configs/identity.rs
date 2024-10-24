use crate::{
	currency::calculate_deposit, AccountId, Balance, Balances, Runtime, RuntimeEvent, Signature,
	Treasury, DAYS,
};
use frame_support::parameter_types;
use frame_system::EnsureRoot;

parameter_types! {
	pub const MaxSubAccounts: u32 = 100;
	pub const MaxAdditionalFields: u32 = 100;
	pub const MaxRegistrars: u32 = 20;
	pub const PendingUsernameExpiration: u32 = 7 * DAYS;
	pub const MaxSuffixLength: u32 = 7;
	pub const MaxUsernameLength: u32 = 32;
	pub const BasicDeposit: Balance = calculate_deposit(1, 258);
	pub const ByteDeposit: Balance = calculate_deposit(0, 1);
	pub const SubAccountDeposit: Balance = calculate_deposit(0, 53);
}

type IdentityForceOrigin = EnsureRoot<AccountId>;
type IdentityRegistrarOrigin = EnsureRoot<AccountId>;

impl pallet_identity::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	// Add one item in storage and take 258 bytes
	type BasicDeposit = BasicDeposit;
	// Does not add any item to the storage but takes 1 bytes
	type ByteDeposit = ByteDeposit;
	// Add one item in storage and take 53 bytes
	type SubAccountDeposit = SubAccountDeposit;
	type MaxSubAccounts = MaxSubAccounts;
	type IdentityInformation = pallet_identity::legacy::IdentityInfo<MaxAdditionalFields>;
	type MaxRegistrars = MaxRegistrars;
	type Slashed = Treasury;
	type ForceOrigin = IdentityForceOrigin;
	type RegistrarOrigin = IdentityRegistrarOrigin;
	type OffchainSignature = Signature;
	type SigningPublicKey = <Signature as sp_runtime::traits::Verify>::Signer;
	type UsernameAuthorityOrigin = EnsureRoot<AccountId>;
	type PendingUsernameExpiration = PendingUsernameExpiration;
	type MaxSuffixLength = MaxSuffixLength;
	type MaxUsernameLength = MaxUsernameLength;
	type WeightInfo = pallet_identity::weights::SubstrateWeight<Runtime>; // TODO is not recommended use default weights but currently there is a function within the
																	   // benchmarks pallet code that cause a panic because it uses `Sr25519` signature type: https://github.com/paritytech/polkadot-sdk/blob/master/substrate/frame/identity/src/benchmarking.rs#L608
																	   // whereas our runtime uses `Ecdsa` signature type
}
