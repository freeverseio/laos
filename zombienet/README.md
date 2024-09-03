# Zombienet

The [Zombienet](https://github.com/paritytech/zombienet) configuration is located in the `native.toml` file.

Zombienet uses environment variables in the `native.toml` configuration file for easy customization. You can define these variables in your shell before running the commands. 

## How to

**Download Zombienet**

Download the Zombienet binary from the [releases page](https://github.com/paritytech/zombienet/releases)

For example:

```sh
$ wget -O zombienet https://github.com/paritytech/zombienet/releases/download/v1.3.106/zombienet-linux-x64 && chmod +x zombienet
```

**Start Zombienet**

This will launch a local Rococo relay chain with one parachain: Laos.

Before running the commands, set up the environment variables `ZOMBIENET_RELAYCHAIN_COMMAND` and `ZOMBIENET_LAOS_COMMAND`, `ZOMBIENET_ASTAR_COMMAND` in your shell. 

For example:

```sh
export ZOMBIENET_RELAYCHAIN_COMMAND=<path-to-relay-chain-executable>
export ZOMBIENET_LAOS_COMMAND=<path-to-parachain-executable>
export ZOMBIENET_ASTAR_COMMAND=<path-to-astar-executable>
```

```sh
$ ./zombienet spawn native.toml
```
