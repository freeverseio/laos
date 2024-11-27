import BN from "bn.js";
import { expect } from "chai";
import { step } from "mocha-steps";

import { ASSET_HUB_PARA_ID, CHECKING_ACCOUNT, LAOS_PARA_ID } from "./config";
import { describeWithExistingNode } from "./util";
import {
	siblingParachainLocation,
	relayChainLocation,
	hereLocation,
	checkEventAfterXcm,
	buildXcmInstruction,
	openHrmpChannels,
} from "@utils/xcm";
import { sendTxAndWaitForFinalization, sendTxAndWaitForFinalizationRococo } from "@utils/transactions";
import { getFinalizedBlockNumber, checkEventInBlock } from "@utils/blocks";
import { DoubleEncodedCall } from "@polkadot/types/interfaces";
import { hexToBn, u8aToHex } from "@polkadot/util";
import debug from "debug";

const debugTeleport = debug("teleport");

const ONE_LAOS = new BN("1000000000000000000");
const ONE_DOT = new BN("1000000000000");

describeWithExistingNode(
	"Teleport Asset Hub <-> LAOS",
	function () {
		step("Open HRMP channels between AssetHub and LAOS", async function () {
			await openHrmpChannels(this.chains.relaychain, LAOS_PARA_ID, ASSET_HUB_PARA_ID);
			expect(
				(
					await this.chains.relaychain.query.hrmp.hrmpChannels({
						sender: LAOS_PARA_ID,
						recipient: ASSET_HUB_PARA_ID,
					})
				).isEmpty
			).to.be.false;
			expect(
				(
					await this.chains.relaychain.query.hrmp.hrmpChannels({
						sender: ASSET_HUB_PARA_ID,
						recipient: LAOS_PARA_ID,
					})
				).isEmpty
			).to.be.false;
		});

		step("Create $LAOS in AssetHub", async function () {
			const laosForeignAssetExists = !(
				await this.chains.assetHub.query.foreignAssets.asset(this.assetHubItems.laosAsset)
			).isEmpty;

			// NOTE: We only create the foreign asset if it hasn't been created yet, in this way we ensure tests are idempotent
			if (!laosForeignAssetExists) {
				// Transfer some funds from alice to the LAOS SA in Asset Hub to ensure it can pay for fees.
				let transferBalanceTx = this.chains.assetHub.tx.balances.transferKeepAlive(
					this.assetHubItems.laosSA,
					ONE_DOT.muln(100)
				);
				await sendTxAndWaitForFinalizationRococo(
					this.chains.relaychain,
					transferBalanceTx,
					this.substratePairs.alice,
					this.chains.assetHub
				);

				// Build XCM instruction
				const createCall = this.chains.assetHub.tx.foreignAssets.create(
					this.assetHubItems.laosAsset,
					this.assetHubItems.multiAddresses.laosSA,
					ONE_LAOS
				);

				const createEncodedCall = this.chains.laos.createType("DoubleEncodedCall", {
					encoded: u8aToHex(createCall.method.toU8a()),
				}) as DoubleEncodedCall;

				const instruction = buildXcmInstruction(
					this.chains.laos,
					relayChainLocation(),
					[createEncodedCall],
					new BN(1000000000),
					new BN(5000),
					ONE_DOT,
					this.chains.laos.createType("XcmOriginKind", "Xcm")
				);

				const call = this.chains.laos.tx.sudo.sudo(
					this.chains.laos.tx.polkadotXcm.send(this.laosItems.assetHubLocation, instruction)
				);

				// Get assetHub best finalized block before sending the XCM.
				const assetHubBestBlockBeforeSending = await getFinalizedBlockNumber(this.chains.assetHub);

				// Send the XCM and wait til the tx is included in a finalized block in LAOS.
				await sendTxAndWaitForFinalization(this.chains.laos, call, this.ethereumPairs.alith);

				// Check that the foreign asset created event has been emitted in AssetHub.
				const event = await checkEventAfterXcm(
					this.chains.assetHub,
					({ event }) => this.chains.assetHub.events.foreignAssets.Created.is(event),
					assetHubBestBlockBeforeSending
				);

				expect(event).to.not.be.null;
				const [assetId, creator, owner] = event.event.data;
				expect(assetId.toString()).to.equal(this.assetHubItems.laosAsset.toString());
				expect(creator.toString()).to.equal(this.assetHubItems.laosSA);
				expect(owner.toString()).to.equal(this.assetHubItems.laosSA);

				// Check that the asset exists in asset hub.
				expect(
					(await this.chains.assetHub.query.foreignAssets.asset(this.assetHubItems.laosAsset)).isEmpty,
					"$LAOS foreign asset has not been created"
				).to.be.false;
			} else {
				debugTeleport("$LAOS foreign asset already exists, skipping creation...");
			}
		});

		step("Mint $LAOS in AssetHub", async function () {
			const MINTED_AMOUNT = ONE_LAOS.muln(10000);
			// Build XCM instructions
			const mintLaosCall = this.chains.assetHub.tx.foreignAssets.mint(
				this.assetHubItems.laosAsset,
				this.assetHubItems.multiAddresses.ferdie,
				MINTED_AMOUNT
			);

			const mintLaosEncodedCall = this.chains.laos.createType("DoubleEncodedCall", {
				encoded: u8aToHex(mintLaosCall.method.toU8a()),
			}) as DoubleEncodedCall;

			const instruction = buildXcmInstruction(
				this.chains.laos,
				relayChainLocation(),
				[mintLaosEncodedCall],
				new BN(2000000000),
				new BN(7000),
				ONE_DOT,
				this.chains.laos.createType("XcmOriginKind", "SovereignAccount")
			);

			// Ferdie's balance before the minting
			const ferdieLaosBalanceBefore = hexToBn(
				(
					await this.chains.assetHub.query.foreignAssets.account(
						this.assetHubItems.laosAsset,
						this.substratePairs.ferdie.address
					)
				).toJSON()?.["balance"] ?? 0
			);

			const call = this.chains.laos.tx.sudo.sudo(
				this.chains.laos.tx.polkadotXcm.send(this.laosItems.assetHubLocation, instruction)
			);

			const assetHubBestBlockBeforeSending = await getFinalizedBlockNumber(this.chains.assetHub);

			await sendTxAndWaitForFinalization(this.chains.laos, call, this.ethereumPairs.alith);

			// Check that the foreign asset minted event happened in AH.
			const event = await checkEventAfterXcm(
				this.chains.assetHub,
				({ event }) => this.chains.assetHub.events.foreignAssets.Issued.is(event),
				assetHubBestBlockBeforeSending
			);

			expect(event).to.not.be.null;

			// Check that Ferdie's balance has been correctly updated.
			const ferdieLaosBalance = hexToBn(
				(
					await this.chains.assetHub.query.foreignAssets.account(
						this.assetHubItems.laosAsset,
						this.substratePairs.ferdie.address
					)
				).toJSON()["balance"]
			);
			expect(ferdieLaosBalance.sub(ferdieLaosBalanceBefore).eq(MINTED_AMOUNT), "Ferdie balance should be > 0");
		});

		step("Create $LAOS/$DOT pool in AssetHub", async function () {
			// NOTE: We only create the pool if it hasn't been created yet, in this way we ensure tests are idempotent
			const poolExists = !(
				await this.chains.assetHub.query.assetConversion.pools([
					this.assetHubItems.relayAsset,
					this.assetHubItems.laosAsset,
				])
			).isEmpty;

			if (!poolExists) {
				// Build XCM instruction to be included in xcm.send call
				const createPoolCall = this.chains.assetHub.tx.assetConversion.createPool(
					this.assetHubItems.relayAsset.toU8a(),
					this.assetHubItems.laosAsset.toU8a()
				);

				let finalizedBlock = await sendTxAndWaitForFinalizationRococo(
					this.chains.relaychain,
					createPoolCall,
					this.substratePairs.alice,
					this.chains.assetHub
				);

				// Check that pool creation event has been emitted.
				let event = await checkEventInBlock(
					this.chains.assetHub,
					({ event }) => this.chains.assetHub.events.assetConversion.PoolCreated.is(event),
					finalizedBlock
				);

				expect(event).to.not.be.null;
				const [creator, poolId] = event.event.data;
				expect(creator.toString()).to.equal(this.substratePairs.alice.address);
				expect(poolId.toJSON()).to.deep.equal([relayChainLocation(), siblingParachainLocation(LAOS_PARA_ID)]);
				expect(
					(
						await this.chains.assetHub.query.assetConversion.pools([
							this.assetHubItems.relayAsset,
							this.assetHubItems.laosAsset,
						])
					).isEmpty
				).to.be.false;

				// Add liquidity to the pool
				const liquidityAmountLaos = new BN(ONE_LAOS.muln(1000));
				const liquidityAmountDot = new BN(ONE_DOT.muln(1000));
				const ferdieBalance = new BN(
					(
						await this.chains.assetHub.query.system.account(this.substratePairs.ferdie.address as string)
					).data.free
				);
				const ferdieLaosBalance = hexToBn(
					(
						await this.chains.assetHub.query.foreignAssets.account(
							this.assetHubItems.laosAsset,
							this.substratePairs.ferdie.address
						)
					).toJSON()?.["balance"] ?? 0
				);
				expect(
					ferdieBalance.gte(liquidityAmountDot),
					"Ferdie's $DOT balance should be greater than the amount to be sent to the pool"
				);
				expect(
					ferdieLaosBalance.gte(liquidityAmountLaos),
					"Ferdie's $LAOS balance should be greater than the amount to be sent to the pool"
				);

				const call = this.chains.assetHub.tx.assetConversion.addLiquidity(
					this.assetHubItems.relayAsset.toU8a(),
					this.assetHubItems.laosAsset.toU8a(),
					liquidityAmountDot,
					liquidityAmountLaos,
					liquidityAmountDot.sub(new BN(ONE_DOT.muln(10))),
					liquidityAmountLaos.sub(new BN(ONE_LAOS.muln(10))),
					this.substratePairs.ferdie.address
				);

				finalizedBlock = await sendTxAndWaitForFinalizationRococo(
					this.chains.relaychain,
					call,
					this.substratePairs.ferdie,
					this.chains.assetHub
				);
				event = await checkEventInBlock(
					this.chains.assetHub,
					({ event }) => this.chains.assetHub.events.assetConversion.LiquidityAdded.is(event),
					finalizedBlock
				);

				expect(event).to.not.be.null;
				const [who, mintTo, poolID, amount1Provided, amount2Provided] = event.event.data;
				expect(who.toString()).to.equal(this.substratePairs.ferdie.address);
				expect(mintTo.toString()).to.equal(this.substratePairs.ferdie.address);
				expect(poolId.toJSON()).to.deep.equal([relayChainLocation(), siblingParachainLocation(LAOS_PARA_ID)]);
				expect(new BN(amount1Provided.toString()).eq(liquidityAmountDot)).to.be.true;
				expect(new BN(amount2Provided.toString()).eq(liquidityAmountLaos)).to.be.true;
			}
		});

		step("Teleport from LAOS to AssetHub", async function () {
			const beneficiary = this.chains.laos.createType("XcmVersionedLocation", {
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
			const assets = this.chains.laos.createType("XcmVersionedAssets", {
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
					await this.chains.assetHub.query.foreignAssets.account(
						this.assetHubItems.laosAsset,
						this.substratePairs.charlie.address
					)
				).toJSON()?.["balance"] ?? 0
			);
			const alithBalanceBefore = (
				await this.chains.laos.query.system.account(this.ethereumPairs.alith.address as string)
			).data.free;

			const call = this.chains.laos.tx.polkadotXcm.limitedTeleportAssets(
				this.laosItems.assetHubLocation,
				beneficiary,
				assets,
				fee_asset_item,
				weight_limit
			);

			const assetHubBestBlockBeforeSending = await getFinalizedBlockNumber(this.chains.assetHub);
			await sendTxAndWaitForFinalization(this.chains.laos, call, this.ethereumPairs.alith);

			// Check that $LAOS has been sent in Asset Hub
			const event = await checkEventAfterXcm(
				this.chains.assetHub,
				({ event }) => this.chains.assetHub.events.foreignAssets.Issued.is(event),
				assetHubBestBlockBeforeSending
			);

			expect(event).to.not.be.null;
			const [assetId, owner, realAmountReceived] = event.event.data;
			expect(assetId.toJSON()).to.deep.equal(this.assetHubItems.laosAsset.toJSON());
			expect(owner.toString()).to.equal(this.substratePairs.charlie.address);
			const charlieBalance = hexToBn(
				(
					await this.chains.assetHub.query.foreignAssets.account(
						this.assetHubItems.laosAsset,
						this.substratePairs.charlie.address
					)
				).toJSON()["balance"]
			);
			expect(
				charlieBalanceBefore.add(new BN(realAmountReceived.toString())).eq(charlieBalance),
				"Charlie's balance should increase by the amount received"
			);
			const realAlithBalance = (
				await this.chains.laos.query.system.account(this.ethereumPairs.alith.address as string)
			).data.free;
			const alithBalance = alithBalanceBefore.sub(amount);
			expect(
				alithBalance.sub(realAlithBalance).lte(ONE_DOT),
				"Alith's balance should decrease by the amount teleported, disregarding fees"
			);
		});

		step("Teleport back from AssetHub to Laos", async function () {
			let beneficiaryAddress = this.ethereumPairs.baltathar.address;
			const beneficiary = this.chains.assetHub.createType("XcmVersionedLocation", {
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
			const assets = this.chains.assetHub.createType("XcmVersionedAssets", {
				V3: [
					{
						id: {
							Concrete: siblingParachainLocation(LAOS_PARA_ID),
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
					await this.chains.assetHub.query.foreignAssets.account(
						this.assetHubItems.laosAsset,
						this.substratePairs.charlie.address
					)
				).toJSON()?.["balance"] ?? 0
			);
			const beneficiaryBalanceBefore = (await this.chains.laos.query.system.account(beneficiaryAddress)).data
				.free;

			const call = this.chains.assetHub.tx.polkadotXcm.limitedTeleportAssets(
				this.assetHubItems.laosLocation,
				beneficiary,
				assets,
				fee_asset_item,
				weight_limit
			);

			const laosBestBlockBeforeSending = await getFinalizedBlockNumber(this.chains.laos);
			await sendTxAndWaitForFinalizationRococo(
				this.chains.relaychain,
				call,
				this.substratePairs.charlie,
				this.chains.assetHub
			);
			// Check that $LAOS has been sent back to Laos
			const event = await checkEventAfterXcm(
				this.chains.laos,
				({ event }) => {
					return (
						this.chains.laos.events.balances.Minted.is(event) &&
						event.data[0].toString() !== CHECKING_ACCOUNT
					);
				},
				laosBestBlockBeforeSending
			);

			expect(event).to.not.be.null;
			const [receiver, realAmountReceived] = event.event.data;
			expect(receiver.toString()).to.equal(beneficiaryAddress);
			const charlieBalance = hexToBn(
				(
					await this.chains.assetHub.query.foreignAssets.account(
						this.assetHubItems.laosAsset,
						this.substratePairs.charlie.address
					)
				).toJSON()["balance"]
			);
			expect(
				charlieBalanceBefore.sub(amount).eq(charlieBalance),
				"Charlie's balance should decrease by the amount teleported"
			);
			const beneficiaryBalance = (await this.chains.laos.query.system.account(beneficiaryAddress)).data.free;
			expect(
				beneficiaryBalanceBefore.add(new BN(realAmountReceived.toString())).eq(beneficiaryBalance),
				"Alith's balance should increase by the amount received in the teleport"
			);
		});
	},
	true
);
