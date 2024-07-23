import {
    addressToCollectionId,
    createCollection,
    describeWithExistingNode,
    extractRevertReason,
    slotAndOwnerToTokenId,
} from "./util";
import {
    GAS_LIMIT,
    FAITH,
    SELECTOR_LOG_EVOLVED_WITH_EXTERNAL_TOKEN_URI,
    SELECTOR_LOG_MINTED_WITH_EXTERNAL_TOKEN_URI,
    SELECTOR_LOG_OWNERSHIP_TRANSFERRED,
    SELECTOR_LOG_PUBLIC_MINTING_ENABLED,
    SELECTOR_LOG_PUBLIC_MINTING_DISABLED,
    ALITH,
    ALITH_PRIVATE_KEY,
    STAKING_ABI,
    STAKING_CONTRACT_ADDRESS,
    GAS_PRICE,
    UNIT,
} from "./config";
import { expect } from "chai";
import Contract from "web3-eth-contract";
import BN from "bn.js";
import { step } from "mocha-steps";
import { Keyring } from "@polkadot/api";

describeWithExistingNode("Frontier RPC (Staking)", (context) => {
    let contract: Contract;

    before(async function () {
        contract = new context.web3.eth.Contract(STAKING_ABI, STAKING_CONTRACT_ADDRESS, {
            from: ALITH,
            gasPrice: GAS_PRICE,
            gas: GAS_LIMIT,
        });
        context.web3.eth.accounts.wallet.add(ALITH_PRIVATE_KEY);
    });
    step("Alith can join as candidate", async function () {
        const alith = new Keyring({ type: "ethereum" }).addFromUri(ALITH_PRIVATE_KEY);
        const key = (await context.polkadot.rpc.author.rotateKeys()).toHex();
        context.polkadot.tx.session.setKeys(key, "").signAndSend(alith, (result) => {
            console.log(`transaction result: ${result}`);
        }).catch((error: any) => {
            console.log('transaction failed', error);
        });

        let nonce = await context.web3.eth.getTransactionCount(ALITH);
        const result = await contract.methods.joinCandidates(BigInt(20000) * UNIT, 0).send({ from: ALITH, gas: GAS_LIMIT, nonce: nonce++ });
        console.log(`result: ${result}`);
    });

});

