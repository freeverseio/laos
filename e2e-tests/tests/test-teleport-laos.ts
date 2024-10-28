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
	hereLocation,
} from "./util";
import { ApiPromise, Keyring } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";
import { DoubleEncodedCall, MultiAddress } from "@polkadot/types/interfaces";
import { StagingXcmV3MultiLocation, XcmVersionedLocation } from "@polkadot/types/lookup";
import { hexToBn, u8aToHex } from "@polkadot/util";
import chaiBn from "chai-bn";
use(chaiBn(BN)); // TODO remove

const ONE_LAOS = new BN("1000000000000000000");
const ONE_DOT = new BN("1000000000000");
const HUNDRED_DOTS = new BN("100000000000000");
const DEPOSIT = new BN(100000000000); // Deposit for creating a foreign asset, TODO get this amount from pallet
const WAITING_BLOCKS_FOR_EVENTS = 10; // Number of blocks we wait at max to receive an event

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
				calls: [createEncodedCall],
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
			let eventFound = null;
			let waitForNBlocks = WAITING_BLOCKS_FOR_EVENTS; // TODO refactor create wait for events function, and maybe add logs when fetching events
			while (WAITING_BLOCKS_FOR_EVENTS > 0 && !eventFound) {
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
			); // Alith balance has not changed

			const laosAccountBalanceDifference = balanceLaosAccountBefore.sub(
				new BN((await apiAssetHub.query.system.account(laosAccountInAssetHub)).data.free)
			);
			expect(laosAccountBalanceDifference).to.be.a.bignumber.that.equals(ONE_DOT.add(DEPOSIT));

			// The asset's found in the pallet
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

	// TODO refacto a little bit this step
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
						Concrete: hereLocation(),
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

		const laosAssetid = apiAssetHub.createType(
			"StagingXcmV3MultiLocation",
			siblingLocation(LAOS_PARA_ID)
		) as StagingXcmV3MultiLocation;
		const charlieBalanceBefore = hexToBn(
			(await apiAssetHub.query.foreignAssets.account(laosAssetid, charlie.address)).toJSON()?.["balance"] ?? "0x0"
		);
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

		// Check that LAOS has been sent in Asset Hub
		let waitForNBlocks = WAITING_BLOCKS_FOR_EVENTS; // TODO refactor create wait for events function
		let eventFound = null;
		while (waitForNBlocks > 0 && !eventFound) {
			const events = await apiAssetHub.query.system.events();
			events.filter((event) => {
				if (apiAssetHub.events.foreignAssets.Issued.is(event.event)) {
					eventFound = event;
				}
			});
			await awaitBlockChange(apiAssetHub);
			waitForNBlocks--;
		}
		expect(eventFound.event.data[0].toJSON()).to.deep.equal(laosAssetid.toJSON()); // assetId
		expect(eventFound.event.data[1].toString()).to.equal(charlie.address); // owner
		const charlieBalance = hexToBn(
			(await apiAssetHub.query.foreignAssets.account(laosAssetid, charlie.address)).toJSON()["balance"]
		);
		const realAmountRecieved = new BN(eventFound.event.data[2].toString())
		expect(charlieBalanceBefore.add(realAmountRecieved).cmp(charlieBalance)).to.be.eq(0);
	});

	step("Teleport back from AssetHub to LAOS", async function () {
		const charlie = new Keyring({ type: "sr25519" }).addFromUri("//Charlie");
		const alith = new Keyring({ type: "ethereum" }).addFromUri(ALITH_PRIVATE_KEY);

		const destination = apiAssetHub.createType("XcmVersionedLocation", {
			V3: siblingLocation(LAOS_PARA_ID),
		});

		// We need to use AssetHub api otherwise we get an error as LAOS does not use AccountId32
		let accountId = apiLaos.createType("AccountId", alith.address);
		const beneficiary = apiAssetHub.createType("XcmVersionedLocation", {
			V3: {
				parents: "0",
				interior: {
					X1: {
						AccountKey20: {
							// network: 'Any',
							key: accountId.toHex(),
						},
					},
				},
			},
		});

		const amount = ONE_LAOS.muln(1);
		const assets = apiAssetHub.createType("XcmVersionedAssets", {
			V3: [
				{
					id: {
						Concrete: siblingLocation(LAOS_PARA_ID),
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

		const laosAssetid = apiAssetHub.createType(
			"StagingXcmV3MultiLocation",
			siblingLocation(LAOS_PARA_ID)
		) as StagingXcmV3MultiLocation;
		const charlieBalanceBefore = hexToBn(
			(await apiAssetHub.query.foreignAssets.account(laosAssetid, charlie.address)).toJSON()["balance"]
		);
		const alithBalanceBefore = (await apiLaos.query.system.account(alith.address)).data.free;

		const call = apiAssetHub.tx.polkadotXcm.limitedTeleportAssets(
			destination,
			beneficiary,
			assets,
			fee_asset_item,
			weight_limit
		);
		call.signAndSend(charlie).catch((error: any) => {
			console.log("transaction failed", error);
		});

		// Check that LAOS has been sent in Asset Hub
		let waitForNBlocks = WAITING_BLOCKS_FOR_EVENTS; // TODO refactor create wait for events function
		let eventFound = null;
		while (waitForNBlocks > 0 && !eventFound) {
			const events = await apiLaos.query.system.events();
			events.filter((event) => {
				if (apiLaos.events.balances.Minted.is(event.event)) {
					eventFound = event;
				}
			});
			await awaitBlockChange(apiLaos);
			waitForNBlocks--;
		}
		expect(eventFound.event.data[0].toString()).to.equal(alith.address); // beneficiary

		const charlieBalance = hexToBn(
			(await apiAssetHub.query.foreignAssets.account(laosAssetid, charlie.address)).toJSON()["balance"]
		);
		expect(charlieBalanceBefore.sub(amount).cmp(charlieBalance)).to.be.eq(0);
		const alithBalance = (await apiLaos.query.system.account(alith.address)).data.free;
		const realAmountRecieved = new BN(eventFound.event.data[1].toString());
		expect(alithBalanceBefore.add(realAmountRecieved).cmp(alithBalance)).to.be.eq(0);
	});
});
