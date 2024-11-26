import { expect } from "chai";
import { step } from "mocha-steps";
import Contract from "web3-eth-contract";
import {
	EVOLUTION_COLLECTION_FACTORY_CONTRACT_ADDRESS,
	EVOLUTION_COLLECTION_FACTORY_ABI,
	GAS_PRICE,
	REVERT_BYTECODE,
	SELECTOR_LOG_NEW_COLLECTION,
} from "./config";
import { createCollection, describeWithExistingNode } from "./util";

describeWithExistingNode("Frontier RPC (Create Collection)", function () {
	let contract: Contract;
	// This is the contract that is created in the test
	let testCollectionContract: Contract;
	// This is the address of another contract that is created in the test
	let testCollectionAddress: string;

	before(async function () {
		contract = new this.web3.eth.Contract(
			EVOLUTION_COLLECTION_FACTORY_ABI,
			EVOLUTION_COLLECTION_FACTORY_CONTRACT_ADDRESS,
			{
				from: this.ethereumPairs.faith.address,
				gasPrice: GAS_PRICE,
			}
		);
	});

	step("when collection is created, it should return owner", async function () {
		const collectionContract = await createCollection(this.context);
		testCollectionContract = collectionContract;

		const owner = await collectionContract.methods.owner().call();
		expect(owner).to.be.eq(this.ethereumPairs.faith.address);
	});

	step("when collection is created event is emitted", async function () {
		const estimatedGas = await contract.methods.createCollection(this.ethereumPairs.faith.address).estimateGas();
		const result = await contract.methods.createCollection(this.ethereumPairs.faith.address).send({
			from: this.ethereumPairs.faith.address,
			gas: estimatedGas,
			gasPrice: GAS_PRICE,
		});
		expect(result.status).to.be.eq(true);

		expect(Object.keys(result.events).length).to.be.eq(1);
		expect(this.web3.utils.isAddress(result.events.NewCollection.returnValues._collectionAddress)).to.be.eq(true);
		testCollectionAddress = result.events.NewCollection.returnValues._collectionAddress;
		expect(result.events.NewCollection.returnValues._owner).to.be.eq(this.ethereumPairs.faith.address);

		// event topics
		expect(result.events.NewCollection.raw.topics.length).to.be.eq(2);
		expect(result.events.NewCollection.raw.topics[0]).to.be.eq(SELECTOR_LOG_NEW_COLLECTION);
		expect(result.events.NewCollection.raw.topics[1]).to.be.eq(
			this.web3.utils.padLeft(this.ethereumPairs.faith.address.toLowerCase(), 64)
		);

		// event data
		expect(result.events.NewCollection.raw.data.toLowerCase()).to.be.eq(
			this.web3.utils.padLeft(result.events.NewCollection.returnValues._collectionAddress, 64).toLowerCase()
		);
	});

	step("when collection is created, bytecode is inserted in the storage", async function () {
		expect(await this.web3.eth.getCode(testCollectionContract.options.address)).to.be.eq(REVERT_BYTECODE);
		expect(await this.web3.eth.getCode(testCollectionAddress)).to.be.eq(REVERT_BYTECODE);

		// non-contract address doesn't have any code
		expect(await this.web3.eth.getCode(this.ethereumPairs.faith.address)).to.be.eq("0x");
	});
});
