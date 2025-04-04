import { ApiPromise, WsProvider } from "@polkadot/api";
import { Keyring } from "@polkadot/api";
import Web3 from "web3";
import { sovereignAccountOf, siblingParachainLocation, relayChainLocation, substrateToEthereum } from "@utils/xcm";
import { CustomSuiteContext, XcmSuiteContext } from "@utils/types";
import {
	ZOMBIE_LAOS_NODE_IP,
	ZOMBIE_POLKADOT_NODE_IP,
	CHOPSTICKS_LAOS_NODE_IP,
	CHOPSTICKS_ASSET_HUB_NODE_IP,
	ALITH_PRIVATE_KEY,
	BALTATHAR_PRIVATE_KEY,
	FAITH_PRIVATE_KEY,
	LAOS_PARA_ID,
	ASSET_HUB_PARA_ID,
	POLKADOT_PREFIX,
	HYDRATION_PREFIX,
	HYDRATION_PARA_ID,
	CHOPSTICKS_HYDRATION_NODE_IP,
	LAOS_ID_HYDRATION,
} from "@utils/constants";

/**
 * Sets up a mocha describe environment with pre-configured utils for testing available through the 'this' variable.
 * See utils/types.ts -> CustomSuiteContext to explore all the available options
 *
 * @param {string} title - The title of the test
 * @param {() => void} cb - The test itself
 * @param {string} [providerLaosNodeIP] - An optional IP to connect with the LAOS node. By default, it's connected to
 * zombienet.
 * @param {string} [providerPolkadotNodeIP] - An optional IP to connect with the Polkadot node. By default, it's connected to zombienet.
 */
export function describeWithExistingNode(
	title: string,
	cb: () => void,
	providerLaosNodeIP?: string,
	providerPolkadotNodeIP?: string
) {
	describe(title, function (this: CustomSuiteContext) {
		before(async function () {
			this.web3 = new Web3(providerLaosNodeIP ? `http://${providerLaosNodeIP}` : `http://${ZOMBIE_LAOS_NODE_IP}`);

			let keyring = new Keyring({ type: "sr25519" });
			this.polkadotPairs = {
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

			const laosProvider = new WsProvider(
				providerLaosNodeIP ? `ws://${providerLaosNodeIP}` : `ws://${ZOMBIE_LAOS_NODE_IP}`
			);
			const apiLaos = await new ApiPromise({ provider: laosProvider }).isReady;

			const polkadotProvider = new WsProvider(
				providerPolkadotNodeIP ? `ws://${providerPolkadotNodeIP}` : `ws://${ZOMBIE_POLKADOT_NODE_IP}`
			);
			const apiPolkadot = await new ApiPromise({ provider: polkadotProvider }).isReady;

			this.chains = { laos: apiLaos, polkadot: apiPolkadot };
			this.wsProvider = laosProvider;
		});

		cb();

		after(async function () {
			this.chains.laos.disconnect();
			this.chains.polkadot.disconnect();
		});
	});
}

/**
 * Sets up a mocha describe environment with pre-configured utils for testing available through the 'this' variable.
 * See utils/types.ts -> CustomSuiteContext to explore all the available options
 *
 * @param {string} title - The title of the test
 * @param {() => void} cb - The test itself
 */
export function describeWithExistingNodeXcm(title: string, cb: () => void) {
	describe(title, function (this: XcmSuiteContext) {
		before(async function () {
			// In Xcm tests we use chopsticks and fork Paseo, which uses prefixed addresses.
			let keyring = new Keyring({ type: "sr25519", ss58Format: POLKADOT_PREFIX });
			this.polkadotPairs = {
				alice: keyring.addFromUri("//Alice"),
				bob: keyring.addFromUri("//Bob"),
				charlie: keyring.addFromUri("//Charlie"),
				dave: keyring.addFromUri("//Dave"),
				eve: keyring.addFromUri("//Eve"),
				ferdie: keyring.addFromUri("//Ferdie"),
			};

			keyring = new Keyring({ type: "sr25519", ss58Format: HYDRATION_PREFIX });
			this.hydrationPairs = { alice: keyring.addFromUri("//Alice") };

			keyring = new Keyring({ type: "ethereum" });

			this.ethereumPairs = {
				alith: keyring.addFromUri(ALITH_PRIVATE_KEY),
				baltathar: keyring.addFromUri(BALTATHAR_PRIVATE_KEY),
				faith: keyring.addFromUri(FAITH_PRIVATE_KEY),
			};

			const laosProvider = new WsProvider(`ws://${CHOPSTICKS_LAOS_NODE_IP}`);
			const apiLaos = await new ApiPromise({ provider: laosProvider }).isReady;

			const assetHubProvider = new WsProvider(`ws://${CHOPSTICKS_ASSET_HUB_NODE_IP}`);
			const apiAssetHub = await ApiPromise.create({ provider: assetHubProvider });

			const hydrationProvider = new WsProvider(`ws://${CHOPSTICKS_HYDRATION_NODE_IP}`);
			const apiHydration = await ApiPromise.create({ provider: hydrationProvider });

			this.chains = { laos: apiLaos, assetHub: apiAssetHub, hydration: apiHydration };

			this.assetHubItems = {
				accounts: {
					alice: apiAssetHub.createType("AccountId", this.polkadotPairs.alice.address),
					bob: apiAssetHub.createType("AccountId", this.polkadotPairs.bob.address),
					charlie: apiAssetHub.createType("AccountId", this.polkadotPairs.charlie.address),
					dave: apiAssetHub.createType("AccountId", this.polkadotPairs.dave.address),
					eve: apiAssetHub.createType("AccountId", this.polkadotPairs.eve.address),
					ferdie: apiAssetHub.createType("AccountId", this.polkadotPairs.ferdie.address),
				},
				laosSA: sovereignAccountOf(LAOS_PARA_ID),

				multiAddresses: {
					alice: apiAssetHub.createType("MultiAddress", this.polkadotPairs.alice.address),
					bob: apiAssetHub.createType("MultiAddress", this.polkadotPairs.bob.address),
					charlie: apiAssetHub.createType("MultiAddress", this.polkadotPairs.charlie.address),
					dave: apiAssetHub.createType("MultiAddress", this.polkadotPairs.dave.address),
					eve: apiAssetHub.createType("MultiAddress", this.polkadotPairs.eve.address),
					ferdie: apiAssetHub.createType("MultiAddress", this.polkadotPairs.ferdie.address),
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

			this.hydrationItems = {
				accounts: { alice: apiHydration.createType("AccountId", this.hydrationPairs.alice.address) },
				laosLocation: apiHydration.createType("XcmVersionedLocation", {
					V4: siblingParachainLocation(LAOS_PARA_ID),
				}),
				laosAsset: LAOS_ID_HYDRATION,
			};

			this.laosItems = {
				assetHubLocation: apiLaos.createType("XcmVersionedLocation", {
					V4: siblingParachainLocation(ASSET_HUB_PARA_ID),
				}),
				hydrationLocation: apiLaos.createType("XcmVersionedLocation", {
					v4: siblingParachainLocation(HYDRATION_PARA_ID),
				}),
				relayChainLocation: apiLaos.createType("XcmVersionedLocation", { V4: relayChainLocation() }),
				hydrationSA: substrateToEthereum(sovereignAccountOf(HYDRATION_PARA_ID)),
			};
		});

		cb();

		after(async function () {
			this.chains.laos.disconnect();
			this.chains.assetHub.disconnect();
			this.chains.hydration.disconnect();
		});
	});
}
