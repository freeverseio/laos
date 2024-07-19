import { expect } from "chai";
import { step } from "mocha-steps";
import Contract from "web3-eth-contract";
import {
    PARACHAIN_STAKING_CONTRACT_ADDRESS,
    GAS_LIMIT,
    GAS_PRICE,
    FAITH,
    FAITH_PRIVATE_KEY,
    PARACHAIN_STAKING_ABI,
} from "./config";
import { describeWithExistingNode } from "./util";

describeWithExistingNode("Frontier RPC (Staking Genesis)", (context) => {
    let contract: Contract;
    // This is the contract that is created in the test
    let testCollectionContract: Contract;
    // This is the address of another contract that is created in the test
    let testCollectionAddress: string;

    before(async function () {
        contract = new context.web3.eth.Contract(PARACHAIN_STAKING_ABI, PARACHAIN_STAKING_CONTRACT_ADDRESS, {
            from: FAITH,
            gasPrice: GAS_PRICE,
            gas: GAS_LIMIT,
        });
        context.web3.eth.accounts.wallet.add(FAITH_PRIVATE_KEY);
    });

    it("TODO", async function () {
        const result = contract.methods.
    });
});
