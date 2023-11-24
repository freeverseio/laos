# The LAOS Chain

LAOS is the Universal layer 1 for digital assets across all blockchains, introducing unique features such as bridgeless minting and evolution. 

![LAOS Logo](docs/LAOS_logo.png)

LAOS's main documentation can be found here:
- **[LAOS Whitepaper](https://github.com/freeverseio/laos-whitepaper/blob/main/laos.pdf)**, for an in-depth, fully detailed description.
- **[LAOS Litepaper](https://spaces.gorengine.com/laos/LAOS_litepaper.pdf)**, for a high-level, use case oriented description.
- **[SBP Roadmap](https://github.com/freeverseio/laos-roadmap)**, the engineering roadmap within the Substrate Builders Program, ending on Q1 2024.

LAOS is fully open source and decentralized. The main code repositories are:

* **[The LAOS Parachain](#the-laos-parachain-monorepo)**. You are already in it. To streamline the development process and encourage diverse contributions, all core components of the Parachain were integrated in this single monorepo.

* **[The Universal Node](https://github.com/freeverseio/laos-universal-node)**. This node streamlines the integration process for DApps aiming to incorporate bridgeless minting and evolution across various chains, including Ethereum, by merely adjusting the RPC endpoint to connect to the relevant Universal Nodes.

* **[The Universal ERC721 Solidity Templates](https://github.com/freeverseio/laos-erc721)**. This template contains the minimal extensions of the ERC721 OpenZeppelin implementation that enables bridgeless minting and evolution in any EVM chain.

LAOS is targeting late Q1'2024 to bid for a Parachain slot on Polkadot.
Presently, LAOS is accessible through its test network, named **Caladan**. Additionally, it is in the process of participating in a Parachain auction on Kusama; the Parachain will be named **KLAOS**, which stands for **K**usama LAOS. 

### Caladan Parachain: 
* Substrate RPC endpoint: [wss://caladan.gorengine.com/own](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fcaladan.gorengine.com%2Fown#/explorer)
* EVM endpoint: https://caladan.gorengine.com/own
* EVM Chain ID: 667
* EVM Currency Symbol: CLD 
* EVM block explorer: https://caladan.gorengine.com 

### KLAOS Parachain: 
* Substrate RPC endpoint: TBD
* EVM endpoint: TBD
* EVM Chain ID: TBD
* EVM Currency Symbol: KLAOS 
* EVM block explorer: TBD

## Running your own node

The quickest entry point to run your own LAOS Parachain node:
```
$ docker run freeverseio/laos-ownership-node:<release> --chain=caladan
```
The available releases are published [here](https://github.com/freeverseio/laos/releases).

# Contributing

Contributions to the LAOS Parachain project are highly appreciated. Please adhere to [Github's contribution guidelines](https://docs.github.com/en/get-started/quickstart/contributing-to-projects) to ensure a smooth collaboration process.