import { ethers } from "ethers";
import Contract from "web3-eth-contract";
import Web3 from "web3";
import { JsonRpcResponse } from "web3-core-helpers";
import {
	EVOLUTION_COLLECTION_FACTORY_CONTRACT_ADDRESS,
	GAS_PRICE,
	FAITH,
	FAITH_PRIVATE_KEY,
	EVOLUTION_COLLECTION_FACTORY_ABI,
	EVOLUTION_COLLECTION_ABI,
	MAX_U96,
	LAOS_NODE_URL,
	ASSET_HUB_NODE_URL,
	RELAYCHAIN_NODE_URL,
	LAOS_PARA_ID,
	ASSET_HUB_PARA_ID,
} from "./config";
import BN from "bn.js";
import { expect } from "chai";
import "@polkadot/api-augment";

import { ApiPromise, HttpProvider, Keyring } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";
import { bnToU8a, stringToU8a } from "@polkadot/util";
import { encodeAddress } from "@polkadot/util-crypto";
import { AssetIdV3, DoubleEncodedCall, XcmOriginKind } from "@polkadot/types/interfaces";
import { XcmVersionedXcm } from "@polkadot/types/lookup";

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
	cb: (context: { web3: Web3; networks: { laos: ApiPromise; assetHub: ApiPromise; relaychain: ApiPromise } }) => void,
	providerLaosNodeUrl?: string,
	providerAssetHubNodeUrl?: string,
	providerRelaychainNodeUrl?: string
) {
	describe(title, () => {
		let context: {
			web3: Web3;
			ethersjs: ethers.JsonRpcProvider;
			networks: { laos: ApiPromise; assetHub: ApiPromise; relaychain: ApiPromise };
		} = {
			web3: null,
			ethersjs: null,
			networks: {
				laos: null,
				assetHub: null,
				relaychain: null,
			},
		};

		before(async () => {
			if (providerLaosNodeUrl) {
				context.web3 = new Web3(providerLaosNodeUrl);
				const wsProvider = new HttpProvider(providerLaosNodeUrl);
				context.networks.laos = await new ApiPromise({ provider: wsProvider }).isReady;
			} else {
				context.web3 = new Web3(LAOS_NODE_URL);
				const wsProvider = new HttpProvider(LAOS_NODE_URL);
				context.networks.laos = await new ApiPromise({ provider: wsProvider }).isReady;
			}

			if (providerAssetHubNodeUrl) {
				const wsProvider = new HttpProvider(providerAssetHubNodeUrl);
				context.networks.assetHub = await new ApiPromise({ provider: wsProvider }).isReady;
			} else {
				const wsProvider = new HttpProvider(ASSET_HUB_NODE_URL);
				context.networks.assetHub = await new ApiPromise({ provider: wsProvider }).isReady;
			}

			if (providerRelaychainNodeUrl) {
				const wsProvider = new HttpProvider(providerRelaychainNodeUrl);
				context.networks.relaychain = await new ApiPromise({ provider: wsProvider }).isReady;
			} else {
				const wsProvider = new HttpProvider(RELAYCHAIN_NODE_URL);
				context.networks.relaychain = await new ApiPromise({ provider: wsProvider }).isReady;
			}
		});
		cb(context);
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

export const delay = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

export const awaitBlockChange = async (api: ApiPromise) => {
	const currentBlock = await api.rpc.chain.getBlock();
	let changedBlock = false;

	while (!changedBlock) {
		const newBlock = await api.rpc.chain.getBlock();
		if (newBlock.block.header.number.toNumber() > currentBlock.block.header.number.toNumber()) {
			changedBlock = true;
		}

		await delay(1000);
	}
};

export const awaitNBlocks = async (api: ApiPromise, n: number) => {
	for (let i = 0; i < n; i++) {
		await awaitBlockChange(api);
	}
};

export const fundAccount = async (api: ApiPromise, source: KeyringPair, dest: string, amount: number) => {
	api.tx.balances
		.transferKeepAlive(dest, amount)
		.signAndSend(source, () => {})
		.catch((error: any) => {
			console.log("transaction failed", error);
		});
	let balance = await api.query.system.account(dest);
	while (balance.data.free.toNumber() == 0) {
		await awaitBlockChange(api);
		balance = await api.query.system.account(dest);
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

export const sendOpenHrmpChannelTxs = async (api: ApiPromise) => {
	const maxCapacity = 8;
	const maxMessageSize = 1048576;
	const sudo = new Keyring({ type: "sr25519" }).addFromUri("//Alice");

	const hrmpChannelCalls = [];

	hrmpChannelCalls.push(
		api.tx.hrmp.forceOpenHrmpChannel(LAOS_PARA_ID, ASSET_HUB_PARA_ID, maxCapacity, maxMessageSize)
	);
	hrmpChannelCalls.push(
		api.tx.hrmp.forceOpenHrmpChannel(ASSET_HUB_PARA_ID, LAOS_PARA_ID, maxCapacity, maxMessageSize)
	);
	await api.tx.sudo
		.sudo(api.tx.utility.batchAll(hrmpChannelCalls))
		.signAndSend(sudo)
		.catch((error: any) => {
			console.log("transaction failed", error);
		});
};

export const waitForBlockProduction = async (api: ApiPromise, name: string) => {
	console.log(`[${name}] Waiting for block production...`);
	await awaitBlockChange(api);
};

export const siblingLocation = (id: number) => ({
	parents: "1",
	interior: {
		X1: { Parachain: id },
	},
});
export const relayLocation = () => ({
	parents: "1",
	interior: {
		Here: "",
	},
});

interface XcmInstructionParams {
	api: ApiPromise;
	call: DoubleEncodedCall;
	refTime: BN;
	proofSize: BN;
	amount: BN;
	originKind: XcmOriginKind;
}

export const buildXcmInstruction = ({
	api,
	call,
	refTime,
	proofSize,
	amount,
	originKind,
}: XcmInstructionParams): XcmVersionedXcm => {
	const relayToken = api.createType("AssetIdV3", {
		Concrete: relayLocation(),
	}) as AssetIdV3;

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
			{
				Transact: {
					originKind,
					requireWeightAtMost: api.createType("WeightV2", {
						refTime,
						proofSize,
					}),
					call,
				},
			},
		],
	});
};
