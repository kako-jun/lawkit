#!/bin/bash

# Create clean output samples for README

echo "=== Running benf command ==="
./target/release/lawkit benf test_data.txt 2>&1 | grep -v "proxychains" | head -10

echo ""
echo "=== Running pareto command ==="
./target/release/lawkit pareto test_data.txt 2>&1 | grep -v "proxychains" | head -10

echo ""
echo "=== Running validate command ==="
./target/release/lawkit validate --laws benf,pareto test_data.txt --consistency-check 2>&1 | grep -v "proxychains" | head -20

echo ""
echo "=== Running diagnose command ==="
./target/release/lawkit diagnose --laws benf,pareto test_data.txt --report detailed 2>&1 | grep -v "proxychains" | head -20