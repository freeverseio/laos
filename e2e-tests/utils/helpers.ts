import { JsonRpcResponse } from "web3-core-helpers";
import Contract from "web3-eth-contract";
import Web3 from "web3";
import BN from "bn.js";
import { MAX_U96, GAS_PRICE, EVOLUTION_COLLECTION_FACTORY_ABI, EVOLUTION_COLLECTION_FACTORY_CONTRACT_ADDRESS, EVOLUTION_COLLECTION_ABI } from "@utils/constants";

import { expect } from "chai";

/**
 * Concats an arbitrary number of Uint8Array's
 * @param {Uint8Array[]} ...arrays - The arrays to be concatenated.
 * @returns {Uint8Array} -The concatenation of the arrays.
 */
export function concatUint8Arrays(...arrays: Uint8Array[]): Uint8Array {
	let totalLength = arrays.reduce((acc, curr) => acc + curr.length, 0);
	let result = new Uint8Array(totalLength);
	let offset = 0;
	for (let arr of arrays) {
		result.set(arr, offset);
		offset += arr.length;
	}
	return result;
}

/**
 * Builds and sends a custom RPC request
 * @param web3 the web3 instance
 * @param method the RPC method 
 * @param params potential data for the RPC method
 * @returns 
 */
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

/**
 * Converts a slot and owner address to a token ID.
 * @param {string} slot - slot The slot number.
 * @param {string} owner - The owner address.
 * @returns {string|null} - The token ID, or null if the slot is larger than 96 bits or the owner address is not 20 bytes.
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
 * Creates a collection using the evolution collection factory contract
 * @param {Web3} web3 - The Web3.js provider
 * @param {string} senderAddress - The ethereum address creating the collection
 * @returns {Promise<Contract>} - The collection contract.
 */
export async function createCollection(web3: Web3, senderAddress: string): Promise<Contract> {
	const contract = new web3.eth.Contract(
		EVOLUTION_COLLECTION_FACTORY_ABI,
		EVOLUTION_COLLECTION_FACTORY_CONTRACT_ADDRESS,
		{
			from: senderAddress,
			gasPrice: GAS_PRICE,
		}
	);

	let nonce = await web3.eth.getTransactionCount(senderAddress);
	const estimatedGas = await contract.methods.createCollection(senderAddress).estimateGas();
	const result = await contract.methods.createCollection(senderAddress).send({
		from: senderAddress,
		gas: estimatedGas,
		gasPrice: GAS_PRICE,
		nonce: nonce++,
	});
	expect(result.status).to.be.eq(true);
	expect(web3.utils.isAddress(result.events.NewCollection.returnValues._collectionAddress)).to.be.eq(true);

	const collectionContract = new web3.eth.Contract(
		EVOLUTION_COLLECTION_ABI,
		result.events.NewCollection.returnValues._collectionAddress,
		{
			from: senderAddress,
			gasPrice: GAS_PRICE,
		}
	);

	return collectionContract;
}
