# Installation Guide

## Prerequisites

- **Rust 1.75+** (for building from source)
- **Operating System**: Linux, macOS, or Windows

## Installation Methods

### Method 1: Cargo Install (Recommended)

```bash
# Install latest stable version
cargo install lawkit

# Install specific version
cargo install lawkit --version 2.0.1
```

### Method 2: Download Pre-built Binaries

1. Go to [GitHub Releases](https://github.com/kako-jun/lawkit/releases)
2. Download the appropriate binary for your platform:
   - `lawkit-linux-x86_64.tar.gz` (Linux)
   - `lawkit-windows-x86_64.zip` (Windows)
   - `lawkit-macos-x86_64.tar.gz` (macOS Intel)
   - `lawkit-macos-aarch64.tar.gz` (macOS Apple Silicon)

3. Extract and add to PATH:

#### Linux/macOS
```bash
tar -xzf lawkit-linux-x86_64.tar.gz
sudo mv lawkit benf /usr/local/bin/
```

#### Windows
```powershell
# Extract lawkit-windows-x86_64.zip
# Add extracted directory to PATH
```

### Method 3: Build from Source

```bash
# Clone repository
git clone https://github.com/kako-jun/lawkit.git
cd lawkit

# Build release version
cargo build --release

# Install locally
cargo install --path ./lawkit-cli
```

## Verification

Test your installation:

```bash
# Check version
lawkit --version

# List available laws
lawkit list

# Run help
lawkit --help
```

## Troubleshooting

### Rust Installation Issues
If you don't have Rust installed:
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### Path Issues
Make sure the installation directory is in your PATH:
```bash
# Check if lawkit is in PATH
which lawkit

# Add to PATH (add to ~/.bashrc or ~/.zshrc)
export PATH="$HOME/.cargo/bin:$PATH"
```

### Platform-Specific Issues

#### Linux
- Ensure you have `build-essential` installed for building from source
- Some distributions may require additional packages

#### macOS
- Xcode Command Line Tools may be required: `xcode-select --install`
- On Apple Silicon, use the `aarch64` binary

#### Windows
- Use PowerShell or Command Prompt
- Windows Defender may flag the binary - add exception if needed

## Next Steps

- [Getting Started](getting-started.md) - Learn the basics
- [Getting Started](getting-started.md) - Learn the basics
- [Examples](examples.md) - See real-world usage