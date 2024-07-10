#!/bin/bash

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
cd toolbox/src

# Compile the main.rs file
rustc main.rs

# Run the compiled Rust program
./main

# Clean up
rm main