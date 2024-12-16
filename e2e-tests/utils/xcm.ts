import { bnToU8a, stringToU8a } from "@polkadot/util";
import { encodeAddress } from "@polkadot/util-crypto";
import { ApiPromise } from "@polkadot/api";
import { EventRecord } from "@polkadot/types/interfaces";
import BN from "bn.js";
import { concatUint8Arrays } from "@utils/helpers";
import { getFinalizedBlockNumber } from "@utils/blocks";
import "@polkadot/api-augment";
import { POLKADOT_PREFIX } from "@utils/constants";

/**
 * Computes the sibling account of a sibling parachain in a Substrate chain
 * @param {number} paraId - The ID of the sibling parachain.
 * @returns {string} - The address of the sibling parachain.
 */
export function sovereignAccountOf(paraId: number): string {
	let type = "sibl";
	let typeEncoded = stringToU8a(type);
	let paraIdEncoded = bnToU8a(paraId, { bitLength: 16 });
	let zeroPadding = new Uint8Array(32 - typeEncoded.length - paraIdEncoded.length).fill(0);
	let address = concatUint8Arrays(typeEncoded, paraIdEncoded, zeroPadding);
	return encodeAddress(address, POLKADOT_PREFIX);
}

/**
 * @param {number} id - The ID of the sibling parachain
 * @returns {Object} - A object representing the sibling parachain location
 */
export function siblingParachainLocation(id: number): Object {
	return {
		parents: 1,
		interior: {
			x1: [{ parachain: id }],
		},
	};
}

/**
 * @returns {Object} - A object representing the relayChain location
 */
export function relayChainLocation(): Object {
	return {
		parents: 1,
		interior: {
			here: null,
		},
	};
}

/**
 * @returns {Object} - A object representing here location
 */
export function hereLocation(): Object {
	return {
		parents: 0,
		interior: {
			here: null,
		},
	};
}

/**
 * Checks that a specific event has been emitted after a XCM transaction.
 * @param {ApiPromise} api - The ApiPromise instance corresponding to the receiver chain.
 * @param {(event: EventRecord) => boolean} filter - A function that filters events.
 * @param {BN} startingBlock - The best finalized block before the XCM transaction was sent by the origin chain. This value ensures the event isn't lost in a block before the best finalized when this is called and the best finalized when the XCM was sent.
 * @returns {Promise<EventRecord>}- A promise that resolves in the event if it's found in a block after startingBlock and rejects if a XCM not processed event has been emitted.
 */
export async function checkEventAfterXcm(
	api: ApiPromise,
	filter: (event: EventRecord) => boolean,
	startingBlock: BN
): Promise<EventRecord> {
	// Check whether the event has been emitted in a specific block. If not found, resolves to null.
	const findEventAfterXcmAtBlock = async (blockNumber: BN): Promise<EventRecord | null> => {
		return new Promise(async (resolve, reject) => {
			let event: EventRecord | null = null;
			let processed = false;
			const blockHash = await api.rpc.chain.getBlockHash(blockNumber);
			const apiAt = await api.at(blockHash);
			const events = await apiAt.query.system.events();
			events.forEach((eventRec: EventRecord) => {
				// Ensure XCM message has been properly executed
				if (api.events.messageQueue.Processed.is(eventRec.event)) {
					//data[3] corresponds to data.success in this event; not accessible by TS with data.success
					if (!eventRec.event.data[3]) {
						reject(new Error("XCM message couldn't be processed"));
					} else {
						processed = true;
					}
				}
				// Ensure the expected event has been emitted
				else if (filter(eventRec)) {
					event = eventRec;
				}
			});

			if (event && processed) {
				resolve(event);
			} else {
				resolve(null);
			}
		});
	};

	// A promise race as we track two block ranges
	return Promise.race<EventRecord>([
		// This promise tracks blocks between startingBlock and the best finalized block
		new Promise<EventRecord>(async (resolve) => {
			const bestFinalizedBlock = await getFinalizedBlockNumber(api);
			for (let block = startingBlock; block.lte(bestFinalizedBlock); block = block.add(new BN(1))) {
				const event = await findEventAfterXcmAtBlock(block);
				if (event) {
					resolve(event);
				}
			}
		}),
		// This promise tracks new finalized blocks as soon as they arrive
		new Promise<EventRecord>(async (resolve) => {
			const unsub = await api.rpc.chain.subscribeFinalizedHeads(async (lastHeader) => {
				const event = await findEventAfterXcmAtBlock(new BN(lastHeader.number.toNumber()));
				if (event) {
					unsub();
					resolve(event);
				}
			});
		}),
	]);
}
