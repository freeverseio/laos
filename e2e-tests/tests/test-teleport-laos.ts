import {BN} from "bn.js";
import { assert, expect } from "chai";
import { step } from "mocha-steps";

import {
	ALITH_PRIVATE_KEY,
	ASSET_HUB_PARA_ID,
	CHAIN_ID,
	FAITH,
	FAITH_PRIVATE_KEY,
	LAOS_PARA_ID,
	RUNTIME_IMPL_VERSION,
	RUNTIME_SPEC_NAME,
	RUNTIME_SPEC_VERSION,
} from "./config";
import { customRequest, describeWithExistingNode } from "./util";
import { Keyring } from "@polkadot/api";
import { MultiLocationV3, JunctionsV3, XcmV3, InstructionV3, AssetIdV3 } from "@polkadot/types/interfaces";
import { u64, u8 } from "@polkadot/types";
import { XcmVersionedLocation, XcmVersionedXcm } from "@polkadot/types/lookup";
// const siblingAccountId = (paraId: number) => {
// 	let type = paraType.value;
// 	let typeEncoded = stringToU8a(type);
// 	let paraIdEncoded = bnToU8a(parseInt(paraId), 16);
// 	let zeroPadding = new Uint8Array(32 - typeEncoded.length - paraIdEncoded.length).fill(0);
// 	let address = new Uint8Array([...typeEncoded, ...paraIdEncoded, ...zeroPadding]);
// 	paraid.address.innerText = encodeAddress(address);
// }

describeWithExistingNode("Teleport Asset Hub <-> LAOS", (context) => {
	step("HRMP channels between Asset Hub and LAOS are open", async function () {
		const laosToAssetHubChannel = await context.networks.relaychain.query.hrmp.hrmpChannels({
			sender: LAOS_PARA_ID,
			recipient: ASSET_HUB_PARA_ID,
		});
		expect(laosToAssetHubChannel.isEmpty).to.be.false;
		const assetHubToLaosChannel = await context.networks.relaychain.query.hrmp.hrmpChannels({
			sender: ASSET_HUB_PARA_ID,
			recipient: LAOS_PARA_ID,
		});
		expect(assetHubToLaosChannel.isEmpty).to.be.false;
	});

	step("Create LAOS foreign asset in AssetHub", async function () {
		const apiAssetHub = await context.networks.assetHub;
		const apiLaos = await context.networks.laos;
		const apiRelay = await context.networks.relaychain;
		const laosAssetId = apiAssetHub.createType("XcmVersionedLocation", {
			V3: {
				parents: "1",
				interior: {
					X1: { Parachain: LAOS_PARA_ID },
				},
			},
		}) as XcmVersionedLocation;
		const relayToken = apiLaos.createType("AssetIdV3", {
			Concrete: {
				parents: "1",
				interior: {
					Here: "",
				},
			},
		}) as AssetIdV3;
		
		const keyring = new Keyring({ type: "sr25519" });
		const alice = keyring.addFromUri("//Alice");
		const alith = new Keyring({ type: "ethereum" }).addFromUri(ALITH_PRIVATE_KEY);
		const laosSiblingInAssetHub = "5Eg2fnssBDaFCWy7JnEZYnEuNPZbbzzEWGw5zryrTpmsTuPL";
		// TODO move transfer to before and wait one block
		apiAssetHub.tx.balances
			.transferKeepAlive(laosSiblingInAssetHub, 100000000000000)
			.signAndSend(alice, () => {})
			.catch((error: any) => {
				console.log("transaction failed", error);
			});
		const balanceInAssetHub = await apiAssetHub.query.system.account(laosSiblingInAssetHub);
		expect(balanceInAssetHub.data.free.toNumber()).to.be.greaterThan(0);
		
		let accountId = apiAssetHub.createType("AccountId", laosSiblingInAssetHub);
		let amount = new BN("1000000000000000000"); // 1 LAOS
		const destination = apiLaos.createType("XcmVersionedLocation", {
			V3: {
				parents: "1",
				interior: {
					X1: { Parachain: ASSET_HUB_PARA_ID },
				},
			},
		}) as XcmVersionedLocation;

		const originKind = apiLaos.createType("XcmOriginKind", "Xcm");
		const requireWeightAtMost = apiLaos.createType("WeightV2", {
			refTime: new BN("1000000000"), // Adjust as needed
			proofSize: new BN("5000"),
		});
		const createCall = apiAssetHub.tx.foreignAssets.create(laosAssetId, accountId, amount);
		console.log(createCall.method.toHex())
		const doubleEncodedCall = apiLaos.createType("DoubleEncodedCall", {
			encoded: "0x3500201010051d007369626c540b0000000000000000000000000000000000000000000000000000000064a7b3b6e00d0000000000000000",
			// encoded: createCall.method.toHex(), // TODO
		});

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
						originKind, // XcmOriginKind instance
						requireWeightAtMost, // WeightV2 instance
						call: doubleEncodedCall, // DoubleEncodedCall instance
					},
				},
			],
		}) as XcmVersionedXcm;

		const sudoCall = apiLaos.tx.sudo.sudo(apiLaos.tx.polkadotXcm.send(destination, instruction));
		
		sudoCall.signAndSend(alith, () => {}).catch((error: any) => {
			console.log("transaction failed", error);
		});

		let maximumNumberOfBlocks = 10;
		while (maximumNumberOfBlocks > 0) {
			const events = await apiAssetHub.query.system.events();
			events.filter((event) => {
				console.log(apiAssetHub.events.foreignAssets.Created.is(event.event))
				if (apiAssetHub.events.foreignAssets.Created.is(event.event)) {
					console.log("Foreign asset created", event.event);
				}
			})
			await new Promise((resolve) => setTimeout(resolve, 12000));
			maximumNumberOfBlocks--;
		}
	});

	// step("Teleport from LAOS to AssetHub", async function () {
	// 	const apiLaos = await context.networks.laos;
	// 	const apiAssetHub = await context.networks.assetHub;
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
	// 	const amount = 1; // TODO 100000000000000000
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
});
