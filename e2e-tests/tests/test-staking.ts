import { describeWithExistingNode } from "@utils/setups";
import { STAKING_ABI, STAKING_CONTRACT_ADDRESS, ONE_LAOS } from "@utils/constants";
import { expect } from "chai";
import Contract from "web3-eth-contract";
import { step } from "mocha-steps";

describeWithExistingNode(
	"Frontier RPC (Staking)",
	function () {
		let contract: Contract;

		before(async function () {
			contract = new this.web3.eth.Contract(STAKING_ABI, STAKING_CONTRACT_ADDRESS, {
				from: this.ethereumPairs.alith.address,
			});
		});

		step("Faith can join as candidate", async function () {
			// insert session key into the node and link to Faith
			const key = (await this.chains.laos.rpc.author.rotateKeys()).toHex();
			this.chains.laos.tx.session
				.setKeys(key, "")
				.signAndSend(this.ethereumPairs.faith)
				.catch((error: any) => {
					console.log("transaction failed", error);
				});

			expect(await contract.methods.isCandidate(this.ethereumPairs.faith.address).call()).to.be.eq(false);
			const candidateCount = await contract.methods.candidateCount().call();
			expect((await this.web3.eth.getBlock("latest")).baseFeePerGas.toString()).to.be.eq(
				await this.web3.eth.getGasPrice()
			); // it starts with 1 Gwei and decreases until 0.5 Gwei
			const estimatedGas = await contract.methods
				.joinCandidates(ONE_LAOS.muln(20000), candidateCount)
				.estimateGas();
			const gasPrice = (await this.web3.eth.getGasPrice()) + 1; // if we don't add +1 tx never gets included in the block
			let nonce = await this.web3.eth.getTransactionCount(this.ethereumPairs.faith.address);
			const result = await contract.methods
				.joinCandidates(ONE_LAOS.muln(20000), candidateCount)
				.send({ from: this.ethereumPairs.faith.address, gas: estimatedGas, gasPrice, nonce: nonce++ });
			expect(result.status).to.be.eq(true);
			expect(await contract.methods.isCandidate(this.ethereumPairs.faith.address).call()).to.be.eq(true);
		});

		step("Baltathar can delegate to Faith", async function () {
			expect(await contract.methods.isDelegator(this.ethereumPairs.baltathar.address).call()).to.be.eq(false);
			let nonce = await this.web3.eth.getTransactionCount(this.ethereumPairs.baltathar.address);
			const gasPrice = (await this.web3.eth.getGasPrice()) + 1; // if we don't add +1 tx never gets included in the block
			const estimatedGas = await contract.methods
				.delegate(this.ethereumPairs.faith.address, ONE_LAOS.muln(1000), 0, 0)
				.estimateGas();
			const result = await contract.methods
				.delegate(this.ethereumPairs.faith.address,ONE_LAOS.muln(1000), 0, 0)
				.send({ from: this.ethereumPairs.baltathar.address, gas: estimatedGas, gasPrice, nonce: nonce++ });
			expect(result.status).to.be.eq(true);
			expect(await contract.methods.isDelegator(this.ethereumPairs.baltathar.address).call()).to.be.eq(true);
		});
	},
	true
);
