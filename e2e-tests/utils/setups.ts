import { ApiPromise, WsProvider } from "@polkadot/api";
import { Keyring } from "@polkadot/api";
import Web3 from "web3";
import { sovereignAccountOf, siblingParachainLocation, relayChainLocation } from "@utils/xcm";
import { CustomSuiteContext, XcmSuiteContext } from "@utils/types";
import {
	ZOMBIE_LAOS_NODE_IP,
	CHOPSTICKS_LAOS_NODE_IP,
	CHOPSTICKS_ASSET_HUB_NODE_IP,
	ALITH_PRIVATE_KEY,
	BALTATHAR_PRIVATE_KEY,
	FAITH_PRIVATE_KEY,
	LAOS_PARA_ID,
	ASSET_HUB_PARA_ID,
	POLKADOT_PREFIX,
} from "@utils/constants";

/**
 * Sets up a mocha describe environment with pre-configured utils for testing available through the 'this' variable.
 * See utils/types.ts -> CustomSuiteContext to explore all the available options
 *
 * @param {string} title - The title of the test
 * @param {() => void} cb - The test itself
 * @param {string} [providerLaosNodeIP] - An optional IP to connect with the LAOS node. By default, it's connected to
 * zombienet.
 */
export function describeWithExistingNode(title: string, cb: () => void, providerLaosNodeIP?: string) {
	describe(title, function (this: CustomSuiteContext) {
		before(async function () {
			this.web3 = new Web3(providerLaosNodeIP ? `http://${providerLaosNodeIP}` : `http://${ZOMBIE_LAOS_NODE_IP}`);

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

			let provider = new WsProvider(
				providerLaosNodeIP ? `ws://${providerLaosNodeIP}` : `ws://${ZOMBIE_LAOS_NODE_IP}`
			);
			const apiLaos = await new ApiPromise({ provider }).isReady;

			this.chains = { laos: apiLaos };
			this.wsProvider = provider;
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
 * @param {string} [providerLaosNodeIP] - An optional IP to connect with the LAOS node. By default, it's connected to chopsticks
 * @param {string} [providerAssetHubNodeIP] - An optional IP to connect with the Asset Hub node. By default, it's connected
 * to chopsticks.
 */
export function describeWithExistingNodeXcm(
	title: string,
	cb: () => void,
	providerLaosNodeIP?: string,
	providerAssetHubNodeIP?: string
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

			const laosProvider = new WsProvider(
				providerLaosNodeIP ? `ws://${providerLaosNodeIP}` : `ws://${CHOPSTICKS_LAOS_NODE_IP}`
			);
			const apiLaos = await new ApiPromise({ provider: laosProvider }).isReady;

			const assetHubProvider = new WsProvider(
				providerAssetHubNodeIP ? `ws://${providerAssetHubNodeIP}` : `ws://${CHOPSTICKS_ASSET_HUB_NODE_IP}`
			);
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
					V4: siblingParachainLocation(LAOS_PARA_ID),
				}),
				laosAsset: apiAssetHub.createType("StagingXcmV4Location", siblingParachainLocation(LAOS_PARA_ID)),
				relayChainLocation: apiAssetHub.createType("XcmVersionedLocation", {
					V4: relayChainLocation(),
				}),
				relayAsset: apiAssetHub.createType("StagingXcmV4Location", relayChainLocation()),
			};

			this.assetHubItems.multiAddresses.laosSA = apiAssetHub.createType(
				"MultiAddress",
				this.assetHubItems.laosSA
			);
			this.laosItems = {
				assetHubLocation: apiLaos.createType("XcmVersionedLocation", {
					V4: siblingParachainLocation(ASSET_HUB_PARA_ID),
				}),
				relayChainLocation: apiLaos.createType("XcmVersionedLocation", { V4: relayChainLocation() }),
			};
		});

		cb();

		after(async function () {
			this.chains.laos.disconnect();
			this.chains.assetHub.disconnect();
		});
	});
}
