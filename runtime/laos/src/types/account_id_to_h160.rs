use crate::AccountId;
use sp_core::H160;
use sp_runtime::traits::Convert;

/// Converts [`AccountId`] to [`H160`]
pub struct AccountIdToH160;

impl sp_runtime::traits::Convert<AccountId, H160> for AccountIdToH160 {
	fn convert(account_id: AccountId) -> H160 {
		H160(account_id.0)
	}
}
