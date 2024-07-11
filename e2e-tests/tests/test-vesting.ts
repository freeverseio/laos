import { expect } from "chai";
import { step } from "mocha-steps";
import Contract from "web3-eth-contract";
import { VESTING_CONTRACT_ADDRESS, VESTING_ABI, GAS_LIMIT, GAS_PRICE, FAITH, FAITH_PRIVATE_KEY } from "./config";
import { describeWithExistingNode, extractRevertReason } from "./util";
import Web3 from "web3";

describeWithExistingNode("Frontier RPC (Create Collection)", (context) => {
	let contract: Contract;

	before(async function () {
		contract = new context.web3.eth.Contract(VESTING_ABI, VESTING_CONTRACT_ADDRESS, {
			from: FAITH,
			gasPrice: GAS_PRICE,
			gas: GAS_LIMIT,
		});
		context.web3.eth.accounts.wallet.add(FAITH_PRIVATE_KEY);
	});

	it("when there is no vesting it returns empty list", async function () {
		const vesting = await contract.methods.vesting(FAITH).call();
		expect(vesting).to.deep.eq([]);
	});
	it("when there is no vesting do vest reverts", async function () {
		try {
			let nonce = await context.web3.eth.getTransactionCount(FAITH);
			await contract.methods.vest().send({ from: FAITH, gas: GAS_LIMIT, nonce: nonce++ });
			expect.fail("Expected error was not thrown"); // Ensure an error is thrown
		} catch (error) {
			expect(await extractRevertReason(context, error.receipt.transactionHash)).to.eq("NotVesting");
		}
	});
	it("when vesting exists it returns the list", async function () {});
	it("when vesting exists do vest returns ok", async function () {});
});
