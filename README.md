# The LAOS Chain

LAOS is the Universal Layer 1 for digital assets across all blockchains.  
LAOS introduces unique features such as universal minting and dynamic asset evolution. This monorepo houses all the essential resources and documentation needed for building and deploying assets on the LAOS blockchain platform.

![LAOS Logo](docs/LAOS_logo.png)

LAOS's main documentation can be found here:
- **[LAOS Whitepaper](https://github.com/freeverseio/laos-whitepaper/blob/main/laos.pdf)**, for an in-depth, fully detailed description.
- **[LAOS Litepaper](https://spaces.gorengine.com/laos/LAOS_litepaper.pdf)**, for a high-level, use case oriented description.
- **[SBP Roadmap](https://github.com/freeverseio/laos-roadmap)**, the engineering roadmap within the Substrate Builders Program, ending on Q1 2024.
- **[Scrum Board](https://github.com/orgs/freeverseio/projects/3)**, the Github Project page used to coordinate the LAOS Parachain-related development.

LAOS is fully open source and decentralized. The main code repositories are:

* **[The LAOS Parachain](#the-laos-parachain-monorepo)**. You are already in it. To streamline the development process and encourage diverse contributions, all core components of the Parachain were integrated in this single monorepo.

* **[The Universal Node](https://github.com/freeverseio/laos-universal-node)**. This node streamlines the integration process for DApps aiming to incorporate bridgeless minting and evolution across various chains, including Ethereum, by merely adjusting the RPC endpoint to connect to the relevant Universal Nodes.

* **[The Universal ERC721 Solidity Templates](https://github.com/freeverseio/laos-erc721)**. This template contains the minimal extensions of the ERC721 OpenZeppelin implementation that enables bridgeless minting and evolution in any EVM chain.

LAOS is targeting late Q1'2024 to bid for a Parachain slot on Polkadot.
Presently, LAOS is accessible through its test network, named **Caladan**. Additionally, it is in the process of participating in a Parachain auction on Kusama; the Parachain will be named **KLAOS**, which stands for **K**usama LAOS. 

### Caladan Parachain: 
* Polkadot.js explorer: [wss://caladan.gorengine.com/own](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fcaladan.gorengine.com%2Fown#/explorer)
* EVM endpoint: https://caladan.gorengine.com/own
* EVM block explorer: https://caladan.gorengine.com 

### KLAOS Parachain: 
* Polkadot.js explorer: TBD
* EVM endpoint: TBD
* EVM block explorer: TBD


## The LAOS Parachain Monorepo

The LAOS Monorepo is the centralized codebase for the LAOS Parachain core components.

### [Ownership Chain](./ownership-chain/)

Ownership Chain is dedicated to the enforcement and management of asset ownership within the LAOS ecosystem. It ensures secure, transparent, and efficient tracking of asset ownership changes, serving as a cornerstone of the LAOS platform.

### [Evolution Chain](./evolution-chain/)

Evolution Chain is engineered to provide dynamic capabilities to NFTs. It enables assets to evolve and adapt over time, based on either pre-defined rules or real-time external factors. This adds a life-like quality to NFTs, making them more engaging and valuable.

### [Bridge](./bridge/)

The Bridge module acts as the communication layer between the Ownership Chain and Evolution Chain, allowing for seamless data exchange and functionality integration.
