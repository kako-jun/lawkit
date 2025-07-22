# Installation Guide

This guide explains how to install lawkit on your system.

## System Requirements

- **OS**: Windows, macOS, Linux
- **Rust**: 1.70.0 or higher (when installing from Cargo)
- **Memory**: Minimum 512MB (2GB+ recommended for large file processing)

## Installation Methods

### 1. Install from Cargo (Recommended)

```bash
# Install latest version
cargo install lawkit

# Install specific version
cargo install lawkit --version 2.0.0
```

### 2. Build from Source

```bash
# Clone repository
git clone https://github.com/user/lawkit.git
cd lawkit

# Build and install
cargo build --release
cargo install --path .
```

### 3. Binary Download

Download the binary for your platform from [GitHub Releases](https://github.com/user/lawkit/releases).

#### Windows
```powershell
# Run in PowerShell
Invoke-WebRequest -Uri "https://github.com/user/lawkit/releases/latest/download/lawkit-windows.zip" -OutFile "lawkit.zip"
Expand-Archive lawkit.zip
```

#### macOS
```bash
# Install with Homebrew (planned)
brew install lawkit

# Or manual download
curl -L https://github.com/user/lawkit/releases/latest/download/lawkit-macos.tar.gz | tar xz
```

#### Linux
```bash
# Manual download
curl -L https://github.com/user/lawkit/releases/latest/download/lawkit-linux.tar.gz | tar xz
sudo mv lawkit /usr/local/bin/
```

## Verify Installation

Confirm that installation was successful:

```bash
# Check version
lawkit --version

# Display help
lawkit --help

# Show available statistical laws
lawkit list
```

## Migration from Legacy benf

If you're using the existing `benf` tool:

```bash
# Legacy usage
benf data.csv

# Equivalent new usage
lawkit benf data.csv
```

100% compatibility is maintained, so you can use existing scripts as-is.

## Updates

### If installed via Cargo
```bash
cargo install lawkit --force
```

### If installed via binary
Download and replace with the new binary.

## Uninstallation

### If installed via Cargo
```bash
cargo uninstall lawkit
```

### If installed manually
```bash
# Check binary location
which lawkit

# Remove file
sudo rm /usr/local/bin/lawkit
```

## Troubleshooting

### Compilation Errors
May be due to outdated Rust version:
```bash
rustup update stable
```

### Path Configuration
If binary is not in PATH:
```bash
# Add to ~/.bashrc or ~/.zshrc
export PATH="$HOME/.cargo/bin:$PATH"
```

### Permission Errors
If permission errors occur on Linux:
```bash
sudo chown $USER:$USER ~/.cargo/bin/lawkit
chmod +x ~/.cargo/bin/lawkit
```

## Next Steps

Once installation is complete, learn basic usage in [Getting Started](getting-started.md).

- [Getting Started](getting-started.md) - Basic usage
- [Examples](examples.md) - Real-world examples
- [CLI Reference](../reference/cli-reference.md) - Command details