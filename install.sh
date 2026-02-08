#!/bin/bash
# Quick install script for k8s-tools

set -e

echo "Installing k8s-tools..."

# Check if cargo is installed
if ! command -v cargo &> /dev/null; then
    echo "Error: cargo not found. Please install Rust first:"
    echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

# Build
echo "Building..."
cargo build --release

# Install
echo "Installing to /usr/local/bin..."
sudo cp target/release/kdbg /usr/local/bin/
sudo cp target/release/kdash /usr/local/bin/

echo ""
echo "✓ Installation complete!"
echo ""
echo "Try it out:"
echo "  kdbg list"
echo "  kdash"
