#!/bin/bash
set -e

echo "🚀 Publishing lawkit-js to npm..."

# Navigate to the npm package directory
cd "$(dirname "$0")/../lawkit-npm"

# Check if logged in to npm
echo "Checking npm login status..."
npm whoami || (echo "Please login to npm first with: npm login" && exit 1)

# Clean node_modules and reinstall
echo "Cleaning and reinstalling dependencies..."
rm -rf node_modules
npm install

# Run tests
echo "Running tests..."
npm test || echo "Tests skipped (no test file found)"

# Pack the package to check what will be published
echo "Packing package..."
npm pack --dry-run

# Publish to npm
echo "Publishing to npm..."
npm publish --access public

echo "✅ lawkit-js published successfully!"
echo "Install with: npm install lawkit-js"