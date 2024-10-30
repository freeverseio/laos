import BN from "bn.js";
import { expect } from "chai";
import { step } from "mocha-steps";

import { ALITH_PRIVATE_KEY, ASSET_HUB_PARA_ID, CHECKING_ACCOUNT, LAOS_PARA_ID } from "./config";
import {
	describeWithExistingNode,
	transferBalance,
	isChannelOpen,
	sendOpenHrmpChannelTxs,
	sovereignAccountOf,
	awaitBlockChange,
	siblingLocation,
	buildXcmInstruction,
	relayLocation,
	hereLocation,
	ExtendedAssetHubApi,
	waitForEvent,
} from "./util";
import { ApiPromise, Keyring } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";
import { DoubleEncodedCall, EventRecord, MultiAddress } from "@polkadot/types/interfaces";
import { XcmVersionedLocation } from "@polkadot/types/lookup";
import { hexToBn, u8aToHex } from "@polkadot/util";
import debug from "debug";

const debugTeleport = debug("teleport");

const ONE_LAOS = new BN("1000000000000000000");
const ONE_DOT = new BN("1000000000000");
const WAITING_BLOCKS_FOR_EVENTS = 10; // Number of blocks we wait at max to receive an event

describeWithExistingNode("Teleport Asset Hub <-> LAOS", (context) => {
	const laosSiblingAccount = sovereignAccountOf(LAOS_PARA_ID);

	// APIS
	let apiAssetHub: ExtendedAssetHubApi;
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

		debugTeleport("Waiting until all the relay chain, Asset Hub and LAOS produce blocks...");
		await Promise.all([awaitBlockChange(apiRelaychain), awaitBlockChange(apiAssetHub), awaitBlockChange(apiLaos)]);

		debugTeleport("[RELAY_CHAIN] Send transaction to open HRMP channels between AssetHub and LAOS..."); // See: https://github.com/paritytech/polkadot-sdk/pull/1616
		await sendOpenHrmpChannelTxs(apiRelaychain, LAOS_PARA_ID, ASSET_HUB_PARA_ID);
	});

	step("HRMP channels between AssetHub and LAOS are open", async function () {
		expect(await isChannelOpen(apiRelaychain, LAOS_PARA_ID, ASSET_HUB_PARA_ID)).to.be.true;
		expect(await isChannelOpen(apiRelaychain, ASSET_HUB_PARA_ID, LAOS_PARA_ID)).to.be.true;
	});

	step("Create LAOS Foreign Asset in AssetHub", async function () {
		const laosForeignAssetExists = !(await apiAssetHub.query.foreignAssets.asset(apiAssetHub.laosAssetId)).isEmpty;

		// NOTE: We only create the foreign asset if it hasn't been created yet, in this way we ensure tests are idempotent
		if (!laosForeignAssetExists) {
			//Fund LAOS sovereigna account
			debugTeleport("[ASSET_HUB] Funding LAOS sovereign account...");
			await transferBalance(apiAssetHub, alice, laosSiblingAccount, ONE_DOT.muln(100));

			// Build XCM instruction
			const foreign_asset_admin = apiAssetHub.createType("MultiAddress", laosSiblingAccount) as MultiAddress;
			const createCall = apiAssetHub.tx.foreignAssets.create(
				apiAssetHub.laosAssetId,
				foreign_asset_admin,
				ONE_LAOS
			);

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
			const alithBalanceBefore = new BN((await apiLaos.query.system.account(alith.address)).data.free);
			const laosBalanceBefore = new BN((await apiAssetHub.query.system.account(laosSiblingAccount)).data.free);

			// Send the XCM instruction from LAOS too Asset Hub
			const sudoCall = apiLaos.tx.sudo.sudo(apiLaos.tx.polkadotXcm.send(destination, instruction));
			try {
				await sudoCall.signAndSend(alith);
			} catch (error) {
				console.log("transaction failed", error);
			}

			// Check if the foreign asset has been created in Asset Hub
			const event = await waitForEvent(
				apiAssetHub,
				({ event }) => apiAssetHub.events.foreignAssets.Created.is(event),
				WAITING_BLOCKS_FOR_EVENTS
			);

			expect(event).to.not.be.null;
			const [assetId, creator, owner] = event.event.data;
			expect(assetId.toString()).to.equal(apiAssetHub.laosAssetId.toString());
			expect(creator.toString()).to.equal(laosSiblingAccount);
			expect(owner.toString()).to.equal(laosSiblingAccount);

			// Check that balances are correct.
			const alithBalance = new BN((await apiLaos.query.system.account(alith.address)).data.free);
			expect(alithBalanceBefore.eq(alithBalance), "Alith balance shouldn't change");

			const laosBalance = new BN((await apiAssetHub.query.system.account(laosSiblingAccount)).data.free);
			const decreaseOfLaosBalance = laosBalanceBefore.sub(laosBalance);
			expect(
				decreaseOfLaosBalance.eq(ONE_DOT.add(apiAssetHub.consts.assets.assetDeposit)),
				"Laos balance should decrease by the XCM withdrawn amount plus the asset deposit"
			);

			expect(
				(await apiAssetHub.query.foreignAssets.asset(apiAssetHub.laosAssetId)).isEmpty,
				"LAOS foreign asset has not been created"
			).to.be.false;
		} else {
			debugTeleport("LAOS foreign asset already exists, skipping creation...");
		}
	});

	// TODO merge this step with the previous one, investigate why mint xcm has to be sent with originKind: SovereignAccount
	// whereas create xcm has to be sent with originKind: Xcm
	step("Mint LAOS foreign asset in AssetHub", async function () {
		// Build XCM instructions
		const ferdie = new Keyring({ type: "sr25519" }).addFromUri("//Ferdie");
		const ferdieMultiaddress = apiAssetHub.createType("MultiAddress", ferdie.address) as MultiAddress;
		const mintLaosCall = apiAssetHub.tx.foreignAssets.mint(
			apiAssetHub.laosAssetId,
			ferdieMultiaddress,
			ONE_LAOS.muln(10000)
		);
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
		const alithBalanceBefore = new BN((await apiLaos.query.system.account(alith.address)).data.free);
		const laosBalanceBefore = new BN((await apiAssetHub.query.system.account(laosSiblingAccount)).data.free);
		const sudoCall = apiLaos.tx.sudo.sudo(apiLaos.tx.polkadotXcm.send(destination, instruction));
		await sudoCall
			.signAndSend(alith, () => {})
			.catch((error: any) => {
				console.log("transaction failed", error);
			});

		// Check if the foreign asset has been minted in Asset Hub
		const event = await waitForEvent(
			apiAssetHub,
			({ event }) => apiAssetHub.events.foreignAssets.Issued.is(event),
			WAITING_BLOCKS_FOR_EVENTS
		);

		expect(event).to.not.be.null;
		const alithBalance = new BN((await apiLaos.query.system.account(alith.address)).data.free);
		expect(alithBalanceBefore.eq(alithBalance), "Alith balance shouldn't change");

		const laosBalance = new BN((await apiAssetHub.query.system.account(laosSiblingAccount)).data.free);
		const decreaseOfLaosBalance = laosBalanceBefore.sub(laosBalance);
		expect(decreaseOfLaosBalance.eq(ONE_DOT), "Laos should decrease XCM withdrawn amount");

		const ferdieXLaosBalance = hexToBn(
			(await apiAssetHub.query.foreignAssets.account(apiAssetHub.laosAssetId, ferdie.address)).toJSON()["balance"]
		);
		expect(ferdieXLaosBalance.gte(new BN(0)), "Ferdie balance should be > 0");
	});

	step("Create LAOS/RelayToken pool in AssetHub", async function () {
		// NOTE: We only create the pool if it hasn't been created yet, in this way we ensure tests are idempotent
		const poolExists = !(
			await apiAssetHub.query.assetConversion.pools([apiAssetHub.relayAssetId, apiAssetHub.laosAssetId])
		).isEmpty;
		if (!poolExists) {
			// Build XCM instruction to be included in xcm.send call
			const createPoolCall = apiAssetHub.tx.assetConversion.createPool(
				apiAssetHub.relayAssetId.toU8a(),
				apiAssetHub.laosAssetId.toU8a()
			);
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
			const laosBalanceBefore = new BN((await apiAssetHub.query.system.account(laosSiblingAccount)).data.free);
			const alithBalanceBefore = new BN((await apiLaos.query.system.account(alith.address)).data.free);
			const sudoCall = apiLaos.tx.sudo.sudo(apiLaos.tx.polkadotXcm.send(destination, instruction));
			await sudoCall
				.signAndSend(alith, () => {})
				.catch((error: any) => {
					console.log("transaction failed", error);
				});

			// Check that pool has been created in Asset Hub
			const event = await waitForEvent(
				apiAssetHub,
				({ event }) => apiAssetHub.events.assetConversion.PoolCreated.is(event),
				WAITING_BLOCKS_FOR_EVENTS
			);

			expect(event).to.not.be.null;
			const [creator, poolId] = event.event.data;
			expect(creator.toString()).to.equal(laosSiblingAccount);
			expect(poolId.toJSON()).to.deep.equal([relayLocation(), siblingLocation(LAOS_PARA_ID)]);
			expect(
				(await apiAssetHub.query.assetConversion.pools([apiAssetHub.relayAssetId, apiAssetHub.laosAssetId]))
					.isEmpty
			).to.be.false;

			const alithBalance = new BN((await apiLaos.query.system.account(alith.address)).data.free);
			expect(alithBalanceBefore.eq(alithBalance), "Alith balance shouldn't change");

			const laosBalance = new BN((await apiAssetHub.query.system.account(laosSiblingAccount)).data.free);
			const decreaseOfLaosBalance = laosBalanceBefore.sub(laosBalance);
			expect(
				decreaseOfLaosBalance.eq(ONE_DOT.add(apiAssetHub.consts.assets.assetAccountDeposit)),
				"Laos should decrease by the XCM withdrawn amount plus the asset account deposit"
			);
		} else {
			debugTeleport("Pool already exists, skipping creation...");
		}

		// Add liquidity to the pool
		const ferdie = new Keyring({ type: "sr25519" }).addFromUri("//Ferdie");
		const liquidityAmountLaos = new BN(ONE_LAOS.muln(1000));
		const liquidityAmountDot = new BN(ONE_DOT.muln(1000));
		const ferdieBalance = new BN((await apiAssetHub.query.system.account(ferdie.address)).data.free);
		const ferdieXLaosBalance = hexToBn(
			(await apiAssetHub.query.foreignAssets.account(apiAssetHub.laosAssetId, ferdie.address)).toJSON()["balance"]
		);
		expect(
			ferdieBalance.gte(liquidityAmountDot),
			"Ferdie's DOT balance should be greater than the amount to be sent to the pool"
		);
		expect(
			ferdieXLaosBalance.gte(liquidityAmountLaos),
			"Ferdie's LAOS balance should be greater than the amount to be sent to the pool"
		);

		await apiAssetHub.tx.assetConversion
			.addLiquidity(
				apiAssetHub.relayAssetId.toU8a(),
				apiAssetHub.laosAssetId.toU8a(),
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

		const charlieBalanceBefore = hexToBn(
			(await apiAssetHub.query.foreignAssets.account(apiAssetHub.laosAssetId, charlie.address)).toJSON()?.[
				"balance"
			] ?? "0x0"
		);
		const alithBalanceBefore = (await apiLaos.query.system.account(alith.address)).data.free;
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
		const event = await waitForEvent(
			apiAssetHub,
			({ event }) => apiAssetHub.events.foreignAssets.Issued.is(event),
			WAITING_BLOCKS_FOR_EVENTS
		);

		expect(event).to.not.be.null;
		const [assetId, owner, realAmountReceived] = event.event.data;
		expect(assetId.toJSON()).to.deep.equal(apiAssetHub.laosAssetId.toJSON());
		expect(owner.toString()).to.equal(charlie.address);
		const charlieBalance = hexToBn(
			(await apiAssetHub.query.foreignAssets.account(apiAssetHub.laosAssetId, charlie.address)).toJSON()[
				"balance"
			]
		);
		expect(
			charlieBalanceBefore.add(new BN(realAmountReceived.toString())).eq(charlieBalance),
			"Charlie's balance should increase by the amount received"
		);
		const realAlithBalance = (await apiLaos.query.system.account(alith.address)).data.free;
		const alithBalance = alithBalanceBefore.sub(amount);
		expect(
			alithBalance.sub(realAlithBalance).lte(ONE_DOT),
			"Alith's balance should decrease by the amount teleported, disregarding fees"
		);
	});

	step("Teleport back from AssetHub to LAOS", async function () {
		const charlie = new Keyring({ type: "sr25519" }).addFromUri("//Charlie");

		const destination = apiAssetHub.createType("XcmVersionedLocation", {
			V3: siblingLocation(LAOS_PARA_ID),
		});

		// We need to use AssetHub api otherwise we get an error as LAOS does not use AccountId32
		let beneficiaryAddress = "0x0000000000000000000000000000000000000001";
		const beneficiary = apiAssetHub.createType("XcmVersionedLocation", {
			V3: {
				parents: "0",
				interior: {
					X1: {
						AccountKey20: {
							// network: 'Any',
							key: beneficiaryAddress,
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

		const charlieBalanceBefore = hexToBn(
			(await apiAssetHub.query.foreignAssets.account(apiAssetHub.laosAssetId, charlie.address)).toJSON()[
				"balance"
			]
		);
		const beneficiaryBalanceBefore = (await apiLaos.query.system.account(beneficiaryAddress)).data.free;

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

		// Check that LAOS has been sent back in LAOS
		const event = await waitForEvent(
			apiLaos,
			({ event }) => {
				return apiLaos.events.balances.Minted.is(event) && event.data[0].toString() !== CHECKING_ACCOUNT;
			},
			WAITING_BLOCKS_FOR_EVENTS
		);

		expect(event).to.not.be.null;
		const [receiver, realAmountReceived] = event.event.data;
		expect(receiver.toString()).to.equal(beneficiaryAddress);
		const charlieBalance = hexToBn(
			(await apiAssetHub.query.foreignAssets.account(apiAssetHub.laosAssetId, charlie.address)).toJSON()[
				"balance"
			]
		);
		expect(
			charlieBalanceBefore.sub(amount).eq(charlieBalance),
			"Charlie's balance should decrease by the amount teleported"
		);
		const beneficiaryBalance = (await apiLaos.query.system.account(beneficiaryAddress)).data.free;
		expect(
			beneficiaryBalanceBefore.add(new BN(realAmountReceived.toString())).eq(beneficiaryBalance),
			"Alith's balance should increase by the amount received in the teleport"
		);
	});
});
