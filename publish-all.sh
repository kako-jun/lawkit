#!/bin/bash
# 全クレートを正しい順序で公開するスクリプト

set -e

echo "🚀 Publishing all lawkit crates in correct dependency order..."

# 1. Core library (依存関係なし)
echo "📦 Publishing lawkit-core..."
cd lawkit-core
cargo publish
cd ..

echo "⏳ Waiting for crates.io index update..."
sleep 30

# 2. Main CLI (lawkit-coreに依存)
echo "📦 Publishing lawkit..."
cd lawkit-cli
cargo publish
cd ..

echo "⏳ Waiting for crates.io index update..."
sleep 30

# 3. Standalone wrappers (lawkitに依存)
echo "📦 Publishing benf standalone..."
cd benf-standalone
cargo publish
cd ..

echo "⏳ Waiting for crates.io index update..."
sleep 15

echo "📦 Publishing pareto standalone..."
cd pareto-standalone
cargo publish
cd ..

echo "✅ All crates published successfully!"
echo ""
echo "📋 Published crates:"
echo "   - lawkit-core (core library)"
echo "   - lawkit (main CLI toolkit)"
echo "   - benf (Benford's Law convenience command)"
echo "   - pareto (Pareto Principle convenience command)"
echo ""
echo "💡 Users can now install:"
echo "   cargo install lawkit          # Full toolkit"
echo "   cargo install benf            # Benford's Law only"
echo "   cargo install pareto          # Pareto analysis only"
echo "   cargo install lawkit benf pareto  # All together"