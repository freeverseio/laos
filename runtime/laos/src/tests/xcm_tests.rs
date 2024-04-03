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

//! Cross-chain asset transfer tests
//!
//! The scenario for these tests is as follows:
//!
//! - LaosPara is our parachain's runtime.
//!
//! - OtherPara is another parachain. We assume that another parachain we are interacting with is
//!   configured with a minimum of
//! `Assets` and `XTokens` pallets.

use super::xcm_mock::{
	parachain::Assets, relay_chain::Runtime as MockRelayChainRuntime, LaosPara, MockNet, OtherPara,
	Relay,
};
use crate::{
	currency::UNIT,
	tests::xcm_mock::{
		parachain, LaosParachainBalances, LaosParachainXcm, ParachainXtokens, ALICE, ALITH, BOBTH,
		INITIAL_BALANCE,
	},
	Runtime, RuntimeOrigin,
};
use cumulus_primitives_core::{
	Fungibility::Fungible,
	Instruction::{BuyExecution, DepositAsset, Transact, WithdrawAsset},
	Junction::{AccountKey20, Parachain},
	Junctions::{X1, X2},
	MultiAsset,
	MultiAssetFilter::Wild,
	MultiLocation, Outcome,
	WeightLimit::Unlimited,
	WildMultiAsset::All,
	Xcm,
};

use frame_support::{assert_noop, assert_ok, traits::fungibles::Inspect, weights::Weight};
use frame_system::RawOrigin;
use parity_scale_codec::Encode;
use staging_xcm::v3;
use xcm_simulator::TestExt;

/// Utility function for transacting a call to other chain
///
/// - Buys 1 UNIT of execution weight
/// - Transacts the call
/// - Deposits surplus asset to ALITH
fn transact<Runtime: pallet_xcm::Config>(dest: MultiLocation, encoded_call: Vec<u8>) {
	let xcm_call = Xcm(vec![
		WithdrawAsset(
			vec![MultiAsset {
				id: MultiLocation::here().into(),
				fun: cumulus_primitives_core::Fungibility::Fungible(UNIT),
			}]
			.into(),
		),
		BuyExecution {
			fees: MultiAsset {
				id: MultiLocation::here().into(),
				fun: cumulus_primitives_core::Fungibility::Fungible(UNIT),
			},
			weight_limit: Unlimited,
		},
		Transact {
			origin_kind: v3::OriginKind::SovereignAccount,
			require_weight_at_most: Weight::from_parts(1_000_000_000, 1024 * 1024),
			call: encoded_call.into(),
		},
		DepositAsset {
			assets: Wild(All),
			beneficiary: MultiLocation {
				parents: 0,
				interior: AccountKey20 { network: None, key: ALITH.0 }.into(),
			}
			.into(),
		},
	]
	.into());

	assert_ok!(pallet_xcm::Pallet::<Runtime>::send(
		RawOrigin::Root.into(),
		Box::new(dest.into()),
		Box::new(staging_xcm::VersionedXcm::V3(xcm_call)),
	));
}

/// Test downward message passing. Does some basic remark in the parachain from the relay chain.
#[test]
fn basic_dmp() {
	MockNet::reset();

	let remark = parachain::RuntimeCall::System(
		frame_system::Call::<parachain::Runtime>::remark_with_event { remark: vec![1, 2, 3] },
	);

	Relay::execute_with(|| {
		transact::<MockRelayChainRuntime>(Parachain(1).into(), remark.encode().into());
	});

	// Execute remote transact and verify that `Remarked` event is emitted.
	LaosPara::execute_with(|| {
		use crate::{RuntimeEvent, System};

		assert!(System::events().iter().any(|r| matches!(
			r.event,
			RuntimeEvent::System(frame_system::Event::Remarked { .. })
		)));
	});
}

/// Registers LAOS native token in OtherPara and transfers it to OtherPara.
/// Then transfers it back to LAOS.
#[test]
fn laos_para_to_other_para_reserver_transfer_and_back() {
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
		transact::<crate::Runtime>(
			MultiLocation { parents: 1, interior: X1(Parachain(2)) },
			create_asset.encode(),
		);
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

#[test]
fn other_para_native_asset_reserve_transfer_fails() {
	MockNet::reset();

	// Laos parachain does not have any foreign assets concept, so any reserve transfer to us
	// should fail
	OtherPara::execute_with(|| {
		assert_ok!(ParachainXtokens::transfer(
			parachain::RuntimeOrigin::signed(BOBTH),
			0,
			10 * UNIT,
			Box::new(
				MultiLocation {
					parents: 1,
					interior: X2(Parachain(1), AccountKey20 { network: None, key: ALITH.0 })
				}
				.into()
			),
			Unlimited,
		),);
	});

	LaosPara::execute_with(|| {
		use crate::{RuntimeEvent, System};

		// the error is `TooExpensive` because we don't have `OtherPara` token
		// registered and set as an asset that can be paid for fees
		// i.e there is not enough payment asset to pay for the fees
		assert!(System::events().iter().any(|r| {
			matches!(
				r.event,
				RuntimeEvent::XcmpQueue(cumulus_pallet_xcmp_queue::Event::Fail {
					error: v3::Error::TooExpensive,
					..
				})
			)
		}));
	});
}

/// This tests following config values:
/// - `pallet_xcm::XcmExecuteFilter`
#[test]
fn local_xcm_execution_is_filtered_out() {
	LaosPara::execute_with(|| {
		assert_noop!(
			LaosParachainXcm::execute(
				RuntimeOrigin::root(),
				Box::new(staging_xcm::VersionedXcm::V3(Xcm(vec![
					WithdrawAsset(
						vec![MultiAsset { id: MultiLocation::here().into(), fun: Fungible(UNIT) }]
							.into(),
					),
					BuyExecution {
						fees: MultiAsset { id: MultiLocation::here().into(), fun: Fungible(UNIT) },
						weight_limit: Unlimited,
					},
					Transact {
						origin_kind: v3::OriginKind::SovereignAccount,
						require_weight_at_most: Weight::from_parts(1_000_000_000, 1024 * 1024),
						call: vec![].into(),
					},
					DepositAsset {
						assets: Wild(All),
						beneficiary: MultiLocation {
							parents: 0,
							interior: AccountKey20 { network: None, key: ALITH.0 }.into(),
						}
						.into(),
					},
				]))),
				Weight::from_parts(1_000_000_000, 1024 * 1024),
			),
			pallet_xcm::Error::<Runtime>::Filtered
		);
	});
}

/// This tests following config values:
/// - `staging_xcm_executor::IsTeleporter`
/// - `pallet_xcm::XcmTeleportFilter`
#[test]
fn teleport_is_disabled() {
	// try teleporting KSM from relay chain to parachain
	Relay::execute_with(|| {
		assert_ok!(pallet_xcm::Pallet::<MockRelayChainRuntime>::limited_teleport_assets(
			super::xcm_mock::relay_chain::RuntimeOrigin::signed(ALICE),
			Box::new(X1(Parachain(1)).into()),
			Box::new(
				MultiLocation {
					parents: 0,
					interior: AccountKey20 { network: None, key: ALITH.0 }.into(),
				}
				.into()
			),
			Box::new(
				vec![MultiAsset { id: MultiLocation::here().into(), fun: Fungible(UNIT) }.into()]
					.into()
			),
			0,
			Unlimited,
		),);
	});

	LaosPara::execute_with(|| {
		use crate::{RuntimeEvent, System};

		assert!(System::events().iter().any(|r| {
			matches!(
				r.event,
				RuntimeEvent::DmpQueue(cumulus_pallet_dmp_queue::Event::ExecutedDownward {
					outcome: Outcome::Incomplete(.., v3::Error::UntrustedTeleportLocation),
					..
				})
			)
		}));
	});
}
