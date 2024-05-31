#!/usr/bin/env bash

# This file is part of Substrate.
# Copyright (C) Parity Technologies (UK) Ltd.
# SPDX-License-Identifier: Apache-2.0
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
# http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

# This script has three parts which all use the Substrate runtime:
# - Pallet benchmarking to update the pallet weights
# - Machine benchmarking
#
# Should be run on a reference machine to gain accurate benchmarks
# current reference machine: https://github.com/paritytech/substrate/pull/5848

while getopts 'bfp:v' flag; do
  case "${flag}" in
    b)
      # Skip build.
      skip_build='true'
      ;;
    f)
      # Fail if any sub-command in a pipe fails, not just the last one.
      set -o pipefail
      # Fail on undeclared variables.
      set -u
      # Fail if any sub-command fails.
      set -e
      # Fail on traps.
      set -E
      ;;
    p)
      # Start at pallet
      start_pallet="${OPTARG}"
      ;;
    v)
      # Echo all executed commands.
      set -x
      ;;
    *)
      # Exit early.
      echo "Bad options. Check Script."
      exit 1
      ;;
  esac
done

if [ "$skip_build" != true ]
then
  echo "[+] Compiling Substrate benchmarks..."
  cargo build --locked --features=runtime-benchmarks --release --bin laos
fi

# The executable to use.
SUBSTRATE=./target/release/laos

# Load all pallet names in an array.
PALLETS=($(
  $SUBSTRATE benchmark pallet --list --chain=dev |\
    tail -n+2 |\
    cut -d',' -f1 |\
    sort |\
    uniq
))

echo "[+] Benchmarking ${#PALLETS[@]} Substrate pallets by excluding ${#EXCLUDED_PALLETS[@]} from ${#ALL_PALLETS[@]}."

# Define the error file.
ERR_FILE="benchmarking_errors.txt"
# Delete the error file before each run.
rm -f $ERR_FILE

# Benchmark each pallet.
for PALLET in "${PALLETS[@]}"; do
  # If `-p` is used, skip benchmarks until the start pallet.
  if [ ! -z "$start_pallet" ] && [ "$start_pallet" != "$PALLET" ]
  then
    echo "[+] Skipping ${PALLET}..."
    continue
  else
    unset start_pallet
  fi

  FOLDER="$(echo "${PALLET#*_}" | tr '_' '-')";
  WEIGHT_FILE="./runtime/laos/src/weights/"

  echo "[+] Benchmarking $PALLET with weight file $WEIGHT_FILE";

  # Check if the pallet is pallet_evm and set extrinsic accordingly
  if [ "$PALLET" == "pallet_evm" ]; then
    EXTRINSIC="withdraw"
  else
    EXTRINSIC="*"
  fi

  OUTPUT=$(
    $SUBSTRATE benchmark pallet \
    --chain=dev \
    --steps=50 \
    --repeat=20 \
    --pallet="$PALLET" \
    --extrinsic="$EXTRINSIC" \
    --wasm-execution=compiled \
    --output="$WEIGHT_FILE" 2>&1
  )
  if [ $? -ne 0 ]; then
    echo "$OUTPUT" >> "$ERR_FILE"
    echo "[-] Failed to benchmark $PALLET. Error written to $ERR_FILE; continuing..."
  fi
done

echo "[+] Benchmarking the machine..."
OUTPUT=$(
  $SUBSTRATE benchmark machine --chain=dev 2>&1
)
if [ $? -ne 0 ]; then
  # Do not write the error to the error file since it is not a benchmarking error.
  echo "[-] Failed the machine benchmark:\n$OUTPUT"
fi

# Check if the error file exists.
if [ -f "$ERR_FILE" ]; then
  echo "[-] Some benchmarks failed. See: $ERR_FILE"
  exit 1
else
  echo "[+] All benchmarks passed."
  exit 0
fi
