import chai, { expect } from "chai";
import chaiAsPromised from "chai-as-promised";
import { step } from "mocha-steps";
import Contract from "web3-eth-contract";
import { VESTING_CONTRACT_ADDRESS, VESTING_ABI, ONE_LAOS, GAS_PRICE } from "@utils/constants";
import { describeWithExistingNode } from "@utils/setups";
import { sendTxAndWaitForFinalization, waitFinalizedEthereumTx } from "@utils/transactions";

// Use chai-as-promised
chai.use(chaiAsPromised);

describeWithExistingNode("Frontier RPC (Vesting)", function () {
	let contract: Contract;

	before(async function () {
		contract = new this.web3.eth.Contract(VESTING_ABI, VESTING_CONTRACT_ADDRESS, {
			gasPrice: GAS_PRICE,
		});
		contract.defaultBlock = "safe";
	});

	step("should revert when vesting is not enabled", async function () {
		const newAccount = this.web3.eth.accounts.create();
		await expect(contract.methods.vest().call({ from: newAccount.address })).to.be.rejectedWith(
			"Returned error: VM Exception while processing transaction: revert NotVesting"
		);
	});

	step("create and execute vesting", async function () {
		const locked = ONE_LAOS.muln(1000);
		const perBlock = ONE_LAOS;
		const finalizedHash = await this.chains.laos.rpc.chain.getFinalizedHead();
		const finalizedBlock = await this.chains.laos.rpc.chain.getBlock(finalizedHash);
		const startingBlock = finalizedBlock.block.header.number;
		const account = this.web3.eth.accounts.create();
		this.web3.eth.accounts.wallet.add(account.privateKey); // Add account for signing transactions

		// Step 1: Verify initial balance is zero
		await expect(this.web3.eth.getBalance(account.address)).to.eventually.equal("0");

		// Step 2: Confirm no existing vesting schedule
		let vestingSchedule = await contract.methods.vesting(account.address).call();
		expect(vestingSchedule).to.deep.equal([]);

		// Step 3: Create vesting schedule via substrate transaction
		const vestingTx = this.chains.laos.tx.vesting.vestedTransfer(account.address, {
			locked,
			perBlock,
			startingBlock,
		});
		await sendTxAndWaitForFinalization(this.chains.laos, vestingTx, this.ethereumPairs.alith);

		// Step 4: Check balance has increased since startingBlock
		const initialBalance = await this.web3.eth.getBalance(account.address);
		expect(Number(initialBalance)).to.be.greaterThan(Number(0));
		expect(Number(initialBalance)).to.be.lessThan(Number(locked));

		// Step 5: Verify vesting schedule was created
		vestingSchedule = await contract.methods.vesting(account.address).call();
		expect(vestingSchedule).to.deep.equal([[locked.toString(), perBlock.toString(), startingBlock.toString()]]);

		// Step 6: Execute vesting with an external account (ALITH)
		let gas = await contract.methods
			.vestOther(account.address)
			.estimateGas({ from: this.ethereumPairs.alith.address });
		let tx = await contract.methods
			.vestOther(account.address)
			.send({ from: this.ethereumPairs.alith.address, gas });
		await waitFinalizedEthereumTx(this.web3, this.chains.laos, tx.transactionHash);

		// Step 7: Confirm balance increase after external vesting
		const balanceAfterVestOther = await this.web3.eth.getBalance(account.address);
		expect(Number(balanceAfterVestOther)).to.be.greaterThan(Number(initialBalance));

		// Step 8: Execute vesting directly from the account
		gas = await contract.methods.vest().estimateGas({ from: account.address });
		tx = await contract.methods.vest().send({ from: account.address, gas });
		await waitFinalizedEthereumTx(this.web3, this.chains.laos, tx.transactionHash);

		// Step 9: Verify final balance increase after second vesting
		const finalBalance = await this.web3.eth.getBalance(account.address);
		expect(Number(finalBalance)).to.be.greaterThan(Number(balanceAfterVestOther));
	});
});
