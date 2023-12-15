//! Cross-chain asset transfer tests

use super::xcm_mock::{parachain::Assets, MockNet, ParaA, ParaB, Relay};
use crate::{
	tests::xcm_mock::{
		msg_queue::mock_msg_queue, parachain, ParachainBalances, ParachainXtokens,
		RelayChainPalletXcm, ALITH, BOBTH, INITIAL_BALANCE,
	},
	UNIT,
};
use cumulus_primitives_core::{
	Fungibility::Fungible,
	Instruction::{BuyExecution, Transact, WithdrawAsset},
	Junction::{AccountKey20, Parachain},
	Junctions::{Here, X1, X2},
	MultiAsset, MultiLocation, NetworkId, OriginKind,
	WeightLimit::Unlimited,
	Xcm,
};
use frame_support::{assert_ok, traits::fungibles::Inspect, weights::Weight};
use parity_scale_codec::Encode;
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
						fun: cumulus_primitives_core::Fungibility::Fungible(UNIT)
					}]
					.into()
				),
				BuyExecution {
					fees: MultiAsset {
						id: MultiLocation::here().into(),
						fun: cumulus_primitives_core::Fungibility::Fungible(UNIT)
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
fn para_to_para_native_transfer_and_back() {
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
		use parachain::{RuntimeEvent, System};

		assert!(System::events().iter().any(|r| {
			matches!(r.event, RuntimeEvent::MsgQueue(mock_msg_queue::Event::Success(_)))
		}));

		assert!(System::events().iter().any(|r| {
			matches!(r.event, RuntimeEvent::Assets(pallet_assets::Event::Issued { .. }))
		}));

		// try to round up the amount to the nearest second, since we don't know how much fee was
		// charged
		let rounded_balance = Assets::total_balance(2, &BOBTH) / UNIT * UNIT;

		// we have 1 UNIT of tolerance, i.e at most 1 UNIT of was charged
		assert_eq!(rounded_balance, amount - UNIT);

		// now transfer back some of the tokens
		let destination = MultiLocation {
			parents: 1,
			interior: X2(
				Parachain(1),
				AccountKey20 { network: Some(NetworkId::Kusama), key: ALITH.0 },
			),
		};

		assert_ok!(ParachainXtokens::transfer_multiasset(
			parachain::RuntimeOrigin::signed(BOBTH),
			Box::new(
				MultiAsset {
					id: MultiLocation { parents: 1, interior: X1(Parachain(1)) }.into(),
					fun: Fungible(Assets::total_balance(2, &BOBTH))
				}
				.into()
			),
			Box::new(destination.into()),
			Unlimited,
		));
	});

	ParaA::execute_with(|| {
		use parachain::{RuntimeEvent, System};

		assert!(System::events().iter().any(|r| {
			matches!(r.event, RuntimeEvent::MsgQueue(mock_msg_queue::Event::Success(_)))
		}));

		assert!(System::events().iter().any(|r| {
			matches!(r.event, RuntimeEvent::Balances(pallet_balances::Event::Deposit { .. }))
		}));

		// calculate the final balance of ALITH
		let rounded_balance = ParachainBalances::free_balance(&ALITH) / UNIT * UNIT;

		assert_eq!(rounded_balance, INITIAL_BALANCE - UNIT);
	});
}
