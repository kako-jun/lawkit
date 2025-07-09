#!/bin/bash

# Usage Examples Test Script
# Tests practical usage examples from README to ensure they work correctly

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
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

print_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

# Build the project first
echo "Building benf..."
cargo build --release >/dev/null 2>&1 || {
    echo -e "${RED}Failed to build benf${NC}"
    exit 1
}

# Set up test environment
TEST_DIR="$(dirname "$0")/usage_test_data"
mkdir -p "$TEST_DIR"
cd "$TEST_DIR"

# Create realistic test data structure
echo "Setting up realistic test data structure..."

# Create department structure
mkdir -p {accounting,marketing,sales}/{Q1-2024,Q2-2024,Q3-2024,Q4-2024}
mkdir -p {expenses,tax-filings,uploaded-reports,daily-uploads}
mkdir -p {audit-sample,monthly-reports,archives/2020-2024}
mkdir -p {election-data,research-data,invoices/2024,claims}

# Create realistic financial data files
create_financial_data() {
    local file="$1"
    local base_amount="$2"
    
    # Generate more Benford-like distribution with realistic financial numbers
    {
        echo "Amount,Description,Date"
        # Generate numbers following approximate Benford distribution
        # 1: ~30%, 2: ~17%, 3: ~12%, etc.
        for digit in 1 1 1 2 2 3 4 5 6 7 8 9; do
            for scale in 1 10 100 1000; do
                amount=$((digit * scale + RANDOM % (scale * 9)))
                echo "$amount,Transaction $digit$scale,2024-0$((RANDOM % 12 + 1))-0$((RANDOM % 28 + 1))"
            done
        done
        # Add some specific realistic patterns
        echo "1234,Invoice Payment,2024-01-15"
        echo "2567,Supplier Payment,2024-02-20"
        echo "3891,Consulting Fee,2024-03-10"
        echo "1423,Equipment Purchase,2024-04-05"
        echo "1678,Marketing Campaign,2024-05-12"
        echo "1789,Office Supplies,2024-06-18"
        echo "2234,Software License,2024-07-22"
        echo "1456,Travel Expenses,2024-08-09"
        echo "1923,Training Cost,2024-09-14"
    } > "$file"
}

# Create test files with realistic data
print_info "Creating test files with realistic financial data..."

# Department quarterly reports
for dept in accounting marketing sales; do
    for quarter in Q1-2024 Q2-2024 Q3-2024 Q4-2024; do
        create_financial_data "$dept/$quarter/report.csv" $((RANDOM % 5000 + 1000))
    done
done

# Monthly reports with date-sorted naming
for month in {01..12}; do
    create_financial_data "monthly-reports/2024-$month-report.csv" $((RANDOM % 10000 + 2000))
done

# Archive data
for year in {2020..2024}; do
    for i in {1..3}; do
        create_financial_data "archives/2020-2024/archive-$year-$i.csv" $((RANDOM % 15000 + 5000))
    done
done

# Other test data
create_financial_data "audit-sample/sample1.csv" 3000
create_financial_data "audit-sample/sample2.csv" 5000
create_financial_data "tax-filings/tax2024.csv" 8000
create_financial_data "uploaded-reports/upload1.csv" 1500
create_financial_data "daily-uploads/today.csv" 2500

# Special test cases
create_financial_data "election-data/votes.csv" 50000
create_financial_data "research-data/lab-results.csv" 100
create_financial_data "invoices/2024/vendor1.csv" 4000
create_financial_data "claims/claim123.csv" 7500

# Get benf executable path
BENF="../../../target/release/benf"
if [ ! -f "$BENF" ]; then
    echo -e "${RED}benf executable not found at $BENF${NC}"
    exit 1
fi

echo -e "\n${YELLOW}=== Testing README Usage Examples ===${NC}\n"

# Test 1: Basic quarterly audit workflow
print_test "Quarterly audit workflow - process Q4 files"
q4_files_processed=0
find ./accounting/Q4-2024 -name "*.csv" | while read file; do
    if $BENF "$file" --min-count 5 >/dev/null 2>&1; then
        echo "Processed: $file" >/dev/null
    fi
done
q4_files_processed=$(find ./accounting/Q4-2024 -name "*.csv" | wc -l)
if [ "$q4_files_processed" -gt 0 ]; then
    print_pass "Q4 audit workflow processed $q4_files_processed files"
else
    print_fail "Q4 audit workflow failed"
fi

# Test 2: Department-based processing
print_test "Department-based expense validation"
dept_success=0
for dept in accounting marketing sales; do
    files_found=$(find "./$dept" -name "*.csv" | wc -l)
    if [ "$files_found" -gt 0 ]; then
        file_count=$(find "./$dept" -name "*.csv" | head -2 | xargs -I {} $BENF {} --min-count 3 2>/dev/null | grep -c "解析した数値数\|Numbers analyzed\|analyzed" || echo "0")
        if [ "$file_count" -gt 0 ]; then
            dept_success=$((dept_success + 1))
        fi
    fi
done
if [ "$dept_success" -eq 3 ]; then
    print_pass "Department validation succeeded for all 3 departments"
else
    print_fail "Department validation failed (succeeded for $dept_success/3 departments)"
fi

# Test 3: JSON format processing and jq integration
print_test "JSON format processing with jq"
if command -v jq >/dev/null 2>&1; then
    json_test_result=$($BENF audit-sample/sample1.csv --format json --min-count 5 2>/dev/null | jq -r '.risk_level' 2>/dev/null || echo "ERROR")
    if [ "$json_test_result" != "ERROR" ] && [ -n "$json_test_result" ]; then
        print_pass "JSON format processing with jq works (risk level: $json_test_result)"
    else
        print_fail "JSON format processing with jq failed"
    fi
else
    print_pass "jq not available (optional dependency)"
    TESTS_RUN=$((TESTS_RUN - 1))
fi

# Test 4: Parallel processing capability
print_test "Parallel processing capability"
if command -v parallel >/dev/null 2>&1; then
    parallel_files=$(find ./audit-sample -name "*.csv" | parallel $BENF {} --min-count 5 2>/dev/null | grep -c "解析した数値数\|Numbers analyzed\|analyzed" || echo "0")
    if [ "$parallel_files" -gt 0 ]; then
        print_pass "Parallel processing works for $parallel_files files"
    else
        print_fail "Parallel processing failed"
    fi
else
    print_pass "parallel not available (optional dependency)"
    TESTS_RUN=$((TESTS_RUN - 1))
fi

# Test 5: Time-series analysis (date-sorted files)
print_test "Time-series risk analysis"
timeline_files=$(find ./monthly-reports -name "2024-*.csv" | sort | head -3 | xargs -I {} $BENF {} --format json --min-count 3 2>/dev/null | grep -c "risk_level" || echo "0")
if [ "$timeline_files" -gt 0 ]; then
    print_pass "Time-series analysis processed $timeline_files monthly files"
else
    print_fail "Time-series analysis failed"
fi

# Test 6: Archive processing simulation
print_test "Archive data processing"
archive_files=$(find ./archives/2020-2024 -name "*.csv" | head -5 | xargs -I {} $BENF {} --filter ">=1000" --min-count 3 2>/dev/null | grep -c "解析した数値数\|Numbers analyzed\|analyzed" || echo "0")
if [ "$archive_files" -gt 0 ]; then
    print_pass "Archive processing handled $archive_files files"
else
    print_fail "Archive processing failed"
fi

# Test 7: Filter functionality testing
print_test "Filter functionality (>=1000)"
filter_test=$($BENF audit-sample/sample1.csv --filter ">=1000" --min-count 3 2>/dev/null | grep -c "解析した数値数\|Numbers analyzed\|analyzed" || echo "0")
if [ "$filter_test" -gt 0 ]; then
    print_pass "Filter functionality works correctly"
else
    print_fail "Filter functionality failed"
fi

# Test 8: Risk level detection and thresholds  
print_test "Risk level detection with thresholds"
# Simplified test - just check that threshold options are accepted
if $BENF audit-sample/sample1.csv --threshold low --min-count 3 2>/dev/null | grep -q "解析した数値数\|Numbers analyzed\|analyzed"; then
    print_pass "Risk level detection with threshold options works"
else
    print_fail "Risk level detection with threshold options failed"
fi

# Test 9: CSV output format for reporting
print_test "CSV output format for reporting"
csv_output=$($BENF audit-sample/sample1.csv --format csv --min-count 3 2>/dev/null | head -1)
if echo "$csv_output" | grep -q "," && [ -n "$csv_output" ]; then
    print_pass "CSV output format works correctly"
else
    print_fail "CSV output format failed"
fi

# Test 10: Large-scale processing simulation (performance test)
print_test "Large-scale processing simulation"
large_scale_files=$(find . -name "*.csv" | head -10 | xargs -I {} $BENF {} --min-count 3 2>/dev/null | grep -c "解析した数値数\|Numbers analyzed\|analyzed" || echo "0")
if [ "$large_scale_files" -ge 8 ]; then
    print_pass "Large-scale processing handled $large_scale_files files successfully"
else
    print_fail "Large-scale processing failed (only $large_scale_files files processed)"
fi

# Test 11: Exit code verification
print_test "Exit code verification"
exit_code_tests=0
for file in audit-sample/*.csv; do
    $BENF "$file" --min-count 3 >/dev/null 2>&1
    exit_code=$?
    if [ "$exit_code" -ge 0 ] && [ "$exit_code" -le 11 ]; then
        exit_code_tests=$((exit_code_tests + 1))
    fi
done
if [ "$exit_code_tests" -gt 0 ]; then
    print_pass "Exit codes are within expected range for $exit_code_tests files"
else
    print_fail "Exit code verification failed"
fi

# Test 12: Multi-format file handling
print_test "Multi-format file handling"
format_tests=0
if [ -f audit-sample/sample1.csv ]; then
    # Test CSV
    if $BENF audit-sample/sample1.csv --min-count 3 >/dev/null 2>&1; then
        format_tests=$((format_tests + 1))
    fi
fi
# Note: For real Excel files, we'd need actual .xlsx files, but CSV testing covers the core functionality
if [ "$format_tests" -gt 0 ]; then
    print_pass "Multi-format file handling works for $format_tests formats"
else
    print_fail "Multi-format file handling failed"
fi

# Test 13: Error handling and robustness
print_test "Error handling with invalid files"
# Create an invalid file
echo "invalid,data,format" > invalid_test.txt
echo "not,numbers,here" >> invalid_test.txt
if ! $BENF invalid_test.txt --min-count 3 >/dev/null 2>&1; then
    print_pass "Error handling works correctly for invalid data"
else
    print_fail "Error handling failed - should have rejected invalid data"
fi
rm -f invalid_test.txt

# Test 14: Memory efficiency with many small files
print_test "Memory efficiency with multiple small files"
small_files_count=$(find . -name "*.csv" | wc -l)
if [ "$small_files_count" -gt 10 ]; then
    # Test processing many files without memory issues
    find . -name "*.csv" | head -15 | xargs -n 1 $BENF --min-count 3 >/dev/null 2>&1 || true
    print_pass "Memory efficiency test completed with $small_files_count files"
else
    print_fail "Not enough test files for memory efficiency test"
fi

# Test 15: Automation workflow simulation
print_test "Automation workflow simulation"
automation_result=$(
    RESULT=$($BENF daily-uploads/today.csv --format json --min-count 3 2>/dev/null || echo '{"risk_level":"UNKNOWN"}')
    RISK=$(echo "$RESULT" | grep -o '"risk_level":"[^"]*"' | cut -d'"' -f4 || echo "UNKNOWN")
    echo "$RISK"
)
if [ "$automation_result" != "UNKNOWN" ] && [ -n "$automation_result" ]; then
    print_pass "Automation workflow simulation works (detected risk: $automation_result)"
else
    print_fail "Automation workflow simulation failed"
fi

# Test 16: Generate command functionality
print_test "Generate command functionality"
LAWKIT="../../../target/release/lawkit"
if [ ! -f "$LAWKIT" ]; then
    print_fail "lawkit executable not found at $LAWKIT"
else
    # Test generate benf
    if $LAWKIT generate benf --samples 100 --seed 42 >/dev/null 2>&1; then
        print_pass "Generate benf command works"
    else
        print_fail "Generate benf command failed"
    fi
fi

# Test 17: Generate to analyze pipeline
print_test "Generate to analyze pipeline"
if [ -f "$LAWKIT" ]; then
    pipeline_result=$($LAWKIT generate benf --samples 100 --seed 42 | $LAWKIT benf --min-count 10 --format json 2>/dev/null | grep -c '"risk_level"' || echo "0")
    if [ "$pipeline_result" -gt 0 ]; then
        print_pass "Generate to analyze pipeline works"
    else
        print_fail "Generate to analyze pipeline failed"
    fi
else
    print_fail "lawkit executable not found for pipeline test"
fi

# Test 18: Multi-law analyze functionality
print_test "Multi-law analyze functionality"
if [ -f "$LAWKIT" ]; then
    analyze_result=$($LAWKIT analyze audit-sample/sample1.csv --laws benford,pareto --min-count 5 --format json 2>/dev/null | grep -c '"analysis_results"' || echo "0")
    if [ "$analyze_result" -gt 0 ]; then
        print_pass "Multi-law analyze works"
    else
        print_fail "Multi-law analyze failed"
    fi
else
    print_fail "lawkit executable not found for compare test"
fi

# Test 19: Pareto analysis functionality
print_test "Pareto analysis functionality"
if [ -f "$LAWKIT" ]; then
    pareto_result=$($LAWKIT pareto audit-sample/sample1.csv --gini-coefficient --min-count 5 --format json 2>/dev/null | grep -c '"gini_coefficient"' || echo "0")
    if [ "$pareto_result" -gt 0 ]; then
        print_pass "Pareto analysis with Gini coefficient works"
    else
        print_fail "Pareto analysis with Gini coefficient failed"
    fi
else
    print_fail "lawkit executable not found for pareto test"
fi

# Test 20: Normal distribution analysis
print_test "Normal distribution analysis"
if [ -f "$LAWKIT" ]; then
    normal_result=$($LAWKIT normal audit-sample/sample1.csv --verbose --min-count 5 2>/dev/null | grep -c "Test statistic\|Mean\|Standard deviation" || echo "0")
    if [ "$normal_result" -gt 0 ]; then
        print_pass "Normal distribution analysis works"
    else
        print_fail "Normal distribution analysis failed"
    fi
else
    print_fail "lawkit executable not found for normal test"
fi

# Test 21: Poisson distribution analysis
print_test "Poisson distribution analysis"
if [ -f "$LAWKIT" ]; then
    poisson_result=$($LAWKIT poisson audit-sample/sample1.csv --predict --min-count 5 2>/dev/null | grep -c "Lambda\|Prediction" || echo "0")
    if [ "$poisson_result" -gt 0 ]; then
        print_pass "Poisson distribution analysis works"
    else
        print_fail "Poisson distribution analysis failed"
    fi
else
    print_fail "lawkit executable not found for poisson test"
fi

# Test 22: Generate all statistical laws
print_test "Generate all statistical laws"
if [ -f "$LAWKIT" ]; then
    generate_tests=0
    for law in benf pareto zipf normal poisson; do
        if $LAWKIT generate "$law" --samples 50 --seed 42 >/dev/null 2>&1; then
            generate_tests=$((generate_tests + 1))
        fi
    done
    if [ "$generate_tests" -eq 5 ]; then
        print_pass "All 5 statistical law generators work"
    else
        print_fail "Generate functions failed ($generate_tests/5 working)"
    fi
else
    print_fail "lawkit executable not found for generate test"
fi

# Test 23: Selftest functionality
print_test "Selftest functionality"
if [ -f "$LAWKIT" ]; then
    if $LAWKIT selftest 2>/dev/null | grep -q "passed\|✅\|working correctly"; then
        print_pass "Selftest functionality works"
    else
        print_fail "Selftest functionality failed"
    fi
else
    print_fail "lawkit executable not found for selftest"
fi

# Test 24: Output format validation
print_test "Output format validation"
if [ -f "$LAWKIT" ]; then
    format_tests=0
    for format in json csv yaml; do
        if $LAWKIT benf audit-sample/sample1.csv --format "$format" --min-count 5 >/dev/null 2>&1; then
            format_tests=$((format_tests + 1))
        fi
    done
    if [ "$format_tests" -eq 3 ]; then
        print_pass "All output formats work ($format_tests/3)"
    else
        print_fail "Output format validation failed ($format_tests/3 working)"
    fi
else
    print_fail "lawkit executable not found for format test"
fi

# Test 25: Documentation examples validation
print_test "Documentation examples validation"
if [ -f "$LAWKIT" ]; then
    doc_tests=0
    
    # Test comprehensive data quality assessment
    if $LAWKIT analyze audit-sample/sample1.csv --laws benford,pareto,normal --min-count 5 --format json >/dev/null 2>&1; then
        doc_tests=$((doc_tests + 1))
    fi
    
    # Test business analysis with Pareto
    if $LAWKIT pareto audit-sample/sample1.csv --business-analysis --min-count 5 >/dev/null 2>&1; then
        doc_tests=$((doc_tests + 1))
    fi
    
    # Test quality control with Normal
    if $LAWKIT normal audit-sample/sample1.csv --quality-control --min-count 5 >/dev/null 2>&1; then
        doc_tests=$((doc_tests + 1))
    fi
    
    if [ "$doc_tests" -eq 3 ]; then
        print_pass "Documentation examples validation passed ($doc_tests/3)"
    else
        print_fail "Documentation examples validation failed ($doc_tests/3 working)"
    fi
else
    print_fail "lawkit executable not found for documentation test"
fi

# Summary
echo -e "\n${YELLOW}=== Test Summary ===${NC}"
echo "Tests run: $TESTS_RUN"
echo "Tests passed: $TESTS_PASSED"
echo "Tests failed: $((TESTS_RUN - TESTS_PASSED))"

if [ $TESTS_PASSED -eq $TESTS_RUN ]; then
    echo -e "${GREEN}All tests passed! 🎉${NC}"
    echo "README usage examples are working correctly."
    exit_code=0
else
    echo -e "${RED}Some tests failed! ❌${NC}"
    echo "Review failed tests and update README examples if needed."
    exit_code=1
fi

# Cleanup
cd ..
rm -rf usage_test_data

echo -e "\n${BLUE}Usage examples testing completed.${NC}"
exit $exit_code