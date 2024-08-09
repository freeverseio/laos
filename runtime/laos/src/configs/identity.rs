use crate::{
	currency::{MILLIUNIT, UNIT},
	AccountId, Balance, Balances, EnsureRoot, Runtime, RuntimeEvent, Treasury,
};
use frame_support::parameter_types;

parameter_types! {
	/// The basic deposit to create an identity.
	pub const BasicDeposit: Balance = 20 * UNIT;
	/// Deposit for each additional field.
	pub const FieldDeposit: Balance = 200 * MILLIUNIT;
	/// The deposit needed to create a sub-account.
	/// We do not allow sub-accounts so can be 0.
	/// Should be set to a non-zero value if sub-accounts are allowed.
	pub const SubAccountDeposit: Balance = 0;
	/// Max number of sub-accounts that can be created.
	/// We do not allow sub-accounts so set to 0.
	pub const MaxSubAccounts: u32 = 0;
	/// Max number of additional fields that can be created.
	pub const MaxAdditionalFields: u32 = 100;
	/// Max number of registrars that can be set.
	pub const MaxRegistrars: u32 = 3;
}

impl pallet_identity::Config for Runtime {
	type BasicDeposit = BasicDeposit;
	type Currency = Balances;
	type FieldDeposit = FieldDeposit;
	type ForceOrigin = EnsureRoot<AccountId>;
	type MaxAdditionalFields = MaxAdditionalFields;
	type MaxRegistrars = MaxRegistrars;
	type MaxSubAccounts = MaxSubAccounts;
	type RegistrarOrigin = EnsureRoot<AccountId>;
	type RuntimeEvent = RuntimeEvent;
	type Slashed = Treasury;
	type SubAccountDeposit = SubAccountDeposit;
	type WeightInfo = pallet_identity::weights::SubstrateWeight<Runtime>;
}
