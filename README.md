# The LAOS Chain

LAOS is the Universal Layer 1 for digital assets across all blockchains.  
LAOS introduces unique features such as universal minting and dynamic asset evolution. This monorepo houses all the essential resources and documentation needed for building and deploying assets on the LAOS blockchain platform.

![LAOS Logo](docs/LAOS_logo.png)

LAOS is fully open source and decentralized. The main code repositories are:

* **[The LAOS Parachain](#the-parachain-monorepo)**. You are already in it. To streamline the development process and encourage diverse contributions, all core components of the Parachain were integrated in this single monorepo.

* **[The Universal Node](https://github.com/freeverseio/laos-universal-node)**. This node streamlines the integration process for DApps aiming to incorporate bridgeless minting and evolution across various chains, including Ethereum, by merely adjusting the RPC endpoint to connect to the relevant Universal Nodes.

* **[The Universal ERC721 Solidity Templates](https://github.com/freeverseio/laos-erc721)**. This template contains the minimal extensions of the ERC721 OpenZeppelin implementation that enables bridgeless minting and evolution in any EVM chain.


## The LAOS Parachain Monorepo

The LAOS Monorepo is the centralized codebase for the LAOS Parachain core components.

### [Ownership Chain](./ownership-chain/)

Ownership Chain is dedicated to the enforcement and management of asset ownership within the LAOS ecosystem. It ensures secure, transparent, and efficient tracking of asset ownership changes, serving as a cornerstone of the LAOS platform.

### [Evolution Chain](./evolution-chain/)

Evolution Chain is engineered to provide dynamic capabilities to NFTs. It enables assets to evolve and adapt over time, based on either pre-defined rules or real-time external factors. This adds a life-like quality to NFTs, making them more engaging and valuable.

### [Bridge](./bridge/)

The Bridge module acts as the communication layer between the Ownership Chain and Evolution Chain, allowing for seamless data exchange and functionality integration.

## Testing Networks

#### Caladan Ownership chain: 
* Polkadot.js explorer: [wss://caladan.gorengine.com/own](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fcaladan.gorengine.com%2Fown#/explorer)
* EVM endpoint: https://caladan.gorengine.com/own
* EVM blockexplorer: https://caladan.gorengine.com 

#### Seldon Evolution chain: 
* Polkadot.js explorer: [wss://seldon.gorengine.com/evo](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fseldon.gorengine.com%2Fevo#/explorer)
* EVM endpoint: https://seldon.gorengine.com/own
* EVM blockexplorer: https://seldon.gorengine.com 

## Additional Resources

- **Whitepaper**: For an in-depth understanding, refer to our [whitepaper](https://github.com/freeverseio/laos-whitepaper/blob/main/laos.pdf).
- **Roadmap**: To explore our future plans and updates, visit our [roadmap](https://github.com/freeverseio/laos-roadmap).
