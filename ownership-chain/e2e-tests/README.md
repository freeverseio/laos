# Functional testing for Ownership Chain Node RPC

This folder contains a set of functional tests designed to perform functional testing on the Frontier Eth RPC.

It is written in typescript, using Mocha/Chai as Test framework.

## Build the node for tests

```bash
cargo build --release -p laos-evolution
```

## Installation

```bash
yarn install
```

## Run the tests

```bash
yarn run build && yarn run test
```
