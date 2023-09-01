## LAOS Bridge

Bridge between [LAOS evolution chain](https://github.com/freeverseio/laos-evolution-node) and [LAOS ownership parachain](https://github.com/freeverseio/laos-ownership-node).

For an in-depth understanding of the LAOS system, refer to the [LAOS Technical Whitepaper](https://github.com/freeverseio/laos-whitepaper/blob/main/laos.pdf), which covers all components extensively.

### Relay

`laos-relay` is used to establish bridge between LAOS chains. It currently supports two light-client based bridges: from LAOS evolution chain to LAOS ownership parachain and from [Rococo](https://substrate.io/developers/rococo-network/) relaychain to LAOS evolution chain.

It syncs latest finalized header, submits finality proof to a light client installed in the target chain and is a modified, customized version of [substrate-relay](https://github.com/paritytech/parity-bridges-common/tree/master/relays/bin-substrate). Currently it doesn't support messaging between the chains, only finality sync.

*Supported Bridges*:

- `evochain-to-ownership-parachain` - finality sync bridge from LAOS evolution chain to LAOS ownership parachain.
- `rococo-to-evochain` - finality sync bridge from Rococo relaychain to LAOS evolution chain.

*Commands*:

- `init-bridge` - initializes a bridge between two chains by registering current finalized header of source chain in target chain.
- `relay-headers` - syncs latest finalized header, submits finality proof to a light client installed in target chain.

## Build

To build `laos-relay` binary, run:

```bash
cargo build -p laos-relay
```

## Run

To run the bridge locally, you will need an instance of ownerhip parachain and evolution chain running. 

First, initialize a bridge between two chains by running:

```bash
RUST_LOG=bridge=debug \
./target/debug/laos-relay init-bridge evochain-to-ownership-parachain \
--source-host localhost \
--source-port {EVOCHAIN_WS_PORT} \
--target-host localhost \
--target-port {OWNERSHIP_PARACHAIN_WS_PORT} \
--target-signer //Alice \
--source-version-mode Bundle \
--target-version-mode Bundle
```

This will initialize a bridge between evolution chain and ownership parachain by registering current finalized header of evolution chain in ownership parachain.

Then, run the bridge by running:

```bash
RUST_LOG=bridge=debug \
./target/debug/laos-relay relay-headers evochain-to-ownership-parachain \
--source-host localhost \
--source-port {EVOCHAIN_WS_PORT} \
--target-host localhost \
--target-port {OWNERSHIP_PARACHAIN_WS_PORT} \
--target-signer //Alice \
--source-version-mode Bundle \
--target-version-mode Bundle
```

You should see logs similar to:

```bash
2023-08-24 14:48:07 +03 INFO bridge Connecting to Evochain node at ws://localhost:9944
2023-08-24 14:48:07 +03 INFO bridge Connecting to OwnershipParachain node at ws://localhost:9999
2023-08-24 14:48:07 +03 INFO bridge Exposed substrate_relay_build_info metric: version=1.0.1 commit=184d0f4-dirty
2023-08-24 14:48:07 +03 INFO bridge Starting Evochain -> OwnershipParachain finality proof relay
[Evochain_to_OwnershipParachain_Sync] 2023-08-24 14:48:07 +03 WARN bridge Evochain finality proofs stream is being started / restarted
[Evochain_to_OwnershipParachain_Sync] 2023-08-24 14:48:07 +03 INFO bridge Synced 1 of 9 headers
[Evochain_to_OwnershipParachain_Sync] 2023-08-24 14:48:17 +03 INFO bridge Synced 1 of 10 headers
[Evochain_to_OwnershipParachain_Sync] 2023-08-24 14:48:17 +03 DEBUG bridge Going to submit finality proof of Evochain header #10 to OwnershipParachain
```
