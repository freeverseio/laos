import BN from "bn.js";
import { expect } from "chai";
import { step } from "mocha-steps";

import { LAOS_PARA_ID, ONE_LAOS } from "@utils/constants";
import { describeWithExistingNodeXcm } from "@utils/setups";
import { siblingParachainLocation, hereLocation, checkEventAfterXcm } from "@utils/xcm";
import { sendTxAndWaitForFinalization } from "@utils/transactions";
import { getFinalizedBlockNumber } from "@utils/blocks";
import { hexToBn } from "@polkadot/util";

describeWithExistingNodeXcm("Local Reserve transfer LAOS <-> Moonbeam", function () {
	step("Local Reserve transfer from LAOS to Moonbeam", async function () {
		const beneficiary = this.chains.laos.createType("XcmVersionedLocation", {
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

		const baltatharBalanceBefore = hexToBn(
			(
				await this.chains.moonbeam.query.assets.account(
					this.moonbeamItems.laosAsset,
					this.ethereumPairs.baltathar.address
				)
			).toJSON()?.["balance"] ?? 0
		);
		const alithBalanceBefore = (
			await this.chains.laos.query.system.account(this.ethereumPairs.alith.address as string)
		).data.free;
		const moonbeamSABalanceBefore = (await this.chains.laos.query.system.account(this.laosItems.moonbeamSA)).data
			.free;

		const call = this.chains.laos.tx.polkadotXcm.limitedReserveTransferAssets(
			this.laosItems.moonbeamLocation,
			beneficiary,
			assets,
			fee_asset_item,
			weight_limit
		);

		const moonbeamBestBlockBeforeSending = await getFinalizedBlockNumber(this.chains.moonbeam);
		await sendTxAndWaitForFinalization(this.chains.laos, call, this.ethereumPairs.alith);

		// Check that $LAOS has been sent to Moonbeam
		const event = await checkEventAfterXcm(
			this.chains.moonbeam,
			({ event }) => {
				return (
					this.chains.moonbeam.events.assets.Issued.is(event) &&
					event.data[1].toString() == this.ethereumPairs.baltathar.address
				);
			},
			moonbeamBestBlockBeforeSending
		);

		expect(event).to.not.be.null;
		const [assetId, owner, realAmountReceived] = event.event.data;
		expect(new BN(assetId.toString()).eq(this.moonbeamItems.laosAsset)).to.be.true;
		expect(owner.toString()).to.equal(this.ethereumPairs.baltathar.address);
		const baltatharBalance = hexToBn(
			(
				await this.chains.moonbeam.query.assets.account(
					this.moonbeamItems.laosAsset,
					this.ethereumPairs.baltathar.address
				)
			).toJSON()["balance"]
		);
		expect(
			baltatharBalanceBefore.add(new BN(realAmountReceived.toString())).eq(baltatharBalance),
			"Baltathar's balance should increase by the amount received"
		).to.be.true;
		const realAlithBalance = (
			await this.chains.laos.query.system.account(this.ethereumPairs.alith.address as string)
		).data.free;
		const supposedAlithBalance = alithBalanceBefore.sub(amount);
		expect(
			supposedAlithBalance.sub(realAlithBalance).lte(ONE_LAOS),
			"Alith's balance should decrease by the amount of the reserve transfer, disregarding fees"
		).to.be.true;

		// with reserve transfers, in LAOS, Alith's amount is transferred to Moonbeam's SA
		const realMoonbeamSABalance = (await this.chains.laos.query.system.account(this.laosItems.moonbeamSA)).data
			.free;
		const supposedMoonbeamSABalance = moonbeamSABalanceBefore.add(amount);
		expect(
			supposedMoonbeamSABalance.eq(realMoonbeamSABalance),
			"Moonbeam's SA balance has not increased by the amount of the reserve transfer"
		).to.be.true;
	});

	step("Reserve transfer from Moonbeam to LAOS", async function () {
		const beneficiary = this.chains.moonbeam.createType("XcmVersionedLocation", {
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
		const assets = this.chains.moonbeam.createType("XcmVersionedAssets", {
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

		const beneficiaryBalanceBefore = (
			await this.chains.laos.query.system.account(this.ethereumPairs.baltathar.address)
		).data.free;
		const moonbeamSABalanceBefore = (await this.chains.laos.query.system.account(this.laosItems.moonbeamSA)).data
			.free;

		// We call transferAssets instead of limitedReserveTransferAssets here due to moonbeam disable this extrinsic in their XCM maintenances. transferAssets executes a reserve transfer under the hood for this asset so this is OK.
		const call = this.chains.moonbeam.tx.polkadotXcm.transferAssets(
			this.moonbeamItems.laosLocation,
			beneficiary,
			assets,
			fee_asset_item,
			weight_limit
		);

		const laosBestBlockBeforeSending = await getFinalizedBlockNumber(this.chains.laos);
		await sendTxAndWaitForFinalization(this.chains.moonbeam, call, this.ethereumPairs.baltathar);
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
		const [_, realAmountReceived] = event.event.data;

		const beneficiaryBalance = (await this.chains.laos.query.system.account(this.ethereumPairs.baltathar.address))
			.data.free;
		expect(
			beneficiaryBalanceBefore.add(new BN(realAmountReceived.toString())).eq(beneficiaryBalance),
			"Baltathar's balance should increase by the amount received in the reserve transfer"
		).to.be.true;

		// check that moonbeam SA balance has been reduced
		const realMoonbeamSABalance = (await this.chains.laos.query.system.account(this.laosItems.moonbeamSA)).data
			.free;
		const supposedMoonbeamSABalance = moonbeamSABalanceBefore.sub(amount);
		expect(
			supposedMoonbeamSABalance.eq(realMoonbeamSABalance),
			"Moonbeam's SA balance has not decreased by the amount of the reserve transfer"
		).to.be.true;
	});
});

describeWithExistingNodeXcm("Local Reserve transfer LAOS <-> Hydration", function () {
	step("Local Reserve transfer from LAOS to Hydration", async function () {
		const beneficiary = this.chains.laos.createType("XcmVersionedLocation", {
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

		const baltatharBalanceBefore = hexToBn(
			(
				await this.chains.moonbeam.query.assets.account(
					this.moonbeamItems.laosAsset,
					this.ethereumPairs.baltathar.address
				)
			).toJSON()?.["balance"] ?? 0
		);
		const alithBalanceBefore = (
			await this.chains.laos.query.system.account(this.ethereumPairs.alith.address as string)
		).data.free;
		const moonbeamSABalanceBefore = (await this.chains.laos.query.system.account(this.laosItems.moonbeamSA)).data
			.free;

		const call = this.chains.laos.tx.polkadotXcm.limitedReserveTransferAssets(
			this.laosItems.moonbeamLocation,
			beneficiary,
			assets,
			fee_asset_item,
			weight_limit
		);

		const moonbeamBestBlockBeforeSending = await getFinalizedBlockNumber(this.chains.moonbeam);
		await sendTxAndWaitForFinalization(this.chains.laos, call, this.ethereumPairs.alith);

		// Check that $LAOS has been sent to Moonbeam
		const event = await checkEventAfterXcm(
			this.chains.moonbeam,
			({ event }) => {
				return (
					this.chains.moonbeam.events.assets.Issued.is(event) &&
					event.data[1].toString() == this.ethereumPairs.baltathar.address
				);
			},
			moonbeamBestBlockBeforeSending
		);

		expect(event).to.not.be.null;
		const [assetId, owner, realAmountReceived] = event.event.data;
		expect(new BN(assetId.toString()).eq(this.moonbeamItems.laosAsset)).to.be.true;
		expect(owner.toString()).to.equal(this.ethereumPairs.baltathar.address);
		const baltatharBalance = hexToBn(
			(
				await this.chains.moonbeam.query.assets.account(
					this.moonbeamItems.laosAsset,
					this.ethereumPairs.baltathar.address
				)
			).toJSON()["balance"]
		);
		expect(
			baltatharBalanceBefore.add(new BN(realAmountReceived.toString())).eq(baltatharBalance),
			"Baltathar's balance should increase by the amount received"
		).to.be.true;
		const realAlithBalance = (
			await this.chains.laos.query.system.account(this.ethereumPairs.alith.address as string)
		).data.free;
		const supposedAlithBalance = alithBalanceBefore.sub(amount);
		expect(
			supposedAlithBalance.sub(realAlithBalance).lte(ONE_LAOS),
			"Alith's balance should decrease by the amount of the reserve transfer, disregarding fees"
		).to.be.true;

		// with reserve transfers, in LAOS, Alith's amount is transferred to Moonbeam's SA
		const realMoonbeamSABalance = (await this.chains.laos.query.system.account(this.laosItems.moonbeamSA)).data
			.free;
		const supposedMoonbeamSABalance = moonbeamSABalanceBefore.add(amount);
		expect(
			supposedMoonbeamSABalance.eq(realMoonbeamSABalance),
			"Moonbeam's SA balance has not increased by the amount of the reserve transfer"
		).to.be.true;
	});

	step("Reserve transfer from Hydration to LAOS", async function () {
		const beneficiary = this.chains.moonbeam.createType("XcmVersionedLocation", {
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
		const assets = this.chains.moonbeam.createType("XcmVersionedAssets", {
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

		const beneficiaryBalanceBefore = (
			await this.chains.laos.query.system.account(this.ethereumPairs.baltathar.address)
		).data.free;
		const moonbeamSABalanceBefore = (await this.chains.laos.query.system.account(this.laosItems.moonbeamSA)).data
			.free;

		// We call transferAssets instead of limitedReserveTransferAssets here due to moonbeam disable this extrinsic in their XCM maintenances. transferAssets executes a reserve transfer under the hood for this asset so this is OK.
		const call = this.chains.moonbeam.tx.polkadotXcm.transferAssets(
			this.moonbeamItems.laosLocation,
			beneficiary,
			assets,
			fee_asset_item,
			weight_limit
		);

		const laosBestBlockBeforeSending = await getFinalizedBlockNumber(this.chains.laos);
		await sendTxAndWaitForFinalization(this.chains.moonbeam, call, this.ethereumPairs.baltathar);
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
		const [_, realAmountReceived] = event.event.data;

		const beneficiaryBalance = (await this.chains.laos.query.system.account(this.ethereumPairs.baltathar.address))
			.data.free;
		expect(
			beneficiaryBalanceBefore.add(new BN(realAmountReceived.toString())).eq(beneficiaryBalance),
			"Baltathar's balance should increase by the amount received in the reserve transfer"
		).to.be.true;

		// check that moonbeam SA balance has been reduced
		const realMoonbeamSABalance = (await this.chains.laos.query.system.account(this.laosItems.moonbeamSA)).data
			.free;
		const supposedMoonbeamSABalance = moonbeamSABalanceBefore.sub(amount);
		expect(
			supposedMoonbeamSABalance.eq(realMoonbeamSABalance),
			"Moonbeam's SA balance has not decreased by the amount of the reserve transfer"
		).to.be.true;
	});
});
