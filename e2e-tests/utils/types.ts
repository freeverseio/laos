import { Suite } from "mocha";
import { MultiAddress, AccountId } from "@polkadot/types/interfaces";
import { XcmVersionedLocation, StagingXcmV3MultiLocation } from "@polkadot/types/lookup";
import { KeyringPair } from "@polkadot/keyring/types";
import { ApiPromise, WsProvider } from "@polkadot/api";
import Web3 from "web3";

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

type laosItems = {
	assetHubLocation: XcmVersionedLocation;
	relayChainLocation: XcmVersionedLocation;
};

type substratePairs = {
	alice: KeyringPair;
	bob: KeyringPair;
	charlie: KeyringPair;
	dave: KeyringPair;
	eve: KeyringPair;
	ferdie: KeyringPair;
};

type ethereumPairs = {
	alith: KeyringPair;
	baltathar: KeyringPair;
	faith: KeyringPair;
};

type zombieChains = { laos: ApiPromise; polkadot: ApiPromise };

type chopsticksChains = { laos: ApiPromise; assetHub: ApiPromise };

export interface CustomSuiteContext extends Suite {
	web3: Web3;
	chains: zombieChains;
	substratePairs: substratePairs;
	ethereumPairs: ethereumPairs;
	laosItems: laosItems;
	wsProvider: WsProvider;
}

export interface XcmSuiteContext extends Suite {
	chains: chopsticksChains;
	substratePairs: substratePairs;
	ethereumPairs: ethereumPairs;
	laosItems: laosItems;
	assetHubItems: assetHubItems;
}
