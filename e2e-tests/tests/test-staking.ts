import { describeWithExistingNode } from "./util";
import {
	ALITH,
	STAKING_ABI,
	STAKING_CONTRACT_ADDRESS,
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

describeWithExistingNode(
	"Frontier RPC (Staking)",
	function () {
		let contract: Contract;

		before(async function () {
			contract = new this.context.web3.eth.Contract(STAKING_ABI, STAKING_CONTRACT_ADDRESS, {
				from: ALITH,
			});
			this.context.web3.eth.accounts.wallet.add(BALTATHAR_PRIVATE_KEY);
			this.context.web3.eth.accounts.wallet.add(FAITH_PRIVATE_KEY);
		});

		step("Faith can join as candidate", async function () {
			// insert session key into the node and link to Faith
			const key = (await this.context.networks.laos.rpc.author.rotateKeys()).toHex();
			this.context.networks.laos.tx.session
				.setKeys(key, "")
				.signAndSend(this.ethereumPairs.faith)
				.catch((error: any) => {
					console.log("transaction failed", error);
				});

			expect(await contract.methods.isCandidate(FAITH).call()).to.be.eq(false);
			const candidateCount = await contract.methods.candidateCount().call();
			expect((await this.context.web3.eth.getBlock("latest")).baseFeePerGas.toString()).to.be.eq(
				await this.context.web3.eth.getGasPrice()
			); // it starts with 1 Gwei and decreases until 0.5 Gwei
			const estimatedGas = await contract.methods
				.joinCandidates(BigInt(20000) * UNIT, candidateCount)
				.estimateGas();
			const gasPrice = (await this.context.web3.eth.getGasPrice()) + 1; // if we don't add +1 tx never gets included in the block
			let nonce = await this.context.web3.eth.getTransactionCount(FAITH);
			const result = await contract.methods
				.joinCandidates(BigInt(20000) * UNIT, candidateCount)
				.send({ from: FAITH, gas: estimatedGas, gasPrice, nonce: nonce++ });
			expect(result.status).to.be.eq(true);
			expect(await contract.methods.isCandidate(FAITH).call()).to.be.eq(true);
		});

		step("Baltathar can delegate to Faith", async function () {
			expect(await contract.methods.isDelegator(BALTATHAR).call()).to.be.eq(false);
			let nonce = await this.context.web3.eth.getTransactionCount(BALTATHAR);
			const gasPrice = (await this.context.web3.eth.getGasPrice()) + 1; // if we don't add +1 tx never gets included in the block
			const estimatedGas = await contract.methods.delegate(FAITH, BigInt(1000) * UNIT, 0, 0).estimateGas();
			const result = await contract.methods
				.delegate(FAITH, BigInt(1000) * UNIT, 0, 0)
				.send({ from: BALTATHAR, gas: estimatedGas, gasPrice, nonce: nonce++ });
			expect(result.status).to.be.eq(true);
			expect(await contract.methods.isDelegator(BALTATHAR).call()).to.be.eq(true);
		});
	},
	true
);
