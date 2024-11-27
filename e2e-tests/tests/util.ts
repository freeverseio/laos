import Contract from "web3-eth-contract";
import Web3 from "web3";
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
import { Keyring } from "@polkadot/api";
import { sovereignAccountOf } from "@utils/xcm";
import { siblingParachainLocation, relayChainLocation } from "@utils/xcm";

import debug from "debug";
const debugTx = debug("transaction");

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
			this.web3 = new Web3(providerLaosNodeUrl || "http://" + LAOS_NODE_IP);

			let keyring = new Keyring({ type: "sr25519" });
			this.substratePairs = {
				alice: keyring.addFromUri("//Alice"),
				bob: keyring.addFromUri("//Bob"),
				charlie: keyring.addFromUri("//Charlie"),
				dave: keyring.addFromUri("//Dave"),
				eve: keyring.addFromUri("//Eve"),
				ferdie: keyring.addFromUri("//Ferdie"),
			};

			keyring = new Keyring({ type: "ethereum" });

			this.ethereumPairs = {
				alith: keyring.addFromUri(ALITH_PRIVATE_KEY),
				baltathar: keyring.addFromUri(BALTATHAR_PRIVATE_KEY),
				faith: keyring.addFromUri(FAITH_PRIVATE_KEY),
			};
			this.web3.eth.accounts.wallet.add(ALITH_PRIVATE_KEY);
			this.web3.eth.accounts.wallet.add(BALTATHAR_PRIVATE_KEY);
			this.web3.eth.accounts.wallet.add(FAITH_PRIVATE_KEY);

			if (openPolkadotConnections) {
				let provider = new WsProvider(providerLaosNodeUrl || "ws://" + LAOS_NODE_IP);
				const apiLaos = await new ApiPromise({ provider }).isReady;

				provider = new WsProvider(providerAssetHubNodeUrl || "ws://" + ASSET_HUB_NODE_IP);
				const apiAssetHub = await ApiPromise.create({ provider: provider });

				provider = new WsProvider(providerRelaychainNodeUrl || "ws://" + RELAYCHAIN_NODE_IP);
				const apiRelay = await new ApiPromise({ provider: provider }).isReady;

				this.chains = { laos: apiLaos, assetHub: apiAssetHub, relaychain: apiRelay };

				this.assetHubItems = {
					accounts: {
						alice: apiAssetHub.createType("AccountId", this.substratePairs.alice.address),
						bob: apiAssetHub.createType("AccountId", this.substratePairs.bob.address),
						charlie: apiAssetHub.createType("AccountId", this.substratePairs.charlie.address),
						dave: apiAssetHub.createType("AccountId", this.substratePairs.dave.address),
						eve: apiAssetHub.createType("AccountId", this.substratePairs.eve.address),
						ferdie: apiAssetHub.createType("AccountId", this.substratePairs.ferdie.address),
					},
					laosSA: sovereignAccountOf(LAOS_PARA_ID),

					multiAddresses: {
						alice: apiAssetHub.createType("MultiAddress", this.substratePairs.alice.address),
						bob: apiAssetHub.createType("MultiAddress", this.substratePairs.bob.address),
						charlie: apiAssetHub.createType("MultiAddress", this.substratePairs.charlie.address),
						dave: apiAssetHub.createType("MultiAddress", this.substratePairs.dave.address),
						eve: apiAssetHub.createType("MultiAddress", this.substratePairs.eve.address),
						ferdie: apiAssetHub.createType("MultiAddress", this.substratePairs.ferdie.address),
					},
					laosLocation: apiAssetHub.createType("XcmVersionedLocation", {
						V3: siblingParachainLocation(LAOS_PARA_ID),
					}),
					laosAsset: apiAssetHub.createType(
						"StagingXcmV3MultiLocation",
						siblingParachainLocation(LAOS_PARA_ID)
					),
					relayChainLocation: apiAssetHub.createType("XcmVersionedLocation", {
						V3: relayChainLocation(),
					}),
					relayAsset: apiAssetHub.createType("StagingXcmV3MultiLocation", relayChainLocation()),
				};

				(this.assetHubItems.multiAddresses.laosSA = apiAssetHub.createType(
					"MultiAddress",
					this.assetHubItems.laosSA
				)),
					(this.laosItems = {
						assetHubLocation: apiLaos.createType("XcmVersionedLocation", {
							V3: siblingParachainLocation(ASSET_HUB_PARA_ID),
						}),
						relayChainLocation: apiLaos.createType("XcmVersionedLocation", { V3: relayChainLocation() }),
					});
			}
		});

		cb();

		after(async function () {
			if (openPolkadotConnections) {
				this.chains.laos.disconnect();
				this.chains.assetHub.disconnect();
				this.chains.relaychain.disconnect();
			}
		});
	});
}

export async function createCollection(web3: Web3): Promise<Contract> {
	const contract = new web3.eth.Contract(
		EVOLUTION_COLLECTION_FACTORY_ABI,
		EVOLUTION_COLLECTION_FACTORY_CONTRACT_ADDRESS,
		{
			from: FAITH,
			gasPrice: GAS_PRICE,
		}
	);

	let nonce = await web3.eth.getTransactionCount(FAITH);
	web3.eth.accounts.wallet.add(FAITH_PRIVATE_KEY);
	const estimatedGas = await contract.methods.createCollection(FAITH).estimateGas();
	const result = await contract.methods.createCollection(FAITH).send({
		from: FAITH,
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

export async function waitFinalizedEthereumTx(web3: Web3, api: ApiPromise, txHash: string) {
	try {
		while (true) {
			const receipt = await web3.eth.getTransactionReceipt(txHash);
			if (receipt && receipt.blockNumber) {
				const finalizedBlock = (
					await api.rpc.chain.getBlock(await api.rpc.chain.getFinalizedHead())
				).block.header.number.toNumber();
				if (finalizedBlock >= receipt.blockNumber) {
					return;
				}
			}
			// Polling to avoid querying the block numbers so frequently cause they aren't produced that fast
			setTimeout(() => {}, 2000);
		}
	} catch (error) {
		debugTx(`Error waiting for confirmations of transaction ${txHash}:`, error);
		throw error;
	}
}
