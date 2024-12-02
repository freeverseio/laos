#!/bin/bash

# Default version and download directory
VERSION=${1:-"stable2407-3"}
DOWNLOAD_DIR=${2:-"./tmp"}  # Default is the current directory

BASE_URL="https://github.com/paritytech/polkadot-sdk/releases/download/polkadot-$VERSION"

# List of artifacts to download
ARTIFACTS=("polkadot" "polkadot-execute-worker" "polkadot-prepare-worker")

# Create the download directory if it doesn't exist
mkdir -p "$DOWNLOAD_DIR"

# Download each artifact, overwrite if it already exists, and set executable permission
for ARTIFACT in "${ARTIFACTS[@]}"; do
  FILE_PATH="$DOWNLOAD_DIR/$ARTIFACT"
  FILE_URL="$BASE_URL/$ARTIFACT"

  echo "Downloading $ARTIFACT from $FILE_URL..."
  curl -L -o "$FILE_PATH" "$FILE_URL"

  if [ $? -ne 0 ]; then
    echo "Failed to download $ARTIFACT."
    exit 1
  else
    echo "$ARTIFACT downloaded successfully!"
  fi

  # Set executable permissions
  chmod u+x "$FILE_PATH"
  if [ $? -eq 0 ]; then
    echo "Set executable permissions for $ARTIFACT."
  else
    echo "Failed to set executable permissions for $ARTIFACT."
    exit 1
  fi
done

echo "All downloads complete. Files are located in $DOWNLOAD_DIR"

echo ""
echo "To set the ZOMBIENET_RELAYCHAIN_COMMAND environment variable for the polkadot binary, run the following command:"
echo "export ZOMBIENET_RELAYCHAIN_COMMAND=\"$DOWNLOAD_DIR/polkadot\""
