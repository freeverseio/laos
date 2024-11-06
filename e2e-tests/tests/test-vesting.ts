import chai, { expect } from "chai";
import chaiAsPromised from "chai-as-promised";
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
	UNIT,
} from "./config";
import { describeWithExistingNode, sendTxAndWaitForFinalization } from "./util";
import { Keyring } from "@polkadot/api";

// Use chai-as-promised
chai.use(chaiAsPromised);

describeWithExistingNode("Frontier RPC (Vesting)", (context) => {
	let contract: Contract;
	let alithPair;

	before(async function () {
		contract = new context.web3.eth.Contract(VESTING_ABI, VESTING_CONTRACT_ADDRESS, {
			gasPrice: GAS_PRICE,
		});
		context.web3.eth.accounts.wallet.add(FAITH_PRIVATE_KEY);
		context.web3.eth.accounts.wallet.add(ALITH_PRIVATE_KEY);

		const keyring = new Keyring({ type: "ethereum" });
		alithPair = keyring.addFromUri(ALITH_PRIVATE_KEY);
	});

	it("when there is no vesting it returns empty list", async function () {
		const vesting = await contract.methods.vesting(FAITH).call();
		expect(vesting).to.deep.eq([]);
	});
	it("when there is no vesting do vest reverts", async function () {
		try {
			let nonce = await context.web3.eth.getTransactionCount(FAITH);
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
		expect(vesting).to.deep.eq([["700000000000000000000000000", "700000000000000000000000", "0"]]);
	});
	step("when vesting exists do vest returns ok", async function () {
		let nonce = await context.web3.eth.getTransactionCount(ALITH);
		contract.options.from = ALITH;
		const estimatedGas = await contract.methods.vest().estimateGas();
		let result = await contract.methods.vest().send({ from: ALITH, gas: estimatedGas, nonce: nonce++ });
		expect(result.status).to.be.eq(true);
	});
	step("when vesting exists do vestOther returns ok", async function () {
		let nonce = await context.web3.eth.getTransactionCount(ALITH);
		contract.options.from = ALITH;
		const estimatedGas = await contract.methods.vest().estimateGas();
		let result = await contract.methods.vestOther(ALITH).send({ from: FAITH, gas: estimatedGas, nonce: nonce++ });
		expect(result.status).to.be.eq(true);
	});
	step("createAndExecuteVesting", async function () {
		const { polkadot, web3 } = context;
		const locked = BigInt(1000) * UNIT;
		const perBlock = UNIT;
		const startingBlock = await polkadot.query.system.number();
		const account = web3.eth.accounts.create();

		// Step 1: Verify the new account's initial balance is zero
		await expect(web3.eth.getBalance(account.address)).to.eventually.be.eq("0");

		// Step 2: Ensure no vesting schedule exists for the new account
		let vestingSchedule = await expect(contract.methods.vesting(account.address).call()).to.be.fulfilled;
		expect(vestingSchedule).to.deep.eq([]);

		// Step 3: Create a vesting schedule via a substrate transaction
		const vestingTx = polkadot.tx.vesting.vestedTransfer(account.address, {
			locked,
			perBlock,
			startingBlock,
		});

		await expect(sendTxAndWaitForFinalization(vestingTx, alithPair)).to.eventually.be.fulfilled;

		// Step 4: Confirm the account balance has increased due to blocks mined since startingBlock
		const initialBalance = await web3.eth.getBalance(account.address);
		expect(Number(initialBalance) > 0 && Number(initialBalance) < Number(locked)).to.be.true;

		// Step 5: Verify the vesting schedule has been created correctly
		vestingSchedule = await expect(contract.methods.vesting(account.address).call()).to.eventually.be.fulfilled;
		expect(vestingSchedule).to.deep.eq([[locked.toString(), perBlock.toString(), startingBlock.toString()]]);

		// Step 6: Execute the vesting using an external account (e.g., ALITH)
		let gasEstimate = await contract.methods.vestOther(account.address).estimateGas();
		await expect(
			contract.methods.vestOther(account.address).send({ from: ALITH, gas: gasEstimate })
		).to.eventually.be.fulfilled;

		// Step 7: Verify the account balance has increased after vesting execution by the external account
		const balanceAfterVestOther = await web3.eth.getBalance(account.address);
		expect(Number(balanceAfterVestOther) > Number(initialBalance)).to.be.true;

		// Step 8: Execute vesting directly from the account itself
		web3.eth.accounts.wallet.add(account.privateKey); // Add the account to the wallet for transaction signing
		gasEstimate = await contract.methods.vest().estimateGas();
		await expect(
			contract.methods.vest().send({ from: account.address, gas: gasEstimate })
		).to.eventually.be.fulfilled;

		// Step 9: Verify the balance has increased after the second vesting execution
		const finalBalance = await web3.eth.getBalance(account.address);
		expect(Number(finalBalance) > Number(balanceAfterVestOther)).to.be.true;
	});
});
