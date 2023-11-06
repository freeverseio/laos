import { ethers } from "ethers";
import Web3 from "web3";
import { JsonRpcResponse } from "web3-core-helpers";
import { MAX_U96, RPC_PORT } from "./config";
import BN from "bn.js";

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

		if (!provider || provider == "http") {
			context.web3 = new Web3(`http://127.0.0.1:${RPC_PORT}`);
		}

		if (provider == "ws") {
			context.web3 = new Web3(`ws://127.0.0.1:${RPC_PORT}`);
		}

		cb(context);
	});
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