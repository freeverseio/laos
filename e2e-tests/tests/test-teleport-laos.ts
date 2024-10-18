import BN from "bn.js";
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

	// TODO ?
	// step("Create LAOS foreign asset in AssetHub", async function () {
	//     expect(false).to.be.true;
	// });

	step("Teleport from LAOS to AssetHub", async function () {
		const apiLaos = await context.networks.laos;
		const apiAssetHub = await context.networks.assetHub;
		const faith = new Keyring().addFromUri(FAITH_PRIVATE_KEY);
		const alith = new Keyring({ type: "ethereum" }).addFromUri(ALITH_PRIVATE_KEY);

		const destination = apiLaos.createType("XcmVersionedLocation", {
			V3: {
				parents: "1",
				interior: {
					X1: { Parachain: ASSET_HUB_PARA_ID },
				},
			},
		});

		// We need to use AssetHub api otherwise we get an error as LAOS does not use AccountId32
		let accountId = apiAssetHub.createType("AccountId", faith.address);
		const beneficiary = apiLaos.createType("XcmVersionedLocation", {
			V3: {
				parents: "0",
				interior: {
					X1: {
						AccountId32: {
							// network: 'Any',
							id: accountId.toHex(),
						},
					},
				},
			},
		});

		// 1 LAOS = 10^18, this is .1 LAOS
		const amount = 1; // TODO 100000000000000000
		const assets = apiLaos.createType("XcmVersionedAssets", {
			V3: [
				{
					id: {
						Concrete: {
							parents: 0,
							interior: {
								Here: "",
							},
						},
					},
					fun: {
						Fungible: amount,
					},
				},
			],
		});
		// TODO check this in production we should pay
		const fee_asset_item = "0";
		const weight_limit = "Unlimited";

		const call = apiLaos.tx.polkadotXcm.limitedTeleportAssets(
			destination,
			beneficiary,
			assets,
			fee_asset_item,
			weight_limit
		);

		call.signAndSend(alith, (result) => {
			console.log(`RESULT =>>> ${result}`);
		}).catch((error: any) => {
			console.log("transaction failed", error);
		});
	});
});
