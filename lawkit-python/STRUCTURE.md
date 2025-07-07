# lawkit-python Package Structure

This document describes the organization and structure of the lawkit-python package.

## Directory Structure

```
lawkit-python/
├── pyproject.toml              # Project configuration and dependencies
├── README.md                   # Package documentation and usage examples
├── STRUCTURE.md               # This file - package structure documentation
├── bin/                       # Directory for downloaded lawkit binaries
├── src/                       # Python source code
│   └── lawkit/                # Main package directory
│       ├── __init__.py        # Package initialization and exports
│       ├── lawkit.py          # Main wrapper implementation
│       ├── installer.py       # Binary download and installation
│       └── compat.py          # Backward compatibility functions
├── test_manual.py             # Manual testing script
└── verify_package.py          # Package verification script
```

## File Descriptions

### `pyproject.toml`
- Project configuration using modern Python packaging standards
- Metadata: version, description, author, license, keywords
- Dependencies and optional development dependencies
- Build system configuration (hatchling)
- Tool configuration (ruff, mypy)
- Entry points for command-line scripts

### `README.md`
- Comprehensive documentation with installation instructions
- Usage examples for all major features
- API reference and supported platforms
- Use cases and real-world examples

### `src/lawkit/__init__.py`
- Package initialization and public API exports
- Version information
- Import statements for all public functions and classes

### `src/lawkit/lawkit.py`
- Main wrapper implementation around the lawkit CLI
- Core analysis functions for all statistical laws:
  - `analyze_benford()` - Benford's Law analysis
  - `analyze_pareto()` - Pareto principle analysis
  - `analyze_zipf()` - Zipf's Law analysis
  - `analyze_normal()` - Normal distribution analysis
  - `analyze_poisson()` - Poisson distribution analysis
  - `compare_laws()` - Multi-law comparison
- Utility functions:
  - `generate_data()` - Sample data generation
  - `analyze_string()` - Direct string analysis
  - `is_lawkit_available()` - Binary availability check
  - `get_version()` - Version information
  - `selftest()` - Self-test functionality
- Classes:
  - `LawkitOptions` - Configuration options
  - `LawkitResult` - Analysis results
  - `LawkitError` - Error handling

### `src/lawkit/installer.py`
- Automatic binary download from GitHub Releases
- Platform detection (Windows, macOS, Linux)
- Architecture detection (x86_64, ARM64)
- Archive extraction (ZIP, TAR.GZ)
- Binary verification and permissions setup
- Command-line script entry point

### `src/lawkit/compat.py`
- Backward compatibility functions for legacy users
- `run_lawkit()` - Direct command execution
- `run_benford_analysis()` - Legacy Benford analysis
- `run_pareto_analysis()` - Legacy Pareto analysis
- `check_lawkit_installation()` - Installation verification
- `get_lawkit_help()` - Help text retrieval

### `bin/`
- Directory for downloaded lawkit binaries
- Platform-specific executable files
- Created automatically by the installer

## Package Features

### Multi-Platform Support
- Windows (x86_64)
- macOS (x86_64, ARM64/Apple Silicon)
- Linux (x86_64, ARM64)

### Statistical Laws
- **Benford's Law**: Fraud detection and anomaly analysis
- **Pareto Principle**: 80/20 rule analysis and concentration measurement
- **Zipf's Law**: Power-law distribution analysis
- **Normal Distribution**: Normality testing and outlier detection
- **Poisson Distribution**: Rare event analysis

### File Format Support
- CSV, JSON, YAML, TOML, XML
- Excel files (.xlsx, .xls)
- PDF documents
- Word documents (.docx)
- PowerPoint presentations (.pptx)

### Advanced Features
- International number format support
- Time series analysis
- Parallel processing
- Memory-efficient streaming
- Outlier detection algorithms
- Multi-law comparison and ranking

## API Design

### Modern API Pattern
```python
import lawkit

# Simple analysis
result = lawkit.analyze_benford('data.csv')

# Advanced analysis with options
result = lawkit.analyze_benford(
    'data.csv',
    lawkit.LawkitOptions(
        format='csv',
        output='json',
        verbose=True,
        optimize=True
    )
)

# Structured result access
print(f"Risk level: {result.risk_level}")
print(f"P-value: {result.p_value}")
```

### Legacy API Pattern
```python
from lawkit.compat import run_lawkit

# Direct command execution
result = run_lawkit(['benf', 'data.csv', '--format', 'csv'])
```

## Installation Methods

### Automatic (Recommended)
```bash
pip install lawkit-python
```

### Manual Binary Download
```bash
lawkit-download-binary
```

### Development Installation
```bash
git clone https://github.com/kako-jun/lawkit
cd lawkit/lawkit-python
pip install -e .[dev]
```

## Testing and Verification

### Package Structure Test
```bash
python verify_package.py
```

### Manual Functionality Test
```bash
python test_manual.py
```

### Import Test
```python
import lawkit
print(lawkit.__version__)
print(lawkit.is_lawkit_available())
```

## Distribution

The package is designed for distribution via:
- PyPI (Python Package Index)
- GitHub Releases
- Direct installation from source

## Dependencies

### Runtime Dependencies
- Python 3.8+
- No external dependencies (all included)

### Development Dependencies
- pytest >= 6.0
- pytest-cov
- pytest-asyncio
- black (code formatting)
- isort (import sorting)
- mypy (type checking)
- ruff (linting)
- types-requests (type hints)

## Version Synchronization

The package version (2.1.0) is synchronized with the main lawkit project to ensure compatibility between the Python wrapper and the underlying CLI tool.