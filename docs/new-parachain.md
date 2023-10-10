## Spawn new parachain

This guide describes starting a new parachain from scratch in an environment where you control your own relay chain.

### Caladan chain spec

In [ownership node](../ownership-chain/) there is a chain [spec](../ownership-chain/specs/caladan-spec.json) that defines the genesis of the chain. This chain spec was generated from the latest binary of the Ownership parachain.

It currently contains some keys and values that are from the development environment. For example, `Aura`, `Session` pallets are using `Alice` accounts.

Generate new key for your node, any mnemonic seed would work. You can use [subkey tool](https://docs.substrate.io/reference/command-line-tools/subkey/) to generate a new key.

```bash
> subkey generate
Secret phrase `budget detail feel rebel trash visit appear royal erode two tent catch` is account:
  Secret seed:       0xcb99c315d136e1f5d051c1d151731eac6f9e9a79ec2c4293abde7c4de856b9ae
  Public key (hex):  0x035bedaef6290169b23e66bcef33d27bcd9e91b07d443e517842c7245b5735faf0
  Public key (SS58): KWAnqxtpneH94rTf2prSKPYyf531enezcj6z9VpE2Rmt1aL1e
  Account ID:        0x6997754397fc72ce678d3a32ab4e378b64b03bd5c39ee747cce2502b4d5ce2a6
  SS58 Address:      5ET9sr9q7TEAj6srZyRbe8jqi8kek5o6g7ynVpCAmgk7xkBB
```

Here, we will use `SS58 Address` as `AuraId` in the chain spec.

Use `Polkadot.js` to get `AccountId20` from the above mnemonic seed.

And now replace all mentions of the following `AccountId20` in the `caladan-spec.json` file `0xf24ff3a9cf04c71dbc94d0b566f7a27b94566cac` with the new `AccountId20` that you generated.

The resulting json would look like this, if we used the account generated above.

```json
      "collatorSelection": {
        "invulnerables": ["0x2F59d178522380955F3b9EA1204F4fB242432Ef4"],
        "candidacyBond": 16000000000,
        "desiredCandidates": 0
      },
      "session": {
        "keys": [
          [
            "0x2F59d178522380955F3b9EA1204F4fB242432Ef4",
            "0x2F59d178522380955F3b9EA1204F4fB242432Ef4",
            {
              "aura": "5ET9sr9q7TEAj6srZyRbe8jqi8kek5o6g7ynVpCAmgk7xkBB"
            }
          ]
        ]
      },
```

#### Convert chain spec to raw format

```bash
./target/release/laos-ownership build-spec --chain=specs/caladan-spec.json --raw --disable-default-bootnode > specs/caladan-raw.json
```

### Prepare genesis artifacts

IMPORTANT: Make sure to use `--chain` flag and point to the `caladan-raw.json` file that you generated in the previous step.

#### Generate genesis state

```bash
./target/release/laos-ownership export-genesis-state --chain=specs/caladan-raw.json > genesis-state
```

#### Extract genesis wasm

```bash
./target/release/laos-ownership export-genesis-wasm --chain=specs/caladan-raw.json > genesis-wasm
```

### Start the parachain

```bash
./target/release/laos-ownership --chain specs/caladan-raw.json --rpc-port 9999 --base-path test-chain --unsafe-rpc-external -- --chain specs/rococo-freeverse-chainspec.json
```

This will start the new parachain but it won't produce blocks yet. You will first need to register the parachain on the relay chain and insert your Aura keys.

### Insert Aura keys

Once the parachain is started, you will need to insert the Aura keys. You can do this by connecting to the parachain in `Polkadot.js` and in the `RPC` tab.


## Register parachain on relay chain

There are two ways to register a parachain on the relay chain. Both require you to dispatch sudo extrinsic.

### 1.Register as parathread and upgrade to parachain

This creates a `para_id`. Since the parachain above uses `para_id: 2001`, we need to generate `ParaId` twice to get the `2001` id of the parachain.


After this, you need to create the parathread:

Once this extrinsic is included in a block, you will see that parathread is onboarding:

And when the parathread is onboarded, you will need to dispatch this call with the genesis artifacts that you generated above:

### 2. Use `paraSudoWrapper.sudoScheduleParaInitialize` to register parachain

This one is more straightforward and you only need one sudo extrinsic to register the parachain.
