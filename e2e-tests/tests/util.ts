import { ethers } from "ethers";
import Contract from "web3-eth-contract";
import Web3 from "web3";
import { JsonRpcResponse } from "web3-core-helpers";
import {
	EVOLUTION_COLLECTION_FACTORY_CONTRACT_ADDRESS,
	GAS_PRICE,
	ALITH_PRIVATE_KEY,
	FAITH,
	FAITH_PRIVATE_KEY,
	EVOLUTION_COLLECTION_FACTORY_ABI,
	EVOLUTION_COLLECTION_ABI,
	MAX_U96,
	LOCAL_NODE_IP,
} from "./config";
import BN from "bn.js";
import { expect } from "chai";
import "@polkadot/api-augment";

import { ApiPromise, HttpProvider, WsProvider } from "@polkadot/api";

require("events").EventEmitter.prototype._maxListeners = 100;

export async function customRequest(web3: Web3, method: string, params: any[]) {
	return new Promise<JsonRpcResponse>((resolve, reject) => {
		(web3.currentProvider as any).send(
			{
				jsonrpc: "2.0",
				id: 1,
				method,
				params,
			},
			(error: Error | null, result?: JsonRpcResponse) => {
				if (error) {
					reject(
						`Failed to send custom request (${method} (${params.join(",")})): ${
							error.message || error.toString()
						}`
					);
				}
				resolve(result);
			}
		);
	});
}

export function describeWithExistingNode(
	title: string,
	cb: (context: { web3: Web3; polkadot: ApiPromise }) => void,
	providerNodeUrl?: string
) {
	describe(title, () => {
		let context: {
			web3: Web3;
			ethersjs: ethers.JsonRpcProvider;
			polkadot: ApiPromise;
		} = {
			web3: null,
			ethersjs: null,
			polkadot: undefined,
		};

		before(async () => {
			if (providerNodeUrl) {
				context.web3 = new Web3(providerNodeUrl);
				const wsProvider = new WsProvider(providerNodeUrl);
				context.polkadot = await new ApiPromise({ provider: wsProvider }).isReady;
			} else {
				context.web3 = new Web3("http://" + LOCAL_NODE_IP);
				const wsProvider = new WsProvider("ws://" + LOCAL_NODE_IP);
				context.polkadot = await new ApiPromise({ provider: wsProvider }).isReady;
			}

			context.web3.eth.accounts.wallet.add(ALITH_PRIVATE_KEY);
			context.web3.eth.accounts.wallet.add(FAITH_PRIVATE_KEY);
		});

		cb(context);

		after(async () => {
			context.polkadot.disconnect();
		});
	});
}

export async function createCollection(context: { web3: Web3 }): Promise<Contract> {
	const contract = new context.web3.eth.Contract(
		EVOLUTION_COLLECTION_FACTORY_ABI,
		EVOLUTION_COLLECTION_FACTORY_CONTRACT_ADDRESS,
		{
			from: FAITH,
			gasPrice: GAS_PRICE,
		}
	);

	let nonce = await context.web3.eth.getTransactionCount(FAITH);
	context.web3.eth.accounts.wallet.add(FAITH_PRIVATE_KEY);
	const estimatedGas = await contract.methods.createCollection(FAITH).estimateGas();
	const result = await contract.methods.createCollection(FAITH).send({
		from: FAITH,
		gas: estimatedGas,
		gasPrice: GAS_PRICE,
		nonce: nonce++,
	});
	expect(result.status).to.be.eq(true);
	expect(context.web3.utils.isAddress(result.events.NewCollection.returnValues._collectionAddress)).to.be.eq(true);

	const collectionContract = new context.web3.eth.Contract(
		EVOLUTION_COLLECTION_ABI,
		result.events.NewCollection.returnValues._collectionAddress,
		{
			from: FAITH,
			gasPrice: GAS_PRICE,
		}
	);

	return collectionContract;
}

/**
 * Converts a slot and owner address to a token ID.
 * @param slot The slot number.
 * @param owner The owner address.
 * @returns The token ID, or null if the slot is larger than 96 bits or the owner address is not 20 bytes.
 */
export function slotAndOwnerToTokenId(slot: string, owner: string): string | null {
	const slotBN: BN = new BN(slot);
	const ownerBytes: Uint8Array = Uint8Array.from(Buffer.from(owner.slice(2), "hex")); // Remove the '0x' prefix and convert hex to bytes

	if (slotBN.gt(MAX_U96) || ownerBytes.length != 20) {
		return null;
	}

	// Convert slot to big-endian byte array
	const slotBytes = slotBN.toArray("be", 16); // 16 bytes (128 bits)

	// We also use the last 12 bytes of the slot, since the first 4 bytes are always 0
	let bytes = new Uint8Array(32);
	bytes.set(slotBytes.slice(-12), 0); // slice from the right to ensure we get the least significant bytes
	bytes.set(ownerBytes, 12);

	return Buffer.from(bytes).toString("hex"); // Convert Uint8Array to hexadecimal string
}

/**
 * Converts an Ethereum-like address into a `CollectionId` represented as a `BN` (big number).
 *
 * This function takes a hexadecimal string representation of an address and attempts to
 * convert it into a `CollectionId`. The address is expected to be in a specific format:
 *  - The first 11 bytes should be zeros.
 *  - The 12th byte should be `1`, indicating the version.
 *  - The last 8 bytes represent the `CollectionId`.
 *
 * If the address does not meet these criteria, the function returns `null`.
 *
 * @param address The Ethereum-like address in hexadecimal string format.
 * @returns The `CollectionId` as a `BN` if the address is valid, or `null` otherwise.
 */
export function addressToCollectionId(address: string): BN | null {
	const addressBytes: Uint8Array = Uint8Array.from(Buffer.from(address.slice(2), "hex")); // Remove the '0x' prefix and convert hex to bytes

	// Check if the address length is 20 bytes
	if (addressBytes.length !== 20) {
		return null;
	}

	// Check if the first 11 bytes are zeros
	for (let i = 0; i < 11; i++) {
		if (addressBytes[i] !== 0) {
			return null;
		}
	}

	// Check if the 12th byte is 1 (version byte)
	if (addressBytes[11] !== 1) {
		return null;
	}

	// Extract the last 8 bytes and convert them to a BigInt
	const collectionIdBytes = addressBytes.slice(12, 20);
	let collectionId = new BN(0);
	for (let i = 0; i < collectionIdBytes.length; i++) {
		collectionId = collectionId.shln(8).add(new BN(collectionIdBytes[i]));
	}

	return collectionId;
}

export async function extractRevertReason(context: { web3: Web3 }, transactionHash: string) {
	try {
		let tx = await context.web3.eth.getTransaction(transactionHash);
		await context.web3.eth.call({ to: tx.to, data: tx.input, gas: tx.gas });
	} catch (error) {
		const reasonHex = error.data.slice(2 + 8); // remove the 0x prefix and the first 8 bytes (function selector for Error(string))
		return context.web3.utils.hexToUtf8("0x" + reasonHex.slice(64)).trim(); // skip the padding and remove return carriage
	}
}

// Generic function to send a transaction and wait for finalization
export async function sendTxAndWaitForFinalization(tx, signer, options = {}) {
	return new Promise((resolve, reject) => {
		tx.signAndSend(signer, options, ({ status, dispatchError }) => {
			console.log("Transaction status:", status.type);

			if (status.isInBlock) {
				console.log("Included at block hash", status.asInBlock.toHex());
			} else if (status.isFinalized) {
				console.log("Finalized block hash", status.asFinalized.toHex());
				// resolve the promise when the transaction is finalized
				resolve(status.asFinalized.toHex());
			} else if (status.isDropped || status.isInvalid || status.isUsurped) {
				reject(dispatchError.toString()); // Reject the promise on error
			}
		}).catch((error) => {
			console.error("Error during transaction:", error);
			reject(error); // Reject the promise on error
		});
	});
}

export async function waitForConfirmations(web3, txHash, requiredConfirmations = 12, pollInterval = 1000) {
	try {
		let currentBlock = await web3.eth.getBlockNumber();
		let receipt = null;

		while (true) {
			// Check for the transaction receipt
			receipt = await web3.eth.getTransactionReceipt(txHash);

			if (receipt && receipt.blockNumber) {
				// Calculate the number of confirmations
				const confirmations = currentBlock - receipt.blockNumber;

				if (confirmations >= requiredConfirmations) {
					console.log(`Transaction ${txHash} has ${confirmations} confirmations.`);
					return receipt; // Transaction has the required confirmations
				} else {
					console.log(`Waiting for confirmations... (${confirmations}/${requiredConfirmations})`);
				}
			} else {
				console.log("Transaction not yet mined. Retrying...");
			}

			// Wait for the next block
			await new Promise((resolve) => setTimeout(resolve, pollInterval));
			currentBlock = await web3.eth.getBlockNumber();
		}
	} catch (error) {
		console.error(`Error waiting for confirmations of transaction ${txHash}:`, error);
		throw error;
	}
}

export async function waitForBlocks(api, n) {
	return new Promise(async (resolve, reject) => {
		console.log(`Waiting for ${n} blocks...`);
		let blockCount = 0;

		try {
			// Await the subscription to get the unsubscribe function
			const unsubscribe = await api.rpc.chain.subscribeNewHeads((lastHeader) => {
				blockCount += 1;
				console.log(`New block: #${lastHeader.number}, waiting for ${n - blockCount} more blocks...`);

				if (blockCount >= n) {
					unsubscribe(); // Stop listening for new blocks
					resolve(true);
				}
			});
		} catch (error) {
			console.error(`Error while subscribing to new heads:`, error);
			reject(error);
		}
	});
}
