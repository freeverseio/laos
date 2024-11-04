import BN from "bn.js";
import { expect } from "chai";
import { step } from "mocha-steps";

import { ASSET_HUB_PARA_ID, CHECKING_ACCOUNT, LAOS_PARA_ID } from "./config";
import {
	describeWithExistingNode,
	transferBalance,
	isChannelOpen,
	sendOpenHrmpChannelTxs,
	awaitBlockChange,
	siblingLocation,
	buildXcmInstruction,
	relayLocation,
	hereLocation,
	waitForEvent,
} from "./util";
import { ApiPromise } from "@polkadot/api";
import { DoubleEncodedCall } from "@polkadot/types/interfaces";
import { hexToBn, u8aToHex } from "@polkadot/util";
import debug from "debug";

const debugTeleport = debug("teleport");

const ONE_LAOS = new BN("1000000000000000000");
const ONE_DOT = new BN("1000000000000");
const WAITING_BLOCKS_FOR_EVENTS = 20; // Number of blocks we wait at max to receive an event

describeWithExistingNode(
	"Teleport Asset Hub <-> LAOS",
	function () {
		// APIS
		let apiAssetHub: ApiPromise;
		let apiLaos: ApiPromise;
		let apiRelaychain: ApiPromise;

		before(async function () {
			// Initialize the APIs
      apiAssetHub = this.context.networks.assetHub;
			apiLaos = this.context.networks.laos;
			apiRelaychain = this.context.networks.relaychain;

			debugTeleport("Waiting until all the relay chain, Asset Hub and LAOS produce blocks...");
			await Promise.all([
				awaitBlockChange(apiRelaychain),
				awaitBlockChange(apiAssetHub),
				awaitBlockChange(apiLaos),
			]);

			debugTeleport("[RELAY_CHAIN] Send transaction to open HRMP channels between AssetHub and LAOS..."); // See: https://github.com/paritytech/polkadot-sdk/pull/1616
			await sendOpenHrmpChannelTxs(apiRelaychain, LAOS_PARA_ID, ASSET_HUB_PARA_ID);
		});

		step("HRMP channels between AssetHub and LAOS are open", async function () {
			expect(await isChannelOpen(apiRelaychain, LAOS_PARA_ID, ASSET_HUB_PARA_ID)).to.be.true;
			expect(await isChannelOpen(apiRelaychain, ASSET_HUB_PARA_ID, LAOS_PARA_ID)).to.be.true;
		});

		step("Create LAOS Foreign Asset in AssetHub", async function () {
			const laosForeignAssetExists = !(await apiAssetHub.query.foreignAssets.asset(this.assetHubItems.laosLocation))
				.isEmpty;

			// NOTE: We only create the foreign asset if it hasn't been created yet, in this way we ensure tests are idempotent
			if (!laosForeignAssetExists) {
				//Fund LAOS sovereigna account
				debugTeleport("[ASSET_HUB] Funding LAOS sovereign account...");
				await transferBalance(apiAssetHub, this.substratePairs.alice, this.assetHubItems.laosSA, ONE_DOT.muln(100));

				// Build XCM instruction
				const createCall = apiAssetHub.tx.foreignAssets.create(
					this.assetHubItems.laosLocation,
					this.assetHubItems.multiAddresses.laosSA,
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

				// Get balances before teleporting.
				const alithBalanceBefore = new BN(
					(await apiLaos.query.system.account(this.ethereumPairs.alith.address as string)).data.free
				);
				const laosBalanceBefore = new BN(
					(await apiAssetHub.query.system.account(this.assetHubItems.laosSA as string)).data.free
				);

				// Send the XCM instruction from LAOS too Asset Hub
				const sudoCall = apiLaos.tx.sudo.sudo(
					apiLaos.tx.polkadotXcm.send(this.laosItems.assetHubLocation, instruction)
				);

				try {
					await sudoCall.signAndSend(this.ethereumPairs.alith);
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
				expect(assetId.toString()).to.equal(this.assetHubItems.laosLocation.toString());
				expect(creator.toString()).to.equal(this.assetHubItems.laosSA);
				expect(owner.toString()).to.equal(this.assetHubItems.laosSA);

				// Check that balances are correct.
				const alithBalance = new BN(
					(await apiLaos.query.system.account(this.ethereumPairs.alith.address as string)).data.free
				);
				expect(alithBalanceBefore.eq(alithBalance), "Alith balance shouldn't change");

				const laosBalance = new BN((await apiAssetHub.query.system.account(this.assetHubItems.laosSA as string)).data.free);
				const decreaseOfLaosBalance = laosBalanceBefore.sub(laosBalance);
				expect(
					decreaseOfLaosBalance.eq(ONE_DOT.add(apiAssetHub.consts.assets.assetDeposit)),
					"Laos balance should decrease by the XCM withdrawn amount plus the asset deposit"
				);

				expect(
					(await apiAssetHub.query.foreignAssets.asset(this.assetHubItems.laosLocation)).isEmpty,
					"LAOS foreign asset has not been created"
				).to.be.false;
			} else {
				debugTeleport("LAOS foreign asset already exists, skipping creation...");
			}
		});

		step("Mint LAOS foreign asset in AssetHub", async function () {
			// Build XCM instructions
			const mintLaosCall = apiAssetHub.tx.foreignAssets.mint(
				this.assetHubItems.laosLocation,
				this.assetHubItems.multiAddresses.ferdie,
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

			// Send the XCM instruction from Laos to Asset Hub
			const alithBalanceBefore = new BN(
				(await apiLaos.query.system.account(this.ethereumPairs.alith.address as string)).data.free
			);
			const laosBalanceBefore = new BN((await apiAssetHub.query.system.account(this.assetHubItems.laosSA as string)).data.free);
			const sudoCall = apiLaos.tx.sudo.sudo(apiLaos.tx.polkadotXcm.send(this.laosItems.assetHubLocation, instruction));
			try {
				await sudoCall.signAndSend(this.ethereumPairs.alith);
			} catch (error) {
				console.log("transaction failed", error);
			}

			// Check if the foreign asset has been minted in Asset Hub
			const event = await waitForEvent(
				apiAssetHub,
				({ event }) => apiAssetHub.events.foreignAssets.Issued.is(event),
				WAITING_BLOCKS_FOR_EVENTS
			);

			expect(event).to.not.be.null;
			const alithBalance = new BN((await apiLaos.query.system.account(this.ethereumPairs.alith.address as string)).data.free);
			expect(alithBalanceBefore.eq(alithBalance), "Alith balance shouldn't change");

			const laosBalance = new BN((await apiAssetHub.query.system.account(this.assetHubItems.laosSA as string)).data.free);
			const decreaseOfLaosBalance = laosBalanceBefore.sub(laosBalance);
			expect(decreaseOfLaosBalance.eq(ONE_DOT), "Laos should decrease XCM withdrawn amount");

			const ferdieXLaosBalance = hexToBn(
				(
					await apiAssetHub.query.foreignAssets.account(
						this.assetHubItems.laosLocation,
						this.substratePairs.ferdie.address
					)
				).toJSON()["balance"]
			);
			expect(ferdieXLaosBalance.gte(new BN(0)), "Ferdie balance should be > 0");
		});

		step("Create LAOS/RelayToken pool in AssetHub", async function () {
			// NOTE: We only create the pool if it hasn't been created yet, in this way we ensure tests are idempotent
			const poolExists = !(
				await apiAssetHub.query.assetConversion.pools([
					this.assetHubItems.relayChainLocation,
					this.assetHubItems.laosLocation,
				])
			).isEmpty;

			if (!poolExists) {
				// Build XCM instruction to be included in xcm.send call
				const createPoolCall = apiAssetHub.tx.assetConversion.createPool(
					this.assetHubItems.relayChainLocation.toU8a(),
					this.assetHubItems.laosLocation.toU8a()
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

				// Send the XCM instruction from Laos to Asset Hub
				const laosBalanceBefore = new BN(
					(await apiAssetHub.query.system.account(this.assetHubItems.laosSA as string)).data.free
				);
				const alithBalanceBefore = new BN(
					(await apiLaos.query.system.account(this.ethereumPairs.alith.address as string)).data.free
				);
				const sudoCall = apiLaos.tx.sudo.sudo(
					apiLaos.tx.polkadotXcm.send(this.laosItems.assetHubLocation, instruction)
				);

				try {
					await sudoCall.signAndSend(this.ethereumPairs.alith);
				} catch (error) {
					console.log("transaction failed", error);
				}

				// Check that pool has been created in Asset Hub
				const event = await waitForEvent(
					apiAssetHub,
					({ event }) => apiAssetHub.events.assetConversion.PoolCreated.is(event),
					WAITING_BLOCKS_FOR_EVENTS
				);

				expect(event).to.not.be.null;
				const [creator, poolId] = event.event.data;
				expect(creator.toString()).to.equal(this.assetHubItems.laosSA);
				expect(poolId.toJSON()).to.deep.equal([relayLocation(), this.assetHubItems.laosSA]);
				expect(
					(
						await apiAssetHub.query.assetConversion.pools([
							this.assetHubItems.relayChainLocation,
							this.assetHubItems.laosLocation,
						])
					).isEmpty
				).to.be.false;

				const alithBalance = new BN(
					(await apiLaos.query.system.account(this.ethereumPairs.alith.address as string)).data.free
				);
				expect(alithBalanceBefore.eq(alithBalance), "Alith balance shouldn't change");

				const laosBalance = new BN((await apiAssetHub.query.system.account(this.assetHubItems.laosSA as string)).data.free);
				const decreaseOfLaosBalance = laosBalanceBefore.sub(laosBalance);
				expect(
					decreaseOfLaosBalance.eq(ONE_DOT.add(apiAssetHub.consts.assets.assetAccountDeposit)),
					"Laos should decrease by the XCM withdrawn amount plus the asset account deposit"
				);
			} else {
				debugTeleport("Pool already exists, skipping creation...");
			}

			// Add liquidity to the pool
			const liquidityAmountLaos = new BN(ONE_LAOS.muln(1000));
			const liquidityAmountDot = new BN(ONE_DOT.muln(1000));
			const ferdieBalance = new BN(
				(await apiAssetHub.query.system.account(this.substratePairs.ferdie.address as string)).data.free
			);
			const ferdieXLaosBalance = hexToBn(
				(
					await apiAssetHub.query.foreignAssets.account(
						this.assetHubItems.laosLocation,
						this.substratePairs.ferdie.address
					)
				).toJSON()["balance"]
			);
			expect(
				ferdieBalance.gte(liquidityAmountDot),
				"Ferdie's DOT balance should be greater than the amount to be sent to the pool"
			);
			expect(
				ferdieXLaosBalance.gte(liquidityAmountLaos),
				"Ferdie's LAOS balance should be greater than the amount to be sent to the pool"
			);

			try {
				await apiAssetHub.tx.assetConversion
					.addLiquidity(
						this.assetHubItems.relayChainLocation.toU8a(),
						this.assetHubItems.laosLocation.toU8a(),
						liquidityAmountDot,
						liquidityAmountLaos,
						liquidityAmountDot.sub(new BN(ONE_DOT.muln(10))),
						liquidityAmountLaos.sub(new BN(ONE_LAOS.muln(10))),
						this.substratePairs.ferdie.address
					)
					.signAndSend(this.substratePairs.ferdie);
			} catch (error) {
				console.log("transaction failed", error);
			}
		});

		step("Teleport from LAOS to AssetHub", async function () {
			const beneficiary = apiLaos.createType("XcmVersionedLocation", {
				V3: {
					parents: "0",
					interior: {
						X1: {
							AccountId32: {
								// network: 'Any',
								id: this.assetHubItems.accounts.charlie.toHex(),
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
			const fee_asset_item = "0";
			const weight_limit = "Unlimited";

			const charlieBalanceBefore = hexToBn(
				(
					await apiAssetHub.query.foreignAssets.account(
						this.assetHubItems.laosLocation,
						this.substratePairs.charlie.address
					)
				).toJSON()?.["balance"] ?? "0x0"
			);
			const alithBalanceBefore = (await apiLaos.query.system.account(this.ethereumPairs.alith.address as string)).data.free;

			const call = apiLaos.tx.polkadotXcm.limitedTeleportAssets(
				this.laosItems.assetHubLocation,
				beneficiary,
				assets,
				fee_asset_item,
				weight_limit
			);

			try {
				await call.signAndSend(this.ethereumPairs.alith);
			} catch (error) {
				console.log("transaction failed", error);
			}

			// Check that LAOS has been sent in Asset Hub
			const event = await waitForEvent(
				apiAssetHub,
				({ event }) => apiAssetHub.events.foreignAssets.Issued.is(event),
				WAITING_BLOCKS_FOR_EVENTS
			);

			expect(event).to.not.be.null;
			const [assetId, owner, realAmountReceived] = event.event.data;
			expect(assetId.toJSON()).to.deep.equal(this.assetHubItems.laosLocation.toJSON());
			expect(owner.toString()).to.equal(this.substratePairs.charlie.address);
			const charlieBalance = hexToBn(
				(
					await apiAssetHub.query.foreignAssets.account(
						this.assetHubItems.laosLocation,
						this.substratePairs.charlie.address
					)
				).toJSON()["balance"]
			);
			expect(
				charlieBalanceBefore.add(new BN(realAmountReceived.toString())).eq(charlieBalance),
				"Charlie's balance should increase by the amount received"
			);
			const realAlithBalance = (await apiLaos.query.system.account(this.ethereumPairs.alith.address as string)).data.free;
			const alithBalance = alithBalanceBefore.sub(amount);
			expect(
				alithBalance.sub(realAlithBalance).lte(ONE_DOT),
				"Alith's balance should decrease by the amount teleported, disregarding fees"
			);
		});

		step("Teleport back from AssetHub to LAOS", async function () {
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
			const fee_asset_item = "0";
			const weight_limit = "Unlimited";

			const charlieBalanceBefore = hexToBn(
				(
					await apiAssetHub.query.foreignAssets.account(
						this.assetHubItems.laosLocation,
						this.substratePairs.charlie.address
					)
				).toJSON()["balance"]
			);
			const beneficiaryBalanceBefore = (await apiLaos.query.system.account(beneficiaryAddress)).data.free;

			const call = apiAssetHub.tx.polkadotXcm.limitedTeleportAssets(
				this.assetHubItems.laosLocation,
				beneficiary,
				assets,
				fee_asset_item,
				weight_limit
			);

			try {
				await call.signAndSend(this.substratePairs.charlie);
			} catch (error) {
				console.log("transaction failed", error);
			}

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
				(
					await apiAssetHub.query.foreignAssets.account(
						this.assetHubItems.laosLocation,
						this.substratePairs.charlie.address
					)
				).toJSON()["balance"]
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
	}
);
