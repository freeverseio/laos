import BN from "bn.js";
import { expect, use } from "chai";
import { step } from "mocha-steps";

import { ALITH_PRIVATE_KEY, ASSET_HUB_PARA_ID, LAOS_PARA_ID } from "./config";
import {
	describeWithExistingNode,
	fundAccount,
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
import { DoubleEncodedCall, MultiAddress } from "@polkadot/types/interfaces";
import { StagingXcmV3MultiLocation, XcmVersionedLocation } from "@polkadot/types/lookup";
import { hexToBn, u8aToHex } from "@polkadot/util";
import chaiBn from "chai-bn";
use(chaiBn(BN)); // TODO remove

const ONE_LAOS = new BN("1000000000000000000");
const ONE_DOT = new BN("1000000000000");
const DEPOSIT = new BN(100000000000); // Deposit for creating a foreign asset
const WAITING_BLOCKS_FOR_EVENTS = 10;

describeWithExistingNode("Teleport Asset Hub <-> LAOS", (context) => {
	const laosAccountInAssetHub = sovereignAccountOf(LAOS_PARA_ID);
	let apiAssetHub: ApiPromise;
	let apiLaos: ApiPromise;
	let apiRelaychain: ApiPromise;

	before(async function () {
		// Initialize the APIs
		apiAssetHub = context.networks.assetHub;
		apiLaos = context.networks.laos;
		apiRelaychain = context.networks.relaychain;

		// Fund the LAOS Sovereign Account in AssetHub
		const alice = new Keyring({ type: "sr25519" }).addFromUri("//Alice");
		const balanceInAssetHub = await apiAssetHub.query.system.account(laosAccountInAssetHub);
		if (balanceInAssetHub.data.free.toNumber() == 0) {
			await fundAccount(apiAssetHub, alice, laosAccountInAssetHub, 100000000000000); // 100 DOTS
		}
		expect((await apiAssetHub.query.system.account(laosAccountInAssetHub)).data.free.toNumber()).to.be.greaterThan(
			0
		);

		// Open channels and wait
		console.log("[RELAY_CHAIN] Opening channels..."); // See: https://github.com/paritytech/polkadot-sdk/pull/1616
		await sendOpenHrmpChannelTxs(apiRelaychain);
		await waitForBlockProduction(apiRelaychain, "RELAY_CHAIN");
		await waitForBlockProduction(apiAssetHub, "ASSET_HUB");
		await waitForBlockProduction(apiLaos, "LAOS");

		while (
			(await isChannelOpen(apiRelaychain, LAOS_PARA_ID, ASSET_HUB_PARA_ID)) == false ||
			(await isChannelOpen(apiRelaychain, ASSET_HUB_PARA_ID, LAOS_PARA_ID)) == false
		) {
			await awaitBlockChange(apiRelaychain);
		}
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
			// Build XCM instructions
			const laosMultiAddress = apiAssetHub.createType("MultiAddress", laosAccountInAssetHub) as MultiAddress;
			const createCall = apiAssetHub.tx.foreignAssets.create(laosAssetid, laosMultiAddress, ONE_LAOS);
			const createEncodedCall = apiLaos.createType("DoubleEncodedCall", {
				encoded: u8aToHex(createCall.method.toU8a()),
			}) as DoubleEncodedCall;
			const instruction = buildXcmInstruction({
				api: apiLaos,
				calls: [createEncodedCall],
				refTime: new BN(1000000000),
				proofSize: new BN(5000),
				amount: ONE_DOT,
				originKind: apiLaos.createType("XcmOriginKind", "Xcm"),
			});
			const destination = apiLaos.createType("XcmVersionedLocation", {
				V3: siblingLocation(ASSET_HUB_PARA_ID),
			}) as XcmVersionedLocation;

			// Send the XCM instruction from Laos to Asset Hub
			const alith = new Keyring({ type: "ethereum" }).addFromUri(ALITH_PRIVATE_KEY);
			const balanceAlithBefore = new BN((await apiLaos.query.system.account(alith.address)).data.free);
			const balanceLaosAccountBefore = new BN(
				(await apiAssetHub.query.system.account(laosAccountInAssetHub)).data.free
			);
			const sudoCall = apiLaos.tx.sudo.sudo(apiLaos.tx.polkadotXcm.send(destination, instruction));
			await sudoCall
				.signAndSend(alith, () => {})
				.catch((error: any) => {
					console.log("transaction failed", error);
				});

			// Check if the foreign asset has been created in Asset Hub
			let waitForNBlocks = WAITING_BLOCKS_FOR_EVENTS; // TODO refactor create wait for events function
			let eventFound = null;
			while (WAITING_BLOCKS_FOR_EVENTS > 0 && !eventFound) {
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
			expect(balanceAlithBefore).to.be.a.bignumber.that.equals(
				new BN((await apiLaos.query.system.account(alith.address)).data.free)
			); // Alith balance has not changed
			const laosAccountBalanceDifference = balanceLaosAccountBefore.sub(
				new BN((await apiAssetHub.query.system.account(laosAccountInAssetHub)).data.free)
			);
			expect(laosAccountBalanceDifference).to.be.a.bignumber.that.equals(ONE_DOT.add(DEPOSIT));
			expect((await apiAssetHub.query.foreignAssets.asset(laosAssetid)).isEmpty).to.be.false;
		}
	});

	// TODO merge this step with the previous one, investigate why mint xcm has to be sent with originKind: SovereignAccount
	// whereas create xcm has to be sent with originKind: Xcm
	step("Mint LAOS foreign asset in AssetHub", async function () {
		const laosAssetid = apiAssetHub.createType(
			"StagingXcmV3MultiLocation",
			siblingLocation(LAOS_PARA_ID)
		) as StagingXcmV3MultiLocation;
		// Build XCM instructions
		const ferdie = new Keyring({ type: "sr25519" }).addFromUri("//Ferdie");
		const ferdieMultiaddress = apiAssetHub.createType("MultiAddress", ferdie.address) as MultiAddress;
		const mintLaosCall = apiAssetHub.tx.foreignAssets.mint(laosAssetid, ferdieMultiaddress, ONE_LAOS.muln(10000));
		const mintLaosEncodedCall = apiLaos.createType("DoubleEncodedCall", {
			encoded: u8aToHex(mintLaosCall.method.toU8a()),
		}) as DoubleEncodedCall;
		const instruction = buildXcmInstruction({
			api: apiLaos,
			calls: [mintLaosEncodedCall],
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
		const balanceAlithBefore = new BN((await apiLaos.query.system.account(alith.address)).data.free);
		const balanceLaosAccountBefore = new BN(
			(await apiAssetHub.query.system.account(laosAccountInAssetHub)).data.free
		);
		const sudoCall = apiLaos.tx.sudo.sudo(apiLaos.tx.polkadotXcm.send(destination, instruction));
		await sudoCall
			.signAndSend(alith, () => {})
			.catch((error: any) => {
				console.log("transaction failed", error);
			});

		// Check if the foreign asset has been minted in Asset Hub
		let waitForNBlocks = WAITING_BLOCKS_FOR_EVENTS; // TODO refactor create wait for events function
		let eventFound = null;
		while (WAITING_BLOCKS_FOR_EVENTS > 0 && !eventFound) {
			const events = await apiAssetHub.query.system.events();
			events.filter((event) => {
				if (apiAssetHub.events.foreignAssets.Issued.is(event.event)) {
					eventFound = event;
				}
			});
			await awaitBlockChange(apiAssetHub);
			waitForNBlocks--;
		}
		expect(balanceAlithBefore).to.be.a.bignumber.that.equals(
			new BN((await apiLaos.query.system.account(alith.address)).data.free)
		); // Alith balance has not changed
		const laosAccountBalanceDifference = balanceLaosAccountBefore.sub(
			new BN((await apiAssetHub.query.system.account(laosAccountInAssetHub)).data.free)
		);
		expect(laosAccountBalanceDifference).to.be.a.bignumber.that.equals(ONE_DOT);
		const ferdieLaosBalanceInAssetHub = hexToBn(
			(await apiAssetHub.query.foreignAssets.account(laosAssetid, ferdie.address)).toJSON()["balance"]
		);
		expect(ferdieLaosBalanceInAssetHub.cmp(new BN(0))).to.be.eq(1);
	});

	step("Create LAOS/RelayToken pool in AssetHub", async function () {
		const relayAssetId = apiAssetHub.createType(
			"StagingXcmV3MultiLocation",
			relayLocation()
		) as StagingXcmV3MultiLocation;
		const laosAssetid = apiAssetHub.createType(
			"StagingXcmV3MultiLocation",
			siblingLocation(LAOS_PARA_ID)
		) as StagingXcmV3MultiLocation;
		// NOTE: We only create the pool if it hasn't been created yet, in this way we ensure tests are idempotent
		const poolExists = !(await apiAssetHub.query.assetConversion.pools([relayAssetId, laosAssetid])).isEmpty;
		if (!poolExists) {
			// Build XCM instruction to be included in xcm.send call
			const createPoolCall = apiAssetHub.tx.assetConversion.createPool(relayAssetId.toU8a(), laosAssetid.toU8a());
			const createPoolEncodedCall = apiLaos.createType("DoubleEncodedCall", {
				encoded: u8aToHex(createPoolCall.method.toU8a()),
			}) as DoubleEncodedCall;
			const instruction = buildXcmInstruction({
				api: apiLaos,
				calls: [createPoolEncodedCall],
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
			const balanceLaosAccountBefore = new BN(
				(await apiAssetHub.query.system.account(laosAccountInAssetHub)).data.free
			);
			const balanceAlithBefore = new BN((await apiLaos.query.system.account(alith.address)).data.free);
			await sudoCall
				.signAndSend(alith, () => {})
				.catch((error: any) => {
					console.log("transaction failed", error);
				});

			// Check that pool has been created in Asset Hub
			let waitForNBlocks = WAITING_BLOCKS_FOR_EVENTS; // TODO refactor create wait for events function
			let eventFound = null;
			while (waitForNBlocks > 0 && !eventFound) {
				const events = await apiAssetHub.query.system.events();
				events.filter((event) => {
					if (apiAssetHub.events.assetConversion.PoolCreated.is(event.event)) {
						eventFound = event;
					}
				});
				await awaitBlockChange(apiAssetHub);
				waitForNBlocks--;
			}
			expect(eventFound.event.data[0].toString()).to.equal(laosAccountInAssetHub); // creator
			expect(eventFound.event.data[1].toJSON()).to.deep.equal([relayLocation(), siblingLocation(LAOS_PARA_ID)]); // poolId
			expect((await apiAssetHub.query.assetConversion.pools([relayAssetId, laosAssetid])).isEmpty).to.be.false;

			expect(balanceAlithBefore).to.be.a.bignumber.that.equals(
				new BN((await apiLaos.query.system.account(alith.address)).data.free)
			); // Alith balance has not changed
			const laosAccountBalanceDifference = balanceLaosAccountBefore.sub(
				new BN((await apiAssetHub.query.system.account(laosAccountInAssetHub)).data.free)
			);
			const assetAccountDeposit = apiAssetHub.consts.assets.assetAccountDeposit;
			expect(laosAccountBalanceDifference).to.be.a.bignumber.that.equals(ONE_DOT.add(assetAccountDeposit));
		}

		// Add liquidity to the pool
		const ferdie = new Keyring({ type: "sr25519" }).addFromUri("//Ferdie");
		const liquidityAmountLaos = new BN(ONE_LAOS.muln(1000));
		const liquidityAmountDot = new BN(ONE_DOT.muln(1000));
		const ferdieBalanceInAssetHub = new BN((await apiAssetHub.query.system.account(ferdie.address)).data.free);
		const ferdieLaosBalanceInAssetHub = hexToBn(
			(await apiAssetHub.query.foreignAssets.account(laosAssetid, ferdie.address)).toJSON()["balance"]
		);
		expect(ferdieBalanceInAssetHub.cmp(liquidityAmountDot)).to.be.eq(1);
		expect(ferdieLaosBalanceInAssetHub.cmp(liquidityAmountLaos)).to.be.eq(1);

		await apiAssetHub.tx.assetConversion
			.addLiquidity(
				relayAssetId.toU8a(),
				laosAssetid.toU8a(),
				liquidityAmountDot,
				liquidityAmountLaos,
				liquidityAmountDot.sub(new BN(ONE_DOT.muln(10))),
				liquidityAmountLaos.sub(new BN(ONE_LAOS.muln(10))),
				ferdie.address
			)
			.signAndSend(ferdie, () => {})
			.catch((error: any) => {
				console.log("transaction failed", error);
			});
	});

	step("Teleport from LAOS to AssetHub", async function () {
		const charlie = new Keyring({ type: "sr25519" }).addFromUri("//Charlie");
		const alith = new Keyring({ type: "ethereum" }).addFromUri(ALITH_PRIVATE_KEY);

		const destination = apiLaos.createType("XcmVersionedLocation", {
			V3: siblingLocation(ASSET_HUB_PARA_ID),
		});

		// We need to use AssetHub api otherwise we get an error as LAOS does not use AccountId32
		let accountId = apiAssetHub.createType("AccountId", charlie.address);
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

		const amount = ONE_LAOS.muln(5);
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

		call.signAndSend(alith).catch((error: any) => {
			console.log("transaction failed", error);
		});

		// TODO check Charlie amount
	});

	after(async function () {
		await apiAssetHub.disconnect();
		await apiLaos.disconnect();
		await apiRelaychain.disconnect();
	});
});
