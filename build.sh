#!/bin/bash
set -e

echo "🔨 Sentinel-Reporter Build Script"
echo "=================================="

# Check Rust
cargo --version || { echo "❌ Rust not found!"; exit 1; }

# Build release
echo "📦 Building release binary..."
cargo build --release

echo ""
echo "✅ Build complete!"
echo ""
echo "Usage:"
echo "  ./target/release/sentinel-reporter --csv <path> --output <path>"
echo ""
echo "Example:"
echo "  ./target/release/sentinel-reporter -c ../sentinel-optimizer/optimization_audit_log.csv"
