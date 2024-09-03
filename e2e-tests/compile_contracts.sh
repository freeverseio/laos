#!/bin/bash

# we need to move .sol files under /contracts folder so that truffle works properly
cp -r ../pallets/laos-evolution/src/precompiles/evolution_collection_factory/contracts contracts
cp -r ../pallets/laos-evolution/src/precompiles/evolution_collection/contracts contracts
cp -r ../pallets/asset-metadata-extender/src/precompiles/asset_metadata_extender/contracts contracts
cp -r ../pallets/precompiles-benchmark/src/precompiles/vesting/contracts contracts
cp -r ../precompiles/parachain-staking contracts

truffle compile
rm -rf contracts
