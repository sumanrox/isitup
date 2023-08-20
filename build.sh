#!/bin/bash

# Check for Rust and Cargo installation
if ! [ -x "$(command -v rustc)" ] || ! [ -x "$(command -v cargo)" ]; then
  echo "Error: Rust and Cargo are required to build this project."
  exit 1
fi

# Determine the system's architecture
ARCH=$(uname -m)

# Check if it's 32-bit (x86)
if [ "$ARCH" == "i686" ]; then
  # Check if the target is installed; if not, add it
  if ! rustup target list | grep -q "i686-unknown-linux-gnu"; then
    echo "Target 'i686-unknown-linux-gnu' not found. Adding it..."
    rustup target add i686-unknown-linux-gnu
  fi

  TARGET_ARCH="i686-unknown-linux-gnu"
  INSTALL_NAME="isitup"
# Check if it's 64-bit (x86-64)
elif [ "$ARCH" == "x86_64" ]; then
  TARGET_ARCH="x86_64-unknown-linux-gnu"
  INSTALL_NAME="isitup"
else
  echo "Error: Unsupported architecture ($ARCH). Only x86 and x86-64 are supported."
  exit 1
fi

echo "Building for $ARCH..."

# Build the binary for the detected architecture
cargo build --release --target="$TARGET_ARCH"
if [ $? -ne 0 ]; then
  echo "Error: Build for $ARCH failed."
  exit 1
fi

echo "Build for $ARCH completed successfully."

# Create installation directory (adjust as needed)
INSTALL_DIR="/usr/local/bin"

# Copy the binary to the installation directory
sudo cp "./target/$TARGET_ARCH/release/isitup" "$INSTALL_DIR/$INSTALL_NAME"
if [ $? -ne 0 ]; then
  echo "Error: Copying binary to $INSTALL_DIR failed."
  exit 1
fi

echo "Installation completed successfully."
echo "Binary installed as '$INSTALL_NAME' in $INSTALL_DIR"
