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

use crate::{
	currency::calculate_deposit, weights, Balance, Balances, Runtime, RuntimeCall, RuntimeEvent,
};
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
	type WeightInfo = weights::pallet_proxy::WeightInfo<Runtime>;
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

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{
		currency::{MILLIUNIT, UNIT},
		tests::{ExtBuilder, ALICE, BOB},
		AccountId, RuntimeOrigin,
	};
	use core::str::FromStr;
	use frame_support::assert_ok;
	use sp_runtime::traits::Dispatchable;

	#[test]
	fn check_deposits() {
		assert_eq!(<Runtime as pallet_proxy::Config>::ProxyDepositBase::get(), 10_080 * MILLIUNIT);
		assert_eq!(<Runtime as pallet_proxy::Config>::ProxyDepositFactor::get(), 210 * MILLIUNIT);
		assert_eq!(
			<Runtime as pallet_proxy::Config>::AnnouncementDepositBase::get(),
			10_080 * MILLIUNIT
		);
		assert_eq!(
			<Runtime as pallet_proxy::Config>::AnnouncementDepositFactor::get(),
			560 * MILLIUNIT
		);
	}

	#[test]
	fn create_pure_proxy() {
		let alice = AccountId::from_str(ALICE).unwrap();

		ExtBuilder::default()
			.with_balances(vec![(alice, 1000 * UNIT)])
			.build()
			.execute_with(|| {
				let call = RuntimeCall::Proxy(pallet_proxy::Call::create_pure {
					proxy_type: ProxyType::Any,
					index: 0, // index
					delay: 0, // delay
				});

				assert_ok!(call.dispatch(RuntimeOrigin::signed(alice)));
			});
	}

	#[test]
	fn add_proxy_to_pure_proxy_should_succeed() {
		let delay = 0;
		let index = 0;
		let pure_proxy = AccountId::from_str("0x37228888117681e8afc3e6ff2de89863be918d34").unwrap();
		let alice = AccountId::from_str(ALICE).unwrap();
		let bob = AccountId::from_str(BOB).unwrap();

		ExtBuilder::default()
			.with_balances(vec![
				(alice, 1000 * UNIT),
				(bob, 1000 * UNIT),
				(pure_proxy, 1000 * UNIT),
			])
			.build()
			.execute_with(|| {
				let call = RuntimeCall::Proxy(pallet_proxy::Call::create_pure {
					proxy_type: ProxyType::Any,
					index, // index
					delay, // delay
				});
				assert_ok!(call.dispatch(RuntimeOrigin::signed(alice)));

				assert_eq!(
					pallet_proxy::Pallet::<Runtime>::pure_account(
						&alice,
						&ProxyType::Any,
						index,
						None,
					),
					pure_proxy
				);

				// Initially, there should be 1 proxy after creation
				assert_eq!(pallet_proxy::Pallet::<Runtime>::proxies(&pure_proxy).0.len(), 1);

				// Add a proxy and verify the count increases to 2
				let call = RuntimeCall::Proxy(pallet_proxy::Call::add_proxy {
					delegate: bob,
					proxy_type: ProxyType::Any,
					delay,
				});
				assert_ok!(call.dispatch(RuntimeOrigin::signed(pure_proxy)));
				assert_eq!(pallet_proxy::Pallet::<Runtime>::proxies(&pure_proxy).0.len(), 2);
			});
	}
}
