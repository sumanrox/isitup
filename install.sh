#!/bin/bash

# Determine the system's architecture
ARCH=$(uname -m)

# Set the binary name based on the architecture
if [ "$ARCH" == "i686" ]; then
  BINARY_NAME="isitupx86"
elif [ "$ARCH" == "x86_64" ]; then
  BINARY_NAME="isitupx64"
else
  echo "Error: Unsupported architecture ($ARCH). Only x86 and x86-64 are supported."
  exit 1
fi

# Define the source and destination paths
SOURCE_BINARY="./bin/$BINARY_NAME"
DESTINATION_PATH="/usr/local/bin/isitup"

# Check if the binary exists
if [ -f "$SOURCE_BINARY" ]; then
  # Copy the binary to the installation directory
  sudo cp "$SOURCE_BINARY" "$DESTINATION_PATH"
  chmod +x "$DESTINATION_PATH"
  echo "Installation completed successfully."
  echo "Binary '$BINARY_NAME' installed as 'isitup' in $DESTINATION_PATH"
else
  echo "Error: The binary '$BINARY_NAME' does not exist in the 'bin' folder."
  exit 1
fi
