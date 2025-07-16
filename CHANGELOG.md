# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.5.14] - 2025-07-16

### 🎯 Major Improvements

#### ✨ New Features
- **Graph Enhancement**: Added expected value bars to statistical analysis graphs for better visual comparison with theoretical values
- **Distribution Visualization**: Enhanced visual representation of statistical distributions with expected value indicators

#### 🐛 Bug Fixes
- **PyPI Publishing**: Completely resolved manylinux compatibility issues for Python wheel distribution
- **Package Distribution**: Fixed all Python package publishing errors across multiple platforms (Linux x86_64/aarch64, Windows, macOS x86_64/aarch64)
- **Module Structure**: Corrected Python module naming from `lawkit_python` to `lawkit` for consistency

#### 🛠️ Technical Improvements
- **GitHub Actions Optimization**: Implemented PyO3/maturin-action for automatic manylinux detection
- **Release Process**: Optimized execution order (Python → npm) for more reliable releases
- **Cross-Platform Support**: Ensured consistent wheel generation across all supported platforms
- **Error Handling**: Improved release failure recovery and retry mechanisms

#### 📦 Package Distribution
- **Multi-Platform Support**: Successfully published to all package managers
  - Rust (crates.io): ✅ Available
  - Python (PyPI): ✅ Available with proper manylinux compatibility
  - Node.js (npm): ✅ Available
  - Binary releases: ✅ Available for all platforms

### 🔄 Migration Notes

This release consolidates multiple failed release attempts (v2.5.5-v2.5.13) into a stable version. Users upgrading from v2.5.4 will see:
- Improved graph visualization with expected value bars
- More reliable package installation across all platforms
- Better error handling in statistical analysis tools

## [2.5.4] - 2025-07-15

### 🔧 GitHub Actions Reliability Fix

#### 🐛 Bug Fixes
- **Windows Build Reliability**: Corrected Windows PowerShell environment variable syntax in GitHub Actions
- **Cross-Platform Compatibility**: Separate environment variable setup for Windows (`$env:GITHUB_ENV`) and Unix (`$GITHUB_ENV`)
- **Artifact Upload Success**: Resolved Windows ZIP file naming issues that caused 404 errors during npm package publishing
- **Version Consistency**: Fixed npm package version synchronization across all release components

#### 🛠️ Technical Improvements
- **Automated Validation**: Enhanced pre-release testing to catch version mismatches before publication
- **Error Prevention**: Improved error handling in GitHub Actions workflows to prevent incomplete releases
- **Release Process**: Standardized version update procedures to maintain consistency across all platforms

#### 📦 Package Updates
- **Rust**: lawkit-core@2.5.4, lawkit@2.5.4 on crates.io
- **npm**: lawkit-js@2.5.4 with universal binary bundle
- **PyPI**: lawkit-python@2.5.4 with maturin-built wheel

#### 🎯 Impact
This release ensures reliable automated releases across all platforms, maintaining the complete statistical analysis toolkit functionality established in v2.4.1. The primary issue was environment variable handling in GitHub Actions workflows that caused empty `ARCHIVE_NAME` variables, leading to malformed file paths and failed releases.

## [2.4.1] - 2025-07-13

### 🎯 Complete Platform Integration & Enhanced Reliability

#### 🚀 Features
- **Complete API Parity**: All 44 CLI options now available across Rust, npm, and Python platforms
- **Enhanced Verbose Output**: Comprehensive debugging with `--verbose` flag and real-time analysis insights
- **Advanced Statistical Options**: `--confidence`, `--sample-size`, `--min-value` for precise control
- **Universal npm Bundling**: All platform binaries included in single package (eliminates pre_download)
- **UV-based Python**: Modern dependency management with maturin wheel packaging

#### 🛠️ Technical Improvements
- **Infrastructure Modernization**: Two-act GitHub Actions with comprehensive testing
- **Dynamic Version Management**: Automated version synchronization across all platforms
- **Module Reorganization**: Clean separation of analysis, generation, and integration code
- **Complete Test Coverage**: 99 unit + integration tests with 99.9% pass rate

#### 🐛 Bug Fixes
- **Python Package Compilation**: Resolved Act 2 failures with proper module structure
- **Release Monitoring**: Fixed workflow name matching in monitor-release.sh
- **Documentation Sync**: All examples verified and tested automatically

#### 📦 Package Updates
- **Rust**: lawkit-core@2.4.1, lawkit@2.4.1 on crates.io
- **npm**: lawkit-js@2.4.1 with universal binary bundle
- **PyPI**: lawkit-python@2.4.1 with maturin-built wheel

#### 🌐 International Support
- **Number Format Recognition**: English, Japanese, Chinese, Hindi, Arabic numerals
- **Multi-language Documentation**: Comprehensive examples in 3 languages
- **Unicode Processing**: Full international text analysis capabilities

## [2.1.0] - 2025-07-07

### 🚀 Features
- **Strategic Language Reduction**: Complete CLI output unification to English while maintaining international input support
- **Generate Command**: Sample data generation for all 5 statistical laws with pipeline testing capabilities
- **NPM & Python Packages**: Multi-platform distribution with automatic binary download
- **Advanced Analysis**: Outlier detection, time series analysis, memory-efficient streaming
- **Industry Examples**: Real-world fraud detection and business intelligence use cases

### 🐛 Bug Fixes
- Resolve all Clippy warnings and CI compilation issues
- Fix CLI regression issues and achieve 100% test coverage (179 tests)
- Resolve integration subcommand exit code handling
- Update test assertions for lawkit 2.1.0 compatibility

### 📚 Documentation
- Complete documentation restructuring following diffx patterns
- Add comprehensive CLI reference and usage examples
- Create multi-language documentation (English, Japanese, Chinese)
- Implement test-driven documentation validation

### 🚜 Refactor
- Modernize README.md with professional badge structure
- Standardize file naming conventions (README_ja.md, README_zh.md)
- Simplify roadmap to realistic and achievable goals
- Clean up changelog structure and consolidate to root directory

### 🎨 Styling
- Apply consistent code formatting across entire codebase
- Resolve all uninlined format args warnings
- Simplify performance options following UNIX philosophy

### 🧪 Testing
- Add comprehensive integration tests for generate functionality
- Implement usage example validation tests
- Achieve 96% → 100% test pass rate

### 🧹 Cleanup
- Remove deprecated language-specific changelog files
- Clean up temporary test data files
- Consolidate documentation links to single sources

### ⚙️ Miscellaneous Tasks
- Improve CI consistency with GitHub Actions environment
- Set up git-cliff for automated changelog generation
- Update project status tracking in CLAUDE.md

<!-- generated by git-cliff -->