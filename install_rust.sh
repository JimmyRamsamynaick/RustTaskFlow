#!/bin/bash

# RustTaskFlow - Rust Installation Script
# This script installs Rust and Cargo on your system

echo "🦀 Installing Rust for RustTaskFlow..."
echo "This will install Rust and Cargo on your system."
echo ""

# Check if Rust is already installed
if command -v rustc &> /dev/null && command -v cargo &> /dev/null; then
    echo "✅ Rust is already installed!"
    echo "Rust version: $(rustc --version)"
    echo "Cargo version: $(cargo --version)"
    echo ""
    echo "You can now build RustTaskFlow with:"
    echo "  cargo build --release"
    echo "  cargo install --path ."
    exit 0
fi

echo "📥 Downloading and installing Rust..."
echo "This will download and run the official Rust installer."
echo ""

# Download and install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Source the cargo environment
source ~/.cargo/env

echo ""
echo "✅ Rust installation completed!"
echo "Rust version: $(rustc --version)"
echo "Cargo version: $(cargo --version)"
echo ""
echo "🚀 You can now build RustTaskFlow with:"
echo "  cargo build --release"
echo "  cargo install --path ."
echo ""
echo "💡 To use Rust in new terminal sessions, either:"
echo "  - Restart your terminal, or"
echo "  - Run: source ~/.cargo/env"
echo ""
echo "📚 Learn more about Rust at: https://www.rust-lang.org/"