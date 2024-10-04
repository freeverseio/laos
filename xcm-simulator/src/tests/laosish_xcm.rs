use super::*;

#[test]
fn alish_has_tokens() {
	MockNet::reset();

	Laosish::execute_with(|| {
		assert_eq!(
			pallet_balances::Pallet::<laosish::Runtime>::free_balance(&ALITH.into()),
			INITIAL_BALANCE
		);
	});
}

#[test]
fn ump_laosish() {
	MockNet::reset();

	let remark = relay_chain::RuntimeCall::System(
		frame_system::Call::<relay_chain::Runtime>::remark_with_event { remark: vec![1, 2, 3] },
	);
	Laosish::execute_with(|| {
		assert_ok!(LaosishPalletXcm::send_xcm(
			Here,
			Parent,
			Xcm(vec![Transact {
				origin_kind: OriginKind::SovereignAccount,
				require_weight_at_most: Weight::from_parts(INITIAL_BALANCE as u64, 1024 * 1024),
				call: remark.encode().into(),
			}]),
		));
	});

	Relay::execute_with(|| {
		use relay_chain::{RuntimeEvent, System};
		assert!(System::events().iter().any(|r| matches!(
			r.event,
			RuntimeEvent::System(frame_system::Event::Remarked { .. })
		)));
	});
}
