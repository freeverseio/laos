import { expect } from "chai";
import { step } from "mocha-steps";
import Contract from "web3-eth-contract";
import {
	VESTING_CONTRACT_ADDRESS,
	VESTING_ABI,
	GAS_LIMIT,
	GAS_PRICE,
	FAITH,
	FAITH_PRIVATE_KEY,
	ALITH,
	ALITH_PRIVATE_KEY,
} from "./config";
import { describeWithExistingNode, extractRevertReason } from "./util";
import Web3 from "web3";

describeWithExistingNode("Frontier RPC (Vesting)", (context) => {
	let contract: Contract;

	before(async function () {
		contract = new context.web3.eth.Contract(VESTING_ABI, VESTING_CONTRACT_ADDRESS, {
			from: FAITH,
			gasPrice: GAS_PRICE,
			gas: GAS_LIMIT,
		});
		context.web3.eth.accounts.wallet.add(FAITH_PRIVATE_KEY);
		context.web3.eth.accounts.wallet.add(ALITH_PRIVATE_KEY);
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
	it("when vesting exists it returns the list", async function () {
		const vesting = await contract.methods.vesting(ALITH).call();
		expect(vesting).to.deep.eq([
			["799999000000000000000000000", "7999990000000000000000000", "0"],
			["799999500000000000000000000", "3999997500000000000000000", "0"],
		]);
	});
	step("when vesting exists do vest returns ok", async function () {
		let nonce = await context.web3.eth.getTransactionCount(ALITH);
		contract.options.from = ALITH;
		let result = await contract.methods.vest().send({ from: ALITH, gas: GAS_LIMIT, nonce: nonce++ });
		expect(result.status).to.be.eq(true);
	});
	step("when vesting exists do vestOther returns ok", async function () {
		let nonce = await context.web3.eth.getTransactionCount(FAITH);
		let result = await contract.methods.vestOther(ALITH).send({ from: FAITH, gas: GAS_LIMIT, nonce: nonce++ });
		expect(result.status).to.be.eq(true);
	});
});
