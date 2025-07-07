#!/bin/bash
set -e

echo "🚀 Publishing lawkit-python to PyPI..."

# Navigate to the Python package directory
cd "$(dirname "$0")/../lawkit-python"

# Create virtual environment if it doesn't exist
if [ ! -d "venv" ]; then
    echo "Creating virtual environment..."
    python -m venv venv
fi

# Activate virtual environment
source venv/bin/activate

# Install build tools
echo "Installing build tools..."
pip install --upgrade pip build twine

# Clean previous builds
echo "Cleaning previous builds..."
rm -rf dist/ build/ src/lawkit_python.egg-info/

# Build the package
echo "Building package..."
python -m build

# Check the package
echo "Checking package..."
python -m twine check dist/*

# Upload to PyPI
echo "Uploading to PyPI..."
echo "Note: You will be prompted for your PyPI username and password"
python -m twine upload dist/*

echo "✅ lawkit-python published successfully!"
echo "Install with: pip install lawkit-python"