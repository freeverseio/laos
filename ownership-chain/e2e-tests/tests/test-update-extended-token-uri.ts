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
	REVERT_BYTECODE,
	SELECTOR_LOG_EXTENDED_TOKEN_URI_UPDATED,
} from "./config";
import { createCollection, describeWithExistingNode } from "./util";

describeWithExistingNode("Frontier RPC (Update Extended Token URI)", (context) => {
	let contract: Contract;
	// This is the contract that is created in the test
	let testCollectionContract: Contract;
	// This is the address of another contract that is created in the test
	let testCollectionAddress: string;

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

		const creation_result = await contract.methods.extendTokenURI(uloc, tokenURI).send({
			from: GENESIS_ACCOUNT,
			gas: GAS_LIMIT,
			gasPrice: GAS_PRICE,
			nonce: nonce++,
		});
		expect(creation_result.status).to.be.eq(true);
		
		const udpateResult = await contract.methods.updateExtendedTokenURI(uloc, newTokenURI).send({
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
		expect(udpateResult.events.ExtendedTokenURIUpdated.returnValues._claimer).to.be.eq(GENESIS_ACCOUNT);
		expect(udpateResult.events.ExtendedTokenURIUpdated.returnValues._universelLocationHash).to.be.eq("");
		expect(udpateResult.events.ExtendedTokenURIUpdated.returnValues._universalLocation).to.be.eq("");
		expect(udpateResult.events.ExtendedTokenURIUpdated.returnValues._tokenURI).to.be.eq("");

		// event topics
		expect(udpateResult.events.ExtendedTokenURIUpdated.raw.topics.length).to.be.eq(3);
		expect(udpateResult.events.ExtendedTokenURIUpdated.raw.topics[0]).to.be.eq(SELECTOR_LOG_EXTENDED_TOKEN_URI_UPDATED);
		expect(udpateResult.events.ExtendedTokenURIUpdated.raw.topics[1]).to.be.eq(
			context.web3.utils.padLeft(GENESIS_ACCOUNT.toLowerCase(), 64)
		);
		expect(udpateResult.events.ExtendedTokenURIUpdated.raw.topics[2]).to.be.eq(
			context.web3.utils.padLeft(GENESIS_ACCOUNT.toLowerCase(), 64)
		); // TODO universal location hash

		// event data
		expect(udpateResult.events.MintedWithExternalURI.raw.data).to.be.eq(
			context.web3.eth.abi.encodeParameters(
				["string", "string"],
				[uloc, newTokenURI]
			)
		);
	});

	step("when token uri extended is updated event is emitted", async function () {
		this.timeout(700000);

		let nonce = await context.web3.eth.getTransactionCount(GENESIS_ACCOUNT);
		context.web3.eth.accounts.wallet.add(GENESIS_ACCOUNT_PRIVATE_KEY);

		let uloc = "universal/location";
		let tokenURI = "https://example.com";
		let newTokenURI = "https://new.example.com";

		const creation_result = await contract.methods.extendTokenURI(uloc, tokenURI).send({
			from: GENESIS_ACCOUNT,
			gas: GAS_LIMIT,
			gasPrice: GAS_PRICE,
			nonce: nonce++,
		});
		expect(creation_result.status).to.be.eq(true);

		const udpateResult = await contract.methods.updateExtendedTokenURI(uloc, newTokenURI).send({
			from: GENESIS_ACCOUNT,
			gas: GAS_LIMIT,
			gasPrice: GAS_PRICE,
			nonce: nonce++,
		});
		expect(udpateResult.status).to.be.eq(true);

		expect(Object.keys(udpateResult.events).length).to.be.eq(1);

		// data returned within the event
		expect(udpateResult.events.ExtendedTokenURIUpdated.returnValues._claimer).to.be.eq(GENESIS_ACCOUNT);
		expect(udpateResult.events.ExtendedTokenURIUpdated.returnValues._universelLocationHash).to.be.eq("");
		expect(udpateResult.events.ExtendedTokenURIUpdated.returnValues._universalLocation).to.be.eq("");
		expect(udpateResult.events.ExtendedTokenURIUpdated.returnValues._tokenURI).to.be.eq("");

		// event topics
		expect(udpateResult.events.ExtendedTokenURIUpdated.raw.topics.length).to.be.eq(3);
		expect(udpateResult.events.ExtendedTokenURIUpdated.raw.topics[0]).to.be.eq(SELECTOR_LOG_EXTENDED_TOKEN_URI_UPDATED);
		expect(udpateResult.events.ExtendedTokenURIUpdated.raw.topics[1]).to.be.eq(
			context.web3.utils.padLeft(GENESIS_ACCOUNT.toLowerCase(), 64)
		);
		expect(udpateResult.events.ExtendedTokenURIUpdated.raw.topics[2]).to.be.eq(
			context.web3.utils.padLeft(GENESIS_ACCOUNT.toLowerCase(), 64)
		); // TODO universal location hash

		// event data
		expect(udpateResult.events.MintedWithExternalURI.raw.data).to.be.eq(
			context.web3.eth.abi.encodeParameters(
				["string", "string"],
				[uloc, newTokenURI]
			)
		);
	});
});
