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
	Staking = 3,
}

impl Default for ProxyType {
	fn default() -> Self {
		Self::Any
	}
}

impl InstanceFilter<RuntimeCall> for ProxyType {
	fn filter(&self, c: &RuntimeCall) -> bool {
		match self {
			ProxyType::Any => true,
			ProxyType::Staking => {
				matches!(c, RuntimeCall::ParachainStaking(..))
			},
		}
	}

	fn is_superset(&self, o: &Self) -> bool {
		match (self, o) {
			(x, y) if x == y => true,
			(ProxyType::Any, _) => true,
			_ => false,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{
		currency::{MILLIUNIT, UNIT},
		tests::{ExtBuilder, ALICE, BOB},
		AccountId, Proxy, RuntimeOrigin, System,
	};
	use core::str::FromStr;
	use frame_support::assert_ok;
	use pallet_proxy::Event as ProxyEvent;
	use sp_core::{H160, H256, U256};
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
		let alice = AccountId::from_str(ALICE).unwrap();
		let bob = AccountId::from_str(BOB).unwrap();

		ExtBuilder::default()
			.with_balances(vec![(alice, 1000 * UNIT), (bob, 1000 * UNIT)])
			.build()
			.execute_with(|| {
				frame_system::Pallet::<Runtime>::set_block_number(1);
				let call = RuntimeCall::Proxy(pallet_proxy::Call::create_pure {
					proxy_type: ProxyType::Any,
					index, // index
					delay, // delay
				});
				assert_ok!(call.dispatch(RuntimeOrigin::signed(alice)));

				// Get pure proxy address from event
				let events = frame_system::Pallet::<Runtime>::events();
				let pure_proxy = match events.last().unwrap().event {
					RuntimeEvent::Proxy(pallet_proxy::Event::PureCreated { pure, .. }) => pure,
					_ => panic!("unexpected event"),
				};

				assert_eq!(
					&pallet_proxy::Pallet::<Runtime>::pure_account(
						&alice,
						&ProxyType::Any,
						index,
						None,
					),
					&pure_proxy
				);

				// Send some money to pure proxy
				let call = RuntimeCall::Balances(pallet_balances::Call::transfer_keep_alive {
					dest: pure_proxy,
					value: 100 * UNIT,
				});
				assert_ok!(call.dispatch(RuntimeOrigin::signed(alice)));

				// Initially, there should be 1 proxy after creation
				assert_eq!(pallet_proxy::Pallet::<Runtime>::proxies(pure_proxy).0.len(), 1);

				// Add a proxy and verify the count increases to 2
				let call = RuntimeCall::Proxy(pallet_proxy::Call::add_proxy {
					delegate: bob,
					proxy_type: ProxyType::Any,
					delay,
				});

				let call = RuntimeCall::Proxy(pallet_proxy::Call::proxy {
					real: pure_proxy,
					force_proxy_type: None,
					call: Box::new(call),
				});
				assert_ok!(call.dispatch(RuntimeOrigin::signed(alice)));
				assert_eq!(pallet_proxy::Pallet::<Runtime>::proxies(pure_proxy).0.len(), 2);
			});
	}

	#[test]
	fn add_proxy_staking_to_pure_proxy_should_succeed() {
		let delay = 0;
		let index = 0;
		let alice = AccountId::from_str(ALICE).unwrap();
		let bob = AccountId::from_str(BOB).unwrap();

		ExtBuilder::default()
			.with_balances(vec![(alice, 1000 * UNIT), (bob, 1000 * UNIT)])
			.build()
			.execute_with(|| {
				frame_system::Pallet::<Runtime>::set_block_number(1);
				let call = RuntimeCall::Proxy(pallet_proxy::Call::create_pure {
					proxy_type: ProxyType::Any,
					index, // index
					delay, // delay
				});
				assert_ok!(call.dispatch(RuntimeOrigin::signed(alice)));

				// Get pure proxy address from event
				let events = frame_system::Pallet::<Runtime>::events();
				let pure_proxy = match events.last().unwrap().event {
					RuntimeEvent::Proxy(pallet_proxy::Event::PureCreated { pure, .. }) => pure,
					_ => panic!("unexpected event"),
				};

				assert_eq!(
					&pallet_proxy::Pallet::<Runtime>::pure_account(
						&alice,
						&ProxyType::Any,
						index,
						None,
					),
					&pure_proxy
				);

				// Send some money to pure proxy
				let call = RuntimeCall::Balances(pallet_balances::Call::transfer_keep_alive {
					dest: pure_proxy,
					value: 100 * UNIT,
				});
				assert_ok!(call.dispatch(RuntimeOrigin::signed(alice)));

				// Initially, there should be 1 proxy after creation
				assert_eq!(pallet_proxy::Pallet::<Runtime>::proxies(pure_proxy).0.len(), 1);

				// Add a proxy and verify the count increases to 2
				let call = RuntimeCall::Proxy(pallet_proxy::Call::add_proxy {
					delegate: bob,
					proxy_type: ProxyType::Staking,
					delay,
				});

				let call = RuntimeCall::Proxy(pallet_proxy::Call::proxy {
					real: pure_proxy,
					force_proxy_type: None,
					call: Box::new(call),
				});
				assert_ok!(call.dispatch(RuntimeOrigin::signed(alice)));
				assert_eq!(pallet_proxy::Pallet::<Runtime>::proxies(pure_proxy).0.len(), 2);
			});
	}

	#[test]
	fn proxy_staking_should_not_be_able_to_transfer() {
		let delay = 0;
		let index = 0;
		let alice = AccountId::from_str(ALICE).unwrap();
		let bob = AccountId::from_str(BOB).unwrap();

		ExtBuilder::default()
			.with_balances(vec![(alice, 1000 * UNIT), (bob, 1000 * UNIT)])
			.build()
			.execute_with(|| {
				frame_system::Pallet::<Runtime>::set_block_number(1);
				let call = RuntimeCall::Proxy(pallet_proxy::Call::create_pure {
					proxy_type: ProxyType::Staking,
					index, // index
					delay, // delay
				});
				assert_ok!(call.dispatch(RuntimeOrigin::signed(alice)));

				// Get pure proxy address from event
				let events = frame_system::Pallet::<Runtime>::events();
				let pure_proxy = match events.last().unwrap().event {
					RuntimeEvent::Proxy(pallet_proxy::Event::PureCreated { pure, .. }) => pure,
					_ => panic!("unexpected event"),
				};

				assert_eq!(
					&pallet_proxy::Pallet::<Runtime>::pure_account(
						&alice,
						&ProxyType::Staking,
						index,
						None,
					),
					&pure_proxy
				);

				// Send some money to pure proxy
				let call = RuntimeCall::Balances(pallet_balances::Call::transfer_keep_alive {
					dest: pure_proxy,
					value: 100 * UNIT,
				});
				assert_ok!(call.dispatch(RuntimeOrigin::signed(alice)));

				// Initially, there should be 1 proxy after creation
				assert_eq!(pallet_proxy::Pallet::<Runtime>::proxies(pure_proxy).0.len(), 1);

				// proxy can not make a transfer
				let transfer_amount = 10;

				let call =
					Box::new(RuntimeCall::Balances(pallet_balances::Call::transfer_keep_alive {
						dest: bob,
						value: transfer_amount,
					}));

				assert_ok!(Proxy::proxy(
					RuntimeOrigin::signed(alice),
					pure_proxy,
					Some(ProxyType::Staking),
					call
				));
				System::assert_last_event(
					ProxyEvent::ProxyExecuted {
						result: Err(frame_system::Error::<Runtime>::CallFiltered.into()),
					}
					.into(),
				);
			});
	}

	#[test]
	fn proxy_staking_should_not_be_able_to_evm_create() {
		let delay = 0;
		let index = 0;
		let alice = AccountId::from_str(ALICE).unwrap();
		let bob = AccountId::from_str(BOB).unwrap();

		ExtBuilder::default()
			.with_balances(vec![(alice, 1000 * UNIT), (bob, 1000 * UNIT)])
			.build()
			.execute_with(|| {
				frame_system::Pallet::<Runtime>::set_block_number(1);
				let call = RuntimeCall::Proxy(pallet_proxy::Call::create_pure {
					proxy_type: ProxyType::Staking,
					index, // index
					delay, // delay
				});
				assert_ok!(call.dispatch(RuntimeOrigin::signed(alice)));

				// Get pure proxy address from event
				let events = frame_system::Pallet::<Runtime>::events();
				let pure_proxy = match events.last().unwrap().event {
					RuntimeEvent::Proxy(pallet_proxy::Event::PureCreated { pure, .. }) => pure,
					_ => panic!("unexpected event"),
				};

				assert_eq!(
					&pallet_proxy::Pallet::<Runtime>::pure_account(
						&alice,
						&ProxyType::Staking,
						index,
						None,
					),
					&pure_proxy
				);

				// proxy can not call evm create
				let call = Box::new(RuntimeCall::EVM(pallet_evm::Call::create {
					source: H160::from(alice.0),
					init: vec![],
					gas_limit: 100_000,
					max_fee_per_gas: U256::from(100_000),
					max_priority_fee_per_gas: None,
					nonce: None,
					access_list: vec![],
					value: U256::zero(),
				}));

				assert_ok!(Proxy::proxy(
					RuntimeOrigin::signed(alice),
					pure_proxy,
					Some(ProxyType::Staking),
					call
				));

				System::assert_last_event(
					ProxyEvent::ProxyExecuted {
						result: Err(frame_system::Error::<Runtime>::CallFiltered.into()),
					}
					.into(),
				);
			});
	}

	#[test]
	fn proxy_staking_should_not_be_able_to_evm_call() {
		let delay = 0;
		let index = 0;
		let alice = AccountId::from_str(ALICE).unwrap();
		let bob = AccountId::from_str(BOB).unwrap();

		ExtBuilder::default()
			.with_balances(vec![(alice, 1000 * UNIT), (bob, 1000 * UNIT)])
			.build()
			.execute_with(|| {
				frame_system::Pallet::<Runtime>::set_block_number(1);
				let call = RuntimeCall::Proxy(pallet_proxy::Call::create_pure {
					proxy_type: ProxyType::Staking,
					index, // index
					delay, // delay
				});
				assert_ok!(call.dispatch(RuntimeOrigin::signed(alice)));

				// Get pure proxy address from event
				let events = frame_system::Pallet::<Runtime>::events();
				let pure_proxy = match events.last().unwrap().event {
					RuntimeEvent::Proxy(pallet_proxy::Event::PureCreated { pure, .. }) => pure,
					_ => panic!("unexpected event"),
				};

				assert_eq!(
					&pallet_proxy::Pallet::<Runtime>::pure_account(
						&alice,
						&ProxyType::Staking,
						index,
						None,
					),
					&pure_proxy
				);

				// proxy can not call evm
				let call = Box::new(RuntimeCall::EVM(pallet_evm::Call::call {
					source: H160::from(alice.0),
					target: H160([0x2; 20]),
					value: U256::zero(),
					input: vec![],
					gas_limit: 100_000,
					max_fee_per_gas: U256::from(100_000),
					max_priority_fee_per_gas: None,
					nonce: None,
					access_list: vec![],
				}));

				assert_ok!(Proxy::proxy(
					RuntimeOrigin::signed(alice),
					pure_proxy,
					Some(ProxyType::Staking),
					call
				));

				System::assert_last_event(
					ProxyEvent::ProxyExecuted {
						result: Err(frame_system::Error::<Runtime>::CallFiltered.into()),
					}
					.into(),
				);
			});
	}

	#[test]
	fn proxy_staking_should_not_be_able_to_evm_withdraw() {
		let delay = 0;
		let index = 0;
		let alice = AccountId::from_str(ALICE).unwrap();
		let bob = AccountId::from_str(BOB).unwrap();

		ExtBuilder::default()
			.with_balances(vec![(alice, 1000 * UNIT), (bob, 1000 * UNIT)])
			.build()
			.execute_with(|| {
				frame_system::Pallet::<Runtime>::set_block_number(1);
				let call = RuntimeCall::Proxy(pallet_proxy::Call::create_pure {
					proxy_type: ProxyType::Staking,
					index, // index
					delay, // delay
				});
				assert_ok!(call.dispatch(RuntimeOrigin::signed(alice)));

				// Get pure proxy address from event
				let events = frame_system::Pallet::<Runtime>::events();
				let pure_proxy = match events.last().unwrap().event {
					RuntimeEvent::Proxy(pallet_proxy::Event::PureCreated { pure, .. }) => pure,
					_ => panic!("unexpected event"),
				};

				assert_eq!(
					&pallet_proxy::Pallet::<Runtime>::pure_account(
						&alice,
						&ProxyType::Staking,
						index,
						None,
					),
					&pure_proxy
				);

				// proxy can not call evm withdraw
				let call = Box::new(RuntimeCall::EVM(pallet_evm::Call::withdraw {
					address: H160([0x2; 20]),
					value: 0,
				}));

				assert_ok!(Proxy::proxy(
					RuntimeOrigin::signed(alice),
					pure_proxy,
					Some(ProxyType::Staking),
					call
				));

				System::assert_last_event(
					ProxyEvent::ProxyExecuted {
						result: Err(frame_system::Error::<Runtime>::CallFiltered.into()),
					}
					.into(),
				);
			});
	}

	#[test]
	fn proxy_staking_should_not_be_able_to_ethereum_transact() {
		let delay = 0;
		let index = 0;
		let alice = AccountId::from_str(ALICE).unwrap();
		let bob = AccountId::from_str(BOB).unwrap();

		ExtBuilder::default()
			.with_balances(vec![(alice, 1000 * UNIT), (bob, 1000 * UNIT)])
			.build()
			.execute_with(|| {
				frame_system::Pallet::<Runtime>::set_block_number(1);
				let call = RuntimeCall::Proxy(pallet_proxy::Call::create_pure {
					proxy_type: ProxyType::Staking,
					index, // index
					delay, // delay
				});
				assert_ok!(call.dispatch(RuntimeOrigin::signed(alice)));

				// Get pure proxy address from event
				let events = frame_system::Pallet::<Runtime>::events();
				let pure_proxy = match events.last().unwrap().event {
					RuntimeEvent::Proxy(pallet_proxy::Event::PureCreated { pure, .. }) => pure,
					_ => panic!("unexpected event"),
				};

				assert_eq!(
					&pallet_proxy::Pallet::<Runtime>::pure_account(
						&alice,
						&ProxyType::Staking,
						index,
						None,
					),
					&pure_proxy
				);

				// proxy can not call ethereum transact
				let call = Box::new(RuntimeCall::Ethereum(pallet_ethereum::Call::transact {
					transaction: pallet_ethereum::Transaction::Legacy(
						ethereum::LegacyTransaction {
							nonce: U256::zero(),
							gas_price: U256::zero(),
							gas_limit: U256::from(100_000),
							action: ethereum::TransactionAction::Call(H160::zero()),
							value: U256::zero(),
							input: vec![],
							signature: ethereum::TransactionSignature::new(
								123,
								H256::from_low_u64_be(1),
								H256::from_low_u64_be(2),
							)
							.unwrap(),
						},
					),
				}));
				assert_ok!(Proxy::proxy(
					RuntimeOrigin::signed(alice),
					pure_proxy,
					Some(ProxyType::Staking),
					call
				));

				System::assert_last_event(
					ProxyEvent::ProxyExecuted {
						result: Err(frame_system::Error::<Runtime>::CallFiltered.into()),
					}
					.into(),
				);
			});
	}

	#[test]
	fn proxy_staking_is_always_three() {
		assert_eq!(ProxyType::Staking as u8, 3);
	}
}
