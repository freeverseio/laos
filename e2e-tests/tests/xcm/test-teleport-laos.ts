import BN from "bn.js";
import { expect } from "chai";
import { step } from "mocha-steps";

import { LAOS_PARA_ID, ONE_LAOS } from "@utils/constants";
import { describeWithExistingNodeXcm } from "@utils/setups";
import { siblingParachainLocation, hereLocation, checkEventAfterXcm } from "@utils/xcm";
import { sendTxAndWaitForFinalization } from "@utils/transactions";
import { getFinalizedBlockNumber } from "@utils/blocks";
import { hexToBn } from "@polkadot/util";

describeWithExistingNodeXcm("Teleport Asset Hub <-> LAOS", function () {
	step("Teleport from LAOS to AssetHub", async function () {
		const beneficiary = this.chains.laos.createType("XcmVersionedLocation", {
			V4: {
				parents: "0",
				interior: {
					X1: [
						{
							AccountId32: {
								id: this.assetHubItems.accounts.charlie.toHex(),
							},
						},
					],
				},
			},
		});

		const amount = ONE_LAOS.muln(5);
		const assets = this.chains.laos.createType("XcmVersionedAssets", {
			V4: [
				{
					id: hereLocation(),

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
		const supposedAlithBalance = alithBalanceBefore.sub(amount);
		expect(
			supposedAlithBalance.sub(realAlithBalance).lte(ONE_LAOS),
			"Alith's balance should decrease by the amount teleported, disregarding fees"
		);
	});

	step("Teleport back from AssetHub to Laos", async function () {
		const beneficiary = this.chains.assetHub.createType("XcmVersionedLocation", {
			V4: {
				parents: "0",
				interior: {
					X1: [
						{
							AccountKey20: {
								key: this.ethereumPairs.baltathar.address,
							},
						},
					],
				},
			},
		});

		const amount = ONE_LAOS;
		const assets = this.chains.assetHub.createType("XcmVersionedAssets", {
			V4: [
				{
					id: siblingParachainLocation(LAOS_PARA_ID),
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
		const beneficiaryBalanceBefore = (
			await this.chains.laos.query.system.account(this.ethereumPairs.baltathar.address)
		).data.free;

		const call = this.chains.assetHub.tx.polkadotXcm.limitedTeleportAssets(
			this.assetHubItems.laosLocation,
			beneficiary,
			assets,
			fee_asset_item,
			weight_limit
		);

		const laosBestBlockBeforeSending = await getFinalizedBlockNumber(this.chains.laos);
		await sendTxAndWaitForFinalization(this.chains.assetHub, call, this.substratePairs.charlie);
		// Check that $LAOS has been sent back to Laos
		const event = await checkEventAfterXcm(
			this.chains.laos,
			({ event }) => {
				return (
					this.chains.laos.events.balances.Minted.is(event) &&
					event.data[0].toString() == this.ethereumPairs.baltathar.address
				);
			},
			laosBestBlockBeforeSending
		);

		expect(event).to.not.be.null;
		const [receiver, realAmountReceived] = event.event.data;
		expect(receiver.toString()).to.equal(this.ethereumPairs.baltathar.address);
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
		const beneficiaryBalance = (await this.chains.laos.query.system.account(this.ethereumPairs.baltathar.address))
			.data.free;
		expect(
			beneficiaryBalanceBefore.add(new BN(realAmountReceived.toString())).eq(beneficiaryBalance),
			"Alith's balance should increase by the amount received in the teleport"
		);
	});
});
