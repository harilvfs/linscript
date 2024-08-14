#!/bin/bash

# Ensure the script is run as root (with sudo)
if [ "$EUID" -ne 0 ]; then
    echo "This script requires root privileges. Please run it with sudo."
    exec sudo "$0" "$@"
    exit 1
fi

# Ensure Rust is installed
if ! command -v rustc &> /dev/null; then
    echo "Rust is not installed. Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
else
    echo "Rust is already installed."
fi

# Navigate to the project root directory
cd "$(dirname "$0")" || { echo "Failed to navigate to script directory."; exit 1; }

# Ensure Cargo.toml exists in the root directory
if [ ! -f Cargo.toml ]; then
    echo "Cargo.toml not found. Please ensure it exists in the project root directory."
    exit 1
fi

# Ensure the src directory exists
if [ ! -d src ]; then
    echo "src directory not found. Please ensure it exists in the project root directory."
    exit 1
fi

# Compile in the background silently and run the binary
(cargo build --release &> /dev/null) &
echo "Setting up, please wait..."
wait

# Run the compiled binary
./target/release/toolbox

# Clean up the target directory if needed (optional)
# rm -rf target

echo "Setup and execution complete."


