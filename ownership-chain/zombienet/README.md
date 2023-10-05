# Zombienet

The [Zombienet](https://github.com/paritytech/zombienet) configuration is located in the `native.toml` file.

Zombienet uses environment variables in the `native.toml` configuration file for easy customization. You can define these variables in your shell before running the commands. 

## How to

**Download Zombienet**

Download the Zombienet binary from the [releases page](https://github.com/paritytech/zombienet/releases)

For example:

```sh
$ wget -O zombienet https://github.com/paritytech/zombienet/releases/download/v1.3.55/zombienet-linux-x64 && chmod +x zombienet
```

**Start Zombienet**

Before running the commands, set up the environment variables `ZOMBIENET_RELAYCHAIN_COMMAND`, `ZOMBIENET_PARACHAIN_COMMAND` and `RAW_CHAIN_SPEC` in your shell. 

We need to explicitly pass `RAW_CHAIN_SPEC` to disable Zombienet from modifying the chain spec. This will be the work around until [this issue](https://github.com/freeverseio/laos/issues/25) is resolved.

For example:

```sh
export ZOMBIENET_RELAYCHAIN_COMMAND=<path-to-relay-chain-executable>
export ZOMBIENET_PARACHAIN_COMMAND=<path-to-parachain-executable>
export RAW_CHAIN_SPEC=<path-to-chain-spec>
```


```sh
$ ./zombienet spawn native.toml -p native
```
