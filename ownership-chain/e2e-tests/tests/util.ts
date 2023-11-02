import { ethers } from "ethers";
import Web3 from "web3";
import { JsonRpcResponse } from "web3-core-helpers";

import { CHAIN_ID } from "./config";

export const RPC_PORT = 9999;

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

export function describeWithExistingNode(title: string, cb: (context: { web3: Web3, ethersjs: ethers.JsonRpcProvider }) => void, provider?: string) {
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

		context.ethersjs = new ethers.JsonRpcProvider(`http://127.0.0.1:${RPC_PORT}`, {
			chainId: CHAIN_ID,
			name: "frontier-dev",
		});

		cb(context);
	});
}

