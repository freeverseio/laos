import { expect } from "chai";
import { step } from "mocha-steps";
import Contract from "web3-eth-contract";
import {
	CONTRACT_ADDRESS,
	GAS,
	GAS_PRICE,
	GENESIS_ACCOUNT,
	GENESIS_ACCOUNT_PRIVATE_KEY,
	LAOS_EVOLUTION_ABI,
	SELECTOR_LOG_NEW_COLLECTION,
} from "./config";
import { describeWithExistingNode } from "./util";

describeWithExistingNode("Frontier RPC (Create Collection)", (context) => {
	let contract: Contract;
	let nonce: number;

	beforeEach(async function () {
		contract = new context.web3.eth.Contract(LAOS_EVOLUTION_ABI, CONTRACT_ADDRESS, {
			from: GENESIS_ACCOUNT,
			gasPrice: GAS_PRICE,
		});

		nonce = await context.web3.eth.getTransactionCount(GENESIS_ACCOUNT);
		context.web3.eth.accounts.wallet.add(GENESIS_ACCOUNT_PRIVATE_KEY);
	});

	step("when collection does not exist owner of call should fail", async function () {
		try {
			await contract.methods.ownerOfCollection("1111").call();
			expect.fail("Expected error was not thrown"); // Ensure an error is thrown
		} catch (error) {
			expect(error.message).to.be.eq("Returned error: VM Exception while processing transaction: revert");
		}
	});

	step("when collection is created, it should return owner", async function () {
		this.timeout(70000);

		const result = await contract.methods
			.createCollection(GENESIS_ACCOUNT)
			.send({ from: GENESIS_ACCOUNT, gas: GAS, nonce: nonce++ });
		expect(result.status).to.be.eq(true);

		const owner = await contract.methods
			.ownerOfCollection(result.events.NewCollection.returnValues.collectionId)
			.call();
		expect(owner).to.be.eq(GENESIS_ACCOUNT);
	});

	step("when collection is created event is emitted", async function () {
		this.timeout(70000);

		const result = await contract.methods
			.createCollection(GENESIS_ACCOUNT)
			.send({ from: GENESIS_ACCOUNT, gas: GAS, nonce: nonce++ });
		expect(result.status).to.be.eq(true);

		expect(Object.keys(result.events).length).to.be.eq(1);
		expect(result.events.NewCollection.returnValues.collectionId).not.to.be.NaN;
		expect(result.events.NewCollection.returnValues.owner).to.be.eq(GENESIS_ACCOUNT);

		// event topics
		expect(result.events.NewCollection.raw.topics.length).to.be.eq(2);
		expect(result.events.NewCollection.raw.topics[0]).to.be.eq(SELECTOR_LOG_NEW_COLLECTION);
		expect(result.events.NewCollection.raw.topics[1]).to.be.eq(
			context.web3.utils.padLeft(GENESIS_ACCOUNT.toLowerCase(), 64)
		);

		// event data
		expect(result.events.NewCollection.raw.data).to.be.eq(
			"0x" + context.web3.utils.padLeft(result.events.NewCollection.returnValues.collectionId, 64)
		);
	});
});
