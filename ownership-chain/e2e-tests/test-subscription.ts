import { expect } from "chai";
import { step } from "mocha-steps";
import { AbiItem } from "web3-utils";

import LaosEvolution from "../build/contracts/LaosEvolution.json";
import { GENESIS_ACCOUNT, GENESIS_ACCOUNT_PRIVATE_KEY, TEST_CONTRACT_BYTECODE } from "./config";
import { createAndFinalizeBlock, customRequest, describeWithFrontierWs } from "./util";

describeWithFrontierWs("Frontier RPC (Subscription)", (context) => {
	let subscription;
	let logsGenerated = 0;

	const NEW_COLLECTION_SELECTOR = "0x6eb24fd767a7bcfa417f3fe25a2cb245d2ae52293d3c4a8f8c6450a09795d289";
	const LAOS_EVOLUTION_ABI = LaosEvolution.abi as AbiItem[];
	const PRECOMPILE_ADDRESS = "0x0000000000000000000000000000000000000403";

	async function sendTransaction(context) {
		const tx = await context.web3.eth.accounts.signTransaction(
			{
				from: GENESIS_ACCOUNT,
				data: TEST_CONTRACT_BYTECODE,
				value: "0x00",
				gasPrice: "0x3B9ACA00",
				gas: "0x1000000",
			},
			GENESIS_ACCOUNT_PRIVATE_KEY
		);

		await customRequest(context.web3, "eth_sendRawTransaction", [tx.rawTransaction]);
		return tx;
	}

	step("should connect", async function () {
		await createAndFinalizeBlock(context.web3);
		// @ts-ignore
		const connected = context.web3.currentProvider.connected;
		expect(connected).to.equal(true);
	}).timeout(20000);

	step("should subscribe", async function () {
		subscription = context.web3.eth.subscribe("newBlockHeaders", function (error, result) {});

		let connected = false;
		let subscriptionId = "";
		expect(subscriptionId).is.empty;
		await new Promise<void>((resolve) => {
			subscription.on("connected", function (d: any) {
				connected = true;
				subscriptionId = d;
				resolve();
			});
		});

		subscription.unsubscribe();
		expect(connected).to.equal(true);
		expect(subscriptionId).not.empty;
	}).timeout(20000);

	step("should get newHeads stream", async function (done) {
		subscription = context.web3.eth.subscribe("newBlockHeaders", function (error, result) {});
		let data = null;
		let dataResolve = null;
		let dataPromise = new Promise((resolve) => {
			dataResolve = resolve;
		});
		subscription.on("data", function (d: any) {
			data = d;
			subscription.unsubscribe();
			dataResolve();
		});

		await createAndFinalizeBlock(context.web3);
		await dataPromise;

		expect(data).to.include({
			author: "0x0000000000000000000000000000000000000000",
			difficulty: "0",
			extraData: "0x",
			logsBloom:
				"0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
			miner: "0x0000000000000000000000000000000000000000",
			number: 2,
			receiptsRoot: "0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421",
			sha3Uncles: "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347",
			transactionsRoot: "0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421",
		});
		expect(data.nonce).to.eql("0x0000000000000000");

		done();
	}).timeout(50000);

	step("should get newPendingTransactions stream", async function (done) {
		subscription = context.web3.eth.subscribe("pendingTransactions", function (error, result) {});

		await new Promise<void>((resolve) => {
			subscription.on("connected", function (d: any) {
				resolve();
			});
		});

		const tx = await sendTransaction(context);
		let data = null;
		await new Promise<void>((resolve) => {
			subscription.on("data", function (d: any) {
				data = d;
				logsGenerated += 1;
				resolve();
			});
		});

		subscription.unsubscribe();
		expect(data).to.be.not.null;
		expect(tx["transactionHash"]).to.be.eq(data);

		done();
	}).timeout(20000);

	step("should subscribe to all logs", async function (done) {
		subscription = context.web3.eth.subscribe("logs", {}, function (error, result) {});

		await new Promise<void>((resolve) => {
			subscription.on("connected", function (d: any) {
				resolve();
			});
		});

		const tx = await sendTransaction(context);
		let data = null;
		let dataResolve = null;
		let dataPromise = new Promise((resolve) => {
			dataResolve = resolve;
		});
		subscription.on("data", function (d: any) {
			data = d;
			logsGenerated += 1;
			dataResolve();
		});

		await createAndFinalizeBlock(context.web3);
		await dataPromise;

		subscription.unsubscribe();
		const block = await context.web3.eth.getBlock("latest");
		expect(data).to.include({
			blockHash: block.hash,
			blockNumber: block.number,
			data: "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
			logIndex: 0,
			removed: false,
			transactionHash: block.transactions[0],
			transactionIndex: 0,
			transactionLogIndex: "0x0",
		});
		done();
	}).timeout(20000);

	step("should subscribe to logs by multiple addresses", async function (done) {
		subscription = context.web3.eth.subscribe(
			"logs",
			{
				address: [
					"0xF8cef78E923919054037a1D03662bBD884fF4edf",
					"0x42e2EE7Ba8975c473157634Ac2AF4098190fc741",
					"0x5c4242beB94dE30b922f57241f1D02f36e906915",
					"0xC2Bf5F29a4384b1aB0C063e1c666f02121B6084a",
				],
			},
			function (error, result) {
				console.log("error", error);
				console.log("res", result);
			}
		);

		await new Promise<void>((resolve) => {
			subscription.on("connected", function (d: any) {
				resolve();
			});
		});

		const tx = await sendTransaction(context);
		let data = null;
		let dataResolve = null;
		let dataPromise = new Promise((resolve) => {
			dataResolve = resolve;
		});
		subscription.on("data", function (d: any) {
			console.log("data", d);
			data = d;
			logsGenerated += 1;
			dataResolve();
		});

		await createAndFinalizeBlock(context.web3);
		await dataPromise;

		subscription.unsubscribe();
		expect(data).to.not.be.null;
		done();
	}).timeout(20000);

	step("should subscribe to logs by topic", async function (done) {
		subscription = context.web3.eth.subscribe(
			"logs",
			{
				topics: ["0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef"],
			},
			function (error, result) {}
		);

		await new Promise<void>((resolve) => {
			subscription.on("connected", function (d: any) {
				resolve();
			});
		});

		const tx = await sendTransaction(context);
		let data = null;
		let dataResolve = null;
		let dataPromise = new Promise((resolve) => {
			dataResolve = resolve;
		});

		subscription.on("data", function (d: any) {
			data = d;
			logsGenerated += 1;
			dataResolve();
		});

		await createAndFinalizeBlock(context.web3);
		await dataPromise;

		subscription.unsubscribe();
		expect(data).to.not.be.null;
		done();
	}).timeout(20000);

	step("should get past events #1: by topic", async function (done) {
		subscription = context.web3.eth.subscribe(
			"logs",
			{
				fromBlock: "0x0",
				topics: ["0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef"],
			},
			function (error, result) {}
		);

		let data = [];
		await new Promise<void>((resolve) => {
			subscription.on("data", function (d: any) {
				data.push(d);
				resolve();
			});
		});
		subscription.unsubscribe();

		expect(data).to.not.be.empty;
		done();
	}).timeout(20000);

	step("should get past events #2: by address", async function (done) {
		subscription = context.web3.eth.subscribe(
			"logs",
			{
				fromBlock: "0x0",
				address: ["0x42e2EE7Ba8975c473157634Ac2AF4098190fc741"],
			},
			function (error, result) {}
		);

		let data = [];
		await new Promise<void>((resolve) => {
			subscription.on("data", function (d: any) {
				data.push(d);
				resolve();
			});
		});
		subscription.unsubscribe();

		expect(data).to.not.be.empty;
		done();
	}).timeout(20000);

	step("should get past events #3: by address + topic", async function (done) {
		subscription = context.web3.eth.subscribe(
			"logs",
			{
				fromBlock: "0x0",
				topics: ["0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef"],
				address: ["0xC2Bf5F29a4384b1aB0C063e1c666f02121B6084a"],
			},
			function (error, result) {}
		);

		let data = [];
		await new Promise<void>((resolve) => {
			subscription.on("data", function (d: any) {
				data.push(d);
				resolve();
			});
		});
		subscription.unsubscribe();

		expect(data).to.not.be.empty;
		done();
	}).timeout(20000);

	step("should get past events #4: multiple addresses", async function (done) {
		subscription = context.web3.eth.subscribe(
			"logs",
			{
				fromBlock: "0x0",
				topics: ["0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef"],
				address: [
					"0xe573BCA813c741229ffB2488F7856C6cAa841041",
					"0xF8cef78E923919054037a1D03662bBD884fF4edf",
					"0x42e2EE7Ba8975c473157634Ac2AF4098190fc741",
					"0x5c4242beB94dE30b922f57241f1D02f36e906915",
					"0xC2Bf5F29a4384b1aB0C063e1c666f02121B6084a",
				],
			},
			function (error, result) {}
		);

		let data = [];
		await new Promise<void>((resolve) => {
			subscription.on("data", function (d: any) {
				data.push(d);
				resolve();
			});
		});
		subscription.unsubscribe();

		expect(data).to.not.be.empty;
		done();
	}).timeout(20000);

	step("should support topic wildcards", async function (done) {
		subscription = context.web3.eth.subscribe(
			"logs",
			{
				topics: [null, "0x0000000000000000000000000000000000000000000000000000000000000000"],
			},
			function (error, result) {}
		);

		await new Promise<void>((resolve) => {
			subscription.on("connected", function (d: any) {
				resolve();
			});
		});

		const tx = await sendTransaction(context);
		let data = null;
		let dataResolve = null;
		let dataPromise = new Promise((resolve) => {
			dataResolve = resolve;
		});

		subscription.on("data", function (d: any) {
			data = d;
			logsGenerated += 1;
			dataResolve();
		});

		await createAndFinalizeBlock(context.web3);
		await dataPromise;

		subscription.unsubscribe();
		expect(data).to.not.be.null;
		done();
	}).timeout(20000);

	step("should support single values wrapped around a sequence", async function (done) {
		subscription = context.web3.eth.subscribe(
			"logs",
			{
				topics: [
					["0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef"],
					["0x0000000000000000000000000000000000000000000000000000000000000000"],
				],
			},
			function (error, result) {}
		);

		await new Promise<void>((resolve) => {
			subscription.on("connected", function (d: any) {
				resolve();
			});
		});

		const tx = await sendTransaction(context);
		let data = null;
		let dataResolve = null;
		let dataPromise = new Promise((resolve) => {
			dataResolve = resolve;
		});

		subscription.on("data", function (d: any) {
			data = d;
			logsGenerated += 1;
			dataResolve();
		});

		await createAndFinalizeBlock(context.web3);
		await dataPromise;

		subscription.unsubscribe();
		expect(data).to.not.be.null;
		done();
	}).timeout(20000);

	step("should support topic conditional parameters", async function (done) {
		subscription = context.web3.eth.subscribe(
			"logs",
			{
				topics: [
					"0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef",
					[
						"0x0000000000000000000000006be02d1d3665660d22ff9624b7be0551ee1ac91b",
						"0x0000000000000000000000000000000000000000000000000000000000000000",
					],
				],
			},
			function (error, result) {}
		);

		await new Promise<void>((resolve) => {
			subscription.on("connected", function (d: any) {
				resolve();
			});
		});

		const tx = await sendTransaction(context);
		let data = null;
		let dataResolve = null;
		let dataPromise = new Promise((resolve) => {
			dataResolve = resolve;
		});

		subscription.on("data", function (d: any) {
			data = d;
			logsGenerated += 1;
			dataResolve();
		});

		await createAndFinalizeBlock(context.web3);
		await dataPromise;

		subscription.unsubscribe();
		expect(data).to.not.be.null;
		done();
	}).timeout(20000);

	step("should support multiple topic conditional parameters", async function (done) {
		subscription = context.web3.eth.subscribe(
			"logs",
			{
				topics: [
					"0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef",
					[
						"0x0000000000000000000000000000000000000000000000000000000000000000",
						"0x0000000000000000000000006be02d1d3665660d22ff9624b7be0551ee1ac91b",
					],
					[
						"0x0000000000000000000000006be02d1d3665660d22ff9624b7be0551ee1ac91b",
						"0x0000000000000000000000000000000000000000000000000000000000000000",
					],
				],
			},
			function (error, result) {}
		);

		await new Promise<void>((resolve) => {
			subscription.on("connected", function (d: any) {
				resolve();
			});
		});

		const tx = await sendTransaction(context);
		let data = null;
		let dataResolve = null;
		let dataPromise = new Promise((resolve) => {
			dataResolve = resolve;
		});
		subscription.on("data", function (d: any) {
			data = d;
			logsGenerated += 1;
			dataResolve();
		});

		await createAndFinalizeBlock(context.web3);
		await dataPromise;

		subscription.unsubscribe();
		expect(data).to.not.be.null;
		done();
	}).timeout(20000);

	step("should combine topic wildcards and conditional parameters", async function (done) {
		subscription = context.web3.eth.subscribe(
			"logs",
			{
				topics: [
					null,
					[
						"0x0000000000000000000000006be02d1d3665660d22ff9624b7be0551ee1ac91b",
						"0x0000000000000000000000000000000000000000000000000000000000000000",
					],
					null,
				],
			},
			function (error, result) {}
		);

		await new Promise<void>((resolve) => {
			subscription.on("connected", function (d: any) {
				resolve();
			});
		});

		const tx = await sendTransaction(context);
		let data = null;
		let dataResolve = null;
		let dataPromise = new Promise((resolve) => {
			dataResolve = resolve;
		});
		subscription.on("data", function (d: any) {
			data = d;
			logsGenerated += 1;
			dataResolve();
		});

		await createAndFinalizeBlock(context.web3);
		await dataPromise;

		subscription.unsubscribe();
		expect(data).to.not.be.null;
		done();
	}).timeout(20000);

	step("should emit events on create collection", async function (done) {
		subscription = context.web3.eth.subscribe("logs", {}, function (error, result) {});

		await new Promise<void>((resolve) => {
			subscription.on("connected", function (d: any) {
				resolve();
			});
		});

		await createAndFinalizeBlock(context.web3);

		const contract = new context.web3.eth.Contract(LAOS_EVOLUTION_ABI, PRECOMPILE_ADDRESS, {
			from: GENESIS_ACCOUNT,
			gasPrice: "0x3B9ACA00",
		});

		const tx_data = contract.methods.createCollection(GENESIS_ACCOUNT).encodeABI();

		// Set the storage and create a block
		const tx1 = await context.web3.eth.accounts.signTransaction(
			{
				from: GENESIS_ACCOUNT,
				to: PRECOMPILE_ADDRESS,
				data: tx_data,
				value: "0x00",
				gasPrice: "0x3B9ACA00",
				gas: "0x500000",
			},
			GENESIS_ACCOUNT_PRIVATE_KEY
		);
		await customRequest(context.web3, "eth_sendRawTransaction", [tx1.rawTransaction]);

		let data = null;
		let dataResolve = null;
		let dataPromise = new Promise((resolve) => {
			dataResolve = resolve;
		});
		subscription.on("data", function (d: any) {
			data = d;
			logsGenerated += 1;
			dataResolve();
		});

		await createAndFinalizeBlock(context.web3);
		await dataPromise;

		subscription.unsubscribe();

		expect(data.topics).to.include(NEW_COLLECTION_SELECTOR);
		expect(data.topics).to.include("0x" + "00".repeat(12) + "47a4320be4b65bf73112e068dc637883490f5b04");
		expect(data.address).to.equal(PRECOMPILE_ADDRESS);

		done();
	}).timeout(20000);
});
