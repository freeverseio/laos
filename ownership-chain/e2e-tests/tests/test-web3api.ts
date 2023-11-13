import { expect } from "chai";
import { step } from "mocha-steps";
import BN from "bn.js";

import { RUNTIME_SPEC_NAME, RUNTIME_SPEC_VERSION, RUNTIME_IMPL_VERSION, CHAIN_ID, GENESIS_ACCOUNT, GENESIS_ACCOUNT_PRIVATE_KEY, GAS_PRICE } from "./config";
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
        const balance = new BN(await context.web3.eth.getBalance(GENESIS_ACCOUNT));
        expect(balance.gt(new BN(0))).to.be.eq(true);
    });

    step("send 1 wei to wallet with 0 balance should increase balance by 1 wei", async function () {
        this.timeout(70000);

        const wallet = context.web3.eth.accounts.create();
        const balanceBefore = new BN(await context.web3.eth.getBalance(wallet.address));
        // check the balance is 0
        expect(balanceBefore.eq(new BN(0))).to.be.eq(true);
        
        context.web3.eth.accounts.wallet.add(GENESIS_ACCOUNT_PRIVATE_KEY);
        // send 1 wei to the wallet
        await context.web3.eth.sendTransaction({
            from: GENESIS_ACCOUNT,
            to: wallet.address,
            value: 1000000001,
            gas: 21000,
        });

        const balanceAfter = new BN(await context.web3.eth.getBalance(wallet.address));
        // check the balance is 1
        expect(balanceAfter.toString()).to.be.eq("1");
    });
});
