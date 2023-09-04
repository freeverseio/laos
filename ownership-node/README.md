# LAOS Ownership Parachain

For an in-depth understanding of the LAOS system, refer to the [LAOS Technical Whitepaper](https://github.com/freeverseio/laos-whitepaper/blob/main/laos.pdf), which covers all components extensively.

The LAOS Ownership Parachain is a specialized chain built on Polkadot. 

It offers several functionalities that enable the management and transfer of LAOS native utility tokens, as well as the ownership of all LA (Living Assets) created directly within LAOS. Additionally, the Parachain handles the runtime upgrades and stores state roots of Evolution Chains (Evochains), providing asset attribute certification methods and rewarding Evochain validators upon receiving new correct roots. 

## Run your own Node

You can start and sync ownership node locally with the following command:
```
$ docker run freeverseio/laos-ownership-node:<release> --chain=arrakis
```

## Networks
### Arrakis (testnet)

The Arrakis network serves as the testnet for the LAOS Ownership Parachain. It can be accessed and interacted with using either the Substrate RPC (Polkadot JS extension) or the Ethereum RPC wallet (Metamask).

#### Substrate RPC
* **RPC URL**: wss://arrakis.gorengine.com/own

#### Ethereum RPC
* **Network ID**: Arrakis
* **Chain ID**: 667
* **RPC URL**: https://arrakis.gorengine.com/own
* **Currency Symbol**: DROP

## Contributing

Contributions to the LAOS Ownership Parachain project are welcome.
