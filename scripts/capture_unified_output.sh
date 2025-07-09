#!/bin/bash

# Create clean output samples for README with unified format

echo "=== Single law analysis - Benford Law ==="
./target/release/lawkit benf test_data.txt 2>&1 | grep -v "proxychains" | head -6

echo ""
echo "=== Single law analysis - Zipf Law ==="
./target/release/lawkit zipf test_data.txt 2>&1 | grep -v "proxychains" | head -6

echo ""
echo "=== Single law analysis - Pareto Principle ==="
./target/release/lawkit pareto test_data.txt 2>&1 | grep -v "proxychains" | head -6

echo ""
echo "=== Generate test data ==="
./target/release/lawkit generate pareto --samples 100 2>&1 | grep -v "proxychains" | head -5

echo ""
echo "=== Multi-law analysis with smart integration ==="
./target/release/lawkit analyze test_data.txt 2>&1 | grep -v "proxychains" | head -15

echo ""
echo "=== Stage 2: Data validation with consistency checks ==="
./target/release/lawkit validate --laws benf,pareto,normal test_data.txt --consistency-check 2>&1 | grep -v "proxychains" | head -25

echo ""
echo "=== Stage 3: Deep conflict analysis and recommendations ==="
./target/release/lawkit diagnose --laws benf,pareto test_data.txt --report detailed 2>&1 | grep -v "proxychains" | head -30