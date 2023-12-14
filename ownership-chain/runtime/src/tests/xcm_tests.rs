//! Cross-chain asset transfer tests

use super::{
	new_test_ext,
	xcm_mock::{MockNet, ParaA, Relay},
};
use crate::tests::xcm_mock::{parachain, RelayChainPalletXcm};
use cumulus_primitives_core::{
	AssetId,
	Instruction::{BuyExecution, Transact, WithdrawAsset},
	Junction::Parachain,
	Junctions::Here,
	MultiAsset, MultiLocation, OriginKind,
	WeightLimit::Unlimited,
	Xcm,
};
use frame_support::{assert_ok, weights::Weight};
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
						fun: cumulus_primitives_core::Fungibility::Fungible(
							1_000_000_000_000_000_000_000_000
						)
					}]
					.into()
				),
				BuyExecution {
					fees: MultiAsset {
						id: MultiLocation::here().into(),
						fun: cumulus_primitives_core::Fungibility::Fungible(
							1_000_000_000_000_000_000_000_000
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
