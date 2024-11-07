#!/bin/bash

# we need to move .sol files under /contracts folder so that hardhat can find them
mkdir -p contracts
cp -r ../pallets/laos-evolution/src/precompiles/evolution_collection_factory/contracts/*.sol contracts
cp -r ../pallets/laos-evolution/src/precompiles/evolution_collection/contracts/*.sol contracts
cp -r ../pallets/asset-metadata-extender/src/precompiles/asset_metadata_extender/contracts/*.sol contracts
cp -r ../pallets/precompiles-benchmark/src/precompiles/vesting/contracts/*.sol contracts
cp -r ../precompiles/parachain-staking/*.sol contracts

hardhat compile
rm -rf contracts
