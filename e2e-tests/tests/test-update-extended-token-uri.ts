import { expect } from "chai";
import { step } from "mocha-steps";
import Contract from "web3-eth-contract";
import {
	ASSET_METADATA_EXTENDER_ADDRESS,
	ASSET_METADATA_EXTENDER_ABI,
	GAS_LIMIT,
	GAS_PRICE,
	FAITH,
	FAITH_PRIVATE_KEY,
	SELECTOR_LOG_UPDATED_EXTENDED_UL_WITH_EXTERNAL_URI,
	SELECTOR_LOG_EXTENDED_UL_WITH_EXTERNAL_URI,
} from "./config";
import { describeWithExistingNode } from "./util";

describeWithExistingNode("Frontier RPC (Update Extended Token URI)", (context) => {
	let contract: Contract;

	beforeEach(async function () {
		contract = new context.web3.eth.Contract(ASSET_METADATA_EXTENDER_ABI, ASSET_METADATA_EXTENDER_ADDRESS, {
			from: FAITH,
			gasPrice: GAS_PRICE,
			gas: GAS_LIMIT,
		});
		context.web3.eth.accounts.wallet.add(FAITH_PRIVATE_KEY);
	});

	step("when token uri extended is updated it should change", async function () {
		this.timeout(700000);

		let nonce = await context.web3.eth.getTransactionCount(FAITH);
		context.web3.eth.accounts.wallet.add(FAITH_PRIVATE_KEY);

		let uloc = "universal/location";
		let tokenURI = "https://example.com";
		let newTokenURI = "https://new.example.com";

		expect(await contract.methods.balanceOfUL(uloc).call()).to.be.eq("0");
		expect(await contract.methods.hasExtensionByClaimer(uloc, FAITH).call()).to.be.eq(false);
		
		const createResult = await contract.methods.extendULWithExternalURI(uloc, tokenURI).send({
			from: FAITH,
			gas: GAS_LIMIT,
			gasPrice: GAS_PRICE,
			nonce: nonce++,
		});
		expect(createResult.status).to.be.eq(true);

		expect(await contract.methods.extensionOfULByIndex(uloc, 0).call()).to.be.eq(tokenURI);
		expect(await contract.methods.extensionOfULByClaimer(uloc, FAITH).call()).to.be.eq(tokenURI);
		expect(await contract.methods.claimerOfULByIndex(uloc, 0).call()).to.be.eq(FAITH);
		expect(await contract.methods.balanceOfUL(uloc).call()).to.be.eq("1");
		expect(await contract.methods.hasExtensionByClaimer(uloc, FAITH).call()).to.be.eq(true);

		// data returned within the event
		expect(Object.keys(createResult.events).length).to.be.eq(1);
		expect(createResult.events.ExtendedULWithExternalURI.returnValues._claimer).to.be.eq(FAITH);
		expect(createResult.events.ExtendedULWithExternalURI.returnValues._universalLocationHash).to.be.eq(context.web3.utils.soliditySha3(uloc));
		expect(createResult.events.ExtendedULWithExternalURI.returnValues._universalLocation).to.be.eq(uloc);
		expect(createResult.events.ExtendedULWithExternalURI.returnValues._tokenURI).to.be.eq(tokenURI);

		// event topics
		expect(createResult.events.ExtendedULWithExternalURI.raw.topics.length).to.be.eq(3);
		expect(createResult.events.ExtendedULWithExternalURI.raw.topics[0]).to.be.eq(SELECTOR_LOG_EXTENDED_UL_WITH_EXTERNAL_URI);
		expect(createResult.events.ExtendedULWithExternalURI.raw.topics[1]).to.be.eq(
			context.web3.utils.padLeft(FAITH.toLowerCase(), 64)
		);
		expect(createResult.events.ExtendedULWithExternalURI.raw.topics[2]).to.be.eq(
			context.web3.utils.padLeft(context.web3.utils.soliditySha3(uloc), 64)
		);

		// event data
		expect(createResult.events.ExtendedULWithExternalURI.raw.data).to.be.eq(
			context.web3.eth.abi.encodeParameters(
				["string", "string"],
				[uloc, tokenURI]
			)
		);
		
		const udpateResult = await contract.methods.updateExtendedULWithExternalURI(uloc, newTokenURI).send({
			from: FAITH,
			gas: GAS_LIMIT,
			gasPrice: GAS_PRICE,
			nonce: nonce++,
		});
		expect(udpateResult.status).to.be.eq(true);
		
		// after update everything remains but new token uri
		expect(await contract.methods.extensionOfULByIndex(uloc, 0).call()).to.be.eq(newTokenURI);
		expect(await contract.methods.extensionOfULByClaimer(uloc, FAITH).call()).to.be.eq(newTokenURI);
		expect(await contract.methods.claimerOfULByIndex(uloc, 0).call()).to.be.eq(FAITH);
		expect(await contract.methods.balanceOfUL(uloc).call()).to.be.eq("1");
		expect(await contract.methods.hasExtensionByClaimer(uloc, FAITH).call()).to.be.eq(true);
		
		// data returned within the event
		expect(udpateResult.events.UpdatedExtendedULWithExternalURI.returnValues._claimer).to.be.eq(FAITH);
		expect(udpateResult.events.UpdatedExtendedULWithExternalURI.returnValues._universalLocationHash).to.be.eq(context.web3.utils.soliditySha3(uloc));
		expect(udpateResult.events.UpdatedExtendedULWithExternalURI.returnValues._universalLocation).to.be.eq(uloc);
		expect(udpateResult.events.UpdatedExtendedULWithExternalURI.returnValues._tokenURI).to.be.eq(newTokenURI);

		// event topics
		expect(udpateResult.events.UpdatedExtendedULWithExternalURI.raw.topics.length).to.be.eq(3);
		expect(udpateResult.events.UpdatedExtendedULWithExternalURI.raw.topics[0]).to.be.eq(SELECTOR_LOG_UPDATED_EXTENDED_UL_WITH_EXTERNAL_URI);
		expect(udpateResult.events.UpdatedExtendedULWithExternalURI.raw.topics[1]).to.be.eq(
			context.web3.utils.padLeft(FAITH.toLowerCase(), 64)
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
