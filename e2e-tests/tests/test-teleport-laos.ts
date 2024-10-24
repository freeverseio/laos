import { BN } from "bn.js";
import { expect } from "chai";
import { step } from "mocha-steps";

import { ALITH_PRIVATE_KEY, ASSET_HUB_PARA_ID, FAITH_PRIVATE_KEY, LAOS_PARA_ID } from "./config";
import { describeWithExistingNode, fundAccount, isChannelOpen, sendOpenHrmpChannelTxs, siblingAccountId } from "./util";
import { ApiPromise, Keyring } from "@polkadot/api";
import { AssetIdV3, DoubleEncodedCall, MultiAddress } from "@polkadot/types/interfaces";
import { StagingXcmV3MultiLocation, XcmVersionedLocation, XcmVersionedXcm } from "@polkadot/types/lookup";
import { u8aToHex } from "@polkadot/util";
import { awaitBlockChange } from "./util";

describeWithExistingNode("Teleport Asset Hub <-> LAOS", (context) => {
	const laosSiblingInAssetHub = siblingAccountId(LAOS_PARA_ID);
	let apiAssetHub: ApiPromise;
	let apiLaos: ApiPromise;
	let apiRelaychain: ApiPromise;

	before(async function () {
		apiAssetHub = context.networks.assetHub;
		apiLaos = context.networks.laos;
		apiRelaychain = context.networks.relaychain;
		const alice = new Keyring({ type: "sr25519" }).addFromUri("//Alice");
		const balanceInAssetHub = await apiAssetHub.query.system.account(laosSiblingInAssetHub);
		if (balanceInAssetHub.data.free.toNumber() == 0) {
			await fundAccount(apiAssetHub, alice, laosSiblingInAssetHub, 100000000000000); // 100 DOTS
		}
		expect((await apiAssetHub.query.system.account(laosSiblingInAssetHub)).data.free.toNumber()).to.be.greaterThan(
			0
		);

		console.log("[RELAY_CHAIN] Waiting for block production...");
		await awaitBlockChange(apiRelaychain);

		console.log("[RELAY_CHAIN] Opening channels..."); // See: https://github.com/paritytech/polkadot-sdk/pull/1616
		await sendOpenHrmpChannelTxs(apiRelaychain);

		console.log("[ASSET_HUB] Waiting for block production...");
		await awaitBlockChange(apiAssetHub);

		console.log("[LAOS] Waiting for block production...");
		await awaitBlockChange(apiLaos);

		while (
			(await isChannelOpen(apiRelaychain, LAOS_PARA_ID, ASSET_HUB_PARA_ID)) == false ||
			(await isChannelOpen(apiRelaychain, ASSET_HUB_PARA_ID, LAOS_PARA_ID)) == false
		) {
			await awaitBlockChange(apiRelaychain);
		}
	});

	step("HRMP channels between Asset Hub and LAOS are open", async function () {
		expect(await isChannelOpen(apiRelaychain, LAOS_PARA_ID, ASSET_HUB_PARA_ID)).to.be.true;
		expect(await isChannelOpen(apiRelaychain, ASSET_HUB_PARA_ID, LAOS_PARA_ID)).to.be.true;
	});

	step("Create LAOS foreign asset in AssetHub", async function () {
		const alith = new Keyring({ type: "ethereum" }).addFromUri(ALITH_PRIVATE_KEY);

		// STEP 1: Build create foreign asset params: laosAssetId, accountId, amount
		const laosAssetId = apiAssetHub.createType("StagingXcmV3MultiLocation", {
			parents: "1",
			interior: {
				X1: { Parachain: LAOS_PARA_ID },
			},
		}) as StagingXcmV3MultiLocation;

		let accountId = apiAssetHub.createType("MultiAddress", laosSiblingInAssetHub) as MultiAddress;
		let amount = new BN("1000000000000000000"); // 1 LAOS
		const createCall = apiAssetHub.tx.foreignAssets.create(laosAssetId, accountId, amount);

		// STEP 2: Build XCM instruction to be included in xcm.send call
		const doubleEncodedCall = apiLaos.createType("DoubleEncodedCall", {
			encoded: u8aToHex(createCall.method.toU8a()),
		}) as DoubleEncodedCall;

		const relayToken = apiLaos.createType("AssetIdV3", {
			Concrete: {
				parents: "1",
				interior: {
					Here: "",
				},
			},
		}) as AssetIdV3;
		const instruction = apiLaos.createType("XcmVersionedXcm", {
			V3: [
				{
					WithdrawAsset: [
						apiLaos.createType("MultiAssetV3", {
							id: relayToken,
							fun: apiLaos.createType("FungibilityV3", {
								Fungible: new BN("1000000000000"), // 1 DOT
							}),
						}),
					],
				},
				{
					BuyExecution: {
						fees: apiLaos.createType("MultiAssetV3", {
							id: relayToken,
							fun: apiLaos.createType("FungibilityV3", {
								Fungible: new BN("1000000000000"), // 1 DOT
							}),
						}),
						weight_limit: "Unlimited",
					},
				},
				{
					Transact: {
						originKind: apiLaos.createType("XcmOriginKind", "Xcm"),
						requireWeightAtMost: apiLaos.createType("WeightV2", {
							refTime: new BN("1000000000"), // Adjust as needed
							proofSize: new BN("5000"),
						}),
						call: doubleEncodedCall, // DoubleEncodedCall instance
					},
				},
			],
		}) as XcmVersionedXcm;

		const destination = apiLaos.createType("XcmVersionedLocation", {
			V3: {
				parents: "1",
				interior: {
					X1: { Parachain: ASSET_HUB_PARA_ID },
				},
			},
		}) as XcmVersionedLocation;

		// STEP 3: Send the XCM instruction from Laos to Asset Hub
		const sudoCall = apiLaos.tx.sudo.sudo(apiLaos.tx.polkadotXcm.send(destination, instruction));
		await sudoCall
			.signAndSend(alith, () => {})
			.catch((error: any) => {
				console.log("transaction failed", error);
			});

		// STEP 4: Check if the foreign asset was created in Asset Hub
		let waitForNBlocks = 5;
		let eventFound = null;
		while (waitForNBlocks > 0 && !eventFound) {
			const events = await apiAssetHub.query.system.events();
			events.filter((event) => {
				if (apiAssetHub.events.foreignAssets.Created.is(event.event)) {
					eventFound = event;
				}
			});
			await awaitBlockChange(apiAssetHub);
			waitForNBlocks--;
		}
		expect(eventFound.event.data[0].toString()).to.equal(laosAssetId.toString());
		expect(eventFound.event.data[1].toString()).to.equal(laosSiblingInAssetHub);
		expect(eventFound.event.data[2].toString()).to.equal(laosSiblingInAssetHub);
		// TODO check balance of sudo alith account in laos
		// TODO balance of laosSiblingInAssetHub in asset hub

	});

	// TODO foreign asset mint

	step("Create LAOS/RelayToken pool", async function () {
		const alith = new Keyring({ type: "ethereum" }).addFromUri(ALITH_PRIVATE_KEY);

		// STEP 1: Build create pool call params
		const laosAssetId = apiAssetHub.createType("StagingXcmV3MultiLocation", {
			// V3: {
			parents: "1",
			interior: {
				X1: { Parachain: LAOS_PARA_ID },
			},
			// },
		}) as StagingXcmV3MultiLocation;
		const relayAssetId = apiAssetHub.createType("StagingXcmV3MultiLocation", {
			// V3: {
				parents: "1",
				interior: {
					Here: "",
				},
			// },
		}) as StagingXcmV3MultiLocation;

		const createPoolCall = apiAssetHub.tx.assetConversion.createPool(relayAssetId.toU8a(), laosAssetId.toU8a())

		// STEP 2: Build XCM instruction to be included in xcm.send call
		console.log(u8aToHex(createPoolCall.method.toU8a()))
		const doubleEncodedCall = apiLaos.createType("DoubleEncodedCall", {
			// encoded: "0x38000100010100512d",
			// encoded: "0x38000000010100512d",
			encoded: u8aToHex(createPoolCall.method.toU8a()),
		}) as DoubleEncodedCall;
		const relayToken = apiLaos.createType("AssetIdV3", {
			Concrete: {
				parents: "1",
				interior: {
					Here: "",
				},
			},
		}) as AssetIdV3;
		const instruction = apiLaos.createType("XcmVersionedXcm", {
			V3: [
				{
					WithdrawAsset: [
						apiLaos.createType("MultiAssetV3", {
							id: relayToken,
							fun: apiLaos.createType("FungibilityV3", {
								Fungible: new BN("1000000000000"), // 1 DOT
							}),
						}),
					],
				},
				{
					BuyExecution: {
						fees: apiLaos.createType("MultiAssetV3", {
							id: relayToken,
							fun: apiLaos.createType("FungibilityV3", {
								Fungible: new BN("1000000000000"), // 1 DOT
							}),
						}),
						weight_limit: "Unlimited",
					},
				},
				{
					Transact: {
						originKind: apiLaos.createType("XcmOriginKind", "SovereignAccount"),
						requireWeightAtMost: apiLaos.createType("WeightV2", {
							refTime: new BN("2000000000"), // Adjust as needed
							proofSize: new BN("7000"),
						}),
						call: doubleEncodedCall, // DoubleEncodedCall instance
					},
				},
			],
		}) as XcmVersionedXcm;

		const destination = apiLaos.createType("XcmVersionedLocation", {
			V3: {
				parents: "1",
				interior: {
					X1: { Parachain: ASSET_HUB_PARA_ID },
				},
			},
		}) as XcmVersionedLocation;

		// STEP 3: Send the XCM instruction from Laos to Asset Hub
		const sudoCall = apiLaos.tx.sudo.sudo(apiLaos.tx.polkadotXcm.send(destination, instruction));
		await sudoCall
			.signAndSend(alith, () => { })
			.catch((error: any) => {
				console.log("transaction failed", error);
			});

			// TODO check pool is created
	});

	// step("Teleport from LAOS to AssetHub", async function () {
	// 	apiLaos = context.networks.laos;
	// 	apiAssetHub = context.networks.assetHub;
	// 	const faith = new Keyring().addFromUri(FAITH_PRIVATE_KEY);
	// 	const alith = new Keyring({ type: "ethereum" }).addFromUri(ALITH_PRIVATE_KEY);

	// 	const destination = apiLaos.createType("XcmVersionedLocation", {
	// 		V3: {
	// 			parents: "1",
	// 			interior: {
	// 				X1: { Parachain: ASSET_HUB_PARA_ID },
	// 			},
	// 		},
	// 	});

	// 	// We need to use AssetHub api otherwise we get an error as LAOS does not use AccountId32
	// 	let accountId = apiAssetHub.createType("AccountId", faith.address);
	// 	const beneficiary = apiLaos.createType("XcmVersionedLocation", {
	// 		V3: {
	// 			parents: "0",
	// 			interior: {
	// 				X1: {
	// 					AccountId32: {
	// 						// network: 'Any',
	// 						id: accountId.toHex(),
	// 					},
	// 				},
	// 			},
	// 		},
	// 	});

	// 	// 1 LAOS = 10^18, this is .1 LAOS
	// 	const amount = new BN("100000000000000000");
	// 	const assets = apiLaos.createType("XcmVersionedAssets", {
	// 		V3: [
	// 			{
	// 				id: {
	// 					Concrete: {
	// 						parents: 0,
	// 						interior: {
	// 							Here: "",
	// 						},
	// 					},
	// 				},
	// 				fun: {
	// 					Fungible: amount,
	// 				},
	// 			},
	// 		],
	// 	});
	// 	// TODO check this in production we should pay
	// 	const fee_asset_item = "0";
	// 	const weight_limit = "Unlimited";

	// 	const call = apiLaos.tx.polkadotXcm.limitedTeleportAssets(
	// 		destination,
	// 		beneficiary,
	// 		assets,
	// 		fee_asset_item,
	// 		weight_limit
	// 	);

	// 	call.signAndSend(alith, (result) => {
	// 		console.log(`RESULT =>>> ${result}`);
	// 	}).catch((error: any) => {
	// 		console.log("transaction failed", error);
	// 	});
	// });

	after(async function () {
		await apiAssetHub.disconnect();
		await apiLaos.disconnect();
		await apiRelaychain.disconnect();
	});
});
