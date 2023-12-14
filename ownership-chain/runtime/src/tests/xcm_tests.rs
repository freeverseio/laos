//! Cross-chain asset transfer tests

use super::xcm_mock::{parachain::Assets, MockNet, ParaA, ParaB, Relay};
use crate::{
	tests::xcm_mock::{
		msg_queue::mock_msg_queue, parachain, ParachainXtokens, RelayChainPalletXcm, ALITH, BOBTH,
	},
	UNIT,
};
use cumulus_primitives_core::{
	Instruction::{BuyExecution, Transact, WithdrawAsset},
	Junction::{AccountKey20, Parachain},
	Junctions::{Here, X2},
	MultiAsset, MultiLocation,
	NetworkId::{self, Kusama},
	OriginKind,
	WeightLimit::Unlimited,
	Xcm,
};
use frame_support::{assert_ok, traits::fungibles::Inspect, weights::Weight};
use pallet_authorship::pallet;
use parity_scale_codec::Encode;
use precompile_utils::assert_event_emitted;
use xcm_simulator::TestExt;

/// Test downward message passing. Does some basic remark in the parachain from the relay chain.
#[test]
fn basic_dmp() {
	MockNet::reset();

	let remark = parachain::RuntimeCall::System(
		frame_system::Call::<parachain::Runtime>::remark_with_event { remark: vec![1, 2, 3] },
	);

	Relay::execute_with(|| {
		assert_ok!(RelayChainPalletXcm::send_xcm(
			Here,
			Parachain(1),
			Xcm(vec![
				WithdrawAsset(
					vec![MultiAsset {
						id: MultiLocation::here().into(),
						fun: cumulus_primitives_core::Fungibility::Fungible(
							1_000_000_000_000_000_000
						)
					}]
					.into()
				),
				BuyExecution {
					fees: MultiAsset {
						id: MultiLocation::here().into(),
						fun: cumulus_primitives_core::Fungibility::Fungible(
							1_000_000_000_000_000_000
						)
					},
					weight_limit: Unlimited,
				},
				Transact {
					origin_kind: OriginKind::SovereignAccount,
					require_weight_at_most: Weight::from_parts(1_000_000_000, 1024 * 1024),
					call: remark.encode().into(),
				}
			]),
		));
	});

	// Execute remote transact and verify that `Remarked` event is emitted.
	ParaA::execute_with(|| {
		use parachain::{RuntimeEvent, System};
		assert!(System::events().iter().any(|r| matches!(
			r.event,
			RuntimeEvent::System(frame_system::Event::Remarked { .. })
		)));
	});
}

#[test]
fn test_reserve_asset_transfer() {
	MockNet::reset();

	// in ParaB, we need to set up and register the token of ParaA
	ParaB::execute_with(|| {
		// we only need to create the asset, registering location and setting unit price
		// per second is not necessary, since it is already mocked by
		assert_ok!(Assets::force_create(
			parachain::RuntimeOrigin::root(),
			2_u32,
			ALITH,
			false,
			1 * UNIT
		));
	});

	// let mut used_weight = Weight::default();

	let amount = 100 * UNIT;
	ParaA::execute_with(|| {
		// bob in ParaB
		let destination = MultiLocation {
			parents: 1,
			interior: X2(
				Parachain(2),
				AccountKey20 { network: Some(NetworkId::Kusama), key: BOBTH.0 },
			),
		};

		assert_ok!(ParachainXtokens::transfer(
			parachain::RuntimeOrigin::signed(ALITH),
			0,
			amount,
			Box::new(destination.into()),
			Unlimited,
		));
	});

	ParaB::execute_with(|| {
		use parachain::{Assets, RuntimeEvent, System};

		assert!(System::events().iter().any(|r| {
			matches!(r.event, RuntimeEvent::MsgQueue(mock_msg_queue::Event::Success(_)))
		}));

		assert!(System::events().iter().any(|r| {
			matches!(r.event, RuntimeEvent::Assets(pallet_assets::Event::Issued { .. }))
		}));
	});
}
