import BN from "bn.js";
import { assert, expect } from "chai";
import { step } from "mocha-steps";

import { CHAIN_ID, RUNTIME_IMPL_VERSION, RUNTIME_SPEC_NAME, RUNTIME_SPEC_VERSION } from "./config";
import { customRequest, describeWithExistingNode } from "./util";

describeWithExistingNode("Frontier RPC (Web3Api)", function () {
	step("should get client version", async function () {
		const version = await this.context.web3.eth.getNodeInfo();
		assert(
			version.includes(`${RUNTIME_SPEC_NAME}/v${RUNTIME_SPEC_VERSION}.${RUNTIME_IMPL_VERSION}/fc-rpc-2.0.0-dev`)
		);
	});

	step("should remote sha3", async function () {
		const data = this.context.web3.utils.stringToHex("hello");
		const hash = await customRequest(this.context.web3, "web3_sha3", [data]);
		const localHash = this.context.web3.utils.sha3("hello");
		expect(hash.result).to.be.equal(localHash);
	});

	step("should get chain id", async function () {
		const chainId = await this.context.web3.eth.getChainId();
		expect(chainId).to.be.equal(CHAIN_ID);
	});

	step("genesis balance is setup correctly", async function () {
		const balance = new BN(await this.context.web3.eth.getBalance(this.ethereumPairs.faith.address));
		expect(balance.gt(new BN(0))).to.be.eq(true);
	});
});
