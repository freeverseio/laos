import BN from "bn.js";
import { expect } from "chai";
import { step } from "mocha-steps";

import { LAOS_PARA_ID, ONE_LAOS, LAOS_ID_HYDRATION } from "@utils/constants";
import { describeWithExistingNodeXcm } from "@utils/setups";
import { siblingParachainLocation, hereLocation, checkEventAfterXcm } from "@utils/xcm";
import { sendTxAndWaitForFinalization } from "@utils/transactions";
import { getFinalizedBlockNumber } from "@utils/blocks";

describeWithExistingNodeXcm("Local Reserve transfer LAOS <-> Hydration", function () {
	step("Local Reserve transfer from LAOS to Hydration", async function () {
		const beneficiary = this.chains.laos.createType("XcmVersionedLocation", {
			V4: {
				parents: "0",
				interior: {
					X1: [
						{
							AccountId32: {
								id: this.hydrationItems.accounts.alice.toHex(),
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

		const aliceBalanceBefore = new BN(
			(
				await this.chains.hydration.query.tokens.accounts(this.hydrationItems.accounts.alice, LAOS_ID_HYDRATION)
			).free
		);
		const alithBalanceBefore = (
			await this.chains.laos.query.system.account(this.ethereumPairs.alith.address as string)
		).data.free;
		const hydrationSABalanceBefore = (await this.chains.laos.query.system.account(this.laosItems.hydrationSA)).data
			.free;

		const call = this.chains.laos.tx.polkadotXcm.limitedReserveTransferAssets(
			this.laosItems.hydrationLocation,
			beneficiary,
			assets,
			fee_asset_item,
			weight_limit
		);

		const hydrationBestBlockBeforeSending = await getFinalizedBlockNumber(this.chains.hydration);
		await sendTxAndWaitForFinalization(this.chains.laos, call, this.ethereumPairs.alith);

		// Check that $LAOS has been sent to Hydration
		const event = await checkEventAfterXcm(
			this.chains.hydration,
			({ event }) => {
				return (
					this.chains.hydration.events.tokens.Deposited.is(event) &&
					event.data[1].toString() == this.hydrationItems.accounts.alice
				);
			},
			hydrationBestBlockBeforeSending
		);

		expect(event).to.not.be.null;
		const [assetId, owner, realAmountReceived] = event.event.data;
		expect(new BN(assetId.toString()).eq(this.hydrationItems.laosAsset)).to.be.true;
		expect(owner.toString()).to.equal(this.hydrationPairs.alice.address);

		const aliceBalance = new BN(
			(
				await this.chains.hydration.query.tokens.accounts(this.hydrationItems.accounts.alice, LAOS_ID_HYDRATION)
			).free
		);

		expect(
			aliceBalanceBefore.add(new BN(realAmountReceived.toString())).eq(aliceBalance),
			"Alice's balance should increase by the amount received"
		).to.be.true;
		const realAlithBalance = (
			await this.chains.laos.query.system.account(this.ethereumPairs.alith.address as string)
		).data.free;
		const supposedAlithBalance = alithBalanceBefore.sub(amount);
		expect(
			supposedAlithBalance.sub(realAlithBalance).lte(ONE_LAOS),
			"Alith's balance should decrease by the amount of the reserve transfer, disregarding fees"
		).to.be.true;

		// with reserve transfers, in LAOS, Alith's amount is transferred to Hydration's SA
		const realHydrationSABalance = (await this.chains.laos.query.system.account(this.laosItems.hydrationSA)).data
			.free;
		const supposedHydrationSABalance = hydrationSABalanceBefore.add(amount);
		expect(
			supposedHydrationSABalance.eq(realHydrationSABalance),
			"Hydration's SA balance has not increased by the amount of the reserve transfer"
		).to.be.true;
	});

	step("Reserve transfer from Hydration to LAOS", async function () {
		const beneficiary = this.chains.hydration.createType("XcmVersionedLocation", {
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
		const assets = this.chains.hydration.createType("XcmVersionedAssets", {
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
		const hydrationSABalanceBefore = (await this.chains.laos.query.system.account(this.laosItems.hydrationSA)).data
			.free;

		const call = this.chains.hydration.tx.polkadotXcm.limitedReserveTransferAssets(
			this.hydrationItems.laosLocation,
			beneficiary,
			assets,
			fee_asset_item,
			weight_limit
		);

		const laosBestBlockBeforeSending = await getFinalizedBlockNumber(this.chains.laos);
		await sendTxAndWaitForFinalization(this.chains.hydration, call, this.hydrationPairs.alice);
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

		// check that hydration SA balance has been reduced
		const realHydrationSABalance = (await this.chains.laos.query.system.account(this.laosItems.hydrationSA)).data
			.free;
		const supposedHydrationSABalance = hydrationSABalanceBefore.sub(amount);
		expect(
			supposedHydrationSABalance.eq(realHydrationSABalance),
			"Hydration's SA balance has not decreased by the amount of the reserve transfer"
		).to.be.true;
	});
});
