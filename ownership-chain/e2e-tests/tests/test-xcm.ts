import { Keyring } from "@polkadot/api";
import { AccountInfo } from "@polkadot/types/interfaces";
import { u8aToHex } from "@polkadot/util";
import { cryptoWaitReady } from "@polkadot/util-crypto";
import { BN } from "bn.js";
import { expect } from "chai";
import { step } from "mocha-steps";
import { OWNCHAIN_SUDO_PRIVATE_KEY } from "./config";
import { describeWithExistingSubstrateNodes, waitForEvents } from "./util";

describeWithExistingSubstrateNodes("XCM tests", (context) => {
	let ownchainSudo = null;
	let astarSudo = null;

	const ASTAR_PARA_ID = 2008;
	const OWNCHAIN_PARA_ID = 2900;

	// Sovereign account of Astar in Ownership Chain
	const ASTAR_IN_OWNCHAIN = "0x7369626cd8070000000000000000000000000000";

	before(async function () {
		await cryptoWaitReady();

		let ownchainKeyring = new Keyring({ type: "ethereum" });
		let seedAsU8a = new BN(OWNCHAIN_SUDO_PRIVATE_KEY.slice(2), "hex").toArray("be", 32);

		ownchainSudo = ownchainKeyring.addFromSeed(Uint8Array.from(seedAsU8a));

		let astarKeyring = new Keyring({ type: "sr25519" });
		astarSudo = astarKeyring.addFromUri("//Alice");

		// fund the sovereign account of Astar in Ownership Chain
		const transfer = context.ownchain.tx.balances.transferKeepAlive(
			ASTAR_IN_OWNCHAIN,
			new BN("100000000000000000000")
		);

		await transfer.signAndSend(ownchainSudo);
	});

	step("hrmp channels should be opened", async function () {
		let astarToOwnchain = await context.relaychain.query.hrmp.hrmpChannels({
			sender: ASTAR_PARA_ID,
			recipient: OWNCHAIN_PARA_ID,
		});
		let ownchainToAstar = await context.relaychain.query.hrmp.hrmpChannels({
			sender: OWNCHAIN_PARA_ID,
			recipient: ASTAR_PARA_ID,
		});

		expect(astarToOwnchain).to.not.be.empty;
		expect(ownchainToAstar).to.not.be.empty;
	});

	step("should be able to transfer a CLDN token from Astar sovereign account", async function () {
		// Simply a `dummy` string converted to H160
		let dummyAccount = "0x64756d6d79000000000000000000000000000000";

		let dummyAccountBalanceBefore = await context.ownchain.query.system.account<AccountInfo>(dummyAccount);

		// make a transfer call from `astarInOwnchain` to `dummyAccount`
		let amount = new BN("1000000000000000000");
		let transfer = context.ownchain.tx.balances.transferKeepAlive(dummyAccount, amount);

		// Destination of the XCM call: ownership chain
		let dest = { V2: { parents: 1, interior: { X1: { Parachain: 2900 } } } };

		let instr1 = {
			WithdrawAsset: [
				{
					id: { Concrete: { parents: 0, interior: { Here: null } } },
					fun: { Fungible: new BN("2000000000000000000") },
				},
			],
		};

		let instr2 = {
			BuyExecution: [
				{
					id: { Concrete: { parents: 0, interior: { Here: null } } },
					fun: { Fungible: new BN("2000000000000000000") },
				},
				{
					Unlimited: null,
				},
			],
		};

		const instr3 = {
			Transact: {
				originType: { SovereignAccount: null },
				requireWeightAtMost: 4_000_000_000,
				call: { encoded: u8aToHex(transfer.method.toU8a()) },
			},
		};

		const instr4 = {
			DepositAsset: [
				{ Wild: { All: null } },
				1,
				{ parents: 0, interior: { X1: { AccountKey20: { network: { Any: null }, key: ASTAR_IN_OWNCHAIN } } } },
			],
		};

		const message = { V2: [instr1, instr2, instr3, instr4] };

		// xcm call
		const tx = context.astar.tx.polkadotXcm.send(dest, message);

		// wrap it with `sudo.sudo`
		const sudoTx = context.astar.tx.sudo.sudo(tx);

		// send the transaction (optional status callback)
		await sudoTx.signAndSend(astarSudo);

		// Wait for `xcmpQueue.Success` and `balances.Transfer` events
		let eventsData = await waitForEvents(
			context.ownchain,
			[
				{ module: "xcmpQueue", name: "Success" },
				{ module: "balances", name: "Transfer" },
			],
			4
		);

		expect(eventsData["xcmpQueue.Success"]).to.not.be.undefined;
		expect(eventsData["balances.Transfer"]).to.not.be.undefined;

		let transferEventData = eventsData["balances.Transfer"].filter((data) => {
			return data[0].toLowerCase() === ASTAR_IN_OWNCHAIN;
		});

		expect(transferEventData[0][0].toLowerCase()).to.equal(ASTAR_IN_OWNCHAIN);
		expect(transferEventData[0][1].toLowerCase()).to.equal(dummyAccount);
		expect(transferEventData[0][2].toString()).to.equal(amount.toString());

		let dummyAccountBalanceAfter = await context.ownchain.query.system.account<AccountInfo>(dummyAccount);
		expect(
			new BN(dummyAccountBalanceBefore.data.free.toString())
				.add(amount)
				.eq(new BN(dummyAccountBalanceAfter.data.free.toString()))
		).to.be.true;
	}).timeout(80000);
});
