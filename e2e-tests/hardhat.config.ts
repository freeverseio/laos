import { HardhatUserConfig } from "hardhat/config";
import "@nomicfoundation/hardhat-toolbox";

const config: HardhatUserConfig = {
  solidity: "0.8.3",
  paths: {
    artifacts: "build",
    tests: "hardhat-tests", // WIP this is not working
  },
  mocha: {
    timeout: 20000,
  },
  networks:{
    zombienet: {
      url: 'http://127.0.0.1:9999',
      chainId: 667, // The Chain ID of your custom network
      gas: 'auto', // Gas settings
      gasPrice: 'auto', // Gas price settings
      accounts: [`0x5fb92d6e98884f76de468fa3f6278f8807c48bebc13595d45af5bdc4da702133`],
    },
  }
};

export default config;
