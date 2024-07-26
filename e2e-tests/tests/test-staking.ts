import { describeWithExistingNode } from "./util";
import {
	GAS_LIMIT,
	ALITH,
	STAKING_ABI,
	STAKING_CONTRACT_ADDRESS,
	GAS_PRICE,
	UNIT,
	FAITH_PRIVATE_KEY,
	FAITH,
	BALTATHAR,
	BALTATHAR_PRIVATE_KEY,
} from "./config";
import { expect } from "chai";
import Contract from "web3-eth-contract";
import { step } from "mocha-steps";
import { Keyring } from "@polkadot/api";

describeWithExistingNode("Frontier RPC (Staking)", (context) => {
	let contract: Contract;

	before(async function () {
		contract = new context.web3.eth.Contract(STAKING_ABI, STAKING_CONTRACT_ADDRESS, {
			from: ALITH,
		});
		context.web3.eth.accounts.wallet.add(BALTATHAR_PRIVATE_KEY);
		context.web3.eth.accounts.wallet.add(FAITH_PRIVATE_KEY);
	});

	step("Faith can join as candidate", async function () {
		// insert session key into the node and link to Faith
		const faith = new Keyring({ type: "ethereum" }).addFromUri(FAITH_PRIVATE_KEY);
		const key = (await context.polkadot.rpc.author.rotateKeys()).toHex();
		context.polkadot.tx.session
			.setKeys(key, "")
			.signAndSend(faith, () => {})
			.catch((error: any) => {
				console.log("transaction failed", error);
			});

		expect(await contract.methods.isCandidate(FAITH).call()).to.be.eq(false);
		const candidateCount = await contract.methods.candidateCount().call();
		expect((await context.web3.eth.getBlock("latest")).baseFeePerGas.toString()).to.be.eq(
			await context.web3.eth.getGasPrice()
		); // it starts with 1 Gwei and decreases until 0.5 Gwei
		const estimatedGas = await contract.methods.joinCandidates(BigInt(20000) * UNIT, candidateCount).estimateGas();
		const gasPrice = (await context.web3.eth.getGasPrice()) + 1; // if we don't add +1 tx never gets included in the block
		let nonce = await context.web3.eth.getTransactionCount(FAITH);
		const result = await contract.methods
			.joinCandidates(BigInt(20000) * UNIT, candidateCount)
			.send({ from: FAITH, gas: estimatedGas, gasPrice, nonce: nonce++ });
		expect(result.status).to.be.eq(true);
		expect(await contract.methods.isCandidate(FAITH).call()).to.be.eq(true);
	});

	step("Baltathar can delegate to Faith", async function () {
		expect(await contract.methods.isDelegator(BALTATHAR).call()).to.be.eq(false);
		let nonce = await context.web3.eth.getTransactionCount(BALTATHAR);
		const gasPrice = (await context.web3.eth.getGasPrice()) + 1; // if we don't add +1 tx never gets included in the block
		const result = await contract.methods
			.delegate(FAITH, BigInt(1000) * UNIT, 0, 0)
			.send({ from: BALTATHAR, gas: GAS_LIMIT, gasPrice, nonce: nonce++ });
		expect(result.status).to.be.eq(true);
		expect(await contract.methods.isDelegator(BALTATHAR).call()).to.be.eq(true);
	});
});
