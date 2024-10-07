use super::*;

use frame_support::{assert_ok, weights::Weight};
use parity_scale_codec::Encode;
use xcm::latest::QueryResponseInfo;
use xcm_simulator::TestExt;

mod laosish_xcm;

// Helper function for forming buy execution message
fn buy_execution<C>(fees: impl Into<Asset>) -> Instruction<C> {
	BuyExecution { fees: fees.into(), weight_limit: Unlimited }
}

#[test]
fn remote_account_ids_work() {
	child_account_account_id(1, ALICE);
	sibling_account_account_id(1, ALICE);
	parent_account_account_id(ALICE);
}

#[test]
fn dmp() {
	MockNet::reset();

	let remark = parachain::RuntimeCall::System(
		frame_system::Call::<parachain::Runtime>::remark_with_event { remark: vec![1, 2, 3] },
	);
	Relay::execute_with(|| {
		assert_ok!(RelayChainPalletXcm::send_xcm(
			Here,
			Parachain(1),
			Xcm(vec![Transact {
				origin_kind: OriginKind::SovereignAccount,
				require_weight_at_most: Weight::from_parts(INITIAL_BALANCE as u64, 1024 * 1024),
				call: remark.encode().into(),
			}]),
		));
	});

	ParaA::execute_with(|| {
		use parachain::{RuntimeEvent, System};
		assert!(System::events().iter().any(|r| matches!(
			r.event,
			RuntimeEvent::System(frame_system::Event::Remarked { .. })
		)));
	});
}

#[test]
fn ump() {
	MockNet::reset();

	let remark = relay_chain::RuntimeCall::System(
		frame_system::Call::<relay_chain::Runtime>::remark_with_event { remark: vec![1, 2, 3] },
	);
	ParaA::execute_with(|| {
		assert_ok!(ParachainPalletXcm::send_xcm(
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
fn xcmp() {
	MockNet::reset();

	let remark = parachain::RuntimeCall::System(
		frame_system::Call::<parachain::Runtime>::remark_with_event { remark: vec![1, 2, 3] },
	);
	ParaA::execute_with(|| {
		assert_ok!(ParachainPalletXcm::send_xcm(
			Here,
			(Parent, Parachain(2)),
			Xcm(vec![Transact {
				origin_kind: OriginKind::SovereignAccount,
				require_weight_at_most: Weight::from_parts(INITIAL_BALANCE as u64, 1024 * 1024),
				call: remark.encode().into(),
			}]),
		));
	});

	ParaB::execute_with(|| {
		use parachain::{RuntimeEvent, System};
		assert!(System::events().iter().any(|r| matches!(
			r.event,
			RuntimeEvent::System(frame_system::Event::Remarked { .. })
		)));
	});
}

#[test]
fn reserve_transfer() {
	// Reset the mock network to a clean state before the test
	MockNet::reset();

	let withdraw_amount = 123;

	// Execute actions within the Relay chain's context
	Relay::execute_with(|| {
		// Perform a limited reserve transfer of assets from ALICE to Parachain 1
		assert_ok!(RelayChainPalletXcm::limited_reserve_transfer_assets(
			relay_chain::RuntimeOrigin::signed(ALICE),
			// Destination: Parachain with ID 1
			Box::new(Parachain(1).into()),
			// Beneficiary: ALICE's account on the parachain
			Box::new(AccountId32 { network: None, id: ALICE.into() }.into()),
			// Assets to transfer: specified amount of the native currency
			Box::new((Here, withdraw_amount).into()),
			// Fee asset item index: 0 (no specific fee asset)
			0,
			// Weight limit for execution: Unlimited
			Unlimited,
		));
		// Assert that the relay chain's child account for Parachain 1 has the expected balance
		assert_eq!(
			relay_chain::Balances::free_balance(child_account_id(1)),
			INITIAL_BALANCE + withdraw_amount
		);
	});

	// Execute actions within Parachain A's context
	ParaA::execute_with(|| {
		// Verify that ALICE's balance on Parachain A has increased by the transferred amount
		assert_eq!(
			pallet_balances::Pallet::<parachain::Runtime>::free_balance(&ALICE),
			INITIAL_BALANCE + withdraw_amount
		);
	});
}

#[test]
fn remote_locking_and_unlocking() {
	MockNet::reset();

	let locked_amount = 100;

	ParaB::execute_with(|| {
		let message = Xcm(vec![LockAsset {
			asset: (Here, locked_amount).into(),
			unlocker: Parachain(1).into(),
		}]);
		assert_ok!(ParachainPalletXcm::send_xcm(Here, Parent, message.clone()));
	});

	Relay::execute_with(|| {
		use pallet_balances::{BalanceLock, Reasons};
		assert_eq!(
			relay_chain::Balances::locks(child_account_id(2)),
			vec![BalanceLock { id: *b"py/xcmlk", amount: locked_amount, reasons: Reasons::All }]
		);
	});

	ParaA::execute_with(|| {
		assert_eq!(
			parachain::MsgQueue::received_dmp(),
			vec![Xcm(vec![NoteUnlockable {
				owner: (Parent, Parachain(2)).into(),
				asset: (Parent, locked_amount).into()
			}])]
		);
	});

	ParaB::execute_with(|| {
		// Request unlocking part of the funds on the relay chain
		let message = Xcm(vec![RequestUnlock {
			asset: (Parent, locked_amount - 50).into(),
			locker: Parent.into(),
		}]);
		assert_ok!(ParachainPalletXcm::send_xcm(Here, (Parent, Parachain(1)), message));
	});

	Relay::execute_with(|| {
		use pallet_balances::{BalanceLock, Reasons};
		// Lock is reduced
		assert_eq!(
			relay_chain::Balances::locks(child_account_id(2)),
			vec![BalanceLock {
				id: *b"py/xcmlk",
				amount: locked_amount - 50,
				reasons: Reasons::All
			}]
		);
	});
}

/// Scenario:
/// A parachain transfers an NFT resident on the relay chain to another parachain account.
///
/// Asserts that the parachain accounts are updated as expected.
#[test]
fn withdraw_and_deposit_nft() {
	MockNet::reset();

	Relay::execute_with(|| {
		assert_eq!(relay_chain::Uniques::owner(1, 42), Some(child_account_id(1)));
	});

	ParaA::execute_with(|| {
		let message = Xcm(vec![TransferAsset {
			assets: (GeneralIndex(1), 42u32).into(),
			beneficiary: Parachain(2).into(),
		}]);
		// Send withdraw and deposit
		assert_ok!(ParachainPalletXcm::send_xcm(Here, Parent, message));
	});

	Relay::execute_with(|| {
		assert_eq!(relay_chain::Uniques::owner(1, 42), Some(child_account_id(2)));
	});
}

/// Scenario:
/// The relay-chain teleports an NFT to a parachain.
///
/// Asserts that the parachain accounts are updated as expected.
#[test]
fn teleport_nft() {
	MockNet::reset();

	Relay::execute_with(|| {
		// Mint the NFT (1, 69) and give it to our "parachain#1 alias".
		assert_ok!(relay_chain::Uniques::mint(
			relay_chain::RuntimeOrigin::signed(ALICE),
			1,
			69,
			child_account_account_id(1, ALICE),
		));
		// The parachain#1 alias of Alice is what must hold it on the Relay-chain for it to be
		// withdrawable by Alice on the parachain.
		assert_eq!(relay_chain::Uniques::owner(1, 69), Some(child_account_account_id(1, ALICE)));
	});
	ParaA::execute_with(|| {
		assert_ok!(parachain::ForeignUniques::force_create(
			parachain::RuntimeOrigin::root(),
			(Parent, GeneralIndex(1)).into(),
			ALICE,
			false,
		));
		assert_eq!(
			parachain::ForeignUniques::owner((Parent, GeneralIndex(1)).into(), 69u32.into()),
			None,
		);
		assert_eq!(parachain::Balances::reserved_balance(&ALICE), 0);

		// IRL Alice would probably just execute this locally on the Relay-chain, but we can't
		// easily do that here since we only send between chains.
		let message = Xcm(vec![
			WithdrawAsset((GeneralIndex(1), 69u32).into()),
			InitiateTeleport {
				assets: AllCounted(1).into(),
				dest: Parachain(1).into(),
				xcm: Xcm(vec![DepositAsset {
					assets: AllCounted(1).into(),
					beneficiary: (AccountId32 { id: ALICE.into(), network: None },).into(),
				}]),
			},
		]);
		// Send teleport
		let alice = AccountId32 { id: ALICE.into(), network: None };
		assert_ok!(ParachainPalletXcm::send_xcm(alice, Parent, message));
	});
	ParaA::execute_with(|| {
		assert_eq!(
			parachain::ForeignUniques::owner((Parent, GeneralIndex(1)).into(), 69u32.into()),
			Some(ALICE),
		);
		assert_eq!(parachain::Balances::reserved_balance(&ALICE), 1000);
	});
	Relay::execute_with(|| {
		assert_eq!(relay_chain::Uniques::owner(1, 69), None);
	});
}

/// Scenario:
/// The relay-chain transfers an NFT into a parachain's sovereign account, who then mints a
/// trustless-backed-derived locally.
///
/// Asserts that the parachain accounts are updated as expected.
#[test]
fn reserve_asset_transfer_nft() {
	sp_tracing::init_for_tests();
	MockNet::reset();

	Relay::execute_with(|| {
		assert_ok!(relay_chain::Uniques::force_create(
			relay_chain::RuntimeOrigin::root(),
			2,
			ALICE,
			false
		));
		assert_ok!(relay_chain::Uniques::mint(
			relay_chain::RuntimeOrigin::signed(ALICE),
			2,
			69,
			child_account_account_id(1, ALICE)
		));
		assert_eq!(relay_chain::Uniques::owner(2, 69), Some(child_account_account_id(1, ALICE)));
	});
	ParaA::execute_with(|| {
		assert_ok!(parachain::ForeignUniques::force_create(
			parachain::RuntimeOrigin::root(),
			(Parent, GeneralIndex(2)).into(),
			ALICE,
			false,
		));
		assert_eq!(
			parachain::ForeignUniques::owner((Parent, GeneralIndex(2)).into(), 69u32.into()),
			None,
		);
		assert_eq!(parachain::Balances::reserved_balance(&ALICE), 0);

		let message = Xcm(vec![
			WithdrawAsset((GeneralIndex(2), 69u32).into()),
			DepositReserveAsset {
				assets: AllCounted(1).into(),
				dest: Parachain(1).into(),
				xcm: Xcm(vec![DepositAsset {
					assets: AllCounted(1).into(),
					beneficiary: (AccountId32 { id: ALICE.into(), network: None },).into(),
				}]),
			},
		]);
		// Send transfer
		let alice = AccountId32 { id: ALICE.into(), network: None };
		assert_ok!(ParachainPalletXcm::send_xcm(alice, Parent, message));
	});
	ParaA::execute_with(|| {
		log::debug!(target: "xcm-executor", "Hello");
		assert_eq!(
			parachain::ForeignUniques::owner((Parent, GeneralIndex(2)).into(), 69u32.into()),
			Some(ALICE),
		);
		assert_eq!(parachain::Balances::reserved_balance(&ALICE), 1000);
	});

	Relay::execute_with(|| {
		assert_eq!(relay_chain::Uniques::owner(2, 69), Some(child_account_id(1)));
	});
}

/// Scenario:
/// The relay-chain creates an asset class on a parachain and then Alice transfers her NFT into
/// that parachain's sovereign account, who then mints a trustless-backed-derivative locally.
///
/// Asserts that the parachain accounts are updated as expected.
#[test]
fn reserve_asset_class_create_and_reserve_transfer() {
	MockNet::reset();

	Relay::execute_with(|| {
		assert_ok!(relay_chain::Uniques::force_create(
			relay_chain::RuntimeOrigin::root(),
			2,
			ALICE,
			false
		));
		assert_ok!(relay_chain::Uniques::mint(
			relay_chain::RuntimeOrigin::signed(ALICE),
			2,
			69,
			child_account_account_id(1, ALICE)
		));
		assert_eq!(relay_chain::Uniques::owner(2, 69), Some(child_account_account_id(1, ALICE)));

		let message = Xcm(vec![Transact {
			origin_kind: OriginKind::Xcm,
			require_weight_at_most: Weight::from_parts(1_000_000_000, 1024 * 1024),
			call: parachain::RuntimeCall::from(
				pallet_uniques::Call::<parachain::Runtime>::create {
					collection: (Parent, 2u64).into(),
					admin: parent_account_id(),
				},
			)
			.encode()
			.into(),
		}]);
		// Send creation.
		assert_ok!(RelayChainPalletXcm::send_xcm(Here, Parachain(1), message));
	});
	ParaA::execute_with(|| {
		// Then transfer
		let message = Xcm(vec![
			WithdrawAsset((GeneralIndex(2), 69u32).into()),
			DepositReserveAsset {
				assets: AllCounted(1).into(),
				dest: Parachain(1).into(),
				xcm: Xcm(vec![DepositAsset {
					assets: AllCounted(1).into(),
					beneficiary: (AccountId32 { id: ALICE.into(), network: None },).into(),
				}]),
			},
		]);
		let alice = AccountId32 { id: ALICE.into(), network: None };
		assert_ok!(ParachainPalletXcm::send_xcm(alice, Parent, message));
	});
	ParaA::execute_with(|| {
		assert_eq!(parachain::Balances::reserved_balance(parent_account_id()), 1000);
		assert_eq!(
			parachain::ForeignUniques::collection_owner((Parent, 2u64).into()),
			Some(parent_account_id())
		);
	});
}

/// Scenario:
/// A parachain transfers funds on the relay chain to another parachain account.
///
/// Asserts that the parachain accounts are updated as expected.
#[test]
fn withdraw_and_deposit() {
	MockNet::reset();

	let send_amount = 10;

	ParaA::execute_with(|| {
		let message = Xcm(vec![
			WithdrawAsset((Here, send_amount).into()),
			buy_execution((Here, send_amount)),
			DepositAsset { assets: AllCounted(1).into(), beneficiary: Parachain(2).into() },
		]);
		// Send withdraw and deposit
		assert_ok!(ParachainPalletXcm::send_xcm(Here, Parent, message.clone()));
	});

	Relay::execute_with(|| {
		assert_eq!(
			relay_chain::Balances::free_balance(child_account_id(1)),
			INITIAL_BALANCE - send_amount
		);
		assert_eq!(
			relay_chain::Balances::free_balance(child_account_id(2)),
			INITIAL_BALANCE + send_amount
		);
	});
}

/// Scenario:
/// A parachain wants to be notified that a transfer worked correctly.
/// It sends a `QueryHolding` after the deposit to get notified on success.
///
/// Asserts that the balances are updated correctly and the expected XCM is sent.
#[test]
fn query_holding() {
	MockNet::reset();

	let send_amount = 10;
	let query_id_set = 1234;

	// Send a message which fully succeeds on the relay chain
	ParaA::execute_with(|| {
		let message = Xcm(vec![
			WithdrawAsset((Here, send_amount).into()),
			buy_execution((Here, send_amount)),
			DepositAsset { assets: AllCounted(1).into(), beneficiary: Parachain(2).into() },
			ReportHolding {
				response_info: QueryResponseInfo {
					destination: Parachain(1).into(),
					query_id: query_id_set,
					max_weight: Weight::from_parts(1_000_000_000, 1024 * 1024),
				},
				assets: All.into(),
			},
		]);
		// Send withdraw and deposit with query holding
		assert_ok!(ParachainPalletXcm::send_xcm(Here, Parent, message.clone(),));
	});

	// Check that transfer was executed
	Relay::execute_with(|| {
		// Withdraw executed
		assert_eq!(
			relay_chain::Balances::free_balance(child_account_id(1)),
			INITIAL_BALANCE - send_amount
		);
		// Deposit executed
		assert_eq!(
			relay_chain::Balances::free_balance(child_account_id(2)),
			INITIAL_BALANCE + send_amount
		);
	});

	// Check that QueryResponse message was received
	ParaA::execute_with(|| {
		assert_eq!(
			parachain::MsgQueue::received_dmp(),
			vec![Xcm(vec![QueryResponse {
				query_id: query_id_set,
				response: Response::Assets(Assets::new()),
				max_weight: Weight::from_parts(1_000_000_000, 1024 * 1024),
				querier: Some(Here.into()),
			}])],
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

	ParaA::execute_with(|| {
		assert_ok!(ParachainPalletXcm::send_xcm(
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
		assert_eq!(
			relay_chain::Balances::free_balance(child_account_id(PARA_A_ID)),
			INITIAL_BALANCE - amount
		);
		assert_eq!(relay_chain::Balances::free_balance(ALICE), INITIAL_BALANCE + amount);
	});
}

#[test]
fn reserve_transfer_native_from_para_a_to_para_b() {
	// Reset the mock network to ensure a clean state before the test
	MockNet::reset();

	let transfer_amount = 100;

	// Execute actions within Parachain A's context
	ParaA::execute_with(|| {
		// Alice initiates a reserve transfer of native tokens to Parachain B
		assert_ok!(ParachainPalletXcm::limited_reserve_transfer_assets(
			parachain::RuntimeOrigin::signed(ALICE),
			// Destination: Parachain B
			Box::new((Parent, Parachain(PARA_B_ID)).into()),
			// Beneficiary: Alice's account on Parachain B
			Box::new(AccountId32 { network: None, id: ALICE.into() }.into()),
			// Assets to transfer: specified amount of the native token
			Box::new((Here, transfer_amount).into()),
			// Fee asset item index: 0 (no specific fee asset)
			0,
			// Weight limit for execution: Unlimited
			Unlimited,
		));

		// Verify that Alice's balance on Parachain A has decreased by the transferred amount
		assert_eq!(
			pallet_balances::Pallet::<parachain::Runtime>::free_balance(&ALICE),
			INITIAL_BALANCE - transfer_amount
		);
	});

	// Execute actions within Parachain B's context
	ParaB::execute_with(|| {
		// Verify that Alice's balance on Parachain B has increased by the transferred amount
		assert_eq!(
			pallet_balances::Pallet::<parachain::Runtime>::free_balance(&ALICE),
			INITIAL_BALANCE + transfer_amount
		);
	});
}
