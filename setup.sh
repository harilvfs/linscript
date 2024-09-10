#!/bin/bash

# Define the URL for the binary
URL="https://github.com/aayushx402/linux-project/releases/download/v0.6.0/toolbox"

# Define the filename for the downloaded binary
FILENAME="toolbox"

# Download the binary
echo "Downloading $FILENAME..."
curl -L -o "$FILENAME" "$URL"

# Check if the download was successful
if [ $? -eq 0 ]; then
  echo "Download complete."
else
  echo "Failed to download $FILENAME."
  exit 1
fi

# Make the binary executable
echo "Setting executable permissions..."
chmod +x "$FILENAME"

# Run the binary
echo "Running $FILENAME..."
./"$FILENAME"

# Check if the binary ran successfully
if [ $? -eq 0 ]; then
  echo "$FILENAME executed successfully."
else
  echo "Failed to execute $FILENAME."
  exit 1
fi

