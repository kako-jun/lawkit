#!/bin/bash

# Create comprehensive output samples for README

echo "=== Single law analysis - Benford's Law ==="
./target/release/lawkit benf test_data.txt 2>&1 | grep -v "proxychains" | head -6

echo ""
echo "=== Generate test data ==="
./target/release/lawkit generate pareto --samples 100 2>&1 | grep -v "proxychains" | head -5

echo ""
echo "=== Multi-law analysis with smart integration ==="
./target/release/lawkit analyze --laws all test_data.txt 2>&1 | grep -v "proxychains" | head -10

echo ""
echo "=== Stage 1: Basic multi-law analysis ==="
./target/release/lawkit analyze --laws all test_data.txt 2>&1 | grep -v "proxychains" | head -20

echo ""
echo "=== Stage 2: Data validation with consistency checks ==="
./target/release/lawkit validate --laws benf,pareto,normal test_data.txt --consistency-check 2>&1 | grep -v "proxychains" | head -25

echo ""
echo "=== Stage 3: Deep conflict analysis and recommendations ==="
./target/release/lawkit diagnose --laws all test_data.txt --report detailed 2>&1 | grep -v "proxychains" | head -30