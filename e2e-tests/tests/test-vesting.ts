import chai, { expect } from "chai";
import chaiAsPromised from "chai-as-promised";
import { step } from "mocha-steps";
import Contract from "web3-eth-contract";
import { VESTING_CONTRACT_ADDRESS, VESTING_ABI, ALITH, ALITH_PRIVATE_KEY, UNIT } from "./config";
import { describeWithExistingNode, sendTxAndWaitForFinalization, waitForConfirmations, waitForBlocks } from "./util";
import { Keyring } from "@polkadot/api";

// Use chai-as-promised
chai.use(chaiAsPromised);

describeWithExistingNode("Frontier RPC (Vesting)", (context) => {
	let contract: Contract;
	let alithPair;

	before(async function () {
		contract = new context.web3.eth.Contract(VESTING_ABI, VESTING_CONTRACT_ADDRESS, {
			gasPrice: "20000000000", // default gas price in wei, 20 gwei in this case
		});

		const keyring = new Keyring({ type: "ethereum" });
		alithPair = keyring.addFromUri(ALITH_PRIVATE_KEY);
	});

	step("should revert when vesting is not enabled", async function () {
		const newAccount = context.web3.eth.accounts.create();
		await expect(contract.methods.vest().call({ from: newAccount.address })).to.be.rejectedWith(
			"Returned error: VM Exception while processing transaction: revert NotVesting"
		);
	});

	step("create and execute vesting", async function () {
		const { polkadot, web3 } = context;
		const locked = BigInt(1000) * UNIT;
		const perBlock = UNIT;
		const startingBlock = await polkadot.query.system.number();
		const account = web3.eth.accounts.create();
		web3.eth.accounts.wallet.add(account.privateKey); // Add the account to the wallet for transaction signing

		// Step 1: Verify the new account's initial balance is zero
		await expect(web3.eth.getBalance(account.address)).to.eventually.be.eq("0");

		// Step 2: Ensure no vesting schedule exists for the new account
		let vestingSchedule = await expect(contract.methods.vesting(account.address).call()).to.be.fulfilled;
		expect(vestingSchedule).to.deep.eq([]);

		await expect(waitForBlocks(polkadot, 1)).to.eventually.be.fulfilled;

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
		let gas = await contract.methods.vestOther(account.address).estimateGas({ from: ALITH });
		let tx = await expect(contract.methods.vestOther(account.address).send({ from: ALITH, gas })).to.eventually.be
			.fulfilled;
		await expect(waitForConfirmations(web3, tx.transactionHash, 3)).to.eventually.be.fulfilled;

		// Step 7: Verify the account balance has increased after vesting execution by the external account
		const balanceAfterVestOther = await web3.eth.getBalance(account.address);
		expect(Number(balanceAfterVestOther) > Number(initialBalance)).to.be.true;

		await expect(waitForBlocks(polkadot, 1)).to.eventually.be.fulfilled;

		// Step 8: Execute vesting directly from the account itself
		gas = await contract.methods.vest().estimateGas({ from: account.address });
		tx = await expect(contract.methods.vest().send({ from: account.address, gas })).to.eventually.be.fulfilled;
		await expect(waitForConfirmations(web3, tx.transactionHash, 3)).to.eventually.be.fulfilled;

		// Step 9: Verify the balance has increased after the second vesting execution
		const finalBalance = await web3.eth.getBalance(account.address);
		expect(Number(finalBalance)).to.be.greaterThan(Number(balanceAfterVestOther));
	});
});
