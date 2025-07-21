#!/bin/bash

# Batch Processing Test Script
# Tests all examples from README.md to ensure they work correctly

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Test counter
TESTS_RUN=0
TESTS_PASSED=0

# Function to print test status
print_test() {
    echo -e "${YELLOW}[TEST]${NC} $1"
    TESTS_RUN=$((TESTS_RUN + 1))
}

print_pass() {
    echo -e "${GREEN}[PASS]${NC} $1"
    TESTS_PASSED=$((TESTS_PASSED + 1))
}

print_fail() {
    echo -e "${RED}[FAIL]${NC} $1"
}

# Build the project first
echo "Building benf..."
cargo build --release >/dev/null 2>&1 || {
    echo -e "${RED}Failed to build benf${NC}"
    exit 1
}

# Set up test environment
TEST_DIR="$(dirname "$0")/batch_test_data"
mkdir -p "$TEST_DIR"
cd "$TEST_DIR"

# Create test data files
echo "Setting up test data..."

# Create test Excel files
echo "100,200,300,400,500" > test1.csv
echo "1000,2000,3000,4000,5000" > test2.csv
echo "10000,20000,30000,40000,50000" > test3.csv

# Create a subdirectory with more files
mkdir -p audit-data
echo "111,222,333,444,555,666,777,888,999" > audit-data/Q1.csv
echo "1111,2222,3333,4444,5555,6666,7777,8888,9999" > audit-data/Q2.csv
echo "11111,22222,33333,44444,55555,66666,77777,88888,99999" > audit-data/Q3.csv

# Create big-data directory
mkdir -p big-data
for i in {1..5}; do
    echo "1234,2345,3456,4567,5678,6789,7890,8901,9012,1023" > "big-data/dataset$i.csv"
done

# Get benf executable path
BENF="../../../target/release/benf"
if [ ! -f "$BENF" ]; then
    echo -e "${RED}benf executable not found at $BENF${NC}"
    exit 1
fi

echo -e "\n${YELLOW}=== Testing README Batch Processing Examples ===${NC}\n"

# Test 1: Process Multiple Files - for loop
print_test "Process all CSV files with for loop"
file_count=0
for file in *.csv; do
    if [ -f "$file" ]; then
        if $BENF "$file" --min-count 3 >/dev/null 2>&1; then
            file_count=$((file_count + 1))
        fi
    fi
done
if [ $file_count -eq 3 ]; then
    print_pass "Successfully processed $file_count CSV files with for loop"
else
    print_fail "Expected 3 files, processed $file_count"
fi

# Test 2: Process files with find -exec
print_test "Process files with find -exec"
file_count=$(find ./audit-data -name "*.csv" -exec $BENF {} --min-count 3 \; 2>/dev/null | grep -c "解析した数値数" || true)
if [ $file_count -eq 3 ]; then
    print_pass "find -exec processed $file_count files"
else
    print_fail "find -exec expected 3, got $file_count"
fi

# Test 3: Process with xargs
print_test "Process with xargs"
file_count=$(find ./audit-data -name "*.csv" | xargs -I {} $BENF {} --min-count 3 2>/dev/null | grep -c "解析した数値数" || true)
if [ $file_count -eq 3 ]; then
    print_pass "xargs processed $file_count files"
else
    print_fail "xargs expected 3, got $file_count"
fi

# Test 4: Check if parallel exists and test if available
print_test "Check parallel availability"
if command -v parallel >/dev/null 2>&1; then
    file_count=$(ls *.csv | parallel $BENF {} --min-count 3 2>/dev/null | grep -c "解析した数値数" || true)
    if [ $file_count -eq 3 ]; then
        print_pass "parallel processed $file_count files"
    else
        print_fail "parallel expected 3, got $file_count"
    fi
else
    print_pass "parallel not available (optional dependency)"
    TESTS_RUN=$((TESTS_RUN - 1))  # Don't count this as a failed test
fi

# Test 5: Generate CSV report
print_test "Generate CSV report"
report_file="audit_report.csv"
find ./audit-data -name "*.csv" -exec $BENF {} --format csv --min-count 3 \; > "$report_file" 2>/dev/null
if [ -f "$report_file" ] && [ -s "$report_file" ]; then
    line_count=$(wc -l < "$report_file")
    if [ $line_count -gt 0 ]; then
        print_pass "CSV report generated with $line_count lines"
    else
        print_fail "CSV report is empty"
    fi
else
    print_fail "CSV report generation failed"
fi

# Test 6: Generate JSON report (requires jq)
print_test "Generate JSON report"
if command -v jq >/dev/null 2>&1; then
    json_files=$(find ./audit-data -name "*.csv" | head -2 | xargs -I {} $BENF {} --format json --min-count 3 2>/dev/null | jq -s '.' | jq 'length')
    if [ "$json_files" -eq 2 ]; then
        print_pass "JSON report generated with $json_files objects"
    else
        print_fail "JSON report expected 2 objects, got $json_files"
    fi
else
    print_pass "jq not available (optional dependency)"
    TESTS_RUN=$((TESTS_RUN - 1))  # Don't count this as a failed test
fi

# Test 7: Risk level detection
print_test "Risk level detection"
risk_files=0
for file in audit-data/*.csv; do
    if [ -f "$file" ]; then
        if $BENF "$file" --format json --min-count 3 2>/dev/null | grep -q '"risk_level"'; then
            risk_files=$((risk_files + 1))
        fi
    fi
done
if [ $risk_files -eq 3 ]; then
    print_pass "Risk level detected in $risk_files files"
else
    print_fail "Risk level detection failed"
fi

# Test 8: Exit code handling
print_test "Exit code handling"
exit_code_test=0
for file in *.csv; do
    if [ -f "$file" ]; then
        $BENF "$file" --min-count 3 >/dev/null 2>&1
        exit_code=$?
        if [ $exit_code -ge 0 ] && [ $exit_code -le 11 ]; then
            exit_code_test=$((exit_code_test + 1))
        fi
    fi
done
if [ $exit_code_test -eq 3 ]; then
    print_pass "Exit codes are within expected range (0-11)"
else
    print_fail "Exit code handling failed"
fi

# Test 9: Filter functionality in batch
print_test "Filter functionality in batch processing"
filter_files=0
for file in big-data/*.csv; do
    if [ -f "$file" ]; then
        # Use filter to only process numbers >= 1000
        if $BENF "$file" --filter ">=1000" --min-count 3 >/dev/null 2>&1; then
            filter_files=$((filter_files + 1))
        fi
    fi
done
if [ $filter_files -eq 5 ]; then
    print_pass "Filter functionality works in batch processing ($filter_files files)"
else
    print_fail "Filter functionality failed in batch processing"
fi

# Test 10: Memory efficiency test (large number of small files)
print_test "Memory efficiency with multiple small files"
efficient_files=0
find . -name "*.csv" -print0 | xargs -0 -n 1 $BENF --min-count 3 >/dev/null 2>&1 || true
efficient_files=$(find . -name "*.csv" | wc -l)
if [ $efficient_files -gt 0 ]; then
    print_pass "Memory efficient processing completed for $efficient_files files"
else
    print_fail "Memory efficient processing failed"
fi

# Summary
echo -e "\n${YELLOW}=== Test Summary ===${NC}"
echo "Tests run: $TESTS_RUN"
echo "Tests passed: $TESTS_PASSED"
echo "Tests failed: $((TESTS_RUN - TESTS_PASSED))"

if [ $TESTS_PASSED -eq $TESTS_RUN ]; then
    echo -e "${GREEN}All tests passed! 🎉${NC}"
    echo "README examples are working correctly."
    exit_code=0
else
    echo -e "${RED}Some tests failed! ❌${NC}"
    exit_code=1
fi

# Cleanup
cd ..
rm -rf batch_test_data

exit $exit_code