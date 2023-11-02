import { customRequest, describeWithExistingNode } from "./util";
import { step } from "mocha-steps";
import { GENESIS_ACCOUNT, GENESIS_ACCOUNT_BALANCE, GENESIS_ACCOUNT_PRIVATE_KEY } from "./config";
import LaosEvolution from "../build/contracts/LaosEvolution.json";
import { AbiItem } from "web3-utils";
import { expect } from "chai";

describeWithExistingNode("Frontier RPC (Create Collection)", (context) => {
    const LAOS_EVOLUTION_ABI = LaosEvolution.abi as AbiItem[]
    const collectionId = "0";
    const contractAddress = "0x0000000000000000000000000000000000000403";
    const contract = new context.web3.eth.Contract(LAOS_EVOLUTION_ABI, contractAddress, {
        from: GENESIS_ACCOUNT,
        gasPrice: "0x3B9ACA00",
    });

    step("genesis balance is setup correctly", async function () {
        expect(await context.web3.eth.getBalance(GENESIS_ACCOUNT)).to.equal(GENESIS_ACCOUNT_BALANCE);
    });

    step("when collection does not exist owner of call should fail", async function () {
        const collectionId = "0";
        try {
            await contract.methods.ownerOfCollection(collectionId).call();
        } catch (error) {
            expect(error.message).to.be.eq(
                "Returned error: VM Exception while processing transaction: revert"
            );
        }
    });

    step("when collection is created should return owner", async function () {
        this.timeout(70000);

        const collectionId = "0";
        let nonce = await context.web3.eth.getTransactionCount(GENESIS_ACCOUNT);
        
        await context.web3.eth.accounts.wallet.add(GENESIS_ACCOUNT_PRIVATE_KEY);
        const result = await contract.methods.createCollection(GENESIS_ACCOUNT).send({ from: GENESIS_ACCOUNT, gas: "0x10000", nonce: nonce++ });
        expect(result.status).to.be.eq(true);
        
        const owner = await contract.methods.ownerOfCollection(collectionId).call();
        expect(owner).to.be.eq(GENESIS_ACCOUNT);
    });
    
    step("when collection is created event is emitted", async function () {
        this.timeout(70000);
        
        const collectionId = "1";
        let nonce = await context.web3.eth.getTransactionCount(GENESIS_ACCOUNT);
        const SELECTOR_LOG_NEW_COLLECTION = "0x6eb24fd767a7bcfa417f3fe25a2cb245d2ae52293d3c4a8f8c6450a09795d289";

        await context.web3.eth.accounts.wallet.add(GENESIS_ACCOUNT_PRIVATE_KEY);
        const result = await contract.methods.createCollection(GENESIS_ACCOUNT).send({ from: GENESIS_ACCOUNT, gas: "0x10000", nonce: nonce++ });
        expect(result.status).to.be.eq(true);

        expect(Object.keys(result.events).length).to.be.eq(1);
        expect(result.events.NewCollection.returnValues.collectionId).to.be.eq(collectionId);
        expect(result.events.NewCollection.returnValues.owner).to.be.eq(GENESIS_ACCOUNT);

        // event topics
        expect(result.events.NewCollection.raw.topics.length).to.be.eq(2);
        expect(result.events.NewCollection.raw.topics[0]).to.be.eq(SELECTOR_LOG_NEW_COLLECTION);
        expect(result.events.NewCollection.raw.topics[1]).to.be.eq(context.web3.utils.padLeft(GENESIS_ACCOUNT.toLowerCase(), 64));

        // event data
        expect(result.events.NewCollection.raw.data).to.be.eq("0x" + context.web3.utils.padLeft(collectionId, 64));
    });

});