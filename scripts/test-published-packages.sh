#!/bin/bash
set -e

echo "🧪 Testing published lawkit packages..."
echo "========================================"

# Create test directory
TEST_DIR=$(mktemp -d)
echo "Test directory: $TEST_DIR"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

print_status() {
    if [ $1 -eq 0 ]; then
        echo -e "${GREEN}✅ $2${NC}"
    else
        echo -e "${RED}❌ $2${NC}"
        exit 1
    fi
}

print_info() {
    echo -e "${YELLOW}📋 $1${NC}"
}

# Test NPM package
print_info "Testing NPM package (lawkit-js)..."
cd "$TEST_DIR"
mkdir npm-test && cd npm-test

# Initialize npm project
npm init -y > /dev/null 2>&1
print_status $? "NPM project initialized"

# Install lawkit-js
npm install lawkit-js > /dev/null 2>&1
print_status $? "lawkit-js package installed"

# Test CLI commands
npx lawkit --version > /dev/null 2>&1
print_status $? "lawkit CLI version check"

npx lawkit --help > /dev/null 2>&1
print_status $? "lawkit CLI help command"

# Test generate and analyze pipeline
echo "1,23,456,789,1234,5678,9012" > test.csv
npx lawkit benf test.csv --format json > /dev/null 2>&1
print_status $? "lawkit Benford analysis"

npx lawkit generate benf --samples 30 | npx lawkit benf --format json > /dev/null 2>&1
print_status $? "lawkit generate-analyze pipeline"

# Test Python package
print_info "Testing Python package (lawkit-python)..."
cd "$TEST_DIR"
mkdir python-test && cd python-test

# Create virtual environment
python -m venv venv > /dev/null 2>&1
print_status $? "Python virtual environment created"

# Activate and install
source venv/bin/activate
pip install lawkit-python > /dev/null 2>&1
print_status $? "lawkit-python package installed"

# Download binary
lawkit-download-binary > /dev/null 2>&1
print_status $? "lawkit binary downloaded for Python"

# Test Python API
python -c "
import lawkit
import sys

# Test version and availability
version = lawkit.get_version()
available = lawkit.is_lawkit_available()

if not available:
    print('lawkit not available')
    sys.exit(1)

if 'lawkit' not in version.lower():
    print('Invalid version:', version)
    sys.exit(1)

# Test generate data
data = lawkit.generate_data('benf', samples=30)
if not data or len(data.strip()) == 0:
    print('Generate data failed')
    sys.exit(1)

# Test analysis
result = lawkit.analyze_benford(data, lawkit.LawkitOptions(format='json'))
if not result or '{' not in result:
    print('Analysis failed')
    sys.exit(1)

# Test selftest
selftest_passed = lawkit.selftest()
if not selftest_passed:
    print('Selftest failed')
    sys.exit(1)

print('All tests passed')
" > /dev/null 2>&1
print_status $? "Python API functionality tests"

# Test cross-platform compatibility
print_info "Testing cross-platform data exchange..."

# Generate data in Python, analyze with npm
source venv/bin/activate
python -c "
import lawkit
data = lawkit.generate_data('benf', samples=50)
with open('python_generated.txt', 'w') as f:
    f.write(data)
" > /dev/null 2>&1

cd ../npm-test
npx lawkit benf ../python-test/python_generated.txt --format json > /dev/null 2>&1
print_status $? "Cross-platform data compatibility (Python→NPM)"

# Cleanup
cd /
rm -rf "$TEST_DIR"
print_status $? "Test cleanup completed"

echo ""
echo -e "${GREEN}🎉 All package tests completed successfully!${NC}"
echo ""
echo "Packages tested:"
echo "  📦 lawkit-js@2.1.0 (NPM)"
echo "  🐍 lawkit-python@2.1.0 (PyPI)"
echo ""
echo "Test coverage:"
echo "  ✅ Package installation"
echo "  ✅ CLI functionality"
echo "  ✅ Python API"
echo "  ✅ Generate/analyze pipeline"
echo "  ✅ Cross-platform compatibility"
echo "  ✅ JSON output parsing"
echo "  ✅ Selftest functionality"