import { expect } from "chai";
import { step } from "mocha-steps";
import Contract from "web3-eth-contract";
import {
	VESTING_CONTRACT_ADDRESS,
	VESTING_ABI,
	GAS_PRICE,
	FAITH,
	FAITH_PRIVATE_KEY,
	ALITH,
	ALITH_PRIVATE_KEY,
} from "./config";
import { describeWithExistingNode } from "./util";

describeWithExistingNode("Frontier RPC (Vesting)", function () {
	let contract: Contract;

	before(async function () {
		contract = new this.context.web3.eth.Contract(VESTING_ABI, VESTING_CONTRACT_ADDRESS, {
			gasPrice: GAS_PRICE,
		});
		this.context.web3.eth.accounts.wallet.add(FAITH_PRIVATE_KEY);
		this.context.web3.eth.accounts.wallet.add(ALITH_PRIVATE_KEY);
	});

	it("when there is no vesting it returns empty list", async function () {
		const vesting = await contract.methods.vesting(FAITH).call();
		expect(vesting).to.deep.eq([]);
	});
	it("when there is no vesting do vest reverts", async function () {
		try {
			let nonce = await this.context.web3.eth.getTransactionCount(FAITH);
			const estimatedGas = await contract.methods.vest().estimateGas();
			contract.options.from = FAITH;
			await contract.methods.vest().send({ from: FAITH, gas: estimatedGas, nonce: nonce++ });
			expect.fail("Expected error was not thrown"); // Ensure an error is thrown
		} catch (error) {
			expect(error.message).to.eq("Returned error: VM Exception while processing transaction: revert NotVesting");
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
		let nonce = await this.context.web3.eth.getTransactionCount(ALITH);
		contract.options.from = ALITH;
		const estimatedGas = await contract.methods.vest().estimateGas();
		let result = await contract.methods.vest().send({ from: ALITH, gas: estimatedGas, nonce: nonce++ });
		expect(result.status).to.be.eq(true);
	});
	step("when vesting exists do vestOther returns ok", async function () {
		let nonce = await this.context.web3.eth.getTransactionCount(FAITH);
		contract.options.from = FAITH;
		const estimatedGas = await contract.methods.vestOther(ALITH).estimateGas();
		let result = await contract.methods.vestOther(ALITH).send({ from: FAITH, gas: estimatedGas, nonce: nonce++ });
		expect(result.status).to.be.eq(true);
	});
});
