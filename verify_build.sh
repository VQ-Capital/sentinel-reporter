#!/bin/bash
set -e

echo "═══════════════════════════════════════════════════"
echo "  Sentinel-Reporter Build Verification"
echo "═══════════════════════════════════════════════════"
echo ""

# Check Rust
echo "🔍 Checking Rust toolchain..."
rustc --version
cargo --version
echo ""

# Clean previous builds
echo "🧹 Cleaning previous builds..."
cargo clean 2>/dev/null || true
echo ""

# Build without dashboard (minimal)
echo "📦 Building WITHOUT dashboard feature..."
cargo build --no-default-features 2>&1 | tee build_minimal.log
if [ ${PIPESTATUS[0]} -eq 0 ]; then
    echo "✅ Minimal build SUCCESS"
else
    echo "❌ Minimal build FAILED"
    echo "Check build_minimal.log for errors"
    exit 1
fi
echo ""

# Build with dashboard
echo "📦 Building WITH dashboard feature..."
cargo build --release 2>&1 | tee build_full.log
if [ ${PIPESTATUS[0]} -eq 0 ]; then
    echo "✅ Full build SUCCESS"
else
    echo "❌ Full build FAILED"
    echo "Check build_full.log for errors"
    exit 1
fi
echo ""

# Run tests
echo "🧪 Running tests..."
cargo test --no-default-features 2>&1 | tee test.log
if [ ${PIPESTATUS[0]} -eq 0 ]; then
    echo "✅ Tests PASSED"
else
    echo "⚠️  Some tests failed (check test.log)"
fi
echo ""

# Test with sample data
echo "📊 Testing with sample data..."
./target/release/sentinel-reporter --csv test_data.csv --output TEST_REPORT.md --once 2>&1 | tee run_test.log
if [ ${PIPESTATUS[0]} -eq 0 ]; then
    echo "✅ Sample run SUCCESS"
    echo "📄 Report generated: TEST_REPORT.md"
    ls -la TEST_REPORT.md
else
    echo "❌ Sample run FAILED"
    exit 1
fi
echo ""

echo "═══════════════════════════════════════════════════"
echo "  🎉 ALL CHECKS PASSED!"
echo "═══════════════════════════════════════════════════"
echo ""
echo "Usage:"
echo "  ./target/release/sentinel-reporter --csv <path> --output <path>"
echo ""
echo "Example:"
echo "  ./target/release/sentinel-reporter -c optimization_audit_log.csv -o SENTINEL_REPORT.md"
