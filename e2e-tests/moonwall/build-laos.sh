#!/bin/bash

# Exit on any error
set -e

# # Always run the commands from the "test" dir
# cd $(dirname $0)/..

if [[ -f tmp/laos ]]; then
  LAOS_VERSION=$(tmp/laos --version)
  echo "LAOS binary already exists, version: $LAOS_VERSION"
echo "Exiting..."
if [[ -f ../laos/target/release/laos ]]; then
  LAOS_VERSION=$(../laos/target/release/laos --version)
  echo "LAOS binary already built, version: $LAOS_VERSION"
  echo "Copying..."
  cp ../laos/target/release/laos tmp/laos
else
  echo "LAOS binary not found, you need to build it first"
  exit 1
fi