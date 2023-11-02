import { expect } from "chai";
import { step } from "mocha-steps";

import { RUNTIME_SPEC_NAME, RUNTIME_SPEC_VERSION, RUNTIME_IMPL_VERSION, CHAIN_ID, GENESIS_ACCOUNT, GENESIS_ACCOUNT_BALANCE } from "./config";
import { describeWithExistingNode, customRequest } from "./util";

describeWithExistingNode("Frontier RPC (Web3Api)", (context) => {
    step("should get client version", async function () {
        const version = await context.web3.eth.getNodeInfo();
        expect(version).to.be.equal(
            `${RUNTIME_SPEC_NAME}/v${RUNTIME_SPEC_VERSION}.${RUNTIME_IMPL_VERSION}/fc-rpc-2.0.0-dev`
        );
    });

    step("should remote sha3", async function () {
        const data = context.web3.utils.stringToHex("hello");
        const hash = await customRequest(context.web3, "web3_sha3", [data]);
        const localHash = context.web3.utils.sha3("hello");
        expect(hash.result).to.be.equal(localHash);
    });

    step("should get chain id", async function () {
        const chainId = await context.web3.eth.getChainId();
        expect(chainId).to.be.equal(CHAIN_ID);
    });

    step("genesis balance is setup correctly", async function () {
        expect(await context.web3.eth.getBalance(GENESIS_ACCOUNT)).to.equal(GENESIS_ACCOUNT_BALANCE);
    });
});
