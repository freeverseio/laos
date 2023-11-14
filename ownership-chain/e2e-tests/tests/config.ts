import BN from "bn.js";
import { AbiItem } from "web3-utils";
import LaosEvolution from "../build/contracts/LaosEvolution.json";

// Node config
export const RUNTIME_SPEC_NAME = "frontier-template";
export const RUNTIME_SPEC_VERSION = 8;
export const RUNTIME_IMPL_VERSION = 0;
export const RPC_PORT = 9999;

// Other nodes
export const ASTAR_RPC_PORT = 9998;
export const ROCOCO_RPC_PORT = 9944;

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
export const GAS = "0x10000"; 

// Other nodes
export const ASTAR_SUDO = "ajYMsCKsEAhEvHpeA4XqsfiA9v1CdzZPrCfS6pEfeGHW9j8";

// LAOS Evolution Contract
export const LAOS_EVOLUTION_ABI = LaosEvolution.abi as AbiItem[];
export const CONTRACT_ADDRESS = "0x0000000000000000000000000000000000000403";
export const SELECTOR_LOG_NEW_COLLECTION = "0x5b84d9550adb7000df7bee717735ecd3af48ea3f66c6886d52e8227548fb228c";
export const SELECTOR_LOG_MINTED_WITH_EXTERNAL_TOKEN_URI = "0x4b3b5da28a351f8bb73b960d7c80b2cef3e3570cb03448234dee173942c74786";
export const SELECTOR_LOG_EVOLVED_WITH_EXTERNAL_TOKEN_URI = "0x95c167d04a267f10e6b3f373c7a336dc65cf459caf048854dc32a2d37ab1607c";

// Constants
export const MAX_U96 = new BN("79228162514264337593543950336"); // 2^96 - 1
