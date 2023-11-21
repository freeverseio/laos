import BN from "bn.js";
import { AbiItem } from "web3-utils";
import EvolutionCollection from "../build/contracts/EvolutionCollection.json";
import EvolutionCollectionFactory from "../build/contracts/EvolutionCollectionFactory.json";

// Node config
export const RUNTIME_SPEC_NAME = "frontier-template";
export const RUNTIME_SPEC_VERSION = 10;
export const RUNTIME_IMPL_VERSION = 0;
export const RPC_PORT = 9999;

// Chain config
export const CHAIN_ID = 667;
export const OWNCHAIN_SUDO = "0xf24FF3a9CF04c71Dbc94D0b566f7A27B94566cac";
export const OWNCHAIN_SUDO_PRIVATE_KEY = "0x5fb92d6e98884f76de468fa3f6278f8807c48bebc13595d45af5bdc4da702133";
export const GENESIS_ACCOUNT = "0xC0F0f4ab324C46e55D02D0033343B4Be8A55532d";
export const GENESIS_ACCOUNT_PRIVATE_KEY = "0xb9d2ea9a615f3165812e8d44de0d24da9bbd164b65c4f0573e1ce2c8dbd9c8df";
export const GENESIS_ACCOUNT_BALANCE = "77559934324363988853790420524572160";
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

// Constants
export const MAX_U96 = new BN("79228162514264337593543950336"); // 2^96 - 1
export const REVERT_BYTECODE = "0x60006000fd";
