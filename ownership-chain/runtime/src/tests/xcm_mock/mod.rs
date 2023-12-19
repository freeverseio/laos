//! Mock types for testing XCM.

pub(crate) mod parachain;
pub(crate) mod relay_chain;

use sp_core::H160;
use sp_runtime::BuildStorage;
use staging_xcm::latest::prelude::*;
use staging_xcm_executor::traits::ConvertLocation;
use xcm_simulator::{decl_test_network, decl_test_parachain, decl_test_relay_chain, TestExt};

use crate::Runtime as RealParachainRuntime;
use parachain::Runtime as MockParachainRuntime;
use relay_chain::Runtime as MockRelayChainRuntime;

use crate::UNIT;

pub const ALICE: sp_runtime::AccountId32 = sp_runtime::AccountId32::new([0xFAu8; 32]);
pub const ALITH: H160 = H160([0xFAu8; 20]);
pub const BOBTH: H160 = H160([0xFBu8; 20]);

pub const INITIAL_BALANCE: u128 = 1_000_000 * UNIT;

decl_test_parachain! {
	pub struct LaosPara {
		Runtime = RealParachainRuntime,
		XcmpMessageHandler = cumulus_pallet_xcmp_queue::Pallet<RealParachainRuntime>,
		DmpMessageHandler = cumulus_pallet_dmp_queue::Pallet<RealParachainRuntime>,
		new_ext = para_ext::<RealParachainRuntime>(1),
	}
}

decl_test_parachain! {
	pub struct OtherPara {
		Runtime = MockParachainRuntime,
		XcmpMessageHandler = cumulus_pallet_xcmp_queue::Pallet<MockParachainRuntime>,
		DmpMessageHandler = cumulus_pallet_dmp_queue::Pallet<MockParachainRuntime>,
		new_ext = para_ext::<MockParachainRuntime>(2),
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
			(1, LaosPara),
			(2, OtherPara),
		],
	}
}

pub type RelayChainPalletXcm = pallet_xcm::Pallet<MockRelayChainRuntime>;
pub type ParachainXtokens = orml_xtokens::Pallet<MockParachainRuntime>;
pub type LaosParachainXcm = pallet_xcm::Pallet<RealParachainRuntime>;
pub type LaosParachainBalances = pallet_balances::Pallet<RealParachainRuntime>;
pub type OtherParachainBalances = pallet_balances::Pallet<MockParachainRuntime>;

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
pub fn para_ext<Runtime>(para_id: u32) -> sp_io::TestExternalities
where
	Runtime: pallet_balances::Config + parachain_info::Config + pallet_xcm::Config,
	Runtime::AccountId: From<H160> + Into<H160>,
	Runtime::Balance: From<u128> + Into<u128>,
{
	let mut t = frame_system::GenesisConfig::<Runtime>::default().build_storage().unwrap();

	pallet_balances::GenesisConfig::<Runtime> {
		balances: vec![
			(ALITH.into(), INITIAL_BALANCE.into()),
			(BOBTH.into(), INITIAL_BALANCE.into()),
			(parent_account_id().into(), INITIAL_BALANCE.into()),
			(sibling_para_account_id(1).into(), INITIAL_BALANCE.into()),
			(sibling_para_account_id(2).into(), INITIAL_BALANCE.into()),
		],
	}
	.assimilate_storage(&mut t)
	.unwrap();

	parachain_info::GenesisConfig::<Runtime> {
		_config: Default::default(),
		parachain_id: para_id.into(),
	}
	.assimilate_storage(&mut t)
	.unwrap();

	pallet_xcm::GenesisConfig::<Runtime> { _config: Default::default(), safe_xcm_version: Some(3) }
		.assimilate_storage(&mut t)
		.unwrap();

	let mut ext = sp_io::TestExternalities::new(t);

	ext.execute_with(|| {
		parachain::System::set_block_number(1);
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

	pallet_xcm::GenesisConfig::<MockRelayChainRuntime> {
		_config: Default::default(),
		safe_xcm_version: Some(3),
	}
	.assimilate_storage(&mut t)
	.unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| relay_chain::System::set_block_number(1));
	ext
}
