//! Mock types for testing XCM.

pub(crate) mod msg_queue;
pub(crate) mod parachain;
pub(crate) mod relay_chain;

use sp_core::H160;
use sp_runtime::BuildStorage;
use staging_xcm::latest::prelude::*;
use staging_xcm_executor::traits::ConvertLocation;
use xcm_simulator::{decl_test_network, decl_test_parachain, decl_test_relay_chain, TestExt};

use parachain::Runtime as MockParachainRuntime;
use relay_chain::Runtime as MockRelayChainRuntime;

use crate::UNIT;

pub const ALICE: sp_runtime::AccountId32 = sp_runtime::AccountId32::new([0xFAu8; 32]);
pub const ALITH: H160 = H160([0xFAu8; 20]);
pub const BOBTH: H160 = H160([0xFBu8; 20]);

pub const INITIAL_BALANCE: u128 = 1_000_000 * UNIT;

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
pub type ParachainXtokens = orml_xtokens::Pallet<MockParachainRuntime>;
pub type ParachainXcm = pallet_xcm::Pallet<MockParachainRuntime>;
pub type ParachainBalances = pallet_balances::Pallet<MockParachainRuntime>;

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

/// Prepare parachain test externality
pub fn para_ext(para_id: u32) -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::<MockParachainRuntime>::default()
		.build_storage()
		.unwrap();

	pallet_balances::GenesisConfig::<MockParachainRuntime> {
		balances: vec![
			(ALITH, INITIAL_BALANCE),
			(BOBTH, INITIAL_BALANCE),
			(parent_account_id(), INITIAL_BALANCE),
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
