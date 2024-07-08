import { addressToCollectionId, createCollection, describeWithExistingNode, slotAndOwnerToTokenId } from "./util";
import { GAS_LIMIT, FAITH, SELECTOR_LOG_EVOLVED_WITH_EXTERNAL_TOKEN_URI, SELECTOR_LOG_MINTED_WITH_EXTERNAL_TOKEN_URI, SELECTOR_LOG_OWNERSHIP_TRANSFERRED, SELECTOR_LOG_PUBLIC_MINTING_ENABLED, SELECTOR_LOG_PUBLIC_MINTING_DISABLED, ALITH } from "./config";
import { expect } from "chai";
import Contract from "web3-eth-contract";
import BN from "bn.js";
import { step } from "mocha-steps";

describeWithExistingNode("Frontier RPC (Mint Assets)", (context) => {
    let collectionContract: Contract
    let mintingResult: any;

    before(async function () {
        collectionContract = await createCollection(context);
    });

    step("when collection does not exist token uri should fail", async function () {
        const tokenId = "0";

        try {
            await collectionContract.methods.tokenURI(tokenId).call();
            expect.fail("Expected error was not thrown"); // Ensure an error is thrown
        } catch (error) {
            expect(error.message).to.be.eq(
                "Returned error: VM Exception while processing transaction: revert asset does not exist"
            );
        }
    });

    const slot = "22";
    const to = FAITH;
    const tokenURI = "https://example.com";

    step("when asset is minted it should return token uri", async function () {
        let nonce = await context.web3.eth.getTransactionCount(FAITH);
        mintingResult = await collectionContract.methods.mintWithExternalURI(to, slot, tokenURI).send({ from: FAITH, gas: GAS_LIMIT, nonce: nonce++ });
        expect(mintingResult.status).to.be.eq(true);

        const tokenId = mintingResult.events.MintedWithExternalURI.returnValues._tokenId;
        const got = await collectionContract.methods.tokenURI(tokenId).call();
        expect(got).to.be.eq(tokenURI);
    });

    step("it should emit an event", async function () {
        expect(Object.keys(mintingResult.events).length).to.be.eq(1);

        // data returned within the event
        expect(mintingResult.events.MintedWithExternalURI.returnValues._to).to.be.eq(to);
        expect(mintingResult.events.MintedWithExternalURI.returnValues._slot).to.be.eq(slot);
        expect(mintingResult.events.MintedWithExternalURI.returnValues._tokenURI).to.be.eq(tokenURI);
        const tokenId = slotAndOwnerToTokenId(slot, to);
        const tokenIdDecimal = new BN(tokenId, 16, "be").toString(10);
        expect(mintingResult.events.MintedWithExternalURI.returnValues._tokenId).to.be.eq(tokenIdDecimal);

        // event topics
        expect(mintingResult.events.MintedWithExternalURI.raw.topics.length).to.be.eq(2);
        expect(mintingResult.events.MintedWithExternalURI.raw.topics[0]).to.be.eq(SELECTOR_LOG_MINTED_WITH_EXTERNAL_TOKEN_URI);
        expect(mintingResult.events.MintedWithExternalURI.raw.topics[1]).to.be.eq(context.web3.utils.padLeft(FAITH.toLowerCase(), 64));

        // event data
        expect(mintingResult.events.MintedWithExternalURI.raw.data).to.be.eq(
            context.web3.eth.abi.encodeParameters(
                ["uint96", "uint256", "string"],
                [slot, tokenIdDecimal, tokenURI]
            )
        );
    });

    it("given slot and owner it should return token id", async function () {
        const slot = "1";
        const to = FAITH;

        const tokenId = slotAndOwnerToTokenId(slot, to);
        expect(tokenId).to.be.eq("000000000000000000000001c0f0f4ab324c46e55d02d0033343b4be8a55532d");
        const tokenIdDecimal = new BN(tokenId, 16, "be").toString(10);
        expect(tokenIdDecimal).to.be.eq("2563001357829637001682277476112176020532353127213");
    });

    step("@qa after changing owner I can't disable", async function () {
        await collectionContract.methods.transferOwnership("0xf24FF3a9CF04c71Dbc94D0b566f7A27B94566cac").send({ from: FAITH, gas: GAS_LIMIT });
        try {
            await collectionContract.methods.disablePublicMinting().send({ from: FAITH, gas: GAS_LIMIT });
            expect.fail("Expected error was not thrown"); // Ensure an error is thrown
        } catch (error) {
            // console.log(error.message);
        }
    });
});

describeWithExistingNode("Frontier RPC (Evolve Assets)", (context) => {
    let collectionContract: Contract
    let evolvingResult;

    before(async function () {
        collectionContract = await createCollection(context);
        const mintingResult = await collectionContract.methods.mintWithExternalURI(to, slot, tokenURI).send({ from: FAITH, gas: GAS_LIMIT });
        expect(mintingResult.status).to.be.eq(true);
    });


    const slot = "22";
    const to = FAITH;
    const tokenURI = "https://example.com";
    const newTokenURI = "https://new_example.com";

    step("when asset is evolved it should change token uri", async function () {
        const tokenId = slotAndOwnerToTokenId(slot, to);
        const tokenIdDecimal = new BN(tokenId, 16, "be").toString(10);
        evolvingResult = await collectionContract.methods.evolveWithExternalURI(tokenIdDecimal, newTokenURI).send({ from: FAITH, gas: GAS_LIMIT });
        expect(evolvingResult.status).to.be.eq(true);

        const got = await collectionContract.methods.tokenURI(tokenIdDecimal).call();
        expect(got).to.be.eq(newTokenURI);
    });

    step("it should emit an event", async function () {
        expect(Object.keys(evolvingResult.events).length).to.be.eq(1);

        // data returned within the event
        const tokenId = slotAndOwnerToTokenId(slot, to);
        const tokenIdDecimal = new BN(tokenId, 16, "be").toString(10);
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

describeWithExistingNode("Frontier RPC (Transfer Ownership)", (context) => {
    let collectionContract: Contract
    let tranferringResult;

    before(async function () {
        collectionContract = await createCollection(context);
    });

    const newOwner = "0xf24FF3a9CF04c71Dbc94D0b566f7A27B94566cac";

    step("@qa when collection is transferred it should return ok owner should change and emit an event", async function () {
        expect(await collectionContract.methods.owner().call()).to.be.eq(FAITH);
        const tranferringResult = await collectionContract.methods.transferOwnership(newOwner).send({ from: FAITH, gas: GAS_LIMIT });
        expect(tranferringResult.status).to.be.eq(true);
    });

    step("@qa it changes ownership", async function () {
        expect(await collectionContract.methods.owner().call()).to.be.eq(newOwner);
    });
    
    step("@qa it emits an event", async function () {
        expect(Object.keys(tranferringResult.events).length).to.be.eq(1);
        
        // data returned within the event
        expect(tranferringResult.events.OwnershipTransferred.returnValues._previousOwner).to.be.eq(FAITH);
        expect(tranferringResult.events.OwnershipTransferred.returnValues._newOwner).to.be.eq(newOwner);
        
        // event topics
        expect(tranferringResult.events.OwnershipTransferred.raw.topics.length).to.be.eq(3);
        expect(tranferringResult.events.OwnershipTransferred.raw.topics[0]).to.be.eq(SELECTOR_LOG_OWNERSHIP_TRANSFERRED);
        expect(tranferringResult.events.OwnershipTransferred.raw.topics[1]).to.be.eq(context.web3.utils.padLeft(FAITH.toLowerCase(), 64));
        expect(tranferringResult.events.OwnershipTransferred.raw.topics[2]).to.be.eq(context.web3.utils.padLeft(newOwner.toLowerCase(), 64));
        // event data
        expect(tranferringResult.events.OwnershipTransferred.raw.data).to.be.eq('0x');
        
        try { // TODO here check this error
            await collectionContract.methods.transferOwnership(FAITH).send({ from: FAITH, gas: GAS_LIMIT });
            expect.fail("Expected error was not thrown"); // Ensure an error is thrown
        } catch (error) {
            // console.log(error.message);
        }
    });
});

describeWithExistingNode("Frontier RPC (Public Minting)", (context) => {
    let collectionContract: Contract
    let disablingPublicMintingResult;
    let enablingPublicMintingResult;

    before(async function () {
        collectionContract = await createCollection(context);
    });

    step("@qa public minting is disabled by default", async function () {
        expect(await collectionContract.methods.isPublicMintingEnabled().call()).to.be.eq(false);
    });

    step("@qa disable twice has no effect", async function () {
        disablingPublicMintingResult = await collectionContract.methods.disablePublicMinting().send({ from: FAITH, gas: GAS_LIMIT });
        expect(await collectionContract.methods.isPublicMintingEnabled().call()).to.be.eq(false);
    });

    step("@qa it emits event", async function () {
        disablingPublicMintingResult = await collectionContract.methods.disablePublicMinting().send({ from: FAITH, gas: GAS_LIMIT });
        expect(disablingPublicMintingResult.status).to.be.eq(true);
        expect(await collectionContract.methods.isPublicMintingEnabled().call()).to.be.eq(false);
        expect(Object.keys(disablingPublicMintingResult.events).length).to.be.eq(1);
        expect(disablingPublicMintingResult.events.PublicMintingDisabled.raw.topics.length).to.be.eq(1);
        expect(disablingPublicMintingResult.events.PublicMintingDisabled.raw.topics[0]).to.be.eq(SELECTOR_LOG_PUBLIC_MINTING_DISABLED);
        expect(disablingPublicMintingResult.events.PublicMintingDisabled.raw.data).to.be.eq('0x');
    });

    step("@qa enable public minting returns ok", async function () {
        enablingPublicMintingResult = await collectionContract.methods.enablePublicMinting().send({ from: FAITH, gas: GAS_LIMIT });
        expect(enablingPublicMintingResult.status).to.be.eq(true);
        expect(await collectionContract.methods.isPublicMintingEnabled().call()).to.be.eq(true);

        // enable twice has no effect
        await collectionContract.methods.enablePublicMinting().send({ from: FAITH, gas: GAS_LIMIT });
        expect(await collectionContract.methods.isPublicMintingEnabled().call()).to.be.eq(true);

        // disable
        const disablingPublicMintingResult = await collectionContract.methods.disablePublicMinting().send({ from: FAITH, gas: GAS_LIMIT });
        expect(disablingPublicMintingResult.status).to.be.eq(true);
        expect(await collectionContract.methods.isPublicMintingEnabled().call()).to.be.eq(false);
        expect(Object.keys(disablingPublicMintingResult.events).length).to.be.eq(1);
        expect(disablingPublicMintingResult.events.PublicMintingDisabled.raw.topics.length).to.be.eq(1);
        expect(disablingPublicMintingResult.events.PublicMintingDisabled.raw.topics[0]).to.be.eq(SELECTOR_LOG_PUBLIC_MINTING_DISABLED);
        expect(disablingPublicMintingResult.events.PublicMintingDisabled.raw.data).to.be.eq('0x');
    });

    step("@qa it emits event", async function () {
        expect(Object.keys(enablingPublicMintingResult.events).length).to.be.eq(1);
        expect(enablingPublicMintingResult.events.PublicMintingEnabled.raw.topics.length).to.be.eq(1);
        expect(enablingPublicMintingResult.events.PublicMintingEnabled.raw.topics[0]).to.be.eq(SELECTOR_LOG_PUBLIC_MINTING_ENABLED);
        expect(enablingPublicMintingResult.events.PublicMintingEnabled.raw.data).to.be.eq('0x');
    });

    step("@qa enable twice has no effect", async function () {
        await collectionContract.methods.enablePublicMinting().send({ from: FAITH, gas: GAS_LIMIT });
        expect(await collectionContract.methods.isPublicMintingEnabled().call()).to.be.eq(true);
    });

    step("@qa I can mint even I'm not the owner", async function () {
        const owner = await collectionContract.methods.owner().call();
        expect(owner).to.be.not.eq(ALITH);

        let nonce = await context.web3.eth.getTransactionCount(ALITH);
        const mintingResult = await collectionContract.methods.mintWithExternalURI(ALITH, "123", "some/random/token/uri").send({ from: ALITH, gas: GAS_LIMIT, nonce: nonce++ });
        expect(mintingResult.status).to.be.eq(true);
    });

    step("@qa after enabling I can disable again", async function () {
        const disablingPublicMintingResult = await collectionContract.methods.disablePublicMinting().send({ from: FAITH, gas: GAS_LIMIT });
        expect(disablingPublicMintingResult.status).to.be.eq(true);
        expect(await collectionContract.methods.isPublicMintingEnabled().call()).to.be.eq(false);
        expect(Object.keys(disablingPublicMintingResult.events).length).to.be.eq(1);
        expect(disablingPublicMintingResult.events.PublicMintingDisabled.raw.topics.length).to.be.eq(1);
        expect(disablingPublicMintingResult.events.PublicMintingDisabled.raw.topics[0]).to.be.eq(SELECTOR_LOG_PUBLIC_MINTING_DISABLED);
        expect(disablingPublicMintingResult.events.PublicMintingDisabled.raw.data).to.be.eq('0x');
    });
});