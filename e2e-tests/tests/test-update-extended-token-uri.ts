import { expect } from "chai";
import { step } from "mocha-steps";
import Contract from "web3-eth-contract";
import {
	ASSET_METADATA_EXTENDER_ADDRESS,
	ASSET_METADATA_EXTENDER_ABI,
	GAS_PRICE,
	SELECTOR_LOG_UPDATED_EXTENDED_UL_WITH_EXTERNAL_URI,
	SELECTOR_LOG_EXTENDED_UL_WITH_EXTERNAL_URI,
} from "./config";
import { describeWithExistingNode } from "./util";

describeWithExistingNode("Frontier RPC (Extend Token URI)", function () {
	let contract: Contract;

	before(async function () {
		contract = new this.context.web3.eth.Contract(ASSET_METADATA_EXTENDER_ABI, ASSET_METADATA_EXTENDER_ADDRESS, {
			from: this.ethereumPairs.faith.address,
			gasPrice: GAS_PRICE,
		});
	});

	let uloc = `universal/location/1/${Date.now()}`;
	let extendResult: any;
	let tokenURI = "https://example.com";

	step("by default token uri is empty", async function () {
		expect(await contract.methods.balanceOfUL(uloc).call()).to.be.eq("0");
		expect(await contract.methods.hasExtensionByClaimer(uloc, this.ethereumPairs.faith.address).call()).to.be.eq(
			false
		);
	});

	step("extend should return ok", async function () {
		let nonce = await this.context.web3.eth.getTransactionCount(this.ethereumPairs.faith.address);
		const estimatedGas = await contract.methods.extendULWithExternalURI(uloc, tokenURI).estimateGas();
		extendResult = await contract.methods.extendULWithExternalURI(uloc, tokenURI).send({
			from: this.ethereumPairs.faith.address,
			gas: estimatedGas,
			gasPrice: GAS_PRICE,
			nonce: nonce++,
		});
		expect(extendResult.status).to.be.eq(true);
	});

	step("it creates an extension that I can query", async function () {
		expect(await contract.methods.extensionOfULByIndex(uloc, 0).call()).to.be.eq(tokenURI);
		expect(await contract.methods.extensionOfULByClaimer(uloc, this.ethereumPairs.faith.address).call()).to.be.eq(
			tokenURI
		);
		expect(await contract.methods.claimerOfULByIndex(uloc, 0).call()).to.be.eq(this.ethereumPairs.faith.address);
		expect(await contract.methods.balanceOfUL(uloc).call()).to.be.eq("1");
		expect(await contract.methods.hasExtensionByClaimer(uloc, this.ethereumPairs.faith.address).call()).to.be.eq(
			true
		);
	});

	step("it emits an event", async function () {
		// data returned within the event
		expect(Object.keys(extendResult.events).length).to.be.eq(1);
		expect(extendResult.events.ExtendedULWithExternalURI.returnValues._claimer).to.be.eq(
			this.ethereumPairs.faith.address
		);
		expect(extendResult.events.ExtendedULWithExternalURI.returnValues._universalLocationHash).to.be.eq(
			this.context.web3.utils.soliditySha3(uloc)
		);
		expect(extendResult.events.ExtendedULWithExternalURI.returnValues._universalLocation).to.be.eq(uloc);
		expect(extendResult.events.ExtendedULWithExternalURI.returnValues._tokenURI).to.be.eq(tokenURI);

		// event topics
		expect(extendResult.events.ExtendedULWithExternalURI.raw.topics.length).to.be.eq(3);
		expect(extendResult.events.ExtendedULWithExternalURI.raw.topics[0]).to.be.eq(
			SELECTOR_LOG_EXTENDED_UL_WITH_EXTERNAL_URI
		);
		expect(extendResult.events.ExtendedULWithExternalURI.raw.topics[1]).to.be.eq(
			this.context.web3.utils.padLeft(this.ethereumPairs.faith.address.toLowerCase(), 64)
		);
		expect(extendResult.events.ExtendedULWithExternalURI.raw.topics[2]).to.be.eq(
			this.context.web3.utils.padLeft(this.context.web3.utils.soliditySha3(uloc), 64)
		);

		// event data
		expect(extendResult.events.ExtendedULWithExternalURI.raw.data).to.be.eq(
			this.context.web3.eth.abi.encodeParameters(["string", "string"], [uloc, tokenURI])
		);
	});
});

describeWithExistingNode("Frontier RPC (Update Extended Token URI)", async function () {
	let contract: Contract;

	let uloc = `universal/location/2/${Date.now()}`;
	let tokenURI = "https://example2.com";
	let newTokenURI = "https://new.example.com";
	let updateExtensionResult: any;

	before(async function () {
		contract = new this.context.web3.eth.Contract(ASSET_METADATA_EXTENDER_ABI, ASSET_METADATA_EXTENDER_ADDRESS, {
			from: this.ethereumPairs.faith.address,
			gasPrice: GAS_PRICE,
		});

		// we first create an extension to be updated later
		let nonce = await this.context.web3.eth.getTransactionCount(this.ethereumPairs.faith.address);
		const estimatedGas = await contract.methods.extendULWithExternalURI(uloc, tokenURI).estimateGas();
		const createResult = await contract.methods.extendULWithExternalURI(uloc, tokenURI).send({
			from: this.ethereumPairs.faith.address,
			gas: estimatedGas,
			gasPrice: GAS_PRICE,
			nonce: nonce++,
		});
		expect(createResult.status).to.be.eq(true);
	});

	step("check existing extension", async function () {
		expect(await contract.methods.extensionOfULByIndex(uloc, 0).call()).to.be.eq(tokenURI);
		expect(await contract.methods.extensionOfULByClaimer(uloc, this.ethereumPairs.faith.address).call()).to.be.eq(
			tokenURI
		);
		expect(await contract.methods.claimerOfULByIndex(uloc, 0).call()).to.be.eq(this.ethereumPairs.faith.address);
		expect(await contract.methods.balanceOfUL(uloc).call()).to.be.eq("1");
		expect(await contract.methods.hasExtensionByClaimer(uloc, this.ethereumPairs.faith.address).call()).to.be.eq(
			true
		);
	});

	step("update extension should return ok", async function () {
		let nonce = await this.context.web3.eth.getTransactionCount(this.ethereumPairs.faith.address);
		const estimatedGas = await contract.methods.updateExtendedULWithExternalURI(uloc, newTokenURI).estimateGas();
		updateExtensionResult = await contract.methods.updateExtendedULWithExternalURI(uloc, newTokenURI).send({
			from: this.ethereumPairs.faith.address,
			gas: estimatedGas,
			gasPrice: GAS_PRICE,
			nonce: nonce++,
		});
		expect(updateExtensionResult.status).to.be.eq(true);
	});

	step("it updates just the extension data", async function () {
		expect(await contract.methods.extensionOfULByIndex(uloc, 0).call()).to.be.eq(newTokenURI);
		expect(await contract.methods.extensionOfULByClaimer(uloc, this.ethereumPairs.faith.address).call()).to.be.eq(
			newTokenURI
		);
		// the following might be the same as before updating
		expect(await contract.methods.claimerOfULByIndex(uloc, 0).call()).to.be.eq(this.ethereumPairs.faith.address);
		expect(await contract.methods.balanceOfUL(uloc).call()).to.be.eq("1");
		expect(await contract.methods.hasExtensionByClaimer(uloc, this.ethereumPairs.faith.address).call()).to.be.eq(
			true
		);
	});

	step("it emits an event", async function () {
		// data returned within the event
		expect(Object.keys(updateExtensionResult.events).length).to.be.eq(1);
		expect(updateExtensionResult.events.UpdatedExtendedULWithExternalURI.returnValues._claimer).to.be.eq(
			this.ethereumPairs.faith.address
		);
		expect(
			updateExtensionResult.events.UpdatedExtendedULWithExternalURI.returnValues._universalLocationHash
		).to.be.eq(this.context.web3.utils.soliditySha3(uloc));
		expect(updateExtensionResult.events.UpdatedExtendedULWithExternalURI.returnValues._universalLocation).to.be.eq(
			uloc
		);
		expect(updateExtensionResult.events.UpdatedExtendedULWithExternalURI.returnValues._tokenURI).to.be.eq(
			newTokenURI
		);

		// event topics
		expect(updateExtensionResult.events.UpdatedExtendedULWithExternalURI.raw.topics.length).to.be.eq(3);
		expect(updateExtensionResult.events.UpdatedExtendedULWithExternalURI.raw.topics[0]).to.be.eq(
			SELECTOR_LOG_UPDATED_EXTENDED_UL_WITH_EXTERNAL_URI
		);
		expect(updateExtensionResult.events.UpdatedExtendedULWithExternalURI.raw.topics[1]).to.be.eq(
			this.context.web3.utils.padLeft(this.ethereumPairs.faith.address.toLowerCase(), 64)
		);
		expect(updateExtensionResult.events.UpdatedExtendedULWithExternalURI.raw.topics[2]).to.be.eq(
			this.context.web3.utils.padLeft(this.context.web3.utils.soliditySha3(uloc), 64)
		);

		// event data
		expect(updateExtensionResult.events.UpdatedExtendedULWithExternalURI.raw.data).to.be.eq(
			this.context.web3.eth.abi.encodeParameters(["string", "string"], [uloc, newTokenURI])
		);
	});
});
