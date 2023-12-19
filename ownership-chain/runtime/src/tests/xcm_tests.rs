//! Cross-chain asset transfer tests
//!
//! The scenario for these tests is as follows:
//!
//! - LaosPara is our parachain's runtime.
//!
//! - OtherPara is another parachain. We assume that another parachain we are interacting with is
//!   configured with a minimum of
//! `Assets` and `XTokens` pallets.

use super::xcm_mock::{parachain::Assets, LaosPara, MockNet, OtherPara, Relay};
use crate::{
	tests::xcm_mock::{
		parachain, LaosParachainBalances, LaosParachainXcm, ParachainXtokens, RelayChainPalletXcm,
		ALITH, BOBTH, INITIAL_BALANCE,
	},
	UNIT,
};
use cumulus_primitives_core::{
	Fungibility::Fungible,
	Instruction::{BuyExecution, Transact, WithdrawAsset},
	Junction::{AccountKey20, Parachain},
	Junctions::{Here, X1, X2},
	MultiAsset, MultiLocation,
	WeightLimit::Unlimited,
	Xcm,
};
use frame_support::{assert_ok, traits::fungibles::Inspect, weights::Weight};
use parity_scale_codec::Encode;
use staging_xcm::v3;
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
					origin_kind: v3::OriginKind::SovereignAccount,
					require_weight_at_most: Weight::from_parts(1_000_000_000, 1024 * 1024),
					call: remark.encode().into(),
				}
			]),
		));
	});

	// Execute remote transact and verify that `Remarked` event is emitted.
	LaosPara::execute_with(|| {
		use crate::{RuntimeEvent, System};

		for event in System::events() {
			println!("{:?}", event.event);
		}
		assert!(System::events().iter().any(|r| matches!(
			r.event,
			RuntimeEvent::System(frame_system::Event::Remarked { .. })
		)));
	});
}

#[test]
fn para_to_para_native_transfer_and_back() {
	MockNet::reset();

	// in OtherPara, we need to set up and register the token of LaosPara.
	// we do this by sending a XCM message to OtherPara
	LaosPara::execute_with(|| {
		let create_asset =
			parachain::RuntimeCall::Assets(pallet_assets::Call::<parachain::Runtime>::create {
				id: 2,
				admin: ALITH,
				min_balance: UNIT,
			});

		// we only need to create the asset, registering location and setting unit price
		// per second is not necessary, since it is already mocked by
		assert_ok!(LaosParachainXcm::send(
			crate::RuntimeOrigin::root(),
			Box::new(MultiLocation { parents: 1, interior: X1(Parachain(2)) }.into()),
			Box::new(staging_xcm::VersionedXcm::V3(Xcm(vec![
				WithdrawAsset(
					vec![MultiAsset { id: MultiLocation::here().into(), fun: Fungible(UNIT) }]
						.into()
				),
				BuyExecution {
					fees: MultiAsset { id: MultiLocation::here().into(), fun: Fungible(UNIT) },
					weight_limit: Unlimited,
				},
				Transact {
					origin_kind: v3::OriginKind::SovereignAccount,
					require_weight_at_most: Weight::from_parts(1_000_000_000, 1024 * 1024),
					call: create_asset.encode().into(),
				},
			]
			.into())))
		));
	});

	OtherPara::execute_with(|| {
		use parachain::{RuntimeEvent, System};
		// check that asset is created
		assert_eq!(Assets::total_balance(1, &ALITH), 0);

		assert!(System::events().iter().any(|r| {
			matches!(
				r.event,
				RuntimeEvent::XcmpQueue(cumulus_pallet_xcmp_queue::Event::Success { .. })
			)
		}));
		assert!(System::events().iter().any(|r| {
			matches!(r.event, RuntimeEvent::Assets(pallet_assets::Event::Created { .. }))
		}));
	});

	let amount = 100 * UNIT;
	LaosPara::execute_with(|| {
		// bob in OtherPara
		let destination = MultiLocation { parents: 1, interior: X1(Parachain(2)) };

		assert_ok!(LaosParachainXcm::limited_reserve_transfer_assets(
			crate::RuntimeOrigin::signed(ALITH.0.into()),
			Box::new(destination.into()),
			Box::new(
				MultiLocation {
					parents: 0,
					interior: X1(AccountKey20 { network: None, key: BOBTH.0 })
				}
				.into(),
			),
			Box::new(
				vec![MultiAsset { id: MultiLocation::here().into(), fun: Fungible(amount) }.into()]
					.into()
			),
			0,
			Unlimited,
		));
	});

	OtherPara::execute_with(|| {
		use parachain::{RuntimeEvent, System};

		assert!(System::events().iter().any(|r| {
			matches!(
				r.event,
				RuntimeEvent::XcmpQueue(cumulus_pallet_xcmp_queue::Event::Success { .. })
			)
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
			interior: X2(Parachain(1), AccountKey20 { network: None, key: ALITH.0 }),
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

		// all the asset should, in theory, be transferred back
		assert_eq!(Assets::total_issuance(2), 0);
	});

	LaosPara::execute_with(|| {
		use crate::{RuntimeEvent, System};

		assert!(System::events().iter().any(|r| {
			matches!(
				r.event,
				RuntimeEvent::XcmpQueue(cumulus_pallet_xcmp_queue::Event::Success { .. })
			)
		}));

		assert!(System::events().iter().any(|r| {
			matches!(r.event, RuntimeEvent::Balances(pallet_balances::Event::Deposit { .. }))
		}));

		// calculate the final balance of ALITH
		let rounded_balance = LaosParachainBalances::free_balance(&ALITH.0.into()) / UNIT * UNIT;

		assert_eq!(rounded_balance, INITIAL_BALANCE - UNIT);
	});
}
