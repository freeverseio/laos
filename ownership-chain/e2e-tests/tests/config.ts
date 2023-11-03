import LaosEvolution from "../build/contracts/LaosEvolution.json";
import { AbiItem } from "web3-utils";

// Node config
export const NODE_BINARY_NAME = "laos-ownership";
export const RUNTIME_SPEC_NAME = "frontier-template";
export const RUNTIME_SPEC_VERSION = 7;
export const RUNTIME_IMPL_VERSION = 0;

// Chain config
export const CHAIN_ID = 667;
export const GENESIS_ACCOUNT = "0xC0F0f4ab324C46e55D02D0033343B4Be8A55532d";
export const GENESIS_ACCOUNT_PRIVATE_KEY = "0xb9d2ea9a615f3165812e8d44de0d24da9bbd164b65c4f0573e1ce2c8dbd9c8df";
export const GENESIS_ACCOUNT_BALANCE = "77559934324363988854052928524572160";
export const GAS_PRICE = "0x3B9ACA00";
export const GAS = "0x10000";

// LAOS Evolution Contract
export const LAOS_EVOLUTION_ABI = LaosEvolution.abi as AbiItem[]
export const CONTRACT_ADDRESS = "0x0000000000000000000000000000000000000403";
export const SELECTOR_LOG_NEW_COLLECTION = "0x6eb24fd767a7bcfa417f3fe25a2cb245d2ae52293d3c4a8f8c6450a09795d289";
export const SELECTOR_LOG_MINTED_WITH_EXTERNAL_TOKEN_URI = "0x4b3b5da28a351f8bb73b960d7c80b2cef3e3570cb03448234dee173942c74786";
export const SELECTOR_LOG_EVOLVED_WITH_EXTERNAL_TOKEN_URI = "0x568b059e9377ea804907ac57dc8d56446b17dbf9f4b30dfe1935b9c8815ae7e1";


