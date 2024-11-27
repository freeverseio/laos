import { JsonRpcResponse } from "web3-core-helpers";
import Web3 from "web3";

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
