# The LAOS Chain

LAOS is the Universal layer 1 for digital assets across all blockchains, introducing unique features such as bridgeless minting and evolution. 

![LAOS Logo](docs/LAOS_logo.png)

LAOS's main documentation can be found here:
- **[LAOS Whitepaper](https://github.com/freeverseio/laos-whitepaper/blob/main/laos.pdf)**, for an in-depth, fully detailed description.
- **[LAOS Developer Docs](https://docs.laosnetwork.io/)**, to start building your DApp using LAOS.
- **[LAOS Litepaper](https://laosnetwork.io/downloads/LAOS_litepaper.pdf)**, for a high-level, use case oriented description.

LAOS is fully open source and decentralized. The main code repositories are:

* **[The LAOS Parachain](#the-laos-parachain-monorepo)**. You are already in it. To streamline the development process and encourage diverse contributions, all core components of the Parachain were integrated in this single monorepo.

* **[The Universal Node](https://github.com/freeverseio/laos-universal-node)**. This node streamlines the integration process for DApps aiming to incorporate bridgeless minting and evolution across various chains, including Ethereum, by merely adjusting the RPC endpoint to connect to the relevant Universal Nodes.

* **[The Universal ERC721 Solidity Templates](https://github.com/freeverseio/laos-erc721)**. This template contains the minimal extensions of the ERC721 OpenZeppelin implementation that enables bridgeless minting and evolution in any EVM chain.

LAOS secured a slot as a Parachain in Polkadot after winning auction 68,
and it is targeting July 11th to start producing blocks.

### LAOS Testnets

LAOS currently has two testnets, named *KLAOS Nova* and *LAOS Omega*. 
After launching to mainnet, KLAOS Nova will eventually be deprecated,
leaving *LAOS Omega* as the only officially supported testnet.

KLAOS Nova is the testnet aimed at building your DApp, leveraging bridgeless minting, and all other EVM functionalities.

* Testnet: **KLAOS Nova**
* EVM Public RPC endpoint: https://rpc.klaosnova.laosfoundation.io
* EVM Chain ID: 27181
* EVM Currency Symbol: KLAOS 
* EVM block explorer: https://blockscout.klaosnova.laosfoundation.io
* Substrate RPC endpoint: [wss://rpc.klaos.laosfoundation.io](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Frpc.klaosnova.laosfoundation.io#/rpc)   
* ParaId: 2001
* EVM PalletInstance: 51

LAOS Omega is the testnet where all development undergoes final testing before being integrated in LAOS mainnet. Currently, LAOS Omega can be used to test staking functionality. Token transfers and EVM functionalities will be soon integrated.

* Testnet: **LAOS Omega**
* Substrate RPC endpoint: [wss://rpc.laosomega.laosfoundation.io](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Frpc.laosomega.laosfoundation.io#/rpc)   
* ParaId: 4006
* EVM Chain ID: 62831


### LAOS Parachain: 
* Substrate RPC endpoint: [wss://rpc.laos.laosfoundation.io](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Frpc.laos.laosfoundation.io#/rpc)   
* ParaId: 3370
* EVM Chain ID: 6283

## Running your own node

The quickest entry point to run your own LAOS Parachain node:
```
$ docker run freeverseio/laos-node:<release> --chain=<chain_name>
```
where:
* `<chain_name>` shall be set to `klaosnova` or `laosomega` to operate on the testnets, and
* `<release>` shall be chosen among the available releases published [here](https://github.com/freeverseio/laos/releases).

# Contributing

Contributions to the LAOS Parachain project are highly appreciated. Please adhere to [GitHub's contribution guidelines](https://docs.github.com/en/get-started/quickstart/contributing-to-projects) to ensure a smooth collaboration process.

For detailed implementation assistance, please engage with the development team on the official [LAOS Discord server](https://discord.gg/5YX9DHda).
