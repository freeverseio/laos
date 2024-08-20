#!/bin/bash

# Exit on any error
set -e

polkadot_release=v0.9.42

# # Always run the commands from the "test" dir
# cd $(dirname $0)/..

if [[ -f tmp/polkadot ]]; then
  POLKADOT_VERSION=$(tmp/polkadot --version)
  echo "Polkadot binary already exists, version: $POLKADOT_VERSION"
  echo "Exiting..."
else
  echo "Polkadot binary not found, downloading..."
  wget https://github.com/paritytech/polkadot-sdk/releases/download/polkadot-v$polkadot_release/polkadot -P tmp
  chmod +x tmp/polkadot
fi