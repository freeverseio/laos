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

use crate::AccountId;
use sp_core::H160;
use sp_runtime::traits::{Convert, ConvertBack};

/// Converts [`AccountId`] to [`H160`]
pub struct AccountIdToH160;

impl Convert<AccountId, H160> for AccountIdToH160 {
	fn convert(account_id: AccountId) -> H160 {
		H160(account_id.0)
	}
}

impl ConvertBack<AccountId, H160> for AccountIdToH160 {
	fn convert_back(account_id: H160) -> AccountId {
		AccountId::from(account_id)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use sp_core::H160;

	#[test]
	fn convert_account_id_to_h160() {
		let account_id = AccountId::from([1u8; 20]);
		let h160 = AccountIdToH160::convert(account_id);
		assert_eq!(h160, H160([1u8; 20]));
	}

	#[test]
	fn convert_h160_to_account_id() {
		let h160 = H160([1u8; 20]);
		let account_id = AccountIdToH160::convert_back(h160);
		assert_eq!(account_id, AccountId::from([1u8; 20]));
	}
}
