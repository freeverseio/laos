import { Suite } from "mocha";
import { MultiAddress, AccountId } from "@polkadot/types/interfaces";
import { XcmVersionedLocation, StagingXcmV3MultiLocation } from "@polkadot/types/lookup";
import { KeyringPair } from "@polkadot/keyring/types";
import { ApiPromise, WsProvider } from "@polkadot/api";
import Web3 from "web3";
import BN from "bn.js";

type assetHubItems = {
	accounts: {
		alice: AccountId;
		bob: AccountId;
		charlie: AccountId;
		dave: AccountId;
		eve: AccountId;
		ferdie: AccountId;
	};
	multiAddresses: {
		alice: MultiAddress;
		bob: MultiAddress;
		charlie: MultiAddress;
		dave: MultiAddress;
		eve: MultiAddress;
		ferdie: MultiAddress;
		laosSA: MultiAddress;
	};
	laosSA: string;
	laosLocation: XcmVersionedLocation;
	laosAsset: StagingXcmV3MultiLocation;
	relayChainLocation: XcmVersionedLocation;
	relayAsset: StagingXcmV3MultiLocation;
};

type moonbeamItems = {
	laosLocation: XcmVersionedLocation;
	laosAsset: BN;
};

type hydrationItems = {
	accounts: {
		alice: AccountId;
	};
	laosLocation: XcmVersionedLocation;
	laosAsset: BN;
};

type laosItems = {
	assetHubLocation: XcmVersionedLocation;
	moonbeamLocation: XcmVersionedLocation;
	hydrationLocation: XcmVersionedLocation;
	relayChainLocation: XcmVersionedLocation;
	moonbeamSA: string;
	hydrationSA: string;
};

type polkadotPairs = {
	alice: KeyringPair;
	bob: KeyringPair;
	charlie: KeyringPair;
	dave: KeyringPair;
	eve: KeyringPair;
	ferdie: KeyringPair;
};

type hydrationPairs = {
	alice: KeyringPair;
};

type ethereumPairs = {
	alith: KeyringPair;
	baltathar: KeyringPair;
	faith: KeyringPair;
};

type zombieChains = { laos: ApiPromise; polkadot: ApiPromise };

type chopsticksChains = { laos: ApiPromise; assetHub: ApiPromise; moonbeam: ApiPromise; hydration: ApiPromise };

export interface CustomSuiteContext extends Suite {
	web3: Web3;
	chains: zombieChains;
	polkadotPairs: polkadotPairs;
	ethereumPairs: ethereumPairs;
	laosItems: laosItems;
	wsProvider: WsProvider;
}

export interface XcmSuiteContext extends Suite {
	chains: chopsticksChains;
	polkadotPairs: polkadotPairs;
	hydrationPairs: hydrationPairs;
	ethereumPairs: ethereumPairs;
	laosItems: laosItems;
	assetHubItems: assetHubItems;
	moonbeamItems: moonbeamItems;
	hydrationItems: hydrationItems;
}
