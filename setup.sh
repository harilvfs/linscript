#!/bin/bash

# Ensure the script has execute permissions
chmod +x "$0"

# Ensure Rust is installed
if ! command -v rustc &> /dev/null
then
    echo "Rust is not installed. Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
else
    echo "Rust is already installed."
fi

# Navigate to the directory containing main.rs
cd src

# Compile the main.rs file
rustc main.rs

# Run the compiled Rust program
cargo run

# Clean up
rm main