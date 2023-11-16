import { createCollection, describeWithExistingNode } from "./util";
import { CONTRACT_ADDRESS, GAS_LIMIT, GAS_PRICE, GENESIS_ACCOUNT, GENESIS_ACCOUNT_PRIVATE_KEY, LAOS_EVOLUTION_ABI, SELECTOR_LOG_NEW_COLLECTION } from "./config";
import { expect } from "chai";
import Contract from "web3-eth-contract";
import { step } from "mocha-steps";


describeWithExistingNode("Frontier RPC (Create Collection)", (context) => {
    let contract: Contract;

    beforeEach(async function () {
        contract = new context.web3.eth.Contract(LAOS_EVOLUTION_ABI, CONTRACT_ADDRESS, {
            from: GENESIS_ACCOUNT,
            gasPrice: GAS_PRICE,
            gas: GAS_LIMIT,
        });
        context.web3.eth.accounts.wallet.add(GENESIS_ACCOUNT_PRIVATE_KEY);
    });

    step("when collection does not exist owner of call should fail", async function () {
        try {
            await contract.methods.owner().call();
            expect.fail("Expected error was not thrown"); // Ensure an error is thrown
        } catch (error) {
            expect(error.message).to.be.eq(
                "Returned error: VM Exception while processing transaction: revert"
            );
        }
    });

    step("when collection is created, it should return owner", async function () {
        this.timeout(70000);

        const collectionContract = await createCollection(context);
        const owner = await collectionContract.methods.owner().call();
        expect(owner).to.be.eq(GENESIS_ACCOUNT);
    });

    step("when collection is created event is emitted", async function () {
        this.timeout(70000);

        const result = await contract.methods.createCollection(GENESIS_ACCOUNT).send({
            from: GENESIS_ACCOUNT,
            gas: GAS_LIMIT,
            gasPrice: GAS_PRICE,
        });
        expect(result.status).to.be.eq(true);

        expect(Object.keys(result.events).length).to.be.eq(1);
        expect(context.web3.utils.isAddress(result.events.NewCollection.returnValues._collectionAddress)).to.be.eq(true);
        expect(result.events.NewCollection.returnValues._owner).to.be.eq(GENESIS_ACCOUNT);

        // event topics
        expect(result.events.NewCollection.raw.topics.length).to.be.eq(2);
        expect(result.events.NewCollection.raw.topics[0]).to.be.eq(SELECTOR_LOG_NEW_COLLECTION);
        expect(result.events.NewCollection.raw.topics[1]).to.be.eq(context.web3.utils.padLeft(GENESIS_ACCOUNT.toLowerCase(), 64));

        // event data
        expect(result.events.NewCollection.raw.data.toLowerCase()).to.be.eq(context.web3.utils.padLeft(result.events.NewCollection.returnValues._collectionAddress, 64).toLowerCase());
    });

});