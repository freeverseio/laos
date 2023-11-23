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

The quickest entry point to run your own LAOS node:
```
$ docker run freeverseio/laos-ownership-node:<release> --chain=caladan
```
The available releases are published [here](https://github.com/freeverseio/laos/releases).

# Contributing

Contributions to the LAOS Parachain project are welcome. We value the participation of each member of the community and want to ensure that everyone has an enjoyable and fulfilling experience. 

## Ways to Contribute

There are many ways to contribute to the LAOS Parachain project:

1. **Code Contributions**: If you're a developer and wish to contribute to our codebase, please check out the Github repositories detailed above. We welcome improvements to existing code and new feature developments.

2. **Bug Reports**: If you find any bugs or issues, please report them in the [Issues section](https://github.com/freeverseio/laos/issues) of our GitHub repository. Clearly describe the issue, including steps to reproduce it, and, if possible, attach screenshots.

3. **Feature Requests**: Have ideas for new features? Feel free to submit them as feature requests in the [Issues section](https://github.com/freeverseio/laos/issues). Please provide a clear and detailed explanation of the feature and its potential benefits.

4. **Documentation**: Help us improve our documentation. Whether it's a typo fix, a better explanation, or new content, we greatly appreciate any contributions to our documentation. 

## Contribution Process

1. **Fork the Repository**: Start by forking the repository you wish to contribute to.

2. **Create a Branch**: Create a new branch in your fork for your contribution.

3. **Make Your Changes**: Implement your changes, bug fixes, or feature enhancements.

4. **Test Your Changes**: Ensure your changes do not break any existing functionality.

5. **Submit a Pull Request**: Once your changes are ready, submit a pull request. Include a clear description of your changes and any relevant issue numbers.

6. **Review and Merge**: Your pull request will be reviewed by the maintainers, and, if approved, it will be merged into the project.

## Getting Help

If you need help with contributing, feel free to [open an issue](https://github.com/freeverseio/laos/issues).

We look forward to your contributions and are excited to see the impact they will have on the LAOS Parachain project!
