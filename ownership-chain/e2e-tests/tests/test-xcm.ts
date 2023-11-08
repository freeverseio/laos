import { Keyring } from "@polkadot/api";
import { u8aToHex } from "@polkadot/util";
import { cryptoWaitReady } from "@polkadot/util-crypto";
import { BN } from "bn.js";
import { expect } from "chai";
import { OWNCHAIN_SUDO_PRIVATE_KEY } from "./config";
import { describeWithExistingSubstrateNodes } from "./util";

describeWithExistingSubstrateNodes("XCM tests", (context) => {
	let ownchainSudo = null;
	let astarSudo = null;

	// Sovereign account of Astar in Ownership Chain
	const ASTAR_IN_OWNCHAIN = "0x7369626cd8070000000000000000000000000000";
	// Sovereign account of Ownership Chain in Astar
	const OWNCHAIN_IN_ASTAR = "5Eg2fnssBDaFCWy7JnEZYnEuNPZbbzzEWGw5zryrTpmsTuPL";

	before(async function () {
		await cryptoWaitReady();

		let ownchainKeyring = new Keyring({ type: "ethereum" });
		let seesAsU8a = new BN(OWNCHAIN_SUDO_PRIVATE_KEY.slice(2), "hex").toArray("be", 32);

		ownchainSudo = ownchainKeyring.addFromSeed(Uint8Array.from(seesAsU8a));

		let astarKeyring = new Keyring({ type: "sr25519" });
		astarSudo = astarKeyring.addFromUri("//Alice");

		// fund the sovereign account of Astar in Ownership Chain
		const transfer = context.ownchain.tx.balances.transferKeepAlive(
			ASTAR_IN_OWNCHAIN,
			new BN("100000000000000000000")
		);

		console.log("SUDO", u8aToHex(ownchainSudo.publicKey));
		await transfer.signAndSend(ownchainSudo);

		console.log(`Sovereign account of Astar in Ownership Chain: ${ASTAR_IN_OWNCHAIN}`);
	});

	it("should be able to transfer a CLDN token from Astar sovereign account", async function () {
		// Simply a `dummy` string converted to H160
		let dummyAccount = "0x64756d6d79000000000000000000000000000000";

		// make a transfer call from `astarInOwnchain` to `GENESIS_ACCOUNT`
		let transfer = context.ownchain.tx.balances.transferKeepAlive(dummyAccount, new BN("1000000000000000000"));

		let dest = { V2: { parents: 1, interior: { X1: { Parachain: 2900 } } } };

		let instr1 = {
			WithdrawAsset: [
				{
					id: { Concrete: { parents: 1, interior: { Here: null } } },
					fun: { Fungible: new BN("10000000000000000000") },
				},
			],
		};

		let instr2 = {
			BuyExecution: [
				{
					id: { Concrete: { parents: 1, interior: { Here: null } } },
					fun: { Fungible: new BN("10000000000000000000") },
				},
				{
					Unlimited: null,
				},
			],
		};

		const instr3 = {
			Transact: {
				originKind: "SovereignAccount",
				requireWeightAtMost: 4_000_000_000,
				call: {
					encoded: transfer.method.toHex(),
				},
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

		// send the message
		const tx = context.astar.tx.polkadotXcm.send(dest, message);

		console.log("tx", tx);
		// send the transaction (optional status callback)
		const txHash = await tx.signAndSend(astarSudo);

		console.log(`Submitted tx: ${txHash}`);

		expect(txHash).to.not.be.null;
	}).timeout(40000);
});
