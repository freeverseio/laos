import { ApiPromise } from "@polkadot/api";
import { EventRecord } from "@polkadot/types/interfaces";
import BN from "bn.js";

/**
 * Get the last finalized block number of a chain
 * @param {ApiPromise} api - The ApiPromise to interact with the chain
 * @returns {Promise<BN>} - The block number of the best finalized block
 */
export async function getFinalizedBlockNumber(api: ApiPromise): Promise<BN> {
	return new Promise(async (resolve) => {
		resolve(
			new BN(
				(await api.rpc.chain.getBlock(await api.rpc.chain.getFinalizedHead())).block.header.number.toNumber()
			)
		);
	});
}

/**
 * Checks that a specific event is included in a specific block.
 * @param {ApiPromise} api - The ApiPromise to interact with the chain.
 * @param {(event: EventRecord) => boolean} filter - A function that filters events.
 * @param {string} blockHash - The hash corresponding to the block where we would like to find the event.
 * @param {boolean} rejectIfNotFound - A boolean to indicate if the Promise has to reject if the event isn't found in the block.If set to false, the returns might resolve to null. Defaults to true
 * @returns {Promise<EventRecord | null>} - A promise that resolves in the event found in the block.
 */
export async function checkEventInBlock(
	api: ApiPromise,
	filter: (event: EventRecord) => boolean,
	blockHash: string,
	rejectIfNotFound: boolean = true
): Promise<EventRecord | null> {
	return new Promise(async (resolve, reject) => {
		let event: EventRecord | null = null;
		const apiAt = await api.at(blockHash);
		const events = await apiAt.query.system.events();
		events.forEach((eventRecord) => {
			if (filter(eventRecord)) {
				event = eventRecord;
			}
		});
		if (event) {
			resolve(event);
		} else {
			if (rejectIfNotFound) {
				reject(new Error(`Event not found in block ${blockHash}`));
			} else {
				resolve(null);
			}
		}
	});
}

/**
 * Force a chain to wait a determined amount of blocks
 * @param {ApiPromise} api - The ApiPromise to interact with the chain
 * @param {number} blocks - The number of blocks to wait
 */
export async function waitForBlocks(api: ApiPromise, blocks: number) {
	return new Promise(async (resolve, reject) => {
		let blockCount = 0;

		try {
			// Await the subscription to get the unsubscribe function
			const unsubscribe = await api.rpc.chain.subscribeFinalizedHeads((_lastHeader) => {
				blockCount += 1;

				if (blockCount >= blocks) {
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
