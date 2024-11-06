import chai, { expect } from "chai";
import chaiAsPromised from "chai-as-promised";
import { step } from "mocha-steps";
import Contract from "web3-eth-contract";
import { VESTING_CONTRACT_ADDRESS, VESTING_ABI, ALITH, ALITH_PRIVATE_KEY, UNIT, GAS_PRICE } from "./config";
import { describeWithExistingNode, sendTxAndWaitForFinalization, waitForConfirmations, waitForBlocks } from "./util";
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
		contract.defaultBlock = "safe";

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
		console.log("Vesting account address: ", account.address);
		web3.eth.accounts.wallet.add(account.privateKey); // Add account for signing transactions

		// Step 1: Verify initial balance is zero
		await expect(web3.eth.getBalance(account.address)).to.eventually.equal("0");

		// Step 2: Confirm no existing vesting schedule
		let vestingSchedule = await contract.methods.vesting(account.address).call();
		expect(vestingSchedule).to.deep.equal([]);

		await waitForBlocks(polkadot, 1);

		// Step 3: Create vesting schedule via substrate transaction
		const vestingTx = polkadot.tx.vesting.vestedTransfer(account.address, {
			locked,
			perBlock,
			startingBlock,
		});
		await sendTxAndWaitForFinalization(polkadot, vestingTx, alithPair);

		// Step 4: Check balance has increased since startingBlock
		const initialBalance = await web3.eth.getBalance(account.address);
		expect(Number(initialBalance)).to.be.greaterThan(Number(0));
		expect(Number(initialBalance)).to.be.lessThan(Number(locked));

		// Step 5: Verify vesting schedule was created
		vestingSchedule = await contract.methods.vesting(account.address).call();
		expect(vestingSchedule).to.deep.equal([[locked.toString(), perBlock.toString(), startingBlock.toString()]]);

		// Step 6: Execute vesting with an external account (ALITH)
		let gas = await contract.methods.vestOther(account.address).estimateGas({ from: ALITH });
		let tx = await contract.methods.vestOther(account.address).send({ from: ALITH, gas });
		await waitForConfirmations(web3, tx.transactionHash, 3);

		// Step 7: Confirm balance increase after external vesting
		const balanceAfterVestOther = await web3.eth.getBalance(account.address);
		expect(Number(balanceAfterVestOther)).to.be.greaterThan(Number(initialBalance));

		await waitForBlocks(polkadot, 1);

		// Step 8: Execute vesting directly from the account
		gas = await contract.methods.vest().estimateGas({ from: account.address });
		tx = await contract.methods.vest().send({ from: account.address, gas });
		await waitForConfirmations(web3, tx.transactionHash, 3);

		// Step 9: Verify final balance increase after second vesting
		const finalBalance = await web3.eth.getBalance(account.address);
		expect(Number(finalBalance)).to.be.greaterThan(Number(balanceAfterVestOther));
	});
});
