import { expect } from "chai";
import { step } from "mocha-steps";
import Contract from "web3-eth-contract";
import {
	CONTRACT_ADDRESS,
	VESTING_ABI,
	GAS_LIMIT,
	GAS_PRICE,
	FAITH,
	FAITH_PRIVATE_KEY,
} from "./config";
import { describeWithExistingNode } from "./util";

describeWithExistingNode("Frontier RPC (Create Collection)", (context) => {
	let contract: Contract;

	before(async function () {
		contract = new context.web3.eth.Contract(VESTING_ABI, CONTRACT_ADDRESS, {
			from: FAITH,
			gasPrice: GAS_PRICE,
			gas: GAS_LIMIT,
		});
		context.web3.eth.accounts.wallet.add(FAITH_PRIVATE_KEY);
	});

	step("when collection is created, it should return owner", async function () {});
});
