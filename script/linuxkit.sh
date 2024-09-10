#!/bin/bash

# Define the URL and the destination directory and file
URL="https://github.com/aayushx402/linux-project/releases/download/v0.6.0/toolbox"
DEST_DIR="$HOME/linuxkit"
DEST_FILE="$DEST_DIR/toolbox"

# Create the new directory if it doesn't exist
mkdir -p "$DEST_DIR"

# Download the binary using wget
wget -O "$DEST_FILE" "$URL"

# Change to the destination directory
cd "$DEST_DIR"

# Make the binary executable
chmod +x "$DEST_FILE"

# Run the binary
"$DEST_FILE"

