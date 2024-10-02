#!/bin/bash

# Default version and download directory
VERSION=${1:-"v1.11.0"}
DOWNLOAD_DIR=${2:-"./tmp"}  # Default is tmp

BASE_URL="https://github.com/paritytech/polkadot-sdk/releases/download/polkadot-$VERSION"

# List of artifacts to download
ARTIFACTS=("polkadot" "polkadot-execute-worker" "polkadot-prepare-worker")

# Create the download directory if it doesn't exist
mkdir -p "$DOWNLOAD_DIR"

# Download each artifact if it doesn't exist
for ARTIFACT in "${ARTIFACTS[@]}"; do
  FILE_PATH="$DOWNLOAD_DIR/$ARTIFACT"

  if [ -f "$FILE_PATH" ]; then
    echo "$ARTIFACT already exists, skipping download."
  else
    FILE_URL="$BASE_URL/$ARTIFACT"
    echo "Downloading $ARTIFACT from $FILE_URL..."
    curl -L -o "$FILE_PATH" "$FILE_URL"

    if [ $? -eq 0 ]; then
      echo "$ARTIFACT downloaded successfully!"
      chmod u+x "$FILE_PATH"
    else
      echo "Failed to download $ARTIFACT."
    fi
  fi
done

echo "All downloads complete. Files are located in $DOWNLOAD_DIR"

# Set ZOMBIENET_RELAYCHAIN_COMMAND to the polkadot binary
POLKADOT_BINARY="$DOWNLOAD_DIR/polkadot"

if [ ! -f "$POLKADOT_BINARY" ]; then
  echo "Error: polkadot binary not found in $DOWNLOAD_DIR"
  exit 1
fi

echo ""
echo "To set the ZOMBIENET_RELAYCHAIN_COMMAND environment variable for the polkadot binary, run the following command:"
echo "export ZOMBIENET_RELAYCHAIN_COMMAND=\"$DOWNLOAD_DIR/polkadot\""