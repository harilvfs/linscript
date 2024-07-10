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

# Create a temporary Rust file
cat << 'EOF' > temp.rs
fn main() {
    println!("Hello, world!");
}
EOF

# Compile the Rust file
rustc temp.rs

# Run the compiled Rust program
./temp

# Clean up
rm temp.rs temp
