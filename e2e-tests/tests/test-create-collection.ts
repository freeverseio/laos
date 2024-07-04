import { expect } from "chai";
import { step } from "mocha-steps";
import Contract from "web3-eth-contract";
import {
	CONTRACT_ADDRESS,
	EVOLUTION_COLLETION_FACTORY_ABI,
	GAS_LIMIT,
	GAS_PRICE,
	TESTING_ACCOUNT,
	TESTING_ACCOUNT_PRIVATE_KEY,
	REVERT_BYTECODE,
	SELECTOR_LOG_NEW_COLLECTION,
} from "./config";
import { createCollection, describeWithExistingNode } from "./util";

describeWithExistingNode("Frontier RPC (Create Collection)", (context) => {
	let contract: Contract;
	// This is the contract that is created in the test
	let testCollectionContract: Contract;
	// This is the address of another contract that is created in the test
	let testCollectionAddress: string;

	beforeEach(async function () {
		contract = new context.web3.eth.Contract(EVOLUTION_COLLETION_FACTORY_ABI, CONTRACT_ADDRESS, {
			from: TESTING_ACCOUNT,
			gasPrice: GAS_PRICE,
			gas: GAS_LIMIT,
		});
		context.web3.eth.accounts.wallet.add(TESTING_ACCOUNT_PRIVATE_KEY);
	});

	step("when collection is created, it should return owner", async function () {
		this.timeout(70000);

		const collectionContract = await createCollection(context);
		testCollectionContract = collectionContract;
		
		const owner = await collectionContract.methods.owner().call();
		expect(owner).to.be.eq(TESTING_ACCOUNT);
	});

	step("when collection is created event is emitted", async function () {
		this.timeout(70000);

		const result = await contract.methods.createCollection(TESTING_ACCOUNT).send({
			from: TESTING_ACCOUNT,
			gas: GAS_LIMIT,
			gasPrice: GAS_PRICE,
		});
		expect(result.status).to.be.eq(true);

		expect(Object.keys(result.events).length).to.be.eq(1);
		expect(context.web3.utils.isAddress(result.events.NewCollection.returnValues._collectionAddress)).to.be.eq(
			true
		);
		testCollectionAddress = result.events.NewCollection.returnValues._collectionAddress;
		expect(result.events.NewCollection.returnValues._owner).to.be.eq(TESTING_ACCOUNT);

		// event topics
		expect(result.events.NewCollection.raw.topics.length).to.be.eq(2);
		expect(result.events.NewCollection.raw.topics[0]).to.be.eq(SELECTOR_LOG_NEW_COLLECTION);
		expect(result.events.NewCollection.raw.topics[1]).to.be.eq(
			context.web3.utils.padLeft(TESTING_ACCOUNT.toLowerCase(), 64)
		);

		// event data
		expect(result.events.NewCollection.raw.data.toLowerCase()).to.be.eq(
			context.web3.utils.padLeft(result.events.NewCollection.returnValues._collectionAddress, 64).toLowerCase()
		);
	});

	step("when collection is created, bytecode is inserted in the storage", async function () {
		expect(await context.web3.eth.getCode(testCollectionContract.options.address)).to.be.eq(REVERT_BYTECODE);
		expect(await context.web3.eth.getCode(testCollectionAddress)).to.be.eq(REVERT_BYTECODE);

		// non-contract address doesn't have any code
		expect(await context.web3.eth.getCode(TESTING_ACCOUNT)).to.be.eq("0x");
	});

	step("owner call can estimate gas", async function () {
		const estimateGas = await testCollectionContract.methods.owner().estimateGas();
		expect(estimateGas).to.be.eq(22431);
	});

	step("create collection call can estimate gas", async function () {
		const contract = new context.web3.eth.Contract(EVOLUTION_COLLETION_FACTORY_ABI, CONTRACT_ADDRESS, {
			from: TESTING_ACCOUNT,
			gasPrice: GAS_PRICE,
		});
		
		let nonce = await context.web3.eth.getTransactionCount(TESTING_ACCOUNT);
		context.web3.eth.accounts.wallet.add(TESTING_ACCOUNT_PRIVATE_KEY);
		
		const estimatedGas = await contract.methods.createCollection(TESTING_ACCOUNT).estimateGas({
			from: TESTING_ACCOUNT,
			gas: GAS_LIMIT,
			gasPrice: GAS_PRICE,
			nonce: nonce++,
		});
		expect(estimatedGas).to.be.eq(46846);
	});
});
