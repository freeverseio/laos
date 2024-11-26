import BN from "bn.js";
import { expect } from "chai";
import { step } from "mocha-steps";

import { ASSET_HUB_PARA_ID, CHECKING_ACCOUNT, LAOS_PARA_ID } from "./config";
import {
	describeWithExistingNode,
	transferBalance,
	isChannelOpen,
	sendOpenHrmpChannelTxs,
	waitForBlocks,
	siblingLocation,
	buildXcmInstruction,
	relayLocation,
	hereLocation,
	getFinalizedBlockNumber,
	checkEventInBlock,
	checkEventAfterXcm,
	sendTxAndWaitForFinalization,
} from "./util";
import { ApiPromise } from "@polkadot/api";
import { DoubleEncodedCall } from "@polkadot/types/interfaces";
import { hexToBn, u8aToHex } from "@polkadot/util";
import debug from "debug";

const debugTeleport = debug("teleport");

const ONE_LAOS = new BN("1000000000000000000");
const ONE_DOT = new BN("1000000000000");

describeWithExistingNode(
	"Teleport Asset Hub <-> LAOS",
	function () {
		// APIS
		let apiAssetHub: ApiPromise;
		let apiLaos: ApiPromise;
		let apiRelaychain: ApiPromise;

		before(async function () {
			// Initialize the APIs
			apiAssetHub = this.chains.assetHub;
			apiLaos = this.chains.laos;
			apiRelaychain = this.chains.relaychain;

			debugTeleport("Waiting until all the relay chain, Asset Hub and LAOS produce blocks...");
			await Promise.all([
				waitForBlocks(apiRelaychain, 1),
				waitForBlocks(apiAssetHub, 1),
				waitForBlocks(apiLaos, 1),
			]);

			debugTeleport("[RELAY_CHAIN] Send transaction to open HRMP channels between AssetHub and LAOS..."); // See: https://github.com/paritytech/polkadot-sdk/pull/1616
			await sendOpenHrmpChannelTxs(apiRelaychain, LAOS_PARA_ID, ASSET_HUB_PARA_ID);
		});

		step("HRMP channels between AssetHub and LAOS are open", async function () {
			expect(await isChannelOpen(apiRelaychain, LAOS_PARA_ID, ASSET_HUB_PARA_ID)).to.be.true;
			expect(await isChannelOpen(apiRelaychain, ASSET_HUB_PARA_ID, LAOS_PARA_ID)).to.be.true;
		});

		step("Create $LAOS in AssetHub", async function () {
			const laosForeignAssetExists = !(await apiAssetHub.query.foreignAssets.asset(this.assetHubItems.laosAsset))
				.isEmpty;

			// NOTE: We only create the foreign asset if it hasn't been created yet, in this way we ensure tests are idempotent
			if (!laosForeignAssetExists) {
				//Fund LAOS sovereigna account
				debugTeleport("[ASSET_HUB] Funding LAOS sovereign account...");
				await transferBalance(
					apiAssetHub,
					this.substratePairs.alice,
					this.assetHubItems.laosSA,
					ONE_DOT.muln(100)
				);

				// Build XCM instruction
				const createCall = apiAssetHub.tx.foreignAssets.create(
					this.assetHubItems.laosAsset,
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

        const assetHubBestBlockBeforeSending = await getFinalizedBlockNumber(apiAssetHub);
				await sendTxAndWaitForFinalization(apiLaos, sudoCall, this.ethereumPairs.alith);

				// Check if the foreign asset has been created in Asset Hub
				const event = await checkEventAfterXcm(
					apiAssetHub,
					({ event }) => apiAssetHub.events.foreignAssets.Created.is(event),
					assetHubBestBlockBeforeSending
				);

				expect(event).to.not.be.null;
				const [assetId, creator, owner] = event.event.data;
				expect(assetId.toString()).to.equal(this.assetHubItems.laosAsset.toString());
				expect(creator.toString()).to.equal(this.assetHubItems.laosSA);
				expect(owner.toString()).to.equal(this.assetHubItems.laosSA);

				// Check that balances are correct.
				const alithBalance = new BN(
					(await apiLaos.query.system.account(this.ethereumPairs.alith.address as string)).data.free
				);
				expect(alithBalanceBefore.eq(alithBalance), "Alith balance shouldn't change");

				const laosBalance = new BN(
					(await apiAssetHub.query.system.account(this.assetHubItems.laosSA as string)).data.free
				);
				const decreaseOfLaosBalance = laosBalanceBefore.sub(laosBalance);
				expect(
					decreaseOfLaosBalance.eq(ONE_DOT.add(apiAssetHub.consts.assets.assetDeposit)),
					"Laos balance should decrease by the XCM withdrawn amount plus the asset deposit"
				);

				expect(
					(await apiAssetHub.query.foreignAssets.asset(this.assetHubItems.laosAsset)).isEmpty,
					"$LAOS foreign asset has not been created"
				).to.be.false;
			} else {
				debugTeleport("$LAOS foreign asset already exists, skipping creation...");
			}
		});

		step("Mint $LAOS in AssetHub", async function () {
			// Build XCM instructions
			const mintLaosCall = apiAssetHub.tx.foreignAssets.mint(
				this.assetHubItems.laosAsset,
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
			const laosBalanceBefore = new BN(
				(await apiAssetHub.query.system.account(this.assetHubItems.laosSA as string)).data.free
			);
			const sudoCall = apiLaos.tx.sudo.sudo(
				apiLaos.tx.polkadotXcm.send(this.laosItems.assetHubLocation, instruction)
			);

			const assetHubBestBlockBeforeSending = await getFinalizedBlockNumber(apiAssetHub);
			await sendTxAndWaitForFinalization(apiLaos, sudoCall, this.ethereumPairs.alith);

			// Check if the foreign asset has been minted in Asset Hub
			const event = await checkEventAfterXcm(
				apiAssetHub,
				({ event }) => apiAssetHub.events.foreignAssets.Issued.is(event),
				assetHubBestBlockBeforeSending
			);

			expect(event).to.not.be.null;
			const alithBalance = new BN(
				(await apiLaos.query.system.account(this.ethereumPairs.alith.address as string)).data.free
			);
			expect(alithBalanceBefore.eq(alithBalance), "Alith balance shouldn't change");

			const laosBalance = new BN(
				(await apiAssetHub.query.system.account(this.assetHubItems.laosSA as string)).data.free
			);
			const decreaseOfLaosBalance = laosBalanceBefore.sub(laosBalance);
			expect(decreaseOfLaosBalance.eq(ONE_DOT), "Laos should decrease XCM withdrawn amount");

			const ferdieXLaosBalance = hexToBn(
				(
					await apiAssetHub.query.foreignAssets.account(
						this.assetHubItems.laosAsset,
						this.substratePairs.ferdie.address
					)
				).toJSON()["balance"]
			);
			expect(ferdieXLaosBalance.gte(new BN(0)), "Ferdie balance should be > 0");
		});

		step("Create $LAOS/$DOT pool in AssetHub", async function () {
			// NOTE: We only create the pool if it hasn't been created yet, in this way we ensure tests are idempotent
			const poolExists = !(
				await apiAssetHub.query.assetConversion.pools([
					this.assetHubItems.relayAsset,
					this.assetHubItems.laosAsset,
				])
			).isEmpty;

			if (!poolExists) {
				// Build XCM instruction to be included in xcm.send call
				const createPoolCall = apiAssetHub.tx.assetConversion.createPool(
					this.assetHubItems.relayAsset.toU8a(),
					this.assetHubItems.laosAsset.toU8a()
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

        const assetHubBestBlockBeforeSending = await getFinalizedBlockNumber(apiAssetHub);
				await sendTxAndWaitForFinalization(apiLaos, sudoCall, this.ethereumPairs.alith);

				// Check that pool has been created in Asset Hub
				const event = await checkEventAfterXcm(
					apiAssetHub,
					({ event }) => apiAssetHub.events.assetConversion.PoolCreated.is(event),
					assetHubBestBlockBeforeSending
				);

				expect(event).to.not.be.null;
				const [creator, poolId] = event.event.data;
				expect(creator.toString()).to.equal(this.assetHubItems.laosSA);
				expect(poolId.toJSON()).to.deep.equal([relayLocation(), siblingLocation(LAOS_PARA_ID)]);
				expect(
					(
						await apiAssetHub.query.assetConversion.pools([
							this.assetHubItems.relayAsset,
							this.assetHubItems.laosAsset,
						])
					).isEmpty
				).to.be.false;

				const alithBalance = new BN(
					(await apiLaos.query.system.account(this.ethereumPairs.alith.address as string)).data.free
				);
				expect(alithBalanceBefore.eq(alithBalance), "Alith balance shouldn't change");

				const laosBalance = new BN(
					(await apiAssetHub.query.system.account(this.assetHubItems.laosSA as string)).data.free
				);
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
						this.assetHubItems.laosAsset,
						this.substratePairs.ferdie.address
					)
				).toJSON()["balance"]
			);
			expect(
				ferdieBalance.gte(liquidityAmountDot),
				"Ferdie's $DOT balance should be greater than the amount to be sent to the pool"
			);
			expect(
				ferdieXLaosBalance.gte(liquidityAmountLaos),
				"Ferdie's $LAOS balance should be greater than the amount to be sent to the pool"
			);

			const call = apiAssetHub.tx.assetConversion.addLiquidity(
				this.assetHubItems.relayAsset.toU8a(),
				this.assetHubItems.laosAsset.toU8a(),
				liquidityAmountDot,
				liquidityAmountLaos,
				liquidityAmountDot.sub(new BN(ONE_DOT.muln(10))),
				liquidityAmountLaos.sub(new BN(ONE_LAOS.muln(10))),
				this.substratePairs.ferdie.address
			);

			await sendTxAndWaitForFinalization(apiAssetHub, call, this.substratePairs.ferdie);
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
						this.assetHubItems.laosAsset,
						this.substratePairs.charlie.address
					)
				).toJSON()?.["balance"] ?? "0x0"
			);
			const alithBalanceBefore = (await apiLaos.query.system.account(this.ethereumPairs.alith.address as string))
				.data.free;

			const call = apiLaos.tx.polkadotXcm.limitedTeleportAssets(
				this.laosItems.assetHubLocation,
				beneficiary,
				assets,
				fee_asset_item,
				weight_limit
			);

      const assetHubBestBlockBeforeSending = await getFinalizedBlockNumber(apiAssetHub);
			await sendTxAndWaitForFinalization(apiLaos, call, this.ethereumPairs.alith);

			// Check that $LAOS has been sent in Asset Hub
			const event = await checkEventAfterXcm(
				apiAssetHub,
				({ event }) => apiAssetHub.events.foreignAssets.Issued.is(event),
				assetHubBestBlockBeforeSending
			);

			expect(event).to.not.be.null;
			const [assetId, owner, realAmountReceived] = event.event.data;
			expect(assetId.toJSON()).to.deep.equal(this.assetHubItems.laosAsset.toJSON());
			expect(owner.toString()).to.equal(this.substratePairs.charlie.address);
			const charlieBalance = hexToBn(
				(
					await apiAssetHub.query.foreignAssets.account(
						this.assetHubItems.laosAsset,
						this.substratePairs.charlie.address
					)
				).toJSON()["balance"]
			);
			expect(
				charlieBalanceBefore.add(new BN(realAmountReceived.toString())).eq(charlieBalance),
				"Charlie's balance should increase by the amount received"
			);
			const realAlithBalance = (await apiLaos.query.system.account(this.ethereumPairs.alith.address as string))
				.data.free;
			const alithBalance = alithBalanceBefore.sub(amount);
			expect(
				alithBalance.sub(realAlithBalance).lte(ONE_DOT),
				"Alith's balance should decrease by the amount teleported, disregarding fees"
			);
		});

		step("Teleport back from AssetHub to Laos", async function () {
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
						this.assetHubItems.laosAsset,
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

      const laosBestBlockBeforeSending = await getFinalizedBlockNumber(apiLaos);
			await sendTxAndWaitForFinalization(apiAssetHub, call, this.substratePairs.charlie);
			// Check that $LAOS has been sent back to Laos
			const event = await checkEventAfterXcm(
				apiLaos,
				({ event }) => {
					return apiLaos.events.balances.Minted.is(event) && event.data[0].toString() !== CHECKING_ACCOUNT;
				},
				laosBestBlockBeforeSending
			);

			expect(event).to.not.be.null;
			const [receiver, realAmountReceived] = event.event.data;
			expect(receiver.toString()).to.equal(beneficiaryAddress);
			const charlieBalance = hexToBn(
				(
					await apiAssetHub.query.foreignAssets.account(
						this.assetHubItems.laosAsset,
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
	},
	true
);
