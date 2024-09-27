# Zombienet
The [Zombienet](https://github.com/paritytech/zombienet) configurations are located under the `zombienet` folder.

Zombienet uses environment variables in the configuration files for easy customization. You can define these variables in your shell before running the commands.

## How to

### Download Zombienet
Download the Zombienet binary from the [releases page](https://github.com/paritytech/zombienet/releases). For example:

```sh
$ wget -O zombienet https://github.com/paritytech/zombienet/releases/download/v1.3.106/zombienet-linux-x64 && chmod +x zombienet
```

### Run Zombienet
Before spawning zombienet, set up the environment variables `ZOMBIENET_RELAYCHAIN_COMMAND` and `ZOMBIENET_LAOS_COMMAND` in a terminal session or in your shell configuration file (e.g. `~/.bashrc`, `~/.zsh.rc`):

```sh
export ZOMBIENET_RELAYCHAIN_COMMAND=<path-to-polkadot-executable>
export ZOMBIENET_LAOS_COMMAND=<path-to-laos-executable>
```

To run the relay chain locally, you need the `polkadot` binary. Either download it from the [polkadot-sdk releases tab](https://github.com/paritytech/polkadot-sdk/releases), or clone the [polkadot-sdk repo](https://github.com/paritytech/polkadot-sdk/) and compile the project with the command:
```sh
cargo build --release --locked
```

#### Parchain-only
This a local Rococo relay chain with one parachain: Laos.

```sh
$ zombienet spawn zombienet/native.toml
```

#### Parachain and AssetHub
Use this configuration in case you need to test XCM. This a local Rococo relay chain with two parachains: Laos and AssetHub.

To run AssetHub locally, you need the `polkadot-parachain` binary. Either download it from the [polkadot-sdk releases tab](https://github.com/paritytech/polkadot-sdk/releases), or clone the [polkadot-sdk repo](https://github.com/paritytech/polkadot-sdk/) and compile the project with the command:
```sh
cargo build --release --locked -p polkadot-parachain-bin --bin polkadot-parachain
```

After that, declare a new environment variable, either in a terminal session or in your shell configuration file (e.g. `~/.bashrc`, `~/.zsh.rc`):
```sh
export ZOMBIENET_ASSETHUB_COMMAND=<path-to-polkadot-parachain-executable>
```

You're now ready to spawn the proper configuration of zombienet:
```sh
$ zombienet spawn zombienet/xcm-native.toml
```

#### CLI options
Please, refer to: https://paritytech.github.io/zombienet/cli/index.html.
