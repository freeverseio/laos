import BN from "bn.js";
import { expect, use } from "chai";
import { step } from "mocha-steps";

import { ALITH_PRIVATE_KEY, ASSET_HUB_PARA_ID, LAOS_PARA_ID } from "./config";
import {
	describeWithExistingNode,
	transferBalance,
	isChannelOpen,
	sendOpenHrmpChannelTxs,
	sovereignAccountOf,
	waitForBlockProduction,
	awaitBlockChange,
	siblingLocation,
	buildXcmInstruction,
	relayLocation,
} from "./util";
import { ApiPromise, Keyring } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";
import { DoubleEncodedCall, MultiAddress } from "@polkadot/types/interfaces";
import { StagingXcmV3MultiLocation, XcmVersionedLocation } from "@polkadot/types/lookup";
import { u8aToHex } from "@polkadot/util";
import chaiBn from "chai-bn";
use(chaiBn(BN));

const ONE_LAOS = new BN("1000000000000000000");
const ONE_DOT = new BN("1000000000000");
const HUNDRED_DOTS = new BN("100000000000000");
const DEPOSIT = new BN("1000000000000"); // Deposit for creating a foreign asset

describeWithExistingNode("Teleport Asset Hub <-> LAOS", (context) => {
	const laosAccountInAssetHub = sovereignAccountOf(LAOS_PARA_ID);

	// APIS
	let apiAssetHub: ApiPromise;
	let apiLaos: ApiPromise;
	let apiRelaychain: ApiPromise;

	//Accounts
	let alice: KeyringPair;
	let alith: KeyringPair;

	before(async function () {
		// Initialize the APIs
		apiAssetHub = context.networks.assetHub;
		apiLaos = context.networks.laos;
		apiRelaychain = context.networks.relaychain;

		//Initialize the accounts
		alice = new Keyring({ type: "sr25519" }).addFromUri("//Alice");
		alith = new Keyring({ type: "ethereum" }).addFromUri(ALITH_PRIVATE_KEY);

		console.log("Waiting until all the relay chain, Asset Hub and LAOS produce blocks...");
		await Promise.all([awaitBlockChange(apiRelaychain), awaitBlockChange(apiAssetHub), awaitBlockChange(apiLaos)]);

		console.log("[RELAY_CHAIN] Send transaction to open HRMP channels between AssetHub and LAOS..."); // See: https://github.com/paritytech/polkadot-sdk/pull/1616
		await sendOpenHrmpChannelTxs(apiRelaychain, LAOS_PARA_ID, ASSET_HUB_PARA_ID);
	});

	step("HRMP channels between AssetHub and LAOS are open", async function () {
		expect(await isChannelOpen(apiRelaychain, LAOS_PARA_ID, ASSET_HUB_PARA_ID)).to.be.true;
		expect(await isChannelOpen(apiRelaychain, ASSET_HUB_PARA_ID, LAOS_PARA_ID)).to.be.true;
	});

	step("Create LAOS foreign asset in AssetHub", async function () {
		const laosAssetid = apiAssetHub.createType(
			"StagingXcmV3MultiLocation",
			siblingLocation(LAOS_PARA_ID)
		) as StagingXcmV3MultiLocation;

		const laosForeignAssetExists = !(await apiAssetHub.query.foreignAssets.asset(laosAssetid)).isEmpty;

		// NOTE: We only create the foreign asset if it hasn't been created yet, in this way we ensure tests are idempotent
		if (!laosForeignAssetExists) {
			//Fund LAOS sovereigna account
			console.log("[ASSET_HUB] Funding LAOS sovereign account...");
			await transferBalance(apiAssetHub, alice, laosAccountInAssetHub, HUNDRED_DOTS);

			// Build XCM instruction
			const foreign_asset_admin = apiAssetHub.createType("MultiAddress", laosAccountInAssetHub) as MultiAddress;

			const createCall = apiAssetHub.tx.foreignAssets.create(laosAssetid, foreign_asset_admin, ONE_LAOS);

			const createEncodedCall = apiLaos.createType("DoubleEncodedCall", {
				encoded: u8aToHex(createCall.method.toU8a()),
			}) as DoubleEncodedCall;

			const instruction = buildXcmInstruction({
				api: apiLaos,
				call: createEncodedCall,
				refTime: new BN(1000000000),
				proofSize: new BN(5000),
				amount: ONE_DOT,
				originKind: apiLaos.createType("XcmOriginKind", "Xcm"),
			});

			const destination = apiLaos.createType("XcmVersionedLocation", {
				V3: siblingLocation(ASSET_HUB_PARA_ID),
			}) as XcmVersionedLocation;

			// Get balances before teleporting.
			const balanceAlithBefore = new BN((await apiLaos.query.system.account(alith.address)).data.free);
			const balanceLaosAccountBefore = new BN(
				(await apiAssetHub.query.system.account(laosAccountInAssetHub)).data.free
			);

			// Send the XCM instruction from LAOS too Asset Hub
			const sudoCall = apiLaos.tx.sudo.sudo(apiLaos.tx.polkadotXcm.send(destination, instruction));
			try {
				await sudoCall.signAndSend(alith);
			} catch (error) {
				console.log("transaction failed", error);
			}

			// Check if the foreign asset create event has been emitted in Asset Hub
			let waitForNBlocks = 5;
			let eventFound = null;
			while (waitForNBlocks > 0 && !eventFound) {
				console.log("[ASSET_HUB] Waiting til LAOS asset has been created...");

				const events = await apiAssetHub.query.system.events();

				events.filter((event) => {
					if (apiAssetHub.events.foreignAssets.Created.is(event.event)) {
						eventFound = event;
					}
				});

				await awaitBlockChange(apiAssetHub);
				waitForNBlocks--;
			}

			expect(eventFound.event.data[0].toString()).to.equal(laosAssetid.toString());
			expect(eventFound.event.data[1].toString()).to.equal(laosAccountInAssetHub);
			expect(eventFound.event.data[2].toString()).to.equal(laosAccountInAssetHub);

			// Check that balances are correct.
			expect(balanceAlithBefore).to.be.a.bignumber.that.equals(
				new BN((await apiLaos.query.system.account(alith.address)).data.free)
			);

			const laosAccountBalanceDifference = balanceLaosAccountBefore.sub(
				new BN((await apiAssetHub.query.system.account(laosAccountInAssetHub)).data.free)
			);
			expect(laosAccountBalanceDifference).to.be.a.bignumber.that.equals(ONE_DOT.add(DEPOSIT));

      // The asset's found in the pallet
      expect((await apiAssetHub.query.foreignAssets.asset(laosAssetid)).isEmpty).to.be.false;
		}
	});

	// TODO foreign asset mint

	step("Create LAOS/RelayToken pool in AssetHub", async function () {
		// Build XCM instruction to be included in xcm.send call
		const relayAssetId = apiAssetHub.createType(
			"StagingXcmV3MultiLocation",
			relayLocation()
		) as StagingXcmV3MultiLocation;
		const laosAssetid = apiAssetHub.createType(
			"StagingXcmV3MultiLocation",
			siblingLocation(LAOS_PARA_ID)
		) as StagingXcmV3MultiLocation;
		const createPoolCall = apiAssetHub.tx.assetConversion.createPool(relayAssetId.toU8a(), laosAssetid.toU8a());
		const createPoolEncodedCall = apiLaos.createType("DoubleEncodedCall", {
			encoded: u8aToHex(createPoolCall.method.toU8a()),
		}) as DoubleEncodedCall;
		const instruction = buildXcmInstruction({
			api: apiLaos,
			call: createPoolEncodedCall,
			refTime: new BN(2000000000),
			proofSize: new BN(7000),
			amount: ONE_DOT,
			originKind: apiLaos.createType("XcmOriginKind", "SovereignAccount"),
		});
		const destination = apiLaos.createType("XcmVersionedLocation", {
			V3: siblingLocation(ASSET_HUB_PARA_ID),
		}) as XcmVersionedLocation;

		// Send the XCM instruction from Laos to Asset Hub
		const alith = new Keyring({ type: "ethereum" }).addFromUri(ALITH_PRIVATE_KEY);
		const sudoCall = apiLaos.tx.sudo.sudo(apiLaos.tx.polkadotXcm.send(destination, instruction));
		await sudoCall
			.signAndSend(alith, () => {})
			.catch((error: any) => {
				console.log("transaction failed", error);
			});

		// TODO check pool is created
		// TODO check balance of native token of laosSiblingInAssetHub in asset hub
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
});
