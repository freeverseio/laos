import { ethers } from "ethers";
import Contract from "web3-eth-contract";
import Web3 from "web3";
import { JsonRpcResponse } from "web3-core-helpers";
import { CONTRACT_ADDRESS, GAS_LIMIT, GAS_PRICE, GENESIS_ACCOUNT, GENESIS_ACCOUNT_PRIVATE_KEY, EVOLUTION_COLLETION_FACTORY_ABI, EVOLUTION_COLLECTION_ABI, MAX_U96, RPC_PORT } from "./config";
import BN from "bn.js";
import { expect } from "chai";

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

export function describeWithExistingNode(title: string, cb: (context: { web3: Web3 }) => void, provider?: string) {
	describe(title, () => {
		let context: {
			web3: Web3;
			ethersjs: ethers.JsonRpcProvider;
		} = { web3: null, ethersjs: null };

		// if (!provider || provider == "http") {
		// 	context.web3 = new Web3(`http://127.0.0.1:${RPC_PORT}`);
		// }

		// if (provider == "ws") {
		// 	context.web3 = new Web3(`ws://127.0.0.1:${RPC_PORT}`);
		// }
		context.web3 = new Web3("wss://rpc.laossun.gorengine.com");

		cb(context);
	});
}

export async function createCollection(context: { web3: Web3 }): Promise<Contract> {
	const contract = new context.web3.eth.Contract(EVOLUTION_COLLETION_FACTORY_ABI, CONTRACT_ADDRESS, {
		from: GENESIS_ACCOUNT,
		gasPrice: GAS_PRICE,
	});
	
	let nonce = await context.web3.eth.getTransactionCount(GENESIS_ACCOUNT);
	context.web3.eth.accounts.wallet.add(GENESIS_ACCOUNT_PRIVATE_KEY);
	const result = await contract.methods.createCollection(GENESIS_ACCOUNT).send({
		from: GENESIS_ACCOUNT,
		gas: GAS_LIMIT,
		gasPrice: GAS_PRICE,
		nonce: nonce++,
	});
	expect(result.status).to.be.eq(true);
	expect(context.web3.utils.isAddress(result.events.NewCollection.returnValues._collectionAddress)).to.be.eq(true);
	
	const collectionContract = new context.web3.eth.Contract(EVOLUTION_COLLECTION_ABI, result.events.NewCollection.returnValues._collectionAddress, {
		from: GENESIS_ACCOUNT,
		gas: GAS_LIMIT,
		gasPrice: GAS_PRICE,
	});
	
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
	const ownerBytes: Uint8Array = Uint8Array.from(Buffer.from(owner.slice(2), 'hex'));  // Remove the '0x' prefix and convert hex to bytes

	if (slotBN.gt(MAX_U96) || ownerBytes.length != 20){
		return null;
	}

	// Convert slot to big-endian byte array
	const slotBytes = slotBN.toArray('be', 16);  // 16 bytes (128 bits)
	
	// We also use the last 12 bytes of the slot, since the first 4 bytes are always 0
	let bytes = new Uint8Array(32);
	bytes.set(slotBytes.slice(-12), 0);  // slice from the right to ensure we get the least significant bytes
	bytes.set(ownerBytes, 12);

	return Buffer.from(bytes).toString('hex'); // Convert Uint8Array to hexadecimal string
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
	const addressBytes: Uint8Array = Uint8Array.from(Buffer.from(address.slice(2), 'hex'));  // Remove the '0x' prefix and convert hex to bytes

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
