#!/bin/bash
set -e

echo "🚀 Publishing all lawkit packages..."
echo "===================================="

SCRIPT_DIR="$(dirname "$0")"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[1;34m'
NC='\033[0m' # No Color

print_step() {
    echo -e "${BLUE}📋 $1${NC}"
}

print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

# Function to ask for confirmation
confirm() {
    read -p "Do you want to continue? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "Aborted."
        exit 1
    fi
}

# Pre-flight checks
print_step "Running pre-flight checks..."

# Check npm login
if ! npm whoami >/dev/null 2>&1; then
    print_error "Not logged in to npm. Please run: npm login"
    exit 1
fi
print_success "npm login verified"

# Check if we're in a git repository with clean state
if [ -d ".git" ]; then
    if [ -n "$(git status --porcelain)" ]; then
        print_warning "Git working directory is not clean"
        git status --short
        confirm
    else
        print_success "Git working directory is clean"
    fi
fi

# Show what will be published
print_step "Packages to be published:"
echo "  📦 lawkit-js (npm)"
echo "  🐍 lawkit-python (PyPI)"
echo ""

confirm

# Publish npm package
print_step "Publishing npm package..."
if bash "$SCRIPT_DIR/publish-npm.sh"; then
    print_success "npm package published successfully"
else
    print_error "npm package publishing failed"
    exit 1
fi

echo ""

# Publish PyPI package
print_step "Publishing PyPI package..."
if bash "$SCRIPT_DIR/publish-pypi.sh"; then
    print_success "PyPI package published successfully"
else
    print_error "PyPI package publishing failed"
    exit 1
fi

echo ""
print_success "🎉 All packages published successfully!"
echo ""
echo "Published packages:"
echo "  📦 lawkit-js@$(cd lawkit-npm && node -p "require('./package.json').version") - https://www.npmjs.com/package/lawkit-js"
echo "  🐍 lawkit-python@$(cd lawkit-python && python -c "import tomllib; print(tomllib.load(open('pyproject.toml', 'rb'))['project']['version'])") - https://pypi.org/project/lawkit-python/"
echo ""
echo "Installation commands:"
echo "  npm install lawkit-js"
echo "  pip install lawkit-python"