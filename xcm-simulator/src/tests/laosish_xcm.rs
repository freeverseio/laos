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

#[test]
fn ump_transfer_balance() {
	MockNet::reset();

	let amount = 1;

	let transfer = relay_chain::RuntimeCall::Balances(pallet_balances::Call::<
		relay_chain::Runtime,
	>::transfer_keep_alive {
		dest: ALICE,
		value: amount,
	});

	Laosish::execute_with(|| {
		assert_ok!(LaosishPalletXcm::send_xcm(
			Here,
			Parent,
			Xcm(vec![Transact {
				origin_kind: OriginKind::SovereignAccount,
				require_weight_at_most: Weight::from_parts(INITIAL_BALANCE as u64, 1024 * 1024),
				call: transfer.encode().into(),
			}]),
		));
	});

	// Check that transfer was executed
	Relay::execute_with(|| {
		assert_eq!(relay_chain::Balances::free_balance(ALICE), INITIAL_BALANCE + amount);
		assert_eq!(
			relay_chain::Balances::free_balance(child_account_id(PARA_LAOSISH_ID)),
			INITIAL_BALANCE - amount
		);
	});
}

#[test]
fn xcmp_create_foreign_asset() {
	MockNet::reset();

	let para_a_native_asset_location =
		xcm::v3::Location::new(1, [xcm::v3::Junction::Parachain(PARA_LAOSISH_ID)]);

	let create_asset = parachain::RuntimeCall::ForeignAssets(ForeignAssetsCall::create {
		id: para_a_native_asset_location,
		admin: sibling_account_id(PARA_LAOSISH_ID),
		min_balance: 1000,
	});

	Laosish::execute_with(|| {
		assert_ok!(LaosishPalletXcm::send_xcm(
			Here,
			(Parent, Parachain(PARA_B_ID)),
			Xcm(vec![Transact {
				origin_kind: OriginKind::Xcm,
				require_weight_at_most: Weight::from_parts(INITIAL_BALANCE as u64, 1024 * 1024),
				call: create_asset.encode().into(),
			}]),
		));
	});

	ParaB::execute_with(|| {
		assert!(parachain::System::events().iter().any(|r| matches!(
			r.event,
			parachain::RuntimeEvent::ForeignAssets(pallet_assets::Event::Created { .. })
		)));
	});
	let withdraw_amount = 123;

	Laosish::execute_with(|| {
		// let destination = xcm::v3::Location::new(1, [xcm::v3::Junction::Parachain(PARA_B_ID)]); // destination (AssetHub) from source (Laos)
		// let beneficiary =  xcm::v3::Location::new(0, [xcm::v3::Junction::AccountId32 { network: None, id: ALICE.into() }]); // Address from (AssetHub)
		assert_ok!(LaosishPalletXcm::limited_reserve_transfer_assets(
			laosish::RuntimeOrigin::signed(ALITH.into()),
			Box::new((Parent, Parachain(PARA_B_ID)).into()),
			Box::new(AccountId32 { network: None, id: ALICE.into() }.into()),
			Box::new(vec![xcm::v3::MultiAsset {id: AssetId(Here.into()).into(), fun: xcm::v3::Fungibility::Fungible(withdraw_amount)}.into()].into()),
			// Fee asset item index: 0 (no specific fee asset)
			0,
			// Weight limit for execution: Unlimited
			Unlimited,
		));
		// Assert that the para B sovereign account has increased by the transferred amount
		assert_eq!(
			parachain::Balances::free_balance(sibling_account_id(PARA_B_ID)),
			INITIAL_BALANCE + withdraw_amount
		);
	});

	ParaB::execute_with(|| {
		// Verify that ALICE's balance on Parachain A has increased by the transferred amount
		assert_eq!(
			pallet_balances::Pallet::<parachain::Runtime>::free_balance(&ALICE),
			INITIAL_BALANCE + withdraw_amount
		);
	});
}
