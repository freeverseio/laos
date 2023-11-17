import { addressToCollectionId, createCollection, describeWithExistingNode, slotAndOwnerToTokenId } from "./util";
import { GAS_LIMIT, GENESIS_ACCOUNT, SELECTOR_LOG_EVOLVED_WITH_EXTERNAL_TOKEN_URI, SELECTOR_LOG_MINTED_WITH_EXTERNAL_TOKEN_URI } from "./config";
import { expect } from "chai";
import Contract from "web3-eth-contract";
import BN from "bn.js";
import { step } from "mocha-steps";

describeWithExistingNode("Frontier RPC (Mint and Evolve Assets)", (context) => {
    let collectionId: BN;
    let collectionContract: Contract

    beforeEach(async function () {
        this.timeout(70000);

        collectionContract = await createCollection(context);
        collectionId = addressToCollectionId(collectionContract.options.address);
    });
    
    step("when collection does not exist token uri should fail", async function () {
        const tokenId = "0";
        
        try {
            await collectionContract.methods.tokenURI(tokenId).call();
            expect.fail("Expected error was not thrown"); // Ensure an error is thrown
        } catch (error) {
            expect(error.message).to.be.eq(
                "Returned error: VM Exception while processing transaction: revert"
            );
        }
    });

    step("when asset is minted it should return token uri", async function () {
        this.timeout(70000);

        const slot = "0";
        const to = GENESIS_ACCOUNT;
        const tokenURI = "https://example.com";

        let nonce = await context.web3.eth.getTransactionCount(GENESIS_ACCOUNT);
        const result = await collectionContract.methods.mintWithExternalURI(to, slot, tokenURI).send({ from: GENESIS_ACCOUNT, gas: GAS_LIMIT, nonce: nonce++ });
        expect(result.status).to.be.eq(true);

        const tokenId = result.events.MintedWithExternalURI.returnValues._tokenId;
        const got = await collectionContract.methods.tokenURI(tokenId).call();
        expect(got).to.be.eq(tokenURI);
    });

    step("given slot and owner it should return token id", async function () {
        this.timeout(70000);

        const slot = "1";
        const to = GENESIS_ACCOUNT;

        const tokenId = slotAndOwnerToTokenId(slot, to);
        expect(tokenId).to.be.eq("000000000000000000000001c0f0f4ab324c46e55d02d0033343b4be8a55532d");
        const tokenIdDecimal = new BN(tokenId, 16, "be").toString(10);
        expect(tokenIdDecimal).to.be.eq("2563001357829637001682277476112176020532353127213");
    });

    step("when asset is minted it should emit an event", async function () {
        this.timeout(70000);

        const slot = "22";
        const to = GENESIS_ACCOUNT;
        const tokenURI = "https://example.com";

        const result = await collectionContract.methods.mintWithExternalURI(to, slot, tokenURI)
            .send({ from: GENESIS_ACCOUNT, gas: GAS_LIMIT });
        expect(result.status).to.be.eq(true);

        expect(Object.keys(result.events).length).to.be.eq(1);

        // data returned within the event
        expect(result.events.MintedWithExternalURI.returnValues._to).to.be.eq(to);
        expect(result.events.MintedWithExternalURI.returnValues._slot).to.be.eq(slot);
        expect(result.events.MintedWithExternalURI.returnValues._tokenURI).to.be.eq(tokenURI);
        const tokenId = slotAndOwnerToTokenId(slot, to);
        const tokenIdDecimal = new BN(tokenId, 16, "be").toString(10);
        expect(result.events.MintedWithExternalURI.returnValues._tokenId).to.be.eq(tokenIdDecimal);

        // event topics
        expect(result.events.MintedWithExternalURI.raw.topics.length).to.be.eq(2);
        expect(result.events.MintedWithExternalURI.raw.topics[0]).to.be.eq(SELECTOR_LOG_MINTED_WITH_EXTERNAL_TOKEN_URI);
        expect(result.events.MintedWithExternalURI.raw.topics[1]).to.be.eq(context.web3.utils.padLeft(GENESIS_ACCOUNT.toLowerCase(), 64));

        // event data
        expect(result.events.MintedWithExternalURI.raw.data).to.be.eq(
            context.web3.eth.abi.encodeParameters(
                ["uint96", "uint256", "string"],
                [slot, tokenIdDecimal, tokenURI]
            )
        );
    });

    step("when asset is evolved it should change token uri", async function () {
        this.timeout(70000);

        const slot = "22";
        const to = GENESIS_ACCOUNT;
        const tokenURI = "https://example.com";
        const newTokenURI = "https://new_example.com";
        const tokenId = slotAndOwnerToTokenId(slot, to);
        const tokenIdDecimal = new BN(tokenId, 16, "be").toString(10);

        const mintingResult = await collectionContract.methods.mintWithExternalURI(to, slot, tokenURI).send({ from: GENESIS_ACCOUNT, gas: GAS_LIMIT });
        expect(mintingResult.status).to.be.eq(true);

        const evolvingResult = await collectionContract.methods.evolveWithExternalURI(tokenIdDecimal, newTokenURI).send({ from: GENESIS_ACCOUNT, gas: GAS_LIMIT });
        expect(evolvingResult.status).to.be.eq(true);

        const got = await collectionContract.methods.tokenURI(tokenIdDecimal).call();
        expect(got).to.be.eq(newTokenURI);
    });

    step("when asset is evolved it should emit an event", async function () {
        this.timeout(70000);

        const slot = "22";
        const to = GENESIS_ACCOUNT;
        const tokenURI = "https://example.com";
        const newTokenURI = "https://new_example.com";
        const tokenId = slotAndOwnerToTokenId(slot, to);
        const tokenIdDecimal = new BN(tokenId, 16, "be").toString(10);

        const mintingResult = await collectionContract.methods.mintWithExternalURI(to, slot, tokenURI).send({ from: GENESIS_ACCOUNT, gas: GAS_LIMIT });
        expect(mintingResult.status).to.be.eq(true);

        const evolvingResult = await collectionContract.methods.evolveWithExternalURI(tokenIdDecimal, newTokenURI).send({ from: GENESIS_ACCOUNT, gas: GAS_LIMIT });
        expect(evolvingResult.status).to.be.eq(true);

        expect(Object.keys(evolvingResult.events).length).to.be.eq(1);

        // data returned within the event
        expect(evolvingResult.events.EvolvedWithExternalURI.returnValues._tokenId).to.be.eq(tokenIdDecimal);
        expect(evolvingResult.events.EvolvedWithExternalURI.returnValues._tokenURI).to.be.eq(newTokenURI);

        // event topics
        expect(evolvingResult.events.EvolvedWithExternalURI.raw.topics.length).to.be.eq(2);
        expect(evolvingResult.events.EvolvedWithExternalURI.raw.topics[0]).to.be.eq(SELECTOR_LOG_EVOLVED_WITH_EXTERNAL_TOKEN_URI);
        expect(evolvingResult.events.EvolvedWithExternalURI.raw.topics[1]).to.be.eq("0x" + tokenId);

        // event data
        expect(evolvingResult.events.EvolvedWithExternalURI.raw.data).to.be.eq(
            context.web3.eth.abi.encodeParameters(
                ["string"],
                [newTokenURI]
            )
        );
    });
});