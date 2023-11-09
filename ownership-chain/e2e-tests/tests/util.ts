import { ApiPromise, WsProvider } from "@polkadot/api";
import BN from "bn.js";
import { ethers } from "ethers";
import Web3 from "web3";
import { JsonRpcResponse } from "web3-core-helpers";
import { ASTAR_RPC_PORT, BLOCK_TIME, MAX_U96, ROCOCO_RPC_PORT, RPC_PORT } from "./config";

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
 * Similar to `describeWithExistingNode`, but provides `Api` interface to the Substrate node.
 * @param title
 * @param cb
 */
export function describeWithExistingSubstrateNodes(
	title: string,
	cb: (context: { ownchain: ApiPromise; astar: ApiPromise; relaychain: ApiPromise }) => void
) {
	describe(title, () => {
		let context: {
			ownchain: ApiPromise;
			astar: ApiPromise;
			relaychain: ApiPromise;
		} = { ownchain: null, astar: null, relaychain: null };

		let ownchainWs = new WsProvider(`ws://127.0.0.1:${RPC_PORT}`);
		ApiPromise.create({ provider: ownchainWs }).then((api) => {
			context.ownchain = api;
		});

		let astarWs = new WsProvider(`ws://127.0.0.1:${ASTAR_RPC_PORT}`);
		ApiPromise.create({ provider: astarWs }).then((api) => {
			context.astar = api;
		});

		let relaychainWs = new WsProvider(`ws://127.0.0.1:${ROCOCO_RPC_PORT}`);
		ApiPromise.create({ provider: relaychainWs }).then((api) => {
			context.relaychain = api;
		});

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
 * Wait for specific events to be emitted.
 * @param api - Substrate API
 * @param module - Module name
 * @param name - Event name
 * @param blocks - Number of blocks to wait for, defaults to 5
 * @returns Promise that resolves to the events data
 */
export async function waitForEvents(
	api: ApiPromise,
	targetEvents: { module: string; name: string }[],
	blocks: number = 5
): Promise<any> {
	let blockCounter = 0;
	let result = {};

	api.query.system.events((events) => {
		// check if `target` is a subset of `source`
		const isSubset = (target: { module: string; name: string }[], source: any[]): any[] => {
			return source.filter((s) =>
				target.some(
					(t) =>
						t.module.toLowerCase() === s.event.section.toLowerCase() &&
						t.name.toLowerCase() === s.event.method.toLowerCase()
				)
			);
		};

		let foundEvents = isSubset(targetEvents, events);

		// make sure we found all the unique events
		const foundAllEvents = targetEvents.every((t) =>
			foundEvents.some(
				(f) =>
					f.event.section.toLowerCase() === t.module.toLowerCase() &&
					f.event.method.toLowerCase() == t.name.toLowerCase()
			)
		);

		if (foundAllEvents && foundEvents.length === targetEvents.length) {
			// Loop through each of the parameters, displaying the type and data
			foundEvents.forEach((e) => {
				const { event } = e;
				let types = event.typeDef;
				// Loop through each of the parameters, displaying the type and data
				event.data.forEach((data, index) => {
					console.log(`\t\t\t${types[index].type}: ${data.toString()}`);
				});
				result[`${event.section}.${event.method}`] = event.data.map((d) => d.toString());
			});
		}
	});

	return new Promise((resolve, _) => {
		setTimeout(() => {
			resolve(result);
		}, BLOCK_TIME * blocks);
	});
}
