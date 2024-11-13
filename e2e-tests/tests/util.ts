import Contract from "web3-eth-contract";
import Web3 from "web3";
import { JsonRpcResponse } from "web3-core-helpers";
import {
	EVOLUTION_COLLECTION_FACTORY_CONTRACT_ADDRESS,
	GAS_PRICE,
	FAITH,
	ALITH_PRIVATE_KEY,
	BALTATHAR_PRIVATE_KEY,
	FAITH_PRIVATE_KEY,
	EVOLUTION_COLLECTION_FACTORY_ABI,
	EVOLUTION_COLLECTION_ABI,
	MAX_U96,
	LAOS_NODE_IP,
	ASSET_HUB_NODE_IP,
	RELAYCHAIN_NODE_IP,
	LAOS_PARA_ID,
	ASSET_HUB_PARA_ID,
} from "./config";
import { CustomSuiteContext } from "./types";
import BN from "bn.js";
import { expect } from "chai";
import "@polkadot/api-augment";

import { ApiPromise, WsProvider } from "@polkadot/api";
import { SubmittableExtrinsic, ApiTypes } from "@polkadot/api/types";
import { SubmittableResult } from "@polkadot/api/submittable";
import { bnToU8a, stringToU8a } from "@polkadot/util";
import { encodeAddress } from "@polkadot/util-crypto";
import { AssetIdV3, DoubleEncodedCall, EventRecord, XcmOriginKind } from "@polkadot/types/interfaces";
import { XcmVersionedXcm } from "@polkadot/types/lookup";
import { Keyring } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";

require("events").EventEmitter.prototype._maxListeners = 100;

import debug from "debug";
const debugTx = debug("transaction");
const debugBlock = debug("block");

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
	cb: () => void,
	openPolkadotConnections: boolean = false,
	providerLaosNodeUrl?: string,
	providerAssetHubNodeUrl?: string,
	providerRelaychainNodeUrl?: string
) {
	describe(title, function (this: CustomSuiteContext) {
		before(async function () {
			this.context = {
				web3: new Web3(providerLaosNodeUrl || "http://" + LAOS_NODE_IP),
				ethersjs: null,
				networks: {
					laos: null,
					assetHub: null,
					relaychain: null,
				},
			};

			this.laosItems = {
				assetHubLocation: null,
				relayChainLocation: null,
			};

			this.assetHubItems = {
				accounts: { alice: null, bob: null, charlie: null, dave: null, eve: null, ferdie: null },
				multiAddresses: {
					alice: null,
					bob: null,
					charlie: null,
					dave: null,
					eve: null,
					ferdie: null,
					laosSA: null,
				},
				laosSA: sovereignAccountOf(LAOS_PARA_ID),
				laosLocation: null,
				laosAsset: null,
				relayChainLocation: null,
				relayAsset: null,
			};

			this.substratePairs = {
				alice: new Keyring({ type: "sr25519" }).addFromUri("//Alice"),
				bob: new Keyring({ type: "sr25519" }).addFromUri("//Bob"),
				charlie: new Keyring({ type: "sr25519" }).addFromUri("//Charlie"),
				dave: new Keyring({ type: "sr25519" }).addFromUri("//Dave"),
				eve: new Keyring({ type: "sr25519" }).addFromUri("//Eve"),
				ferdie: new Keyring({ type: "sr25519" }).addFromUri("//Ferdie"),
			};

			this.ethereumPairs = {
				alith: new Keyring({ type: "ethereum" }).addFromUri(ALITH_PRIVATE_KEY),
				baltathar: new Keyring({ type: "ethereum" }).addFromUri(BALTATHAR_PRIVATE_KEY),
				faith: new Keyring({ type: "ethereum" }).addFromUri(FAITH_PRIVATE_KEY),
			};
			this.context.web3.eth.accounts.wallet.add(ALITH_PRIVATE_KEY);
			this.context.web3.eth.accounts.wallet.add(BALTATHAR_PRIVATE_KEY);
			this.context.web3.eth.accounts.wallet.add(FAITH_PRIVATE_KEY);

			if (openPolkadotConnections) {
				// Laos
				let provider = new WsProvider(providerLaosNodeUrl || "ws://" + LAOS_NODE_IP);
				const apiLaos = await new ApiPromise({ provider }).isReady;
				this.context.networks.laos = apiLaos;

				provider = new WsProvider(providerAssetHubNodeUrl || "ws://" + ASSET_HUB_NODE_IP);
				const apiAssetHub = await ApiPromise.create({ provider: provider });

				this.context.networks.assetHub = apiAssetHub;

				provider = new WsProvider(providerRelaychainNodeUrl || "ws://" + RELAYCHAIN_NODE_IP);
				this.context.networks.relaychain = await new ApiPromise({ provider: provider }).isReady;

				this.assetHubItems.accounts = {
					alice: apiAssetHub.createType("AccountId", this.substratePairs.alice.address),
					bob: apiAssetHub.createType("AccountId", this.substratePairs.bob.address),
					charlie: apiAssetHub.createType("AccountId", this.substratePairs.charlie.address),
					dave: apiAssetHub.createType("AccountId", this.substratePairs.dave.address),
					eve: apiAssetHub.createType("AccountId", this.substratePairs.eve.address),
					ferdie: apiAssetHub.createType("AccountId", this.substratePairs.ferdie.address),
				};

				this.assetHubItems.multiAddresses = {
					alice: apiAssetHub.createType("MultiAddress", this.substratePairs.alice.address),
					bob: apiAssetHub.createType("MultiAddress", this.substratePairs.bob.address),
					charlie: apiAssetHub.createType("MultiAddress", this.substratePairs.charlie.address),
					dave: apiAssetHub.createType("MultiAddress", this.substratePairs.dave.address),
					eve: apiAssetHub.createType("MultiAddress", this.substratePairs.eve.address),
					ferdie: apiAssetHub.createType("MultiAddress", this.substratePairs.ferdie.address),
					laosSA: apiAssetHub.createType("MultiAddress", this.assetHubItems.laosSA),
				};

				this.assetHubItems.laosLocation = apiAssetHub.createType("XcmVersionedLocation", {
					V3: siblingLocation(LAOS_PARA_ID),
				});

				this.assetHubItems.laosAsset = apiAssetHub.createType(
					"StagingXcmV3MultiLocation",
					siblingLocation(LAOS_PARA_ID)
				);

				this.assetHubItems.relayChainLocation = apiAssetHub.createType("XcmVersionedLocation", {
					V3: relayLocation(),
				});

				this.assetHubItems.relayAsset = apiAssetHub.createType("StagingXcmV3MultiLocation", relayLocation());

				this.laosItems.assetHubLocation = apiLaos.createType("XcmVersionedLocation", {
					V3: siblingLocation(ASSET_HUB_PARA_ID),
				});
				this.laosItems.relayChainLocation = apiLaos.createType("XcmVersionedLocation", { V3: relayLocation() });
			}
		});
		cb();

		after(async function () {
			this.context.networks.laos && this.context.networks.laos.disconnect();
			this.context.networks.assetHub && this.context.networks.assetHub.disconnect();
			this.context.networks.relaychain && this.context.networks.relaychain.disconnect();
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

export async function waitForBlocks(api: ApiPromise, n: number) {
	return new Promise(async (resolve, reject) => {
		debugBlock(`Waiting for ${n} blocks...`);
		let blockCount = 0;

		try {
			// Await the subscription to get the unsubscribe function
			const unsubscribe = await api.rpc.chain.subscribeFinalizedHeads((lastHeader) => {
				blockCount += 1;
				debugBlock(`New block: #${lastHeader.number}, waiting for ${n - blockCount} more blocks...`);

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

export const transferBalance = async (api: ApiPromise, origin: KeyringPair, beneficiary: string, amount: BN) => {
	let beforeBalance = await api.query.system.account(beneficiary);

	try {
		await api.tx.balances.transferKeepAlive(beneficiary, amount).signAndSend(origin);
	} catch (error) {
		console.log("transaction failed: ", error);
	}

	let balance = await api.query.system.account(beneficiary);

	while (balance.data.free.eq(beforeBalance.data.free.add(amount)) == false) {
		await waitForBlocks(api, 1);

		balance = await api.query.system.account(beneficiary);
	}
};

const concatUint8Arrays = (...arrays: Uint8Array[]): Uint8Array => {
	let totalLength = arrays.reduce((acc, curr) => acc + curr.length, 0);
	let result = new Uint8Array(totalLength);
	let offset = 0;
	for (let arr of arrays) {
		result.set(arr, offset);
		offset += arr.length;
	}
	return result;
};

export const sovereignAccountOf = (paraId: number): string => {
	let type = "sibl";
	let typeEncoded = stringToU8a(type);
	let paraIdEncoded = bnToU8a(paraId, { bitLength: 16 });
	let zeroPadding = new Uint8Array(32 - typeEncoded.length - paraIdEncoded.length).fill(0);
	let address = concatUint8Arrays(typeEncoded, paraIdEncoded, zeroPadding);
	return encodeAddress(address);
};

export const isChannelOpen = async (api: ApiPromise, sender: number, recipient: number) => {
	const channel = await api.query.hrmp.hrmpChannels({
		sender,
		recipient,
	});
	return !channel.isEmpty;
};

export const sendOpenHrmpChannelTxs = async (api: ApiPromise, paraA: number, paraB: number) => {
	const maxCapacity = 8;
	const maxMessageSize = 1048576;
	const sudo = new Keyring({ type: "sr25519" }).addFromUri("//Alice");

	const hrmpChannelCalls = [];

	hrmpChannelCalls.push(api.tx.hrmp.forceOpenHrmpChannel(paraA, paraB, maxCapacity, maxMessageSize));
	hrmpChannelCalls.push(api.tx.hrmp.forceOpenHrmpChannel(paraB, paraA, maxCapacity, maxMessageSize));

	try {
		await api.tx.sudo.sudo(api.tx.utility.batchAll(hrmpChannelCalls)).signAndSend(sudo);
	} catch (error) {
		console.log(`Open HRMP channels transactions between parachains ${paraA} and ${paraB} failed: `, error);
	}

	while ((await isChannelOpen(api, paraA, paraB)) == false || (await isChannelOpen(api, paraB, paraA)) == false) {
		await waitForBlocks(api, 1);
	}
};

export const siblingLocation = (id: number) => ({
	parents: 1,
	interior: {
		x1: { parachain: id },
	},
});

export const relayLocation = () => ({
	parents: 1,
	interior: {
		here: null,
	},
});

export const hereLocation = () => ({
	parents: 0,
	interior: {
		here: null,
	},
});

interface XcmInstructionParams {
	api: ApiPromise;
	calls: DoubleEncodedCall[];
	refTime: BN;
	proofSize: BN;
	amount: BN;
	originKind: XcmOriginKind;
}

/**
 * Builds an XCM (Cross-Consensus Message) instruction.
 *
 * This function constructs an XCM instruction using the provided parameters. The instruction
 * includes a series of operations such as withdrawing assets, buying execution, and performing
 * transactions.
 *
 * @param {Object} params - The parameters for building the XCM instruction.
 * @param {ApiPromise} params.api - The Polkadot API instance.
 * @param {DoubleEncodedCall[]} params.calls - An array of double-encoded calls to be included in the XCM instruction.
 * @param {BN} params.refTime - The reference time for the weight limit.
 * @param {BN} params.proofSize - The proof size for the weight limit.
 * @param {BN} params.amount - The amount of the asset to be used in the XCM instruction.
 * @param {XcmOriginKind} params.originKind - The origin kind for the XCM instruction.
 *
 * @returns {XcmVersionedXcm} - The constructed XCM instruction.
 *
 * @example
 * const instruction = buildXcmInstruction({
 *   api,
 *   calls: [encodedCall1, encodedCall2],
 *   refTime: new BN(1000000),
 *   proofSize: new BN(1000),
 *   amount: new BN(1000000000),
 *   originKind: api.createType('XcmOriginKind', 'Native'),
 * });
 */
export const buildXcmInstruction = ({
	api,
	calls,
	refTime,
	proofSize,
	amount,
	originKind,
}: XcmInstructionParams): XcmVersionedXcm => {
	const relayToken = api.createType("AssetIdV3", {
		Concrete: relayLocation(),
	}) as AssetIdV3;

	const transacts = calls.map((call) => ({
		Transact: {
			originKind,
			requireWeightAtMost: api.createType("WeightV2", {
				refTime,
				proofSize,
			}),
			call,
		},
	}));

	return api.createType("XcmVersionedXcm", {
		V3: [
			{
				WithdrawAsset: [
					api.createType("MultiAssetV3", {
						id: relayToken,
						fun: api.createType("FungibilityV3", {
							Fungible: amount,
						}),
					}),
				],
			},
			{
				BuyExecution: {
					fees: api.createType("MultiAssetV3", {
						id: relayToken,
						fun: api.createType("FungibilityV3", {
							Fungible: amount,
						}),
					}),
					weight_limit: "Unlimited",
				},
			},
			...transacts,
		],
	});
};

const debugEvents = debug("events");

/**
 * Waits for a specific event starting from the newest block, with a block-based timeout.
 * @param api - The ApiPromise instance.
 * @param filter - A function that filters events.
 * @param blockTimeout - The maximum number of blocks to wait before timing out.
 * @returns A promise that resolves to the matching event when found.
 */
export const waitForEvent = async (
	api: ApiPromise,
	filter: (event: EventRecord) => boolean,
	blockTimeout: number
): Promise<EventRecord> => {
	return new Promise(async (resolve, reject) => {
		try {
			let eventFound: EventRecord | null = null;
			let remainingBlocks = blockTimeout;

			const unsub = await api.rpc.chain.subscribeFinalizedHeads(async (header) => {
				// Fetch events at the current block
				const blockHash = await api.rpc.chain.getBlockHash(header.number.toNumber());
				const events = await api.query.system.events.at(blockHash);
				debugEvents(
					`[${api.runtimeVersion.specName.toString()}] Looking for events at block ${header.number.toNumber()}`
				);
				// Check if any event matches the filter
				events.forEach((eventRecord) => {
					if (filter(eventRecord)) {
						eventFound = eventRecord;
					}
				});

				if (eventFound) {
					debugEvents(
						`[${api.runtimeVersion.specName.toString()}] Event found at block ${header.number.toNumber()}`
					);
					unsub();
					resolve(eventFound);
					return;
				}

				remainingBlocks--;
				if (remainingBlocks === 0) {
					// If the loop completes without finding the event
					unsub();
					reject(new Error(`Timeout waiting for event after ${blockTimeout} blocks`));
				}
			});
		} catch (error) {
			reject(error);
		}
	});
};

// Generic function to send a transaction and wait for finalization
export async function sendTxAndWaitForFinalization(
	api: ApiPromise,
	tx: SubmittableExtrinsic<ApiTypes>,
	signer: KeyringPair,
	waitNBlocks = 20
) {
	return new Promise((resolve, reject) => {
		try {
			let blockCount = 0;

			const onStatusChange = async (result: SubmittableResult) => {
				const { status, events, dispatchError } = result;
				debugTx("Status:", status.type);

				// Additional event info
				if (events && events.length > 0) {
					events.forEach(({ event: { method, section, data } }) => {
						debugTx(`Event: ${section}.${method} - Data: ${data.toString()}`);
					});
				}

				if (dispatchError) {
					debugTx("Raw dispatch error:", dispatchError.toString());

					if (dispatchError.isModule) {
						const decoded = api.registry.findMetaError(dispatchError.asModule);
						const { section, name, docs } = decoded;
						debugTx(`Transaction failed with error: ${section}.${name}`);
						debugTx(`Error documentation: ${docs.join(" ")}`);
						reject(new Error(`${section}.${name}: ${docs.join(" ")}`));
					} else {
						debugTx(`Transaction failed with error: ${dispatchError.toString()}`);
						reject(new Error(dispatchError.toString()));
					}
					return;
				}

				if (status.isInBlock) {
					debugTx("Included at block hash", status.asInBlock.toHex());
				} else if (status.isFinalized) {
					debugTx("Finalized block hash", status.asFinalized.toHex());
					resolve(status.asFinalized.toHex());
				} else if (
					status.isDropped ||
					status.isInvalid ||
					status.isUsurped ||
					status.isFuture ||
					status.isRetracted ||
					status.isFinalityTimeout
				) {
					debugTx("Transaction failed with status:", status.type);
					// Start waiting for N blocks and re-check transaction status
					const extrinsicHash = result.txHash.toHuman();
					debugTx(`Transaction is invalid. Waiting for ${waitNBlocks} blocks while rechecking status...`);

					// Subscribe to new finalized blocks to re-check transaction status
					const unsubscribeAll = await api.rpc.chain.subscribeFinalizedHeads(async (lastHeader) => {
						blockCount++;
						debugTx(`Finalized Block #${lastHeader.number} received (${blockCount}/${waitNBlocks})`);

						// Check if the transaction has been included in the block
						const blockHash = lastHeader.hash;
						const block = await api.rpc.chain.getBlock(blockHash);

						let txIncluded = false;

						for (const extrinsic of block.block.extrinsics) {
							if (extrinsic.hash.toHex() === extrinsicHash) {
								debugTx(`Transaction included in block ${lastHeader.number}`);
								txIncluded = true;
								unsubscribeAll();
								resolve(blockHash.toHex());
								break;
							}
						}

						if (txIncluded) {
							// Transaction has been included; resolve has been called
							return;
						} else if (blockCount >= waitNBlocks) {
							debugTx(`Waited for ${waitNBlocks} blocks after invalid status.`);
							unsubscribeAll(); // Unsubscribe from block headers
							reject(new Error(`Transaction remained invalid after waiting for ${waitNBlocks} blocks.`));
						}
					});
				}
			};

			tx.signAndSend(signer, onStatusChange);
		} catch (error) {
			debugTx("Error during transaction setup:", error);
			reject(error);
		}
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
					debugTx(`${txHash} has ${confirmations} confirmations.`);
					return receipt; // Transaction has the required confirmations
				} else {
					debugTx(`Waiting for confirmations... (${confirmations}/${requiredConfirmations})`);
				}
			} else {
				debugTx("Not yet mined. Retrying...");
			}

			// Wait for the next block
			await new Promise((resolve) => setTimeout(resolve, pollInterval));
			currentBlock = await web3.eth.getBlockNumber();
		}
	} catch (error) {
		debugTx(`Error waiting for confirmations of transaction ${txHash}:`, error);
		throw error;
	}
}
