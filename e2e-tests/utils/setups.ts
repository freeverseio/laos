import { ApiPromise, WsProvider } from "@polkadot/api";
import { Keyring } from "@polkadot/api";
import Web3 from "web3";
import { sovereignAccountOf, siblingParachainLocation, relayChainLocation } from "@utils/xcm";
import { CustomSuiteContext, XcmSuiteContext } from "@utils/types";
import {
	LAOS_NODE_IP,
	ALITH_PRIVATE_KEY,
	BALTATHAR_PRIVATE_KEY,
	FAITH_PRIVATE_KEY,
	LAOS_PARA_ID,
	ASSET_HUB_PARA_ID,
	XCM_LAOS_NODE_IP,
	XCM_ASSET_HUB_NODE_IP,
	POLKADOT_PREFIX,
} from "@utils/constants";

/**
 * Sets up a mocha describe environment with pre-configured utils for testing available through the 'this' variable.
 * See utils/types.ts -> CustomSuiteContext to explore all the available options
 *
 * @param {string} title - The title of the test
 * @param {() => void} cb - The test itself
 * @param {string} [providerLaosNodeUrl] - An optional URL to connect with the LAOS node
 */
export function describeWithExistingNode(title: string, cb: () => void, providerLaosNodeUrl?: string) {
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

			let provider = new WsProvider(providerLaosNodeUrl || "ws://" + LAOS_NODE_IP);
			const apiLaos = await new ApiPromise({ provider }).isReady;

			this.chains = { laos: apiLaos };
		});

		cb();

		after(async function () {
			this.chains.laos.disconnect();
		});
	});
}

/**
 * Sets up a mocha describe environment with pre-configured utils for testing available through the 'this' variable.
 * See utils/types.ts -> CustomSuiteContext to explore all the available options
 *
 * @param {string} title - The title of the test
 * @param {() => void} cb - The test itself
 * @param {string} [providerLaosNodeUrl] - An optional URL to connect with the LAOS node
 * @param {string} [providerAssetHubNodeUrl] - An optional URL to connect with the Asset Hub node
 */
export function describeWithExistingNodeXcm(
	title: string,
	cb: () => void,
	providerLaosNodeUrl?: string,
	providerAssetHubNodeUrl?: string
) {
	describe(title, function (this: XcmSuiteContext) {
		before(async function () {
			// In Xcm tests we use chopsticks and fork Paseo, which uses prefixed addresses.
			let keyring = new Keyring({ type: "sr25519", ss58Format: POLKADOT_PREFIX });
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

			const laosProvider = new WsProvider(providerLaosNodeUrl || "ws://" + XCM_LAOS_NODE_IP);
			const apiLaos = await new ApiPromise({ provider: laosProvider }).isReady;

			const assetHubProvider = new WsProvider(providerAssetHubNodeUrl || "ws://" + XCM_ASSET_HUB_NODE_IP);
			const apiAssetHub = await ApiPromise.create({ provider: assetHubProvider });

			this.chains = { laos: apiLaos, assetHub: apiAssetHub };

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
				laosAsset: apiAssetHub.createType("StagingXcmV3MultiLocation", siblingParachainLocation(LAOS_PARA_ID)),
				relayChainLocation: apiAssetHub.createType("XcmVersionedLocation", {
					V3: relayChainLocation(),
				}),
				relayAsset: apiAssetHub.createType("StagingXcmV3MultiLocation", relayChainLocation()),
			};

			this.assetHubItems.multiAddresses.laosSA = apiAssetHub.createType(
				"MultiAddress",
				this.assetHubItems.laosSA
			);
			this.laosItems = {
				assetHubLocation: apiLaos.createType("XcmVersionedLocation", {
					V3: siblingParachainLocation(ASSET_HUB_PARA_ID),
				}),
				relayChainLocation: apiLaos.createType("XcmVersionedLocation", { V3: relayChainLocation() }),
			};
		});

		cb();

		after(async function () {
			this.chains.laos.disconnect();
			this.chains.assetHub.disconnect();
		});
	});
}
