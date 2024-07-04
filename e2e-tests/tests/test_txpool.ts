import { expect } from "chai";
import { step } from "mocha-steps";

import { ALITH, ALITH_PRIVATE_KEY } from "./config";
import { customRequest, describeWithExistingNode } from "./util";

describeWithExistingNode("Frontier RPC (TxPoolApi)", (context) => {
	const TEST_CONTRACT_BYTECODE = "0x608060405234801561";

	let nonce;
	let pendingTx;
	let futureTx;
	async function sendTransaction(context, nonce) {
		const tx = await context.web3.eth.accounts.signTransaction(
			{
				from: ALITH,
				data: TEST_CONTRACT_BYTECODE,
				value: "0x00",
				gasPrice: "0x3B9ACA00",
				gas: "0x100000",
				nonce: nonce,
			},
			ALITH_PRIVATE_KEY
		);
		await customRequest(context.web3, "eth_sendRawTransaction", [tx.rawTransaction]);
		return tx;
	}

	before(async function () {
		nonce = await context.web3.eth.getTransactionCount(ALITH);
	});

	step("txpool_status should return correct result", async function () {
		let txpoolStatusBefore = await customRequest(context.web3, "txpool_status", []);

		pendingTx = await sendTransaction(context, nonce);
		futureTx = await sendTransaction(context, nonce + 1000);
		let txpoolStatusAfter = await customRequest(context.web3, "txpool_status", []);

		expect(parseInt(txpoolStatusAfter.result.pending, 16)).to.be.equal(
			parseInt(txpoolStatusBefore.result.pending, 16) + 1
		);
		expect(parseInt(txpoolStatusAfter.result.queued, 16)).to.be.equal(
			parseInt(txpoolStatusBefore.result.queued, 16) + 1
		);
	});

	step("txpool_content should return correct result", async function () {
		let txpoolContent = await customRequest(context.web3, "txpool_content", []);

		let genesisAccount = ALITH.toLowerCase();
		let futureNonce = `0x${(nonce + 1000).toString(16)}`;

		expect(txpoolContent.result.queued[genesisAccount][futureNonce].nonce).to.be.equal(futureNonce);
		expect(txpoolContent.result.queued[genesisAccount][futureNonce].hash).to.be.equal(futureTx.transactionHash);
	});

	step("txpool_inspect should return correct result", async function () {
		let txpoolInspect = await customRequest(context.web3, "txpool_inspect", []);
		let genesisAccount = ALITH.toLowerCase();

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
