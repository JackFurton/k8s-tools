#!/bin/bash
# Run all CI checks locally before pushing

set -e

echo "🔍 Running CI checks locally..."
echo ""

echo "1️⃣  Building..."
cargo build --release
echo "✅ Build passed"
echo ""

echo "2️⃣  Checking formatting..."
cargo fmt -- --check
echo "✅ Formatting passed"
echo ""

echo "3️⃣  Running clippy..."
cargo clippy -- -D warnings
echo "✅ Clippy passed"
echo ""

echo "4️⃣  Running tests..."
cargo test
echo "✅ Tests passed"
echo ""

echo "🎉 All checks passed! Safe to push."
