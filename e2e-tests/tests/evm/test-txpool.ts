import { expect } from "chai";
import { step } from "mocha-steps";

import { ALITH_PRIVATE_KEY } from "@utils/constants";
import { describeWithExistingNode } from "@utils/setups";
import { customRequest } from "@utils/helpers";
import Web3 from "web3";

describeWithExistingNode("Frontier RPC (TxPoolApi)", function () {
	const TEST_CONTRACT_BYTECODE = "0x608060405234801561";

	let nonce: number;
	let pendingTx;
	let futureTx;
	async function sendTransaction(web3: Web3, nonce: number, senderAddress: string) {
		const tx = await web3.eth.accounts.signTransaction(
			{
				from: senderAddress,
				data: TEST_CONTRACT_BYTECODE,
				value: "0x00",
				gasPrice: "0x3B9ACA00",
				gas: "0x100000",
				nonce: nonce,
			},
			ALITH_PRIVATE_KEY
		);
		await customRequest(web3, "eth_sendRawTransaction", [tx.rawTransaction]);
		return tx;
	}

	before(async function () {
		nonce = await this.web3.eth.getTransactionCount(this.ethereumPairs.alith.address);
	});

	step("txpool_status should return correct result", async function () {
		let txpoolStatusBefore = await customRequest(this.web3, "txpool_status", []);

		pendingTx = await sendTransaction(this.web3, nonce, this.ethereumPairs.alith.address);
		futureTx = await sendTransaction(this.web3, nonce + 1000, this.ethereumPairs.alith.address);
		let txpoolStatusAfter = await customRequest(this.web3, "txpool_status", []);

		expect(parseInt(txpoolStatusAfter.result.pending, 16)).to.be.equal(
			parseInt(txpoolStatusBefore.result.pending, 16) + 1
		);
		expect(parseInt(txpoolStatusAfter.result.queued, 16)).to.be.equal(
			parseInt(txpoolStatusBefore.result.queued, 16) + 1
		);
	});

	step("txpool_content should return correct result", async function () {
		let txpoolContent = await customRequest(this.web3, "txpool_content", []);

		let genesisAccount = this.ethereumPairs.alith.address.toLowerCase();
		let futureNonce = `0x${(nonce + 1000).toString(16)}`;

		expect(txpoolContent.result.queued[genesisAccount][futureNonce].nonce).to.be.equal(futureNonce);
		expect(txpoolContent.result.queued[genesisAccount][futureNonce].hash).to.be.equal(futureTx.transactionHash);
	});

	step("txpool_inspect should return correct result", async function () {
		let txpoolInspect = await customRequest(this.web3, "txpool_inspect", []);
		let genesisAccount = this.ethereumPairs.alith.address.toLowerCase();

		let currentNonce = `0x${nonce.toString(16)}`;
		let futureNonce = `0x${(nonce + 1000).toString(16)}`;

		expect(txpoolInspect.result.pending[genesisAccount][currentNonce]).to.be.equal(
			"0x0000000000000000000000000000000000000000: 0 wei + 1048576 gas x 1000000000 wei"
		);
		expect(txpoolInspect.result.queued[genesisAccount][futureNonce]).to.be.equal(
			"0x0000000000000000000000000000000000000000: 0 wei + 1048576 gas x 1000000000 wei"
		);
	});
});
