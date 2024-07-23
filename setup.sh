#!/bin/bash

# Check if the script has execute permissions
if [ ! -x "$0" ]; then
    echo "The script does not have execute permissions. Please run: chmod +x $0"
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

# Ensure Cargo is installed
if ! command -v cargo &> /dev/null; then
    echo "Cargo is not installed. Something went wrong with the Rust installation."
    exit 1
else
    echo "Cargo is already installed."
fi

# Navigate to the directory containing main.rs
cd src || { echo "src directory not found."; exit 1; }

# Ensure Cargo.toml exists for cargo to work
if [ ! -f Cargo.toml ]; then
    echo "Cargo.toml not found. Please ensure it exists in the src directory."
    exit 1
fi

# Run the Rust program using cargo
cargo run

# Clean up the target directory if needed (optional)
# rm -rf target

echo "Setup and execution complete."

