import BN from "bn.js";
import { AbiItem } from "web3-utils";
import AssetMetadataExtender from "../build/contracts/AssetMetadataExtender.json";
import EvolutionCollection from "../build/contracts/EvolutionCollection.json";
import EvolutionCollectionFactory from "../build/contracts/EvolutionCollectionFactory.json";

// Node config
export const RUNTIME_SPEC_NAME = "laos";
export const RUNTIME_SPEC_VERSION = 1701;
export const RUNTIME_IMPL_VERSION = 0;
export const LOCAL_NODE_URL = "http://127.0.0.1:9999";
export const NODE_URL = "wss://rpc.laossun.gorengine.com";

// Chain config
export const CHAIN_ID = 667;
export const TESTING_ACCOUNT = "0xC0F0f4ab324C46e55D02D0033343B4Be8A55532d";
export const TESTING_ACCOUNT_PRIVATE_KEY = "0xb9d2ea9a615f3165812e8d44de0d24da9bbd164b65c4f0573e1ce2c8dbd9c8df";
export const GAS_PRICE = "0x3B9ACA00";
export const ETH_BLOCK_GAS_LIMIT = 15000000; // The same configuration as runtime
export const GAS_LIMIT = ETH_BLOCK_GAS_LIMIT - 10000000;

// LAOS Evolution Contract
export const EVOLUTION_COLLETION_FACTORY_ABI = EvolutionCollectionFactory.abi as AbiItem[];
export const EVOLUTION_COLLECTION_ABI = EvolutionCollection.abi as AbiItem[];
export const CONTRACT_ADDRESS = "0x0000000000000000000000000000000000000403";
export const SELECTOR_LOG_NEW_COLLECTION = "0x5b84d9550adb7000df7bee717735ecd3af48ea3f66c6886d52e8227548fb228c";
export const SELECTOR_LOG_MINTED_WITH_EXTERNAL_TOKEN_URI =
	"0xa7135052b348b0b4e9943bae82d8ef1c5ac225e594ef4271d12f0744cfc98348";
export const SELECTOR_LOG_EVOLVED_WITH_EXTERNAL_TOKEN_URI =
	"0xdde18ad2fe10c12a694de65b920c02b851c382cf63115967ea6f7098902fa1c8";
export const SELECTOR_LOG_OWNERSHIP_TRANSFERRED =
	"0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0";
export const SELECTOR_LOG_PUBLIC_MINTING_ENABLED =
	"0x8ff3deee4c40ab085dd8d7d0c848cb5295e4ab5faa32e5b60e3936cf1bdc77bf";
export const SELECTOR_LOG_PUBLIC_MINTING_DISABLED =
	"0xebe230014056e5cb4ca6d8e534189bf5bfb0759489f16170654dce7c014b6699";

// Asset Metadata Extender Contract
export const ASSET_METADATA_EXTENDER_ADDRESS = "0x0000000000000000000000000000000000000405";
export const ASSET_METADATA_EXTENDER_ABI = AssetMetadataExtender.abi as AbiItem[];
export const SELECTOR_LOG_EXTENDED_UL_WITH_EXTERNAL_URI =
	"0xf744da499cb735a8fc987aa2a331a1cbeca79e449e4c04eeccfe57c538e79070";
export const SELECTOR_LOG_UPDATED_EXTENDED_UL_WITH_EXTERNAL_URI =
	"0xe7ebe38355126fe0c3eab0ec03eb1b94ff501458a80713c9eb8b737334a651ff";

// Constants
export const MAX_U96 = new BN("79228162514264337593543950336"); // 2^96 - 1
export const REVERT_BYTECODE = "0x60006000fd";
