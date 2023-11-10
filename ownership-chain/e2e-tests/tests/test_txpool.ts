import { expect } from "chai";
import { step } from "mocha-steps";

import { OWNCHAIN_SUDO, OWNCHAIN_SUDO_PRIVATE_KEY } from "./config";
import { customRequest, describeWithExistingNode } from "./util";

describeWithExistingNode("Frontier RPC (TxPoolApi)", (context) => {
	const TEST_CONTRACT_BYTECODE = "0x608060405234801561";

	var nonce = 0;
	let pending_tx;
	let future_tx;
	async function sendTransaction(context, nonce) {
		const tx = await context.web3.eth.accounts.signTransaction(
			{
				from: OWNCHAIN_SUDO,
				data: TEST_CONTRACT_BYTECODE,
				value: "0x00",
				gasPrice: "0x3B9ACA00",
				gas: "0x100000",
				nonce: nonce,
			},
			OWNCHAIN_SUDO_PRIVATE_KEY
		);
		await customRequest(context.web3, "eth_sendRawTransaction", [tx.rawTransaction]);
		return tx;
	}

	step("txpool_status should return correct result", async function () {
		let txpoolStatus = await customRequest(context.web3, "txpool_status", []);
		expect(txpoolStatus.result.pending).to.be.equal("0x0");
		expect(txpoolStatus.result.queued).to.be.equal("0x0");

		pending_tx = await sendTransaction(context, nonce);
		future_tx = await sendTransaction(context, nonce + 3);
		txpoolStatus = await customRequest(context.web3, "txpool_status", []);

		expect(txpoolStatus.result.pending).to.be.equal("0x1");
		expect(txpoolStatus.result.queued).to.be.equal("0x1");
	});

	step("txpool_content should return correct result", async function () {
		let txpoolContent = await customRequest(context.web3, "txpool_content", []);
		let genesisAccount = OWNCHAIN_SUDO.toLowerCase();

		expect(txpoolContent.result.queued[genesisAccount]["0x3"].nonce).to.be.equal("0x3");
		expect(txpoolContent.result.queued[genesisAccount]["0x3"].hash).to.be.equal(future_tx.transactionHash);
	});

	step("txpool_inspect should return correct result", async function () {
		let txpoolInspect = await customRequest(context.web3, "txpool_inspect", []);
		let genesisAccount = OWNCHAIN_SUDO.toLowerCase();

		expect(txpoolInspect.result.pending[genesisAccount]["0x0"]).to.be.equal(
			"0x0000000000000000000000000000000000000000: 0 wei + 1048576 gas x 1000000000 wei"
		);
		expect(txpoolInspect.result.queued[genesisAccount]["0x3"]).to.be.equal(
			"0x0000000000000000000000000000000000000000: 0 wei + 1048576 gas x 1000000000 wei"
		);
	});
});
