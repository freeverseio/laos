import { expect } from "chai";
import { step } from "mocha-steps";
import Contract from "web3-eth-contract";
import {
	ASSET_METADATA_EXTENDER_ADDRESS,
	ASSET_METADATA_EXTENDER_ABI,
	GAS_LIMIT,
	GAS_PRICE,
	GENESIS_ACCOUNT,
	GENESIS_ACCOUNT_PRIVATE_KEY,
	SELECTOR_LOG_EXTENDED_TOKEN_URI_UPDATED,
} from "./config";
import { describeWithExistingNode } from "./util";

describeWithExistingNode("Frontier RPC (Update Extended Token URI)", (context) => {
	let contract: Contract;

	beforeEach(async function () {
		contract = new context.web3.eth.Contract(ASSET_METADATA_EXTENDER_ABI, ASSET_METADATA_EXTENDER_ADDRESS, {
			from: GENESIS_ACCOUNT,
			gasPrice: GAS_PRICE,
			gas: GAS_LIMIT,
		});
		context.web3.eth.accounts.wallet.add(GENESIS_ACCOUNT_PRIVATE_KEY);
	});

	step("when token uri extended is updated it should change", async function () {
		this.timeout(700000);

		let nonce = await context.web3.eth.getTransactionCount(GENESIS_ACCOUNT);
		context.web3.eth.accounts.wallet.add(GENESIS_ACCOUNT_PRIVATE_KEY);

		let uloc = "universal/location";
		let tokenURI = "https://example.com";
		let newTokenURI = "https://new.example.com";

		const createResult = await contract.methods.extendULWithExternalURI(uloc, tokenURI).send({
			from: GENESIS_ACCOUNT,
			gas: GAS_LIMIT,
			gasPrice: GAS_PRICE,
			nonce: nonce++,
		});
		expect(createResult.status).to.be.eq(true);
		
		const udpateResult = await contract.methods.updateExtendedULWithExternalURI(uloc, newTokenURI).send({
			from: GENESIS_ACCOUNT,
			gas: GAS_LIMIT,
			gasPrice: GAS_PRICE,
			nonce: nonce++,
		});
		expect(udpateResult.status).to.be.eq(true);

		const got = await contract.methods.extensionOfULByIndex(uloc, 0).call();
		expect(got).to.be.eq(newTokenURI);
		expect(udpateResult.status).to.be.eq(true);
		expect(Object.keys(udpateResult.events).length).to.be.eq(1);

		// data returned within the event
		expect(udpateResult.events.UpdatedExtendedULWithExternalURI.returnValues._claimer).to.be.eq(GENESIS_ACCOUNT);
		expect(udpateResult.events.UpdatedExtendedULWithExternalURI.returnValues._universalLocationHash).to.be.eq(context.web3.utils.soliditySha3(uloc));
		expect(udpateResult.events.UpdatedExtendedULWithExternalURI.returnValues._universalLocation).to.be.eq(uloc);
		expect(udpateResult.events.UpdatedExtendedULWithExternalURI.returnValues._tokenURI).to.be.eq(newTokenURI);

		// event topics
		expect(udpateResult.events.UpdatedExtendedULWithExternalURI.raw.topics.length).to.be.eq(3);
		expect(udpateResult.events.UpdatedExtendedULWithExternalURI.raw.topics[0]).to.be.eq(SELECTOR_LOG_EXTENDED_TOKEN_URI_UPDATED);
		expect(udpateResult.events.UpdatedExtendedULWithExternalURI.raw.topics[1]).to.be.eq(
			context.web3.utils.padLeft(GENESIS_ACCOUNT.toLowerCase(), 64)
		);
		expect(udpateResult.events.UpdatedExtendedULWithExternalURI.raw.topics[2]).to.be.eq(
			context.web3.utils.padLeft(context.web3.utils.soliditySha3(uloc), 64)
		);

		// event data
		expect(udpateResult.events.UpdatedExtendedULWithExternalURI.raw.data).to.be.eq(
			context.web3.eth.abi.encodeParameters(
				["string", "string"],
				[uloc, newTokenURI]
			)
		);
	});
});
