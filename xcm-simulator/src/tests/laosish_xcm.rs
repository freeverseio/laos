use super::*;

#[test]
fn alish_has_tokens() {
	MockNet::reset();

	Laosish::execute_with(|| {
		let alith: laosish::AccountId = ALITH.into();
		assert_eq!(
			pallet_balances::Pallet::<laosish::Runtime>::free_balance(alith),
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

#[test]
fn dmp_laosish_of_remark_tx_should_be_unallowed() {
	MockNet::reset();

	let remark =
		laosish::RuntimeCall::System(frame_system::Call::<laosish::Runtime>::remark_with_event {
			remark: vec![1, 2, 3],
		});
	Relay::execute_with(|| {
		assert_ok!(RelayChainPalletXcm::send_xcm(
			Here,
			Parachain(PARA_LAOSISH_ID),
			Xcm(vec![Transact {
				origin_kind: OriginKind::SovereignAccount,
				require_weight_at_most: Weight::from_parts(INITIAL_BALANCE as u64, 1024 * 1024),
				call: remark.encode().into(),
			}]),
		));
	});

	Laosish::execute_with(|| {
		use laosish::{RuntimeEvent, System};
		assert!(
			!System::events().iter().any(|r| matches!(
				r.event,
				RuntimeEvent::System(frame_system::Event::Remarked { .. })
			)),
			"Unexpected Remark event found"
		);
	});
}
