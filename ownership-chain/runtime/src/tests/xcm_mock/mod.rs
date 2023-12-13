// This file is part of Astar.

// Copyright (C) 2019-2023 Stake Technologies Pte.Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later

// Astar is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Astar is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Astar. If not, see <http://www.gnu.org/licenses/>.

pub(crate) mod msg_queue;
pub(crate) mod parachain;
pub(crate) mod relay_chain;

use frame_support::traits::{Currency, IsType, OnFinalize, OnInitialize};
use sp_core::H160;
use sp_runtime::BuildStorage;
use staging_xcm::latest::prelude::*;
use staging_xcm_executor::traits::ConvertLocation;
use xcm_simulator::{decl_test_network, decl_test_parachain, decl_test_relay_chain, TestExt};

use parachain::Runtime as MockParachainRuntime;
use relay_chain::Runtime as MockRelayChainRuntime;

pub const ALICE: sp_runtime::AccountId32 = sp_runtime::AccountId32::new([0xFAu8; 32]);
pub const BOB: sp_runtime::AccountId32 = sp_runtime::AccountId32::new([0xFBu8; 32]);
pub const ALITH: H160 = H160::from([0xFAu8; 20]);
pub const BOBTH: H160 = H160::from([0xFBu8; 20]);

pub const INITIAL_BALANCE: u128 = 1_000_000_000_000_000_000_000_000;

decl_test_parachain! {
	pub struct ParaA {
		Runtime = MockParachainRuntime,
		XcmpMessageHandler = parachain::MsgQueue,
		DmpMessageHandler = parachain::MsgQueue,
		new_ext = para_ext(1),
	}
}

decl_test_parachain! {
	pub struct ParaB {
		Runtime = MockParachainRuntime,
		XcmpMessageHandler = parachain::MsgQueue,
		DmpMessageHandler = parachain::MsgQueue,
		new_ext = para_ext(2),
	}
}

decl_test_relay_chain! {
	pub struct Relay {
		Runtime = MockRelayChainRuntime,
		RuntimeCall = relay_chain::RuntimeCall,
		RuntimeEvent = relay_chain::RuntimeEvent,
		XcmConfig = relay_chain::XcmConfig,
		MessageQueue = relay_chain::MessageQueue,
		System = relay_chain::System,
		new_ext = relay_ext(),
	}
}

decl_test_network! {
	pub struct MockNet {
		relay_chain = Relay,
		parachains = vec![
			(1, ParaA),
			(2, ParaB),
		],
	}
}

pub type RelayChainPalletXcm = pallet_xcm::Pallet<MockRelayChainRuntime>;
pub type ParachainPalletXcm = pallet_xcm::Pallet<MockParachainRuntime>;
pub type ParachainBalances = pallet_balances::Pallet<MockParachainRuntime>;
pub type ParachainXtokens = orml_xtokens::Pallet<MockParachainRuntime>;

pub fn parent_account_id() -> parachain::AccountId {
	parachain::LocationToAccountId::convert_location(&MultiLocation { parents: 1, interior: Here })
		.unwrap()
}

/// Derive parachain sovereign account on relay chain, from parachain Id
pub fn child_para_account_id(para: u32) -> relay_chain::AccountId {
	relay_chain::LocationToAccountId::convert_location(&MultiLocation {
		parents: 0,
		interior: Junctions::X1(Parachain(para)),
	})
	.unwrap()
}

/// Derive parachain sovereign account on a sibling parachain, from parachain Id
pub fn sibling_para_account_id(para: u32) -> parachain::AccountId {
	parachain::LocationToAccountId::convert_location(&MultiLocation {
		parents: 1,
		interior: X1(Parachain(para)),
	})
	.unwrap()
}

// /// Derive parachain's account's account on a sibling parachain
// pub fn sibling_para_account_account_id(
// 	para: u32,
// 	who: sp_runtime::AccountId32,
// ) -> parachain::AccountId {
// 	let location = (
// 		Parent,
// 		Parachain(para),
// 		AccountId32 {
// 			// we have kusama as relay in mock
// 			network: Some(Kusama),
// 			id: who.into(),
// 		},
// 	);
// 	parachain::LocationToAccountId::convert_location(location.into()).unwrap()
// }

/// Prepare parachain test externality
pub fn para_ext(para_id: u32) -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::<MockParachainRuntime>::default()
		.build_storage()
		.unwrap();

	pallet_balances::GenesisConfig::<MockParachainRuntime> {
		balances: vec![
			(ALITH, INITIAL_BALANCE),
			// (sibling_para_account_account_id(1, ALITH), INITIAL_BALANCE),
			// (sibling_para_account_account_id(2, ALITH), INITIAL_BALANCE),
			(sibling_para_account_id(1), INITIAL_BALANCE),
			(sibling_para_account_id(2), INITIAL_BALANCE),
		],
	}
	.assimilate_storage(&mut t)
	.unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| {
		parachain::System::set_block_number(1);
		parachain::MsgQueue::set_para_id(para_id.into());
	});
	ext
}

/// Prepare relay chain test externality
pub fn relay_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::<MockRelayChainRuntime>::default()
		.build_storage()
		.unwrap();

	pallet_balances::GenesisConfig::<MockRelayChainRuntime> {
		balances: vec![
			(ALICE, INITIAL_BALANCE),
			(child_para_account_id(1), INITIAL_BALANCE),
			(child_para_account_id(2), INITIAL_BALANCE),
		],
	}
	.assimilate_storage(&mut t)
	.unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| relay_chain::System::set_block_number(1));
	ext
}

/// Advance parachain blocks until `block_number`.
/// No effect if parachain is already at that number or exceeds it.
pub fn advance_parachain_block_to(block_number: u64) {
	while parachain::System::block_number() < block_number {
		// On Finalize
		let current_block_number = parachain::System::block_number();
		parachain::PolkadotXcm::on_finalize(current_block_number);
		parachain::Balances::on_finalize(current_block_number);
		parachain::System::on_finalize(current_block_number);

		// Forward 1 block
		let current_block_number = current_block_number + 1;
		parachain::System::set_block_number(current_block_number);
		parachain::System::reset_events();

		// On Initialize
		parachain::System::on_initialize(current_block_number);
		parachain::Balances::on_initialize(current_block_number);
		parachain::PolkadotXcm::on_initialize(current_block_number);
	}
}
